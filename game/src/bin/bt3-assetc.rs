//! Offline resource compiler using exactly the game's native asset readers.
use std::path::Path;

fn compile(source:&str,target:&str)->Result<(),String> {
    let bytes=bt3_original::host::assets::assemble(Path::new(source))
        .map_err(|e|format!("{source}: {e}"))?;
    let target=Path::new(target);
    if let Some(parent)=target.parent().filter(|p|!p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e|e.to_string())?;
    }
    std::fs::write(target,&bytes).map_err(|e|e.to_string())?;
    Ok(())
}

fn run()->Result<(),String> {
    if bt3_original::host::resource_transport::enabled() {
        return Err("unset BT3_PC_ONLY for offline compilation: in-process resource handles cannot be saved as asset files".into());
    }
    let args:Vec<String>=std::env::args().skip(1).collect();
    match args.as_slice() {
        [flag,root] if flag=="--verify-motion-sources"=>{
            let (banks,motions,empty)=bt3_original::host::assets::verify_motion_sources(Path::new(root))?;
            println!("bt3-assetc: verified {motions} typed motions and {empty} empty entries in {banks} banks; 0 byte differences");Ok(())
        },
        [flag,list] if flag=="--batch"=>{
            let text=std::fs::read_to_string(list).map_err(|e|e.to_string())?;
            let mut count=0;
            for (line_no,line) in text.lines().enumerate() {
                if line.is_empty() {continue;}
                let (source,target)=line.split_once('\t').ok_or_else(||format!("{list}:{}: expected source TAB output",line_no+1))?;
                compile(source,target)?;count+=1;
            }
            println!("bt3-assetc: compiled {count} resources");Ok(())
        },
        [source,target]=>{compile(source,target)?;println!("bt3-assetc: compiled {source}");Ok(())},
        _=>Err("usage: bt3-assetc <resource-directory> <output> | --batch <TSV-list> | --verify-motion-sources <resource-root>".into()),
    }
}

fn main() {
    if let Err(error)=run() {eprintln!("bt3-assetc: {error}");std::process::exit(1);}
}
