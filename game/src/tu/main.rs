#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "main", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Game_Main(c);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "main", &PROBE_N, &[], true); }
    r_
}

pub fn Overlay_Load(c: &mut Ctx, mut p_idx_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Overlay_Load", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_fd_2: i32 = 0;
    let mut l_size_3: i32 = 0;
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            l_fd_2 = { let a0_ = c.m.r32(((sym::gOverlayTbl).wrapping_add((((p_idx_1) as u32)).wrapping_mul(8))).wrapping_add(4)); F::sceOpen(c, a0_, 1i32) };
        }
        if !(((((l_fd_2) < (0i32)) as i32)) != 0) { break 'l1; }
    }
    l_size_3 = F::sceLseek(c, l_fd_2, 0i32, 2i32);
    { let _ = F::sceLseek(c, l_fd_2, 0i32, 0i32); };
    F::FlushCache(c, 0i32);
    'l3: while (((((F::sceRead(c, l_fd_2, sym::MainMenu_StartBallLoops, l_size_3)) != (l_size_3)) as i32)) != 0) {
    }
    F::FlushCache(c, 2i32);
    { let _ = F::sceClose(c, l_fd_2); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Overlay_Load", &PROBE_N, &[], true); }
    r_
}

pub fn Sys_RebootIop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sys_RebootIop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::sceSifInitRpc(c, ((0i32) as u32));
    { let _ = F::sceCdInit(c, 0i32); };
    { let _ = F::sceCdMmode(c, 2i32); };
    'l1: while ((((!((F::sceSifRebootIop(c, sym::main__str0)) != 0)) as i32)) != 0) {
    }
    'l2: while ((((!((F::sceSifSyncIop(c)) != 0)) as i32)) != 0) {
    }
    F::sceSifInitRpc(c, ((0i32) as u32));
    { let _ = F::sceCdInit(c, 0i32); };
    { let _ = F::sceCdMmode(c, 2i32); };
    { let _ = F::sceFsReset(c); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Sys_RebootIop", &PROBE_N, &[], true); }
    r_
}

pub fn Sys_LoadIopModules(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sys_LoadIopModules", &PROBE_N, &[], false); }
    let r_ = 'f: {
    'l1: while (((((F::sceSifLoadModule(c, sym::main__str1, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l2: while (((((F::sceSifLoadModule(c, sym::main__str2, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l3: while (((((F::sceSifLoadModule(c, sym::main__str3, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l4: while (((((F::sceSifLoadModule(c, sym::main__str4, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l5: while (((((F::sceSifLoadModule(c, sym::main__str5, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l6: while (((((F::sceSifLoadModule(c, sym::main__str6, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l7: while (((((F::sceSifLoadModule(c, sym::main__str7, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l8: while (((((F::sceSifLoadModule(c, sym::main__str8, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l9: while (((((F::sceSifLoadModule(c, sym::main__str9, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l10: while (((((F::sceSifLoadModule(c, sym::main__str10, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l11: while (((((F::sceSifLoadModule(c, sym::main__str11, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l12: while (((((F::sceSifLoadModule(c, sym::main__str12, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    'l13: while (((((F::sceSifLoadModule(c, sym::main__str13, 0i32, ((0i32) as u32))) < (0i32)) as i32)) != 0) {
    }
    F::Sys_InitIopHeap(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Sys_LoadIopModules", &PROBE_N, &[], true); }
    r_
}

pub fn Game_Main(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Game_Main", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sys_RebootIop(c);
    F::Sys_LoadIopModules(c);
    { let _ = { let a0_ = F::GetThreadId(c); F::ChangeThreadPriority(c, a0_, 2i32) }; };
    F::Heap_Init(c);
    F::Gfx_Init(c);
    F::Snd_Init(c);
    F::File_Init(c);
    F::Movie_Init(c);
    F::Common_Init(c);
    F::Common_LoadBoot(c);
    F::Common_Reload(c);
    F::Progress_Init(c);
    F::Save_Init(c);
    F::Job_Init(c);
    F::Vu0_Init(c);
    F::Dma_InitBuffers(c);
    F::Dbg_Init(c);
    F::Pad_Init(c);
    F::McCard_Init(c);
    F::Font_Init(c, 1i32);
    F::FontIcon_Init(c);
    F::Fade_Init(c);
    F::PadWatch_Init(c);
    {
        'l1: loop {
            'c2: {
                { let _ = F::Overlay_Load(c, 0i32); };
                { let _ = F::Progress_Main(c, 0i32); };
                { let _ = 0i32; { let _ = F::Battle_Main(c); } };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Game_Main", &PROBE_N, &[], true); }
    r_
}

pub fn View_ApplyScissor(c: &mut Ctx, mut p_view_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "View_ApplyScissor", &PROBE_N, &[(p_view_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_view_1).wrapping_add(512)) as i32); let a1_ = (c.m.r32((p_view_1).wrapping_add(516)) as i32); let a2_ = (c.m.r32((p_view_1).wrapping_add(520)) as i32); let a3_ = (c.m.r32((p_view_1).wrapping_add(524)) as i32); F::Dma_AddScissor(c, a0_, a1_, a2_, a3_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "View_ApplyScissor", &PROBE_N, &[], true); }
    r_
}
