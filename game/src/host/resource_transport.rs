//! In-process transport for already prepared PC resources. These handles are
//! ephemeral, not a distributable asset format and not a native renderer.
use crate::rt::Ctx;
use std::sync::{Arc,Mutex};
use std::collections::{HashMap,hash_map::DefaultHasher};
use std::hash::{Hash,Hasher};

const MAGIC:u32=u32::from_le_bytes(*b"B3PC");
#[derive(Default)]
struct Payloads {
    bytes: Vec<Arc<Vec<u8>>>,
    by_hash: HashMap<u64, Vec<u32>>,
}
static PAYLOADS:Mutex<Option<Payloads>>=Mutex::new(None);

impl Payloads {
    fn retain(&mut self, bytes: Vec<u8>, hash: u64) -> Result<u32,String> {
        // Hashes only narrow the search. Compare bytes so collisions cannot
        // substitute a different model, texture or animation.
        if let Some(ids)=self.by_hash.get(&hash) {
            for &id in ids {
                if self.bytes[id as usize].as_slice()==bytes.as_slice() {return Ok(id);}
            }
        }
        let id=u32::try_from(self.bytes.len()).map_err(|_|"too many PC resources")?;
        self.bytes.push(Arc::new(bytes));
        self.by_hash.entry(hash).or_default().push(id);
        Ok(id)
    }
}

pub fn enabled()->bool {
    std::env::var_os("BT3_PC_ONLY").is_some() && std::env::var_os("BT3_VERIFY_FILES").is_none()
}

pub fn retain(bytes:Vec<u8>)->Result<Vec<u8>,String> {
    let size=u32::try_from(bytes.len()).map_err(|_|"PC resource too large")?;
    if size>i32::MAX as u32 {return Err("PC resource exceeds translated allocator size range".into());}
    let mut hasher=DefaultHasher::new();
    bytes.hash(&mut hasher);
    let mut payloads=PAYLOADS.lock().unwrap();
    let id=payloads.get_or_insert_with(Payloads::default).retain(bytes,hasher.finish())?;
    let mut token=Vec::with_capacity(16);
    for word in [size,MAGIC,id,0] {token.extend_from_slice(&word.to_le_bytes());}
    Ok(token)
}

pub fn Bpe_Decode(c:&mut Ctx,src:u32,dst:u32,raw_size:u32)->u32 {
    use crate::gen::funcs as F;
    if c.m.r32(src+4)!=MAGIC {
        if std::env::var_os("BT3_NO_BPE").is_some() {
            panic!("bt3: legacy BPE decode attempted at 0x{src:08x}");
        }
        return F::__real_Bpe_Decode(c,src,dst,raw_size);
    }
    let bytes=PAYLOADS.lock().unwrap().as_ref()
        .and_then(|p|p.bytes.get(c.m.r32(src+8) as usize)).cloned()
        .unwrap_or_else(||panic!("bt3: invalid PC resource handle at 0x{src:08x}"));
    if c.m.r32(src)!=bytes.len() as u32 || c.m.r32(src+12)!=0 {
        panic!("bt3: PC resource handle size/version mismatch");
    }
    if raw_size!=0 {c.m.w32(raw_size,bytes.len() as u32);}
    let out=if dst==0 {F::Heap_Alloc(c,bytes.len() as i32,32,0,2)}else{dst};
    c.m.write_bytes(out,&bytes);
    if std::env::var_os("BT3_TRANSPORT_VERBOSE").is_some() {
        eprintln!("bt3: PC resource copied: {} bytes, no BPE",bytes.len());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::Payloads;

    #[test]
    fn repeated_loads_reuse_storage_and_collisions_preserve_old_handles() {
        let mut p=Payloads::default();
        let a=vec![1,2,3];
        let b=vec![3,2,1];
        let first=p.retain(a.clone(),7).unwrap();
        for _ in 0..100 {assert_eq!(p.retain(a.clone(),7).unwrap(),first);}
        let second=p.retain(b.clone(),7).unwrap();
        assert_ne!(first,second);
        assert_eq!(p.retain(b.clone(),7).unwrap(),second);
        assert_eq!(p.bytes.len(),2);
        assert_eq!(*p.bytes[first as usize],a);
        assert_eq!(*p.bytes[second as usize],b);
    }
}
