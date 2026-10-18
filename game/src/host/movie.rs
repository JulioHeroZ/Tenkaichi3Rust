//! The game's opening and ending movies on the PC: the MPEG-2 video of the disc (assets/video/<name>.m2v,
//! tools/assets/pss_video.py) decoded in Rust (host/mpeg2.rs). The audio player, GS upload chains, pad handling and
//! fade rules of the game stay in use; the game receives the pictures in its column-major 16x16 blocks.
use crate::rt::prelude::*;
use std::io;
use std::path::Path;

const W: usize = 512;
const H: usize = 448;

pub struct Decoder {
    video: super::mpeg2::Decoder,
    rgba: Vec<u8>,
    blocks: Vec<u8>,
    ready: bool,
    ended: bool,
    frames: u32,
}

impl Decoder {
    fn open(path: &Path) -> io::Result<Self> {
        let data = super::vfs::read(path)?;
        let video = super::mpeg2::Decoder::new(data).map_err(io::Error::other)?;
        if video.width != W || video.height != H {
            return Err(io::Error::other(format!("video is {}x{}, the game shows {W}x{H}", video.width, video.height)));
        }
        let mut decoder = Self { video, rgba: vec![0; W * H * 4], blocks: vec![0; W * H * 4], ready: false, ended: false, frames: 0 };
        if !decoder.prepare()? {
            return Err(io::Error::other("video contains no pictures"));
        }
        Ok(decoder)
    }

    fn prepare(&mut self) -> io::Result<bool> {
        if self.ready {
            return Ok(true);
        }
        if self.ended {
            return Ok(false);
        }
        if !self.video.next_picture().map_err(io::Error::other)? {
            self.ended = true;
            return Ok(false);
        }
        self.video.rgba(&mut self.rgba);
        macroblocks(&self.rgba, &mut self.blocks);
        self.ready = true;
        Ok(true)
    }
}

fn macroblocks(rgba:&[u8],blocks:&mut [u8]) {
    for x in 0..W/16 {for y in 0..H/16 {for row in 0..16 {
        let src=((y*16+row)*W+x*16)*4;
        let dst=((x*(H/16)+y)*256+row*16)*4;
        blocks[dst..dst+64].copy_from_slice(&rgba[src..src+64]);
        // libmpeg's RGB32 uses GS alpha (128 is opaque).
        for a in (dst+3..dst+64).step_by(4) {blocks[a]=128;}
    }}}
}

pub fn Movie_Open(c:&mut Ctx,pss:u32,adx:u32) {
    c.host.movie.take();
    let name=c.m.cstr(pss);
    let stem=String::from_utf8_lossy(&name).split(';').next().unwrap_or("")
        .replace('\\',"/").rsplit('/').next().unwrap_or("")
        .to_ascii_lowercase().trim_end_matches(".pss").to_owned();
    let path=super::files::assets_root(c).join("video").join(format!("{stem}.m2v"));
    match Decoder::open(&path) {
        Ok(decoder)=>{
            eprintln!("bt3: movie {}: MPEG-2, 512x448, PC audio stream",path.display());
            c.host.movie=Some(decoder);
            // Only the display buffers are needed by the PC decoder. No ADXF file,
            // PS2 MPEG/IPU object or disc read-ahead buffers are opened or allocated.
            F::Adx_PlayFilePaused(c,0,adx,64,0);
            let m=F::Heap_Alloc(c,248,32,0,2);
            F::memset(c,m,0,248);
            c.m.w32(sym::gMovie,m);
            let audio=c.m.r32(sym::gAdxPlayerTbl+16);
            c.m.w32(m+4,audio);
            c.m.w32(m+212,(W*H*4) as u32);
            c.m.w32(m+224,86272);
            let rgb=F::Heap_Alloc(c,(W*H*4) as i32,64,0,2);
            c.m.w32(m+208,rgb);
            for i in 0..2 {
                let tag=F::Heap_Alloc(c,86272,64,0,2);
                c.m.w32(m+216+i*4,tag);
                F::MovieTag_BuildImage(c,tag,rgb,0,if i==0 {0}else{448},512,448);
            }
        },
        Err(e)=>eprintln!("bt3: cannot play movie {}: {e}",path.display()),
    }
}

pub fn Movie_Close(c:&mut Ctx) {
    c.host.movie.take();
    let m=c.m.r32(sym::gMovie);
    if m!=0 {
        F::Adx_Stop(c,0);
        for offset in [208,216,220] {
            let buffer=c.m.r32(m+offset);
            F::Heap_Free(c,buffer);
        }
        F::Heap_Free(c,m);
        c.m.w32(sym::gMovie,0);
    }
}

pub fn Movie_Run(c:&mut Ctx) {
    let movie=c.m.r32(sym::gMovie);
    if movie==0 || c.host.movie.is_none() {return;}
    c.m.w32(sym::gMovieRunCount,c.m.r32(sym::gMovieRunCount).wrapping_add(1));
    F::sceGsSyncV(c,0);
    c.m.w32(movie+8,1); // audioOn
    F::Adx_Resume(c,0);
    let mut skip=false;
    let mut wait=0;
    loop {
        let available=c.host.movie.as_mut().unwrap().prepare();
        match available {
            Ok(true)=>{}, Ok(false)=>break,
            Err(e)=>{eprintln!("bt3: movie decoder: {e}");break;},
        }
        F::Gfx_BeginFrame(c);
        F::Pad_Update(c);
        if skip {
            if F::Fade_IsDone(c,0)!=0 {
                if wait>=3 {break;}
                wait+=1;
            }
        } else if c.m.r32(sym::gPad+0x18c)&0x1000!=0 {
            F::Fade_Start(c,0,0,F32(0.5f32.to_bits()));skip=true;
        }
        let rgb=c.m.r32(movie+0xd0);
        {
            let decoder=c.host.movie.as_mut().unwrap();
            c.m.write_bytes(rgb,&decoder.blocks);
            decoder.ready=false;decoder.frames+=1;
        }
        F::sceGsSyncPath(c,0,0);
        // include/sys/gfx.h: Gfx.field is s32 at +0x230.
        let field=c.m.r32(sym::gGfx+0x230);
        let tag=c.m.r32(movie+if field!=0 {0xd8}else{0xdc});
        F::MovieTag_Send(c,tag);
        F::sceGsSyncPath(c,0,0);
        F::Dma_Flush(c);
        F::Gfx_EndFrame(c,2);
    }
    let frames=c.host.movie.as_ref().map_or(0,|d|d.frames);
    eprintln!("bt3: movie finished: {frames} frames, skipped={skip}");
    c.host.movie.take();
    F::Adx_Stop(c,0);
    F::Fade_Start(c,0,0,F32(0));
    for _ in 0..2 {F::Gfx_BeginFrame(c);F::Dma_Flush(c);F::Gfx_EndFrame(c,1);}
    F::Fade_Reset(c,0);
}
