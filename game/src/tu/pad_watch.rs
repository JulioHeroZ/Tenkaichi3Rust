#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn PadWatch_IsConnected(c: &mut Ctx, mut p_port_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_IsConnected", &PROBE_N, &[(p_port_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32(((sym::gPad).wrapping_add((((p_port_1) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) != (255i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_IsConnected", &PROBE_N, &[], true); }
    r_
}

pub fn PadWatch_SetEnabled(c: &mut Ctx, mut p_enable_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_SetEnabled", &PROBE_N, &[(p_enable_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((p_enable_1) == (0i32)) as i32); let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_SetEnabled", &PROBE_N, &[], true); }
    r_
}

pub fn PadWatch_GetMissing(c: &mut Ctx, mut p_port_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_GetMissing", &PROBE_N, &[(p_port_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(36); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) || ((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        if (((if ((((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6648); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { (((((((F::BtlGame_IsReplay(c)) == (0i32)) as i32)) != 0) && (((((F::Battle_IsSplitScreen(c)) != (0i32)) as i32)) != 0)) as i32) } else { (((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_sub(39i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32) })) != 0) {
            if ((((!(((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                if ((((!(((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                    if (((((p_port_1) != (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = p_port_1; c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    if (((((p_port_1) != (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = p_port_1; c.m.w32(a_, (v_) as u32); };
                    }
                }
            } else {
                if ((((!(((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                    if (((((p_port_1) != (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = p_port_1; c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
        } else {
            if ((((!(((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                if (((((p_port_1) != (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = p_port_1; c.m.w32(a_, (v_) as u32); };
                }
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_GetMissing", &PROBE_N, &[], true); }
    r_
}

pub fn PadWatch_IsPortMissing(c: &mut Ctx, mut p_port_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_IsPortMissing", &PROBE_N, &[(p_port_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((p_port_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(36); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((if ((((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6648); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { (((((((F::BtlGame_IsReplay(c)) == (0i32)) as i32)) != 0) && (((((F::Battle_IsSplitScreen(c)) != (0i32)) as i32)) != 0)) as i32) } else { (((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_sub(39i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32) })) != 0) {
        if ((((!(((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((p_port_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    } else {
        if ((((!(((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((p_port_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_IsPortMissing", &PROBE_N, &[], true); }
    r_
}

pub fn PadWatch_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F::PadWatch_IsConnected(c, l_i_1); let a_ = (fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        if ((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (6i32)) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = { let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32); let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = { let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        if ((((((((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(32); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) || ((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            if (((if ((((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6648); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { (((((((F::BtlGame_IsReplay(c)) == (0i32)) as i32)) != 0) && (((((F::Battle_IsSplitScreen(c)) != (0i32)) as i32)) != 0)) as i32) } else { (((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_sub(39i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32) })) != 0) {
                if ((((!(((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                    if ((((!(((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    if ((((!(((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    }
                }
            } else {
                if ((((!(((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_Update", &PROBE_N, &[], true); }
    r_
}

pub fn PadWatch_DrawMessage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_DrawMessage", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_message_1: i32 = 0;
    let mut l_boot_2: u32 = 0;
    let mut l_text_3: u32 = 0;
    let mut l_height_4: i32 = 0;
    let mut l_fontPtr_5: u32 = 0;
    let mut l_p_6: u32 = 0;
    let r_ = 'f: {
        l_message_1 = { let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(36); (c.m.r32(a_) as i32) };
        if ((((((((((((((((({ let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add(32); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((((((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6648); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) || (((((F::Battle_GetMode(c)) != (7i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) || ((((({ let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && (((((l_message_1) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_boot_2 = { let a_ = c.m.r32(sym::gCommonRes); c.m.r32(a_) };
            { let v_ = F::Font_GetCmdCount(c); c.m.w32(fp, (v_) as u32); }
            l_text_3 = (l_boot_2).wrapping_add(((c.m.r32((l_boot_2).wrapping_add(((((9i32).wrapping_add(l_message_1)) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
            l_height_4 = F::Font_GetHeight(c, l_text_3);
            l_fontPtr_5 = fp;
            F::Gfx_AddDefaultEnv(c);
            l_p_6 = F::Dma_BeginDirect(c);
            { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
            { let v_ = ((68i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((66i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
            { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((21776i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
            { let v_ = ((70i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = 4575657223019036672u64; let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
            { let v_ = ((((((28416i32) as u64)) | ((((28928i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((16777215i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((((((37120i32) as u64)) | ((((36608i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((16777215i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            F::Dma_EndDirect(c, (l_p_6).wrapping_add((((2i32) as u32)).wrapping_mul(8)));
            F::Font_PushStyle(c);
            F::Font_SetAlign(c, 1i32);
            F::Font_SetShadowMode(c, 1i32);
            F::Font_SetClip(c, 0i32, 0i32, 512i32, 448i32);
            F::Font_SetColorRGBA(c, ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), 128i32);
            F::Font_SetShadowColorRGBA(c, ((((0i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8), 128i32);
            F::Font_PrintAt(c, 256i32, (224i32).wrapping_sub(rt::div_i32(l_height_4, 2i32)), l_text_3);
            F::Font_PopStyle(c);
            { let a0_ = (c.m.r32(l_fontPtr_5) as i32); F::Font_Flush(c, a0_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_DrawMessage", &PROBE_N, &[], true); }
    r_
}

pub fn PadWatch_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_Init", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((40u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gPadWatch; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gPadWatch); F::memset(c, a0_, 0i32, 40u32) }; };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = 1i32; let a_ = ((c.m.r32(sym::gPadWatch)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gPadWatch)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_Init", &PROBE_N, &[], true); }
    r_
}

pub fn PadWatch_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::PadWatch_DrawMessage(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn PadWatch_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gPadWatch); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gPadWatch; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PadWatch_Term", &PROBE_N, &[], true); }
    r_
}
