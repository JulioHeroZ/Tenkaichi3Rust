#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlCharSnd_GetSet(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_GetSet", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(52));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_GetSet", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_GetLoopSet(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_GetLoopSet", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(52));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_GetLoopSet", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_IsLoopPlaying(c: &mut Ctx, mut p_player_1: i32, mut p_kind_2: i32, mut p_id_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_IsLoopPlaying", &PROBE_N, &[(p_player_1) as u32, (p_kind_2) as u32, (p_id_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_slot_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    l_slot_5 = F::BtlCharSnd_GetLoopSet(c, l_chr_4);
    if (((((l_chr_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((((((c.m.r32(l_slot_5) as i32)) >= (0i32)) as i32)) != 0) && (((((p_id_3) == ((c.m.r32((l_slot_5).wrapping_add(4)) as i32))) as i32)) != 0)) as i32)) != 0) && (((((p_kind_2) == (((c.m.r8((l_slot_5).wrapping_add(8))) as i32))) as i32)) != 0)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_slot_5; let n_ = (o_).wrapping_add(12); l_slot_5 = n_; }; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_IsLoopPlaying", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_IsRequested(c: &mut Ctx, mut p_owner_1: i32, mut p_kind_2: i32, mut p_id_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_IsRequested", &PROBE_N, &[(p_owner_1) as u32, (p_kind_2) as u32, (p_id_3) as u32], false); }
    let mut l_list_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_list_4 = (c.m.r32(sym::gBtlChars)).wrapping_add(160);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < ((c.m.r32((l_list_4).wrapping_add(128)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((((((p_owner_1) == ((((c.m.r8((((l_list_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(12)) as i8)) as i32))) as i32)) != 0) && (((((p_id_3) == ((c.m.r32((((l_list_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8)) as i32))) as i32)) != 0)) as i32)) != 0) && (((((p_kind_2) == (((c.m.r8((((l_list_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(13))) as i32))) as i32)) != 0)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_IsRequested", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_Request(c: &mut Ctx, mut p_pos_1: u32, mut p_owner_2: i32, mut p_kind_3: i32, mut p_near_4: F32, mut p_far_5: F32, mut p_id_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_Request", &PROBE_N, &[(p_pos_1) as u32, (p_owner_2) as u32, (p_kind_3) as u32, (p_near_4).0, (p_far_5).0, (p_id_6) as u32], false); }
    let mut l_list_7: u32 = 0;
    let mut l_req_8: u32 = 0;
    let mut l_ok_9: i32 = 0;
    let r_ = 'f: {
    l_list_7 = (c.m.r32(sym::gBtlChars)).wrapping_add(160);
    if (((((c.m.r32(sym::gBtlChars)) != (((0i32) as u32))) as i32)) != 0) {
        l_ok_9 = ((((c.m.r32((l_list_7).wrapping_add(128)) as i32)) < (4i32)) as i32);
        l_req_8 = (l_list_7).wrapping_add(((((c.m.r32((l_list_7).wrapping_add(128)) as i32)) as u32)).wrapping_mul(32));
        if ((l_ok_9) != 0) {
            F::Vec4_Copy(c, l_req_8, p_pos_1);
            { let v_ = ((p_owner_2) as i8); let a_ = ((l_req_8).wrapping_add(16)).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((p_kind_3) as u8); let a_ = ((l_req_8).wrapping_add(16)).wrapping_add(13); c.m.w8(a_, v_); };
            { let v_ = p_id_6; let a_ = ((l_req_8).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = p_near_4; let a_ = (l_req_8).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = p_far_5; let a_ = ((l_req_8).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let a_ = (l_list_7).wrapping_add(128); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_Request", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_StoreHandle(c: &mut Ctx, mut p_set_1: u32, mut p_handle_2: i32, mut p_player_3: i32, mut p_kind_4: i32, mut p_id_5: i32, mut p_force_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_StoreHandle", &PROBE_N, &[(p_set_1) as u32, (p_handle_2) as u32, (p_player_3) as u32, (p_kind_4) as u32, (p_id_5) as u32, (p_force_6) as u32], false); }
    let mut l_i_7: i32 = 0;
    let mut l_n_8: i32 = 0;
    let r_ = 'f: {
    if (((((F::BtlChar_Get(c, p_player_3)) != (((0i32) as u32))) as i32)) != 0) {
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_8 = rt::rem_i32(((c.m.r32((p_set_1).wrapping_add(48)) as i32)).wrapping_add(l_i_7), 4i32);
                    if ((((((((((c.m.r32((p_set_1).wrapping_add((((l_n_8) as u32)).wrapping_mul(12))) as i32)) < (0i32)) as i32)) != 0) || ((p_force_6) != 0)) as i32)) != 0) {
                        { let v_ = p_handle_2; let a_ = (p_set_1).wrapping_add((((l_n_8) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((p_kind_4) as u8); let a_ = ((p_set_1).wrapping_add((((l_n_8) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.w8(a_, v_); };
                        { let v_ = ((((p_id_5) as u8)) as i32); let a_ = ((p_set_1).wrapping_add((((l_n_8) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = rt::rem_i32(((c.m.r32((p_set_1).wrapping_add(48)) as i32)).wrapping_add(1i32), 4i32); let a_ = (p_set_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 'l1;
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_StoreHandle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_StopUnrequestedLoops(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_StopUnrequestedLoops", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_slot_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_slot_2 = F::BtlCharSnd_GetLoopSet(c, p_chr_1);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((c.m.r32(l_slot_2) as i32)) >= (0i32)) as i32)) != 0) && ((((!(({ let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = ((c.m.r8((l_slot_2).wrapping_add(8))) as i32); let a2_ = (c.m.r32((l_slot_2).wrapping_add(4)) as i32); F::BtlCharSnd_IsRequested(c, a0_, a1_, a2_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = (c.m.r32(l_slot_2) as i32); F::Snd_StopHandle(c, a0_) };
                    { let v_ = (1i32).wrapping_neg(); let a_ = l_slot_2; c.m.w32(a_, (v_) as u32); };
                }
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_slot_2; let n_ = (o_).wrapping_add(12); l_slot_2 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_StopUnrequestedLoops", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_CalcVolPan(c: &mut Ctx, mut p_pos_1: u32, mut p_vol_2: u32, mut p_pan_3: u32, mut p_near_4: F32, mut p_far_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_CalcVolPan", &PROBE_N, &[(p_pos_1) as u32, (p_vol_2) as u32, (p_pan_3) as u32, (p_near_4).0, (p_far_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_dist_6: F32 = F32(0);
    let mut l_ang_7: F32 = F32(0);
    let mut l_v_8: i32 = 0;
    let mut l_p_9: i32 = 0;
    let r_ = 'f: {
        if ((F::Battle_IsSplitScreen(c)) != 0) {
            { let v_ = 64i32; let a_ = p_vol_2; c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = p_pan_3; c.m.w32(a_, (v_) as u32); };
            break 'f;
        }
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(32), a1_) };
        F::Vec4_Sub(c, fp, p_pos_1, (fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        l_dist_6 = F::Vec3_Length(c, fp);
        F::Vec4_Copy(c, fp.wrapping_add(16), (fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        if (((((((rt::flt({ let a0_ = F32(c.m.r32(fp)); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0) && (((rt::flt({ let a0_ = F32(c.m.r32((fp).wrapping_add(8))); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0)) as i32)) != 0) {
            l_ang_7 = F32(0x00000000);
        } else {
            if (((((((rt::flt({ let a0_ = F32(c.m.r32(fp.wrapping_add(16))); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0) && (((rt::flt({ let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0)) as i32)) != 0) {
                l_ang_7 = F32(0x00000000);
            } else {
                l_ang_7 = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                l_ang_7 = { let a0_ = rt::fsub(l_ang_7, { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }); F::BtlUtil_WrapAngle(c, a0_) };
            }
        }
        l_v_8 = F::BtlUtil_Clamp(c, rt::f2i(rt::fmul(rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(l_dist_6, p_near_4), rt::fsub(p_far_5, p_near_4))), F32(0x43000000))), 0i32, 128i32);
        l_p_9 = { let a0_ = rt::f2i(rt::fmul(F::Mathf_Sin(c, l_ang_7), F32(0x42800000))); F::BtlUtil_Clamp(c, a0_, (64i32).wrapping_neg(), 64i32) };
        if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
            if ((F::BtlScript_IsTextShown(c)) != 0) {
                l_v_8 = F::BtlUtil_Max(c, (l_v_8).wrapping_sub(32i32), 0i32);
            }
        }
        { let v_ = l_v_8; let a_ = p_vol_2; c.m.w32(a_, (v_) as u32); };
        { let v_ = l_p_9; let a_ = p_pan_3; c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_CalcVolPan", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_Init", &PROBE_N, &[], false); }
    let mut l_list_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_chr_5: u32 = 0;
    let mut l_set_6: u32 = 0;
    let mut l_loop_7: u32 = 0;
    let r_ = 'f: {
    l_list_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(160);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_p_4 = ((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add(16);
                { let v_ = (((1i32).wrapping_neg()) as u8); let a_ = (l_p_4).wrapping_add(13); c.m.w8(a_, v_); };
                { let v_ = 255i32; let a_ = (l_p_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_p_4).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
            'c4: {
                l_chr_5 = F::BtlChar_Get(c, l_i_2);
                l_set_6 = F::BtlCharSnd_GetSet(c, l_chr_5);
                l_loop_7 = F::BtlCharSnd_GetLoopSet(c, l_chr_5);
                {
                    l_j_3 = 0i32;
                    'l5: loop {
                        if !(((((l_j_3) < (4i32)) as i32)) != 0) { break 'l5; }
                        'c6: {
                            { let v_ = (1i32).wrapping_neg(); let a_ = (l_set_6).wrapping_add((((l_j_3) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (1i32).wrapping_neg(); let a_ = (l_loop_7).wrapping_add((((l_j_3) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_ClearRequests(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_ClearRequests", &PROBE_N, &[], false); }
    let mut l_list_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_list_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(160);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_p_3 = ((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add(16);
                { let v_ = (((1i32).wrapping_neg()) as u8); let a_ = (l_p_3).wrapping_add(13); c.m.w8(a_, v_); };
                { let v_ = 255i32; let a_ = (l_p_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_p_3).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = (l_list_1).wrapping_add(128); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_ClearRequests", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_PlayRequests(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayRequests", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_list_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_req_3: u32 = 0;
    let mut l_owner_4: i32 = 0;
    let mut l_chr_5: u32 = 0;
    let mut l_chara_6: i32 = 0;
    let mut l_handle_7: i32 = 0;
    let r_ = 'f: {
        l_list_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(160);
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((l_list_1).wrapping_add(128)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_req_3 = (l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(32));
                    l_owner_4 = 0i32;
                    if ((((((((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(12)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
                        l_owner_4 = (((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(12)) as i8)) as i32);
                    }
                    l_chr_5 = F::BtlChar_Get(c, l_owner_4);
                    l_chara_6 = { let a_ = (F::BtlChar_GetObj(c, l_chr_5)).wrapping_add(((12i32) as u32)); (c.m.r32(a_) as i32) };
                    if ((F::BtlChar_TestMemberUnk70(c, l_chr_5)) != 0) {
                        l_chara_6 = 86i32;
                    }
                    's3: {
                        let sv8: i32 = ((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(13))) as i32);
                        match sv8 {
                            4i32 => {
                                { let a3_ = F32(c.m.r32((l_req_3).wrapping_add(16))); let a4_ = F32(c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(4))); F::BtlCharSnd_CalcVolPan(c, l_req_3, fp, fp.wrapping_add(4), a3_, a4_) };
                                { let a1_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); let a2_ = (c.m.r32(fp) as i32); let a3_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::StreamSe_Play(c, l_owner_4, a1_, a2_, a3_) };
                                break 's3;
                            }
                            5i32 | 6i32 => {
                                { let a3_ = F32(c.m.r32((l_req_3).wrapping_add(16))); let a4_ = F32(c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(4))); F::BtlCharSnd_CalcVolPan(c, l_req_3, fp, fp.wrapping_add(4), a3_, a4_) };
                                { let a2_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(fp) as i32); let a4_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::Voice_PlayCharaEx(c, l_owner_4, l_chara_6, a2_, a3_, a4_) };
                                break 's3;
                            }
                            7i32 => {
                                { let a3_ = F32(c.m.r32((l_req_3).wrapping_add(16))); let a4_ = F32(c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(4))); F::BtlCharSnd_CalcVolPan(c, l_req_3, fp, fp.wrapping_add(4), a3_, a4_) };
                                { let a1_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); let a2_ = (c.m.r32(fp) as i32); let a3_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::Voice_Play(c, l_owner_4, a1_, a2_, a3_) };
                                break 's3;
                            }
                            _ => {
                                if (({ let a0_ = (({ let a_ = (sym::gBtlSndBankMask).wrapping_add((((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(13))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); let a1_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); F::Snd_IsLoopSe(c, a0_, a1_) }) != 0) {
                                    if ((((!(({ let a1_ = ((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(13))) as i32); let a2_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); F::BtlCharSnd_IsLoopPlaying(c, l_owner_4, a1_, a2_) }) != 0)) as i32)) != 0) {
                                        { let a3_ = F32(c.m.r32((l_req_3).wrapping_add(16))); let a4_ = F32(c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(4))); F::BtlCharSnd_CalcVolPan(c, l_req_3, fp, fp.wrapping_add(4), a3_, a4_) };
                                        l_handle_7 = { let a0_ = (({ let a_ = (sym::gBtlSndBankMask).wrapping_add((((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(13))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); let a1_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); let a2_ = (c.m.r32(fp) as i32); let a3_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::Snd_PlaySeEx(c, a0_, a1_, a2_, a3_, 0i32) };
                                        { let a0_ = F::BtlCharSnd_GetLoopSet(c, l_chr_5); let a3_ = ((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(13))) as i32); let a4_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); F::BtlCharSnd_StoreHandle(c, a0_, l_handle_7, l_owner_4, a3_, a4_, 0i32) };
                                    }
                                } else {
                                    { let a3_ = F32(c.m.r32((l_req_3).wrapping_add(16))); let a4_ = F32(c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(4))); F::BtlCharSnd_CalcVolPan(c, l_req_3, fp, fp.wrapping_add(4), a3_, a4_) };
                                    l_handle_7 = { let a0_ = (({ let a_ = (sym::gBtlSndBankMask).wrapping_add((((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(13))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); let a1_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); let a2_ = (c.m.r32(fp) as i32); let a3_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::Snd_PlaySeEx(c, a0_, a1_, a2_, a3_, 0i32) };
                                    { let a0_ = F::BtlCharSnd_GetSet(c, l_chr_5); let a3_ = ((c.m.r8(((l_req_3).wrapping_add(16)).wrapping_add(13))) as i32); let a4_ = (c.m.r32(((l_req_3).wrapping_add(16)).wrapping_add(8)) as i32); F::BtlCharSnd_StoreHandle(c, a0_, l_handle_7, l_owner_4, a3_, a4_, 1i32) };
                                }
                                break 's3;
                            }
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = F::BtlChar_Get(c, 0i32); F::BtlCharSnd_StopUnrequestedLoops(c, a0_) };
        { let a0_ = F::BtlChar_Get(c, 1i32); F::BtlCharSnd_StopUnrequestedLoops(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayRequests", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_RequestAt(c: &mut Ctx, mut p_pos_1: u32, mut p_kind_2: i32, mut p_id_3: i32, mut p_near_4: F32, mut p_far_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_RequestAt", &PROBE_N, &[(p_pos_1) as u32, (p_kind_2) as u32, (p_id_3) as u32, (p_near_4).0, (p_far_5).0], false); }
    let r_ = 'f: {
    F::BtlCharSnd_Request(c, p_pos_1, (1i32).wrapping_neg(), p_kind_2, p_near_4, p_far_5, p_id_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_RequestAt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_PlayCommon(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayCommon", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
        { let a1_ = (c.m.r32(p_chr_1) as i32); F::BtlCharSnd_Request(c, fp, a1_, 0i32, F32(0x43480000), F32(0x44bb8000), p_id_2) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayCommon", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_PlayCommonFar(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayCommonFar", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
        { let a1_ = (c.m.r32(p_chr_1) as i32); F::BtlCharSnd_Request(c, fp, a1_, 0i32, F32(0x47c35000), F32(0x47c35000), p_id_2) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayCommonFar", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_PlayBank8(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayBank8", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
        { let a1_ = (c.m.r32(p_chr_1) as i32); F::BtlCharSnd_Request(c, fp, a1_, 1i32, F32(0x43480000), F32(0x44bb8000), p_id_2) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayBank8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_PlayOwn(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayOwn", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 48i32, fp) };
        { let a1_ = (c.m.r32(p_chr_1) as i32); let a2_ = ((c.m.r32(p_chr_1) as i32)).wrapping_add(2i32); F::BtlCharSnd_Request(c, fp, a1_, a2_, F32(0x43480000), F32(0x44bb8000), p_id_2) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayOwn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_PlayStream(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayStream", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
        { let a1_ = (c.m.r32(p_chr_1) as i32); F::BtlCharSnd_Request(c, fp, a1_, 4i32, F32(0x43480000), F32(0x44bb8000), p_id_2) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayStream", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_PlayVoice(c: &mut Ctx, mut p_chr_1: u32, mut p_line_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayVoice", &PROBE_N, &[(p_chr_1) as u32, (p_line_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 48i32, fp) };
        { let a1_ = (c.m.r32(p_chr_1) as i32); let a2_ = ((c.m.r32(p_chr_1) as i32)).wrapping_add(5i32); F::BtlCharSnd_Request(c, fp, a1_, a2_, F32(0x43480000), F32(0x44bb8000), p_line_2) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayVoice", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_PlayTechSounds(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayTechSounds", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_line_2: i32 = 0;
    let mut l_se_3: i32 = 0;
    let mut l_obj_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let r_ = 'f: {
    l_line_2 = (1i32).wrapping_neg();
    l_se_3 = (1i32).wrapping_neg();
    l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
    l_slot_5 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((l_slot_5) < (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv6: i32 = l_slot_5;
        match sv6 {
            0i32 => {
                l_se_3 = (1i32).wrapping_neg();
                l_line_2 = 83i32;
                break 's1;
            }
            1i32 => {
                l_se_3 = (1i32).wrapping_neg();
                l_line_2 = 85i32;
                break 's1;
            }
            2i32 => {
                l_se_3 = 45i32;
                l_line_2 = 87i32;
                break 's1;
            }
            3i32 => {
                l_se_3 = 49i32;
                l_line_2 = 89i32;
                break 's1;
            }
            4i32 => {
                l_se_3 = 53i32;
                l_line_2 = 91i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    if (((((l_se_3) >= (0i32)) as i32)) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((65536i32) as u64))) != 0) {
            F::BtlCharSnd_PlayOwn(c, p_chr_1, l_se_3);
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((131072i32) as u64))) != 0) {
            if ((((((((!((((F::BtlOpp_GetParamWord0(c, p_chr_1)) & (128i32))) != 0)) as i32)) != 0) || ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, l_slot_5)) as u64)) as i32)) & (32i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                F::BtlCharSnd_PlayOwn(c, p_chr_1, (l_se_3).wrapping_add(1i32));
            }
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((262144i32) as u64))) != 0) {
            if ((((((((!((((F::BtlOpp_GetParamWord0(c, p_chr_1)) & (128i32))) != 0)) as i32)) != 0) || ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, l_slot_5)) as u64)) as i32)) & (64i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                F::BtlCharSnd_PlayOwn(c, p_chr_1, (l_se_3).wrapping_add(2i32));
            }
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((524288i32) as u64))) != 0) {
            F::BtlCharSnd_PlayOwn(c, p_chr_1, (l_se_3).wrapping_add(3i32));
        }
    }
    if (((((l_line_2) >= (0i32)) as i32)) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((32768i32) as u64))) != 0) {
            F::BtlCharSnd_PlayVoice(c, p_chr_1, l_line_2);
            F::BtlObj_SetSubState(c, l_obj_4, 2i32, l_line_2);
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((549755813888i64) as u64))) != 0) {
            F::BtlCharSnd_PlayVoice(c, p_chr_1, (l_line_2).wrapping_add(1i32));
            F::BtlObj_SetSubState(c, l_obj_4, 2i32, (l_line_2).wrapping_add(1i32));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_PlayTechSounds", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_Stop(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: i32, mut p_id_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_Stop", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32, (p_id_3) as u32], false); }
    let mut l_slot_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_slot_4 = F::BtlCharSnd_GetSet(c, p_chr_1);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((((((((((c.m.r32(l_slot_4) as i32)) >= (0i32)) as i32)) != 0) && ((((!(({ let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = ((c.m.r8((l_slot_4).wrapping_add(8))) as i32); let a2_ = (c.m.r32((l_slot_4).wrapping_add(4)) as i32); F::BtlCharSnd_IsRequested(c, a0_, a1_, a2_) }) != 0)) as i32)) != 0)) as i32)) != 0) && (((((((c.m.r8((l_slot_4).wrapping_add(8))) as i32)) == (p_kind_2)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((l_slot_4).wrapping_add(4)) as i32)) == (p_id_3)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = (c.m.r32(l_slot_4) as i32); F::Snd_StopHandle(c, a0_) };
                    { let v_ = (1i32).wrapping_neg(); let a_ = l_slot_4; c.m.w32(a_, (v_) as u32); };
                }
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_slot_4; let n_ = (o_).wrapping_add(12); l_slot_4 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_StopCommon(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_StopCommon", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    F::BtlCharSnd_Stop(c, p_chr_1, 0i32, p_id_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_StopCommon", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_StopOwn(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_StopOwn", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    { let a1_ = ((c.m.r32(p_chr_1) as i32)).wrapping_add(2i32); F::BtlCharSnd_Stop(c, p_chr_1, a1_, p_id_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_StopOwn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharSnd_GetBankMask(c: &mut Ctx, mut p_kind_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_GetBankMask", &PROBE_N, &[(p_kind_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gBtlSndBankMask).wrapping_add((((p_kind_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharSnd_GetBankMask", &PROBE_N, &[], true); }
    r_
}
