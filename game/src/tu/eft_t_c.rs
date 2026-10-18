#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftShotFx_FindFree(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_FindFree", &PROBE_N, &[], false); }
    let mut l_req_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_req_1 = { let a_ = c.m.r32(sym::gEftShotFx); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_req_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                    break 'f l_req_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_req_1; let n_ = (o_).wrapping_add(16); l_req_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_FindFree", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_Start(c: &mut Ctx, mut p_objId_1: i32, mut p_kind_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Start", &PROBE_N, &[(p_objId_1) as u32, (p_kind_2) as u32], false); }
    let mut l_req_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftShotFx)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_req_3 = F::EftShotFx_FindFree(c);
    if (((((l_req_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv4: i32 = ((p_kind_2) as i32);
        match sv4 {
            0i32 => {
                { let v_ = p_objId_1; let a_ = l_req_3; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_req_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let r_ = 33i32; let a_ = (l_req_3).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = p_objId_1; let a_ = l_req_3; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_req_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let r_ = 161i32; let a_ = (l_req_3).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = p_objId_1; let a_ = l_req_3; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_req_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let r_ = 18i32; let a_ = (l_req_3).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = p_objId_1; let a_ = l_req_3; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_req_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let r_ = 146i32; let a_ = (l_req_3).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = p_objId_1; let a_ = l_req_3; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_req_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFxMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFxMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_tex_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftShotFx)) == (((0i32) as u32))) as i32)) != 0) {
        l_w_3 = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((96u32) as i32)) };
        { let v_ = l_w_3; let a_ = sym::gEftShotFx; c.m.w32(a_, v_); };
        { let _ = F::memset(c, l_w_3, 0i32, 96u32); };
    }
    if ((((({ let a_ = c.m.r32(sym::gEftShotFx); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, (((((2i32) as u32)).wrapping_mul(16u32)) as i32)) }; let a_ = c.m.r32(sym::gEftShotFx); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftShotFx); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((2i32) as u32)).wrapping_mul(16u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(4); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 19200i32) }; let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(4); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((19200i32) as u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 72i32) }; let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((72i32) as u32)) }; };
    }
    l_tex_2 = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.r32(a_) };
    { let a1_ = F::BtlScene_GetCommonEntry(c, 567i32); F::EftTexSet_Load4(c, l_tex_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFxMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = c.m.r32(sym::gEftShotFx); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = c.m.r32(sym::gEftShotFx); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftShotFx); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(4); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gEftShotFx)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftShotFx); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftShotFx; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftShotFx_UpdateReqs(c);
    F::EftShotFx_UpdateLines(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftShotFx_UpdateTexture(c, 1i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(16); let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, a0_, a1_) };
    { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = (((c.m.r32(sym::gEftShotFx)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = (((c.m.r32(sym::gEftShotFx)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = (((c.m.r32(sym::gEftShotFx)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    F::EftShotFx_DrawLines(c);
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Alloc", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_p_1 = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(4); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (200i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_p_1).wrapping_add(72)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_p_1, 0i32, 96u32); };
                    break 'f l_p_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_p_1; let n_ = (o_).wrapping_add(96); l_p_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_Add(c: &mut Ctx, p_pos_1: u32, p_off_2: u32, mut p_toward_3: F32, mut p_len_4: F32, mut p_growth_5: F32, mut p_accelDiv_6: F32, mut p_width_7: F32, mut p_alpha_8: F32, mut p_life_9: F32, mut p_tex_10: u8, mut p_blend_11: u8, mut p_flags_12: i32, mut p_delay_13: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Add", &PROBE_N, &[(p_pos_1) as u32, (p_off_2) as u32, (p_toward_3).0, (p_len_4).0, (p_growth_5).0, (p_accelDiv_6).0, (p_width_7).0, (p_alpha_8).0, (p_life_9).0, (p_tex_10) as u32, (p_blend_11) as u32, (p_flags_12) as u32, (p_delay_13) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_p_14: u32 = 0;
    let mut l_last_15: u32 = 0;
    rt::memcpy(c, fp, p_pos_1, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_off_2, 16);
    let r_ = 'f: {
        l_p_14 = F::EftShotFx_Alloc(c);
        if (((((l_p_14) != (((0i32) as u32))) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(84); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_p_14; let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(84); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_p_14).wrapping_add(80); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_p_14).wrapping_add(84); c.m.w32(a_, v_); };
            } else {
                l_last_15 = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(88); c.m.r32(a_) };
                { let v_ = ((0i32) as u32); let a_ = (l_p_14).wrapping_add(84); c.m.w32(a_, v_); };
                { let v_ = l_last_15; let a_ = (l_p_14).wrapping_add(80); c.m.w32(a_, v_); };
            }
            if ((((({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(88); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_p_14; let a_ = ({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(88); c.m.r32(a_) }).wrapping_add(84); c.m.w32(a_, v_); };
            }
            { let v_ = l_p_14; let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(88); c.m.w32(a_, v_); };
            F::Vec4_Copy(c, l_p_14, fp);
            F::Vec4_Copy(c, (l_p_14).wrapping_add(16), fp.wrapping_add(16));
            { let v_ = p_toward_3; let a_ = (l_p_14).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = p_len_4; let a_ = (l_p_14).wrapping_add(36); c.m.w32(a_, (v_).0); };
            { let v_ = p_growth_5; let a_ = (l_p_14).wrapping_add(40); c.m.w32(a_, (v_).0); };
            { let v_ = (if (((rt::fne(p_accelDiv_6, F32(0x00000000)) as i32)) != 0) { rt::fdiv(F32(0x3f800000), p_accelDiv_6) } else { F32(0x00000000) }); let a_ = (l_p_14).wrapping_add(44); c.m.w32(a_, (v_).0); };
            { let v_ = p_width_7; let a_ = (l_p_14).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = p_tex_10; let a_ = (l_p_14).wrapping_add(68); c.m.w8(a_, v_); };
            { let v_ = p_blend_11; let a_ = (l_p_14).wrapping_add(69); c.m.w8(a_, v_); };
            { let v_ = p_delay_13; let a_ = (l_p_14).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
            { let v_ = p_flags_12; let a_ = (l_p_14).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::fmul(p_life_9, F32(0x41f00000)); let a_ = (l_p_14).wrapping_add(64); c.m.w32(a_, (v_).0); };
            if ((((p_flags_12) & (16i32))) != 0) {
                { let v_ = p_alpha_8; let a_ = (l_p_14).wrapping_add(60); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_p_14).wrapping_add(52); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(p_alpha_8, F32(c.m.r32((l_p_14).wrapping_add(64)))); let a_ = (l_p_14).wrapping_add(56); c.m.w32(a_, (v_).0); };
            } else {
                if ((((p_flags_12) & (32i32))) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (l_p_14).wrapping_add(60); c.m.w32(a_, (v_).0); };
                    { let v_ = p_alpha_8; let a_ = (l_p_14).wrapping_add(52); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(p_alpha_8, F32(c.m.r32((l_p_14).wrapping_add(64)))); let a_ = (l_p_14).wrapping_add(56); c.m.w32(a_, (v_).0); };
                }
            }
            { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Add", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_UpdateLines(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_UpdateLines", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_cur_2: u32 = 0;
    let mut l_done_3: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        l_p_1 = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(84); c.m.r32(a_) };
        'l1: while (((((l_p_1) != (((0i32) as u32))) as i32)) != 0) {
            l_cur_2 = l_p_1;
            l_done_3 = F::EftShotFx_Step(c, l_p_1);
            l_p_1 = c.m.r32((l_p_1).wrapping_add(84));
            if ((l_done_3) != 0) {
                { let v_ = 0i32; let a_ = (l_cur_2).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                if (((((c.m.r32((l_cur_2).wrapping_add(80))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(84)); let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(84); c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(84)); let a_ = (c.m.r32((l_cur_2).wrapping_add(80))).wrapping_add(84); c.m.w32(a_, v_); };
                }
                if (((((c.m.r32((l_cur_2).wrapping_add(84))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(80)); let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(88); c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(80)); let a_ = (c.m.r32((l_cur_2).wrapping_add(84))).wrapping_add(80); c.m.w32(a_, v_); };
                }
                { let v_ = ((0i32) as u32); let a_ = (l_cur_2).wrapping_add(80); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_cur_2).wrapping_add(84); c.m.w32(a_, v_); };
                { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_UpdateLines", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_Step(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Step", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_p_1).wrapping_add(72)) as i32)) & (4i32))) != 0) {
        if ((((({ let a_ = (p_p_1).wrapping_add(76); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
            { let r_ = (!(4i32)); let a_ = (p_p_1).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 2i32; let a_ = (p_p_1).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((c.m.r32((p_p_1).wrapping_add(72)) as i32)) & (2i32))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (p_p_1).wrapping_add(64); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fle(F32(c.m.r32((p_p_1).wrapping_add(64))), F32(0x00000000)) as i32)) != 0) {
            { let r_ = 8i32; let a_ = (p_p_1).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32((p_p_1).wrapping_add(72)) as i32)) & (64i32))) != 0) {
            { let r_ = F32(c.m.r32((p_p_1).wrapping_add(40))); let a_ = (p_p_1).wrapping_add(36); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(40))), F32(c.m.r32((p_p_1).wrapping_add(44)))); let a_ = (p_p_1).wrapping_add(40); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        } else {
            if (((((c.m.r32((p_p_1).wrapping_add(72)) as i32)) & (128i32))) != 0) {
                { let r_ = F32(c.m.r32((p_p_1).wrapping_add(40))); let a_ = (p_p_1).wrapping_add(36); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(40))), F32(c.m.r32((p_p_1).wrapping_add(44)))); let a_ = (p_p_1).wrapping_add(40); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
        if (((((c.m.r32((p_p_1).wrapping_add(72)) as i32)) & (16i32))) != 0) {
            { let r_ = F32(c.m.r32((p_p_1).wrapping_add(56))); let a_ = (p_p_1).wrapping_add(52); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge(F32(c.m.r32((p_p_1).wrapping_add(52))), F32(c.m.r32((p_p_1).wrapping_add(60)))) as i32)) != 0) {
                { let v_ = F32(c.m.r32((p_p_1).wrapping_add(60))); let a_ = (p_p_1).wrapping_add(52); c.m.w32(a_, (v_).0); };
                { let r_ = (!(16i32)); let a_ = (p_p_1).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if (((((c.m.r32((p_p_1).wrapping_add(72)) as i32)) & (32i32))) != 0) {
                { let r_ = F32(c.m.r32((p_p_1).wrapping_add(56))); let a_ = (p_p_1).wrapping_add(52); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((p_p_1).wrapping_add(52))), F32(c.m.r32((p_p_1).wrapping_add(60)))) as i32)) != 0) {
                    { let v_ = F32(c.m.r32((p_p_1).wrapping_add(60))); let a_ = (p_p_1).wrapping_add(52); c.m.w32(a_, (v_).0); };
                    { let r_ = (!(32i32)); let a_ = (p_p_1).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
    if (((((c.m.r32((p_p_1).wrapping_add(72)) as i32)) & (8i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_DrawLines(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_DrawLines", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_Copy(c, fp, a1_) };
        {
            l_p_1 = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(84); c.m.r32(a_) };
            'l1: loop {
                if !(((((l_p_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_p_1).wrapping_add(72)) as i32)) & (2i32))) != 0) {
                        F::EftShotFx_BuildQuad(c, fp.wrapping_add(64), l_p_1);
                        { let a5_ = ((rt::f2u(F32(c.m.r32((l_p_1).wrapping_add(52))))) as u8); let a6_ = ({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.r32(a_) }).wrapping_add((((c.m.r8((l_p_1).wrapping_add(68))) as u32)).wrapping_mul(16)); let a7_ = c.m.r8((l_p_1).wrapping_add(69)); F::EftShotFx_DrawQuad(c, fp.wrapping_add(64), fp, ((0i32) as u8), ((0i32) as u8), ((0i32) as u8), a5_, a6_, a7_) };
                    }
                }
                l_p_1 = c.m.r32((l_p_1).wrapping_add(84));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_DrawLines", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_UpdateReqs(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_UpdateReqs", &PROBE_N, &[], false); }
    let mut l_r_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_r_1 = ({ let a_ = c.m.r32(sym::gEftShotFx); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(16));
                    if ((((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (4i32))) != 0) {
                        break 'c2;
                    }
                    if (((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (((32i32) | (64i32))))) != 0) {
                        if (((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (128i32))) != 0) {
                            if ((((!(((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                                {
                                    l_j_3 = 0i32;
                                    'l3: loop {
                                        if !(((((l_j_3) < (4i32)) as i32)) != 0) { break 'l3; }
                                        'c4: {
                                            { let a0_ = (c.m.r32(l_r_1) as i32); F::EftShotFx_SpawnBodyLines(c, a0_, F32(0x3dcccccc), l_j_3) };
                                        }
                                        { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                                    }
                                }
                                { let r_ = 256i32; let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            { let a0_ = (c.m.r32(l_r_1) as i32); F::EftShotFx_SpawnBodyLines(c, a0_, F32(0x3dcccccc), 0i32) };
                        }
                    }
                    if (((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (1i32))) != 0) {
                        if ((((((c.m.r32((l_r_1).wrapping_add(4)) as i32)) == (1i32)) as i32)) != 0) {
                            { let r_ = 8i32; let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        if ((((((c.m.r32((l_r_1).wrapping_add(4)) as i32)) == (4i32)) as i32)) != 0) {
                            { let r_ = (!(1i32)); let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            if ((((!(((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (l_r_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        if (((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (8i32))) != 0) {
                            if ((((!(((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
                                { let a0_ = (c.m.r32(l_r_1) as i32); F::EftShotFx_SpawnVanishColumn(c, a0_, F32(0x3df5c28f)) };
                            }
                            { let r_ = (!(8i32)); let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            { let r_ = (!(32i32)); let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        { let a_ = (l_r_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (2i32))) != 0) {
                        if ((((((c.m.r32((l_r_1).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
                            { let r_ = 64i32; let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        if ((((((c.m.r32((l_r_1).wrapping_add(8)) as i32)) == (4i32)) as i32)) != 0) {
                            { let r_ = (!(2i32)); let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            { let r_ = (!(64i32)); let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            if ((((!(((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (l_r_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        if (((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (16i32))) != 0) {
                            if ((((!(((((c.m.r32((l_r_1).wrapping_add(12)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
                                { let a0_ = (c.m.r32(l_r_1) as i32); F::EftShotFx_SpawnAppearColumn(c, a0_, F32(0x3dcccccc)) };
                            }
                            { let r_ = (!(16i32)); let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            { let r_ = (!(64i32)); let a_ = (l_r_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        { let a_ = (l_r_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_UpdateReqs", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_UpdateTexture(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_UpdateTexture", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(80); c.m.r32(a_) }; F::EftTexSet_Keep4(c, a0_, p_a_1, p_b_2) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_UpdateTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_BuildQuad(c: &mut Ctx, mut p_out_1: u32, mut p_p_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_BuildQuad", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let mut l_out3_3: u32 = 0;
    let mut l_out1_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(112), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(112).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(112).wrapping_add(12), (v_).0); }
        l_out3_3 = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16));
        l_out1_4 = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(128), a1_) };
        F::Vec4_Sub(c, fp.wrapping_add(16), (fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_p_2);
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        { let a2_ = F32(c.m.r32((p_p_2).wrapping_add(32))); F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
        F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), p_p_2);
        { let a1_ = (c.m.r32(sym::gEftShotFx)).wrapping_add(16); F::Mtx_MulVec4(c, fp.wrapping_add(32), a1_, (p_p_2).wrapping_add(16)) };
        F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), fp.wrapping_add(32));
        { let a2_ = rt::fmul(F32(c.m.r32((p_p_2).wrapping_add(36))), F32(0x3f000000)); F::Vec3_Scale(c, fp.wrapping_add(64), fp.wrapping_add(112), a2_) };
        F::Vec4_Add(c, fp.wrapping_add(80), fp.wrapping_add(16), fp.wrapping_add(64));
        F::Vec4_Sub(c, fp.wrapping_add(96), fp.wrapping_add(16), fp.wrapping_add(64));
        F::Vec4_Copy(c, fp, (fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(48), fp, fp.wrapping_add(80));
        F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
        F::Vec3_Cross(c, fp.wrapping_add(64), fp.wrapping_add(112), fp.wrapping_add(48));
        F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
        { let a2_ = rt::fmul(F32(c.m.r32((p_p_2).wrapping_add(48))), F32(0x3f000000)); F::Vec4_Scale(c, fp.wrapping_add(64), fp.wrapping_add(64), a2_) };
        F::Vec4_Add(c, p_out_1, fp.wrapping_add(80), fp.wrapping_add(64));
        F::Vec4_Sub(c, l_out1_4, fp.wrapping_add(80), fp.wrapping_add(64));
        { let v_ = F32(0x3f800000); let a_ = (l_out1_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(48), fp, fp.wrapping_add(96));
        F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
        F::Vec3_Cross(c, fp.wrapping_add(64), fp.wrapping_add(112), fp.wrapping_add(48));
        F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
        { let a2_ = rt::fmul(F32(c.m.r32((p_p_2).wrapping_add(48))), F32(0x3f000000)); F::Vec4_Scale(c, fp.wrapping_add(64), fp.wrapping_add(64), a2_) };
        F::Vec4_Add(c, (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(96), fp.wrapping_add(64));
        F::Vec4_Sub(c, l_out3_3, fp.wrapping_add(96), fp.wrapping_add(64));
        { let v_ = F32(0x3f800000); let a_ = (l_out3_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_BuildQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_DrawQuad(c: &mut Ctx, mut p_quad_1: u32, p_mtx_2: u32, mut p_r_3: u8, mut p_g_4: u8, mut p_b_5: u8, mut p_a_6: u8, mut p_tex_7: u32, mut p_blend_8: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_DrawQuad", &PROBE_N, &[(p_quad_1) as u32, (p_mtx_2) as u32, (p_r_3) as u32, (p_g_4) as u32, (p_b_5) as u32, (p_a_6) as u32, (p_tex_7) as u32, (p_blend_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_p_9: u32 = 0;
    let mut l_e_10: u32 = 0;
    let mut l_layer_11: i32 = 0;
    let mut l_z_12: i32 = 0;
    let mut l_i_13: i32 = 0;
    rt::memcpy(c, fp, p_mtx_2, 64);
    let r_ = 'f: {
        F::Vec4_Set(c, (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x00000000), F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000));
        { let _ = F::Vu0Cur_ProjectPointsStq(c, fp.wrapping_add(192), fp.wrapping_add(64), p_quad_1, fp.wrapping_add(128), 4i32); };
        {
            l_i_13 = 0i32;
            'l1: loop {
                if !(((((l_i_13) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((!((F::EftShotFx_IsOnScreen(c, (fp.wrapping_add(192)).wrapping_add((((l_i_13) as u32)).wrapping_mul(16)), 4i32)) != 0)) as i32)) != 0) {
                        break 'f;
                    }
                }
                { let o_ = l_i_13; let n_ = (o_).wrapping_add(1); l_i_13 = n_; };
            }
        }
        l_p_9 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_9).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        if (((((l_p_9) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = ((92i32) as u64); let a_ = (l_p_9).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870920i32) as u32); let a_ = l_p_9; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_9).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177288i32) as u32); let a_ = (l_p_9).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 16429131440647602177u64; let a_ = (l_p_9).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((18599481537995104i64) as u64); let a_ = (l_p_9).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_9).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = p_r_3; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_g_4; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_b_5; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_a_6; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_r_3; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_g_4; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_b_5; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_a_6; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_r_3; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_g_4; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_b_5; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_a_6; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_r_3; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_g_4; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_b_5; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_a_6; let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_z_12 = (((((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
        { let v_ = c.m.r64(p_tex_7); let a_ = (l_p_9).wrapping_add(40); c.m.w64(a_, v_); };
        l_layer_11 = ((p_blend_8) as i32);
        if (((((l_layer_11) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_layer_11).wrapping_sub(r_); l_layer_11 = n_; };
        }
        if (((((l_z_12) < (0i32)) as i32)) != 0) {
            l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_11) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_12) >= (4096i32)) as i32)) != 0) {
                l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_11) as u32)).wrapping_mul(8));
            } else {
                l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_12) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_11) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_9; let a_ = (c.m.r32((l_e_10).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_9; let a_ = (l_e_10).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_DrawQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_SpawnBodyLines(c: &mut Ctx, mut p_objId_1: i32, mut p_life_2: F32, mut p_delay_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_SpawnBodyLines", &PROBE_N, &[(p_objId_1) as u32, (p_life_2).0, (p_delay_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_toward_4: F32 = F32(0);
    let mut l_angStep_5: F32 = F32(0);
    let mut l_angBase_6: F32 = F32(0);
    let mut l_len_7: F32 = F32(0);
    let mut l_dist_8: F32 = F32(0);
    let mut l_radius_9: F32 = F32(0);
    let mut l_ang_10: F32 = F32(0);
    let mut l_i_11: u32 = 0;
    let mut l_steps_12: i32 = 0;
    let mut l_j_13: i32 = 0;
    let mut l_k_14: i32 = 0;
    let r_ = 'f: {
        l_toward_4 = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), F32(0x40a00000));
        {
            l_i_11 = ((0i32) as u32);
            'l1: loop {
                if !(((((((((l_i_11) < (((23i32) as u32))) as i32)) != 0) && ((((((c.m.r32((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))) as i32)) != (4i32)) as i32)) != 0)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = (c.m.r32((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))) as i32); F::BtlCharApi_GetNodePos(c, p_objId_1, a1_, fp) };
                    F::Vec4_Copy(c, fp.wrapping_add(16), fp);
                    if ((((((c.m.r32(((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))).wrapping_add(4)) as i32)) >= (0i32)) as i32)) != 0) {
                        { let a1_ = (c.m.r32(((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))).wrapping_add(4)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_1, a1_, fp.wrapping_add(48)) };
                        F::Vec4_Sub(c, fp.wrapping_add(48), fp.wrapping_add(48), fp);
                        l_dist_8 = F::Vec3_Length(c, fp.wrapping_add(48));
                        if (((rt::fgt(l_dist_8, F32(c.m.r32(((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))).wrapping_add(8)))) as i32)) != 0) {
                            l_steps_12 = rt::f2i(rt::fadd(rt::fdiv(l_dist_8, F32(c.m.r32(((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))).wrapping_add(8)))), F32(0x3f800000)));
                            F::Vec3_Scale(c, fp.wrapping_add(32), fp.wrapping_add(48), rt::fdiv(F32(0x3f800000), rt::i2f(l_steps_12)));
                        } else {
                            l_steps_12 = 1i32;
                            F::Vec4_Set(c, fp.wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                        }
                    } else {
                        l_steps_12 = 1i32;
                        F::Vec4_Set(c, fp.wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                    }
                    {
                        l_j_13 = 0i32;
                        'l3: loop {
                            if !(((((l_j_13) < (l_steps_12)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_angStep_5 = rt::fdiv(F32(0x40c90fda), rt::i2f((c.m.r32(((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))).wrapping_add(20)) as i32)));
                                l_angBase_6 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                                { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(16))), rt::fmul(rt::i2f(l_j_13), F32(c.m.r32(fp.wrapping_add(32))))); let a_ = fp; c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fmul(rt::i2f(l_j_13), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), rt::fmul(rt::i2f(l_j_13), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                {
                                    l_k_14 = 0i32;
                                    'l5: loop {
                                        if !(((((l_k_14) < ((c.m.r32(((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))).wrapping_add(20)) as i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_len_7 = rt::fmul(rt::fadd(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(c.m.r32(((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))).wrapping_add(12)))), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
                                            l_radius_9 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(c.m.r32(((sym::gEftShotFxSegs).wrapping_add((l_i_11).wrapping_mul(24))).wrapping_add(16)))), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
                                            l_ang_10 = F::EftMath_WrapAngle(c, rt::fadd(l_angBase_6, rt::fmul(l_angStep_5, rt::i2f(l_k_14))));
                                            { let v_ = F::Mathf_SinFast(c, l_ang_10); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
                                            { let v_ = F::Mathf_CosFast(c, l_ang_10); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                            F::Vec4_Scale(c, fp.wrapping_add(64), fp.wrapping_add(64), rt::fmul(l_radius_9, F32(0x3fc00000)));
                                            { let a7_ = rt::i2f((96i32).wrapping_sub(rt::rem_i32(F::Port_Rand(c), 16i32))); F::EftShotFx_Add(c, fp, fp.wrapping_add(64), l_toward_4, rt::fmul(l_len_7, F32(0x40000000)), F32(0x00000000), F32(0x00000000), F32(0x3e999999), a7_, p_life_2, ((0i32) as u8), ((0i32) as u8), ((4i32) | (32i32)), p_delay_3) };
                                        }
                                        { let o_ = l_k_14; let n_ = (o_).wrapping_add(1); l_k_14 = n_; };
                                    }
                                }
                            }
                            { let o_ = l_j_13; let n_ = (o_).wrapping_add(1); l_j_13 = n_; };
                        }
                    }
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_SpawnBodyLines", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_SpawnVanishColumn(c: &mut Ctx, mut p_objId_1: i32, mut p_life_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_SpawnVanishColumn", &PROBE_N, &[(p_objId_1) as u32, (p_life_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_height_3: F32 = F32(0);
    let mut l_toward_4: F32 = F32(0);
    let mut l_growth_5: F32 = F32(0);
    let mut l_angStep_6: F32 = F32(0);
    let mut l_angBase_7: F32 = F32(0);
    let mut l_len_8: F32 = F32(0);
    let mut l_dist_9: F32 = F32(0);
    let mut l_radius_10: F32 = F32(0);
    let mut l_ang_11: F32 = F32(0);
    let mut l_i_12: u32 = 0;
    let mut l_steps_13: i32 = 0;
    let mut l_j_14: i32 = 0;
    let mut l_k_15: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        l_height_3 = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3f000000));
        F::BtlCharApi_GetNodePos(c, p_objId_1, 3i32, fp.wrapping_add(48));
        F::Vec4_Scale(c, fp.wrapping_add(16), fp, rt::fmul(l_height_3, F32(0x3f000000)));
        F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), fp.wrapping_add(48));
        F::Vec4_Scale(c, fp.wrapping_add(32), fp, rt::fmul(rt::fneg(l_height_3), F32(0x3f000000)));
        F::Vec4_Add(c, fp.wrapping_add(32), fp.wrapping_add(32), fp.wrapping_add(48));
        F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(32), fp.wrapping_add(16));
        F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
        l_toward_4 = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), F32(0x40a00000));
        l_growth_5 = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), F32(0x40e00000));
        {
            l_i_12 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_12) < (((6i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a2_ = rt::fmul(l_height_3, F32(c.m.r32((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))))); F::Vec4_Scale(c, fp.wrapping_add(80), fp.wrapping_add(64), a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(80), fp.wrapping_add(16), fp.wrapping_add(80));
                    F::Vec4_Copy(c, fp.wrapping_add(96), fp.wrapping_add(80));
                    if (((rt::fge(F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
                        { let a2_ = rt::fmul(l_height_3, F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(4)))); F::Vec4_Scale(c, fp.wrapping_add(128), fp.wrapping_add(64), a2_) };
                        F::Vec4_Add(c, fp.wrapping_add(128), fp.wrapping_add(16), fp.wrapping_add(128));
                        F::Vec4_Sub(c, fp.wrapping_add(128), fp.wrapping_add(128), fp.wrapping_add(80));
                        l_dist_9 = F::Vec3_Length(c, fp.wrapping_add(128));
                        if (((rt::fgt(l_dist_9, F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(8)))) as i32)) != 0) {
                            l_steps_13 = rt::f2i(rt::fadd(rt::fdiv(l_dist_9, F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(8)))), F32(0x3f800000)));
                            F::Vec3_Scale(c, fp.wrapping_add(112), fp.wrapping_add(128), rt::fdiv(F32(0x3f800000), rt::i2f(l_steps_13)));
                        } else {
                            l_steps_13 = 1i32;
                            F::Vec4_Set(c, fp.wrapping_add(112), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                        }
                    } else {
                        l_steps_13 = 1i32;
                        F::Vec4_Set(c, fp.wrapping_add(112), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                    }
                    {
                        l_j_14 = 0i32;
                        'l3: loop {
                            if !(((((l_j_14) < (l_steps_13)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_angStep_6 = rt::fdiv(F32(0x40c90fda), rt::i2f((c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(20)) as i32)));
                                l_angBase_7 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                                { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(96))), rt::fmul(rt::i2f(l_j_14), F32(c.m.r32(fp.wrapping_add(112))))); let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4))), rt::fmul(rt::i2f(l_j_14), F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))))); let a_ = (fp.wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))), rt::fmul(rt::i2f(l_j_14), F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))))); let a_ = (fp.wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                {
                                    l_k_15 = 0i32;
                                    'l5: loop {
                                        if !(((((l_k_15) < ((c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(20)) as i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_len_8 = rt::fmul(rt::fadd(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(12)))), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
                                            l_radius_10 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(16)))), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
                                            l_ang_11 = F::EftMath_WrapAngle(c, rt::fadd(l_angBase_7, rt::fmul(l_angStep_6, rt::i2f(l_k_15))));
                                            { let v_ = F::Mathf_SinFast(c, l_ang_11); let a_ = fp.wrapping_add(144); c.m.w32(a_, (v_).0); };
                                            { let v_ = F::Mathf_CosFast(c, l_ang_11); let a_ = (fp.wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                            F::Vec4_Scale(c, fp.wrapping_add(144), fp.wrapping_add(144), rt::fmul(l_radius_10, F32(0x3f999999)));
                                            { let a7_ = rt::i2f((96i32).wrapping_sub(rt::rem_i32(F::Port_Rand(c), 16i32))); F::EftShotFx_Add(c, fp.wrapping_add(80), fp.wrapping_add(144), l_toward_4, rt::fmul(l_len_8, F32(0x40000000)), l_growth_5, F32(0xc1200000), F32(0x3e999999), a7_, p_life_2, ((0i32) as u8), ((0i32) as u8), ((((2i32) | (32i32))) | (64i32)), 0i32) };
                                        }
                                        { let o_ = l_k_15; let n_ = (o_).wrapping_add(1); l_k_15 = n_; };
                                    }
                                }
                            }
                            { let o_ = l_j_14; let n_ = (o_).wrapping_add(1); l_j_14 = n_; };
                        }
                    }
                }
                { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_SpawnVanishColumn", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_SpawnAppearColumn(c: &mut Ctx, mut p_objId_1: i32, mut p_life_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_SpawnAppearColumn", &PROBE_N, &[(p_objId_1) as u32, (p_life_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_height_3: F32 = F32(0);
    let mut l_toward_4: F32 = F32(0);
    let mut l_growth_5: F32 = F32(0);
    let mut l_angStep_6: F32 = F32(0);
    let mut l_angBase_7: F32 = F32(0);
    let mut l_len_8: F32 = F32(0);
    let mut l_dist_9: F32 = F32(0);
    let mut l_radius_10: F32 = F32(0);
    let mut l_ang_11: F32 = F32(0);
    let mut l_i_12: u32 = 0;
    let mut l_steps_13: i32 = 0;
    let mut l_j_14: i32 = 0;
    let mut l_k_15: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        l_height_3 = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3f000000));
        F::BtlCharApi_GetNodePos(c, p_objId_1, 3i32, fp.wrapping_add(48));
        F::Vec4_Scale(c, fp.wrapping_add(16), fp, rt::fmul(l_height_3, F32(0x3f000000)));
        F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), fp.wrapping_add(48));
        F::Vec4_Scale(c, fp.wrapping_add(32), fp, rt::fmul(rt::fneg(l_height_3), F32(0x3f000000)));
        F::Vec4_Add(c, fp.wrapping_add(32), fp.wrapping_add(32), fp.wrapping_add(48));
        F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(32), fp.wrapping_add(16));
        F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
        l_toward_4 = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), F32(0x40a00000));
        l_growth_5 = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), F32(0x40e00000));
        {
            l_i_12 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_12) < (((6i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a2_ = rt::fmul(l_height_3, F32(c.m.r32((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))))); F::Vec4_Scale(c, fp.wrapping_add(80), fp.wrapping_add(64), a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(80), fp.wrapping_add(16), fp.wrapping_add(80));
                    F::Vec4_Copy(c, fp.wrapping_add(96), fp.wrapping_add(80));
                    if (((rt::fge(F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
                        { let a2_ = rt::fmul(l_height_3, F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(4)))); F::Vec4_Scale(c, fp.wrapping_add(128), fp.wrapping_add(64), a2_) };
                        F::Vec4_Add(c, fp.wrapping_add(128), fp.wrapping_add(16), fp.wrapping_add(128));
                        F::Vec4_Sub(c, fp.wrapping_add(128), fp.wrapping_add(128), fp.wrapping_add(80));
                        l_dist_9 = F::Vec3_Length(c, fp.wrapping_add(128));
                        if (((rt::fgt(l_dist_9, F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(8)))) as i32)) != 0) {
                            l_steps_13 = rt::f2i(rt::fadd(rt::fdiv(l_dist_9, F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(8)))), F32(0x3f800000)));
                            F::Vec3_Scale(c, fp.wrapping_add(112), fp.wrapping_add(128), rt::fdiv(F32(0x3f800000), rt::i2f(l_steps_13)));
                        } else {
                            l_steps_13 = 1i32;
                            F::Vec4_Set(c, fp.wrapping_add(112), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                        }
                    } else {
                        l_steps_13 = 1i32;
                        F::Vec4_Set(c, fp.wrapping_add(112), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                    }
                    {
                        l_j_14 = 0i32;
                        'l3: loop {
                            if !(((((l_j_14) < (l_steps_13)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_angStep_6 = rt::fdiv(F32(0x40c90fda), rt::i2f((c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(20)) as i32)));
                                l_angBase_7 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                                { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(96))), rt::fmul(rt::i2f(l_j_14), F32(c.m.r32(fp.wrapping_add(112))))); let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4))), rt::fmul(rt::i2f(l_j_14), F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))))); let a_ = (fp.wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))), rt::fmul(rt::i2f(l_j_14), F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))))); let a_ = (fp.wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                {
                                    l_k_15 = 0i32;
                                    'l5: loop {
                                        if !(((((l_k_15) < ((c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(20)) as i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_len_8 = rt::fmul(rt::fadd(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(12)))), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
                                            l_radius_10 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(c.m.r32(((sym::gEftShotFxRows).wrapping_add((l_i_12).wrapping_mul(24))).wrapping_add(16)))), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
                                            l_ang_11 = F::EftMath_WrapAngle(c, rt::fadd(l_angBase_7, rt::fmul(l_angStep_6, rt::i2f(l_k_15))));
                                            { let v_ = F::Mathf_SinFast(c, l_ang_11); let a_ = fp.wrapping_add(144); c.m.w32(a_, (v_).0); };
                                            { let v_ = F::Mathf_CosFast(c, l_ang_11); let a_ = (fp.wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                            F::Vec4_Scale(c, fp.wrapping_add(144), fp.wrapping_add(144), rt::fmul(l_radius_10, F32(0x3f999999)));
                                            { let a7_ = rt::i2f((96i32).wrapping_sub(rt::rem_i32(F::Port_Rand(c), 16i32))); F::EftShotFx_Add(c, fp.wrapping_add(80), fp.wrapping_add(144), l_toward_4, rt::fmul(l_len_8, F32(0x41200000)), l_growth_5, F32(0xc1000000), F32(0x3e999999), a7_, p_life_2, ((0i32) as u8), ((0i32) as u8), ((((2i32) | (16i32))) | (128i32)), 0i32) };
                                        }
                                        { let o_ = l_k_15; let n_ = (o_).wrapping_add(1); l_k_15 = n_; };
                                    }
                                }
                            }
                            { let o_ = l_j_14; let n_ = (o_).wrapping_add(1); l_j_14 = n_; };
                        }
                    }
                }
                { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_SpawnAppearColumn", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotFx_IsOnScreen(c: &mut Ctx, mut p_v_1: u32, mut p_margin_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_IsOnScreen", &PROBE_N, &[(p_v_1) as u32, (p_margin_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_margin_2;
        match sv3 {
            0i32 => {
                if ((((((c.m.r32((p_v_1).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) > (36863i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) <= (28672i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) > (36351i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) <= (29184i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                break 's1;
            }
            1i32 => {
                if ((((((c.m.r32((p_v_1).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) > (37887i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) <= (27648i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) > (37247i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) <= (28288i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                break 's1;
            }
            2i32 => {
                if ((((((c.m.r32((p_v_1).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) > (38911i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) <= (26624i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) > (38143i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) <= (27392i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                break 's1;
            }
            3i32 => {
                if ((((((c.m.r32((p_v_1).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) > (40959i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) <= (24576i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) > (39935i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) <= (25600i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                break 's1;
            }
            _ => {
                if ((((((c.m.r32((p_v_1).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) > (65519i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(p_v_1) as i32)) <= (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) > (65519i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) <= (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                break 's1;
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotFx_IsOnScreen", &PROBE_N, &[], true); }
    r_
}
