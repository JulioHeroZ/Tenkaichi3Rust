use bt3_original::{host::resource_transport,rt::Ctx};

#[test]
fn prepared_resource_preserves_the_decoder_memory_contract() {
    // Cross two legacy BPE block boundaries, with only the handle in game RAM.
    let payload:Vec<u8>=(0..23117).map(|i|((i*37+i/251)&255) as u8).collect();
    let token=resource_transport::retain(payload.clone()).unwrap();
    assert_eq!(resource_transport::retain(payload.clone()).unwrap(),token,
        "loading identical bytes again must reuse the same handle");
    let mut c=Ctx::new();
    let src=0x0101_0000;
    let dst=0x0110_0000;
    let size=0x0120_0000;
    c.m.write_bytes(src,&token);
    c.m.w8(dst-1,0xa5);
    c.m.w8(dst+payload.len() as u32,0x5a);
    c.m.w32(size,0xdead_beef);
    let result=resource_transport::Bpe_Decode(&mut c,src,dst,size);
    let mut output=vec![0;payload.len()];c.m.read_bytes(dst,&mut output);
    assert_eq!(result,dst);
    assert_eq!(output,payload);
    assert_eq!(c.m.r32(size),payload.len() as u32);
    assert_eq!(c.m.r8(dst-1),0xa5);
    assert_eq!(c.m.r8(dst+payload.len() as u32),0x5a);
    let mut retained=vec![0;token.len()];c.m.read_bytes(src,&mut retained);
    assert_eq!(retained,token,"the handle must remain reusable");
    // A second use without a size output still copies the same resource.
    c.m.w8(dst,0);
    assert_eq!(resource_transport::Bpe_Decode(&mut c,src,dst,0),dst);
    c.m.read_bytes(dst,&mut output);
    assert_eq!(output,payload);
    assert_eq!(c.m.r32(size),payload.len() as u32);
}
