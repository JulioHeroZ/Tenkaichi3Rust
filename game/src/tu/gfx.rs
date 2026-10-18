#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Gfx_InitDrawEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_InitDrawEnv", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::sceGsResetGraph(c, ((0i32) as i16), ((1i32) as i16), ((2i32) as i16), ((0i32) as i16));
    { let _ = F::sceGsSetDefDBuff(c, sym::gGfx, ((0i32) as i16), ((512i32) as i16), ((448i32) as i16), ((3i32) as i16), ((49i32) as i16), ((1i32) as i16)); };
    { let v_ = ((((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(224)).wrapping_add(48); c.m.w64(a_, v_); };
    { let v_ = (((((((28672i32).wrapping_add((512i32).wrapping_shl((4i32) as u32))) as u64)) | (((((29184i32).wrapping_add((448i32).wrapping_shl((4i32) as u32))) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(224)).wrapping_add(64); c.m.w64(a_, v_); };
    { let v_ = ((4575657221408423936i64) as u64); let a_ = ((sym::gGfx).wrapping_add(224)).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(96)).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((((((((0i32) as u64)) | (((((512i32).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((448i32).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(96)).wrapping_add(48); c.m.w64(a_, v_); };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(96)).wrapping_add(16); c.m.w64(a_, v_); };
    { let v_ = ((((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(464)).wrapping_add(48); c.m.w64(a_, v_); };
    { let v_ = (((((((28672i32).wrapping_add((512i32).wrapping_shl((4i32) as u32))) as u64)) | (((((29184i32).wrapping_add((448i32).wrapping_shl((4i32) as u32))) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(464)).wrapping_add(64); c.m.w64(a_, v_); };
    { let v_ = ((4575657221408423936i64) as u64); let a_ = ((sym::gGfx).wrapping_add(464)).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((((0i32) | ((8i32).wrapping_shl((9i32) as u32)))) as u64); let a_ = ((sym::gGfx).wrapping_add((((0i32) as u32)).wrapping_mul(40))).wrapping_add(16); c.m.w64(a_, v_); };
    { let v_ = ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (sym::gGfx).wrapping_add(96); c.m.w64(a_, v_); };
    { let v_ = ((((112i32) | ((8i32).wrapping_shl((9i32) as u32)))) as u64); let a_ = ((sym::gGfx).wrapping_add((((1i32) as u32)).wrapping_mul(40))).wrapping_add(16); c.m.w64(a_, v_); };
    { let v_ = ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (sym::gGfx).wrapping_add(336); c.m.w64(a_, v_); };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(336)).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((((((((0i32) as u64)) | (((((512i32).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((448i32).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(336)).wrapping_add(48); c.m.w64(a_, v_); };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((sym::gGfx).wrapping_add(336)).wrapping_add(16); c.m.w64(a_, v_); };
    F::FlushCache(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_InitDrawEnv", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_SyncField(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_SyncField", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = (((F::sceGsSyncV(c, 0i32)) == (0i32)) as i32); let a_ = (sym::gGfx).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_SyncField", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_SetDisplayRegs(c: &mut Ctx, mut p_field_1: i32, mut p_blend_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_SetDisplayRegs", &PROBE_N, &[(p_field_1) as u32, (p_blend_2) as u32], false); }
    let mut l_save_3: u32 = 0;
    let mut l_odd_4: i32 = 0;
    let mut l_dy_5: i32 = 0;
    let mut l_dx_6: i32 = 0;
    let mut l_size_7: u64 = 0;
    let mut l_display2_8: u64 = 0;
    let mut l_display1_9: u64 = 0;
    let r_ = 'f: {
    l_save_3 = c.m.r32(sym::gSaveData);
    l_odd_4 = (((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32));
    l_dy_5 = ((c.m.r32((l_save_3).wrapping_add(5792)) as i32)).wrapping_mul(2i32);
    l_dx_6 = (((c.m.r32((l_save_3).wrapping_add(5788)) as i32)).wrapping_mul(4i32)).wrapping_add(636i32);
    l_size_7 = ((7874698016718848i64) as u64);
    l_display2_8 = ((((((l_dx_6) as u64)) | (((((l_dy_5).wrapping_add(50i32)) as u64)).wrapping_shl((12i32) as u32)))) | (l_size_7));
    l_display1_9 = ((((((l_dx_6) as u64)) | (((((l_dy_5).wrapping_add(51i32)) as u64)).wrapping_shl((12i32) as u32)))) | (l_size_7));
    { let v_ = (((if ((((p_blend_2) & (1i32))) != 0) { 32547i32 } else { 65315i32 })) as u64); let a_ = ((301989888i32) as u32); c.m.w64(a_, v_); };
    { let v_ = (((if ((((!((l_odd_4) != 0)) as i32)) != 0) { ((112i32) | ((8i32).wrapping_shl((9i32) as u32))) } else { ((0i32) | ((8i32).wrapping_shl((9i32) as u32))) })) as u64); let a_ = ((301990032i32) as u32); c.m.w64(a_, v_); };
    { let v_ = l_display2_8; let a_ = ((301990048i32) as u32); c.m.w64(a_, v_); };
    { let v_ = (((if ((((!((l_odd_4) != 0)) as i32)) != 0) { ((112i32) | ((8i32).wrapping_shl((9i32) as u32))) } else { ((0i32) | ((8i32).wrapping_shl((9i32) as u32))) })) as u64); let a_ = ((301990000i32) as u32); c.m.w64(a_, v_); };
    { let v_ = l_display1_9; let a_ = ((301990016i32) as u32); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = ((301990096i32) as u32); c.m.w64(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_SetDisplayRegs", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Init", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(656) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 656);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(560), 0, 96);
        { let v_ = ((((((((5i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); c.m.w64(fp.wrapping_add(560).wrapping_add(0), v_); }
        { let v_ = ((14i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(8), v_); }
        { let v_ = ((0i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(16), v_); }
        { let v_ = ((73i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(24), v_); }
        { let v_ = ((0i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(32), v_); }
        { let v_ = ((34i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(40), v_); }
        { let v_ = ((0i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(48), v_); }
        { let v_ = ((69i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(56), v_); }
        { let v_ = ((0i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(64), v_); }
        { let v_ = ((8i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(72), v_); }
        { let v_ = ((0i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(80), v_); }
        { let v_ = ((9i32) as u64); c.m.w64(fp.wrapping_add(560).wrapping_add(88), v_); }
        { let _ = F::memset(c, sym::gGfx, 0i32, 592u32); };
        F::sceGsResetPath(c);
        F::sceGsResetGraph(c, ((0i32) as i16), ((1i32) as i16), ((2i32) as i16), ((1i32) as i16));
        F::Dma_SendGif(c, fp.wrapping_add(560), ((6i32) as u32));
        F::sceGsResetGraph(c, ((0i32) as i16), ((1i32) as i16), ((2i32) as i16), ((1i32) as i16));
        { let _ = F::sceGsSetDefDBuff(c, fp, ((0i32) as i16), ((640i32) as i16), ((448i32) as i16), ((2i32) as i16), ((49i32) as i16), ((1i32) as i16)); };
        F::FlushCache(c, 0i32);
        { let _ = F::sceGsSyncV(c, 0i32); };
        { let _ = F::sceGsPutDrawEnv(c, (fp).wrapping_add(80)); };
        { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        { let _ = F::sceGsPutDrawEnv(c, (fp).wrapping_add(320)); };
        { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        F::Gfx_SyncField(c);
        F::Gfx_InitDrawEnv(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_BeginFrame(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_BeginFrame", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Dbg_BeginFrame(c);
    F::BtlObj_RebuildTable(c);
    F::Fade_UpdateAll(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_BeginFrame", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_EndFrame(c: &mut Ctx, mut p_vsyncs_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_EndFrame", &PROBE_N, &[(p_vsyncs_1) as u32], false); }
    let r_ = 'f: {
    F::Fade_DrawScreen(c);
    F::FontIcon_Tick(c, p_vsyncs_1);
    F::PadWatch_Update(c);
    F::PadWatch_Draw(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2151694400u32); };
    F::Dbg_EndFrame(c);
    F::Vu0_CheckState(c);
    { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
    F::Vsync_Wait(c, ((p_vsyncs_1) as u32));
    { let a_ = (sym::gGfx).wrapping_add(564); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    { let v_ = ((((c.m.r32((sym::gGfx).wrapping_add(560)) as i32)) == (0i32)) as i32); let a_ = (sym::gGfx).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
    { let a1_ = (c.m.r32((sym::gGfx).wrapping_add(564)) as i32); F::sceGsSwapDBuff(c, sym::gGfx, a1_) };
    { let a0_ = (c.m.r32((sym::gGfx).wrapping_add(560)) as i32); F::Gfx_SetDisplayRegs(c, a0_, 1i32) };
    { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_EndFrame", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_Stub102118(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Stub102118", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Stub102118", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_ClearScreen(c: &mut Ctx, mut p_fbmsk_1: i32, mut p_rgba_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_ClearScreen", &PROBE_N, &[(p_fbmsk_1) as u32, (p_rgba_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, p_fbmsk_1) };
    F::Dma_AddScissor(c, 0i32, (512i32).wrapping_sub(1i32), 0i32, (448i32).wrapping_sub(1i32));
    F::Dma_AddZbuf(c, 224i32, 1i32);
    F::Dma_AddFillRect(c, (28672i32).wrapping_shr((4i32) as u32), (29184i32).wrapping_shr((4i32) as u32), 512i32, 448i32, p_rgba_2);
    F::Dma_AddZbuf(c, 224i32, 0i32);
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_ClearScreen", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_AddDefaultEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_AddDefaultEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = F::Dma_BeginDirect(c); c.m.w32(fp, v_); }
        F::Gfx_PutDefaultEnv(c, fp);
        { let a0_ = c.m.r32(fp); F::Dma_EndDirect(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Gfx_AddDefaultEnv", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_PutDefaultEnv(c: &mut Ctx, mut p_pp_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_PutDefaultEnv", &PROBE_N, &[(p_pp_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((((((12i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((549755813888i64) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((59i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((0i32) as u64)) | (((((512i32).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((448i32).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) }); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((327680i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((68i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((1i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((26i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((1i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((96i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_PutDefaultEnv", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_Stub102438(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Stub102438", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Stub102438", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_Stub102440(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Stub102440", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Stub102440", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_MarkPass(c: &mut Ctx, mut p_pass_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_MarkPass", &PROBE_N, &[(p_pass_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_MarkPass", &PROBE_N, &[], true); }
    r_
}

pub fn Gfx_Stub102450(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Stub102450", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gfx_Stub102450", &PROBE_N, &[], true); }
    r_
}
