#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftFollow_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32(l_w_2) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        { let r_ = 128i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(132)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollow_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollowGroup_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollowGroup_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_m_3: u32 = 0;
    let r_ = 'f: {
    l_m_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = 804i32; F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_m_3, 0i32, ((804i32) as u32)); };
    { let a3_ = c.m.r32((p_src_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_src_2, (l_m_3).wrapping_add(4), ((((0i32) as i64 as u64)) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((1456i32) as i64 as u64)) as u32)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollowGroup_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollowGroup_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollowGroup_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollowGroup_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollowGroup_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollowGroup_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); F::EftEmit_BeginFrame(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollowGroup_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollowGroup_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollowGroup_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollowGroup_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_AllocPiece(c: &mut Ctx, mut p_w_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_AllocPiece", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_p_3 = (p_w_1).wrapping_add(1476);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_p_3).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = l_p_3; c.m.w32(a_, (v_) as u32); };
                    break 'f l_p_3;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_p_3; let n_ = (o_).wrapping_add(8); l_p_3 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_AllocPiece", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_FirePiece(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_FirePiece", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_parent_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_mgr_5: u32 = 0;
    let mut l_src_6: u32 = 0;
    let mut l_piece_7: u32 = 0;
    let mut l_h_8: u32 = 0;
    let r_ = 'f: {
        l_parent_3 = F::BtlTask_GetParent(c, p_task_2);
        l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
        l_mgr_5 = c.m.r32((l_parent_3).wrapping_add(56));
        l_src_6 = c.m.r32((l_w_4).wrapping_add(160));
        l_piece_7 = F::EftMulti_AllocPiece(c, l_w_4);
        if (((((l_piece_7) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let s_ = { rt::memset(c, fp.wrapping_add(192), 0, 96);; { let v_ = l_src_6; c.m.w32(fp.wrapping_add(192).wrapping_add(0), v_); }; { let v_ = c.m.r32((l_w_4).wrapping_add(1472)); c.m.w32(fp.wrapping_add(192).wrapping_add(4), v_); }; rt::memset(c, fp.wrapping_add(192).wrapping_add(16), 0, 16);; rt::memset(c, fp.wrapping_add(192).wrapping_add(32), 0, 16);; { let v_ = { let a_ = (c.m.r32((l_src_6).wrapping_add(36))).wrapping_add(48); F32(c.m.r32(a_)) }; c.m.w32(fp.wrapping_add(192).wrapping_add(48), (v_).0); }; { let v_ = { let a_ = (c.m.r32((l_src_6).wrapping_add(36))).wrapping_add(52); F32(c.m.r32(a_)) }; c.m.w32(fp.wrapping_add(192).wrapping_add(52), (v_).0); }; { let v_ = rt::fdiv(rt::i2f({ let a_ = (c.m.r32((l_src_6).wrapping_add(36))).wrapping_add(40); (c.m.r32(a_) as i32) }), F32(0x41f00000)); c.m.w32(fp.wrapping_add(192).wrapping_add(56), (v_).0); }; { let v_ = F32(0x3dcccccc); c.m.w32(fp.wrapping_add(192).wrapping_add(60), (v_).0); }; { let v_ = F32(0x3f666666); c.m.w32(fp.wrapping_add(192).wrapping_add(64), (v_).0); }; { let v_ = { let a_ = (c.m.r32((l_src_6).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }; c.m.w32(fp.wrapping_add(192).wrapping_add(68), (v_).0); }; { let v_ = F32(0x40600000); c.m.w32(fp.wrapping_add(192).wrapping_add(72), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(76), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(80), (v_).0); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(192).wrapping_add(84), v_); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(192).wrapping_add(85), v_); }; { let v_ = ((p_objId_1) as u8); c.m.w8(fp.wrapping_add(192).wrapping_add(86), v_); }; { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(192).wrapping_add(87), v_); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(192).wrapping_add(88), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(192).wrapping_add(92), (v_) as u32); }; fp.wrapping_add(192) }; let d_ = fp; rt::memcpy(c, d_, s_, 96); };
        F::Vec4_Copy(c, (fp).wrapping_add(16), (l_w_4).wrapping_add(48));
        if (((((c.m.r32(l_w_4) as i32)) & (512i32))) != 0) {
            { let v_ = ((2i32) as u8); let a_ = (fp).wrapping_add(87); c.m.w8(a_, v_); };
            's1: {
                let sv9: i32 = (({ let a_ = c.m.r32((l_src_6).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32);
                match sv9 {
                    343i32 => {
                        { let v_ = ((1i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                        if ((((((c.m.r32((l_w_4).wrapping_add(1556)) as i32)) == (0i32)) as i32)) != 0) {
                            { let v_ = F32(0x3e800000); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_).0); };
                        } else {
                            { let v_ = F32(0x3d23d70a); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_).0); };
                        }
                        if ((((((c.m.r32((l_w_4).wrapping_add(1556)) as i32)) < (2i32)) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = (((c.m.r32((l_w_4).wrapping_add(1556)) as i32)) & (1i32)); let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
                        }
                        { let v_ = F32(0x40800000); let a_ = (l_w_4).wrapping_add(24); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    403i32 => {
                        { let v_ = ((2i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                        { let v_ = (((rt::rem_i32((c.m.r32((l_w_4).wrapping_add(1556)) as i32), 2i32)) == (0i32)) as i32); let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
                        { let v_ = F32(0x3f800000); let a_ = (l_w_4).wrapping_add(24); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    404i32 => {
                        if ((((((c.m.r32((l_w_4).wrapping_add(1556)) as i32)) == (0i32)) as i32)) != 0) {
                            { let v_ = F32(0x3e4ccccc); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_).0); };
                        } else {
                            { let v_ = F32(0x3d23d70a); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_).0); };
                        }
                        { let v_ = ((3i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                        { let v_ = (((rt::rem_i32((c.m.r32((l_w_4).wrapping_add(1556)) as i32), 2i32)) == (0i32)) as i32); let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
                        { let v_ = F32(0x40a00000); let a_ = (l_w_4).wrapping_add(24); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    392i32 => {
                        { let v_ = ((0i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                        { let v_ = F32(0x3eb33333); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_).0); };
                        { let v_ = (((c.m.r32((l_w_4).wrapping_add(1556)) as i32)) & (1i32)); let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
                        { let v_ = F32(0x40a00000); let a_ = (l_w_4).wrapping_add(24); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        } else {
            { let v_ = ((4i32) as u8); let a_ = (fp).wrapping_add(87); c.m.w8(a_, v_); };
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_).0); };
            F::Vec4_Copy(c, (fp).wrapping_add(32), (l_w_4).wrapping_add(976));
            F::Mtx_StoreIdentity(c, fp.wrapping_add(96));
            F::BtlCharApi_GetRot(c, p_objId_1, fp.wrapping_add(160));
            { let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(4)))); F::Mtx_RotateY(c, fp.wrapping_add(96), fp.wrapping_add(96), a2_) };
            F::Vec3_Sub(c, fp.wrapping_add(176), (l_w_4).wrapping_add(64), (l_w_4).wrapping_add(80));
            F::Mtx_MulVec4(c, fp.wrapping_add(176), fp.wrapping_add(96), fp.wrapping_add(176));
            F::Vec3_Normalize(c, fp.wrapping_add(176), fp.wrapping_add(176));
            { let v_ = rt::fdiv({ let a0_ = F32(c.m.r32(fp.wrapping_add(176))); let a1_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(4)))); F::Port_atan2f(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
            's2: {
                let sv10: i32 = (({ let a_ = c.m.r32((l_src_6).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32);
                match sv10 {
                    0i32 => {
                        break 's2;
                    }
                    419i32 => {
                        { let v_ = 1i32; let a_ = (fp).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                        break 's2;
                    }
                    514i32 => {
                        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        }
        l_h_8 = F::EftDisc_Create(c, fp);
        { let v_ = l_h_8; let a_ = (l_piece_7).wrapping_add(4); c.m.w32(a_, v_); };
        if (((((l_h_8) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((((({ let a_ = c.m.r32((l_src_6).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (344i32)) as i32)) != 0) {
            { let _ = F::EftDisc_SetModelTex(c, l_h_8, (l_mgr_5).wrapping_add(8), 0i32, 0i32); };
        }
        if ((((((({ let a_ = c.m.r32((l_src_6).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (514i32)) as i32)) != 0) {
            { let a0_ = c.m.r32((l_piece_7).wrapping_add(4)); { let _ = F::EftDisc_SetModelTex(c, a0_, (l_mgr_5).wrapping_add(8), 0i32, 0i32); } };
            { let a0_ = c.m.r32((l_piece_7).wrapping_add(4)); let a1_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3c23d70a)), F32(0x3e051eb8)), F32(0xc0490fda)); F::EftDisc_SetSpin(c, a0_, a1_) };
        }
        { let r_ = 1i32; let a_ = l_piece_7; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a_ = (l_w_4).wrapping_add(1556); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32((l_w_4).wrapping_add(1556)) as i32)) >= ((({ let a_ = (c.m.r32((l_src_6).wrapping_add(36))).wrapping_add(9); (c.m.r8(a_) as i8) }) as i32))) as i32)) != 0) {
            if ((((((({ let a_ = (c.m.r32((l_src_6).wrapping_add(36))).wrapping_add(9); (c.m.r8(a_) as i8) }) as i32)) >= (2i32)) as i32)) != 0) {
                { let a0_ = c.m.r32((l_piece_7).wrapping_add(4)); F::EftDisc_SetLastHit(c, a0_, 1i32) };
            }
            { let r_ = 256i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_FirePiece", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_UpdatePieces(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_UpdatePieces", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_fire_4: i32 = 0;
    let mut l_release_5: i32 = 0;
    let mut l_alive_6: i32 = 0;
    let mut l_done_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_p_9: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_2).wrapping_add(56));
    l_fire_4 = 0i32;
    l_release_5 = 0i32;
    l_alive_6 = 0i32;
    if ((F::EftShot_TestBits(c, p_objId_1, 2i32)) != 0) {
        l_fire_4 = 1i32;
    } else {
        if ((F::EftShot_TestBits(c, p_objId_1, 4i32)) != 0) {
            l_release_5 = 1i32;
        } else {
            if ((((((((!((F::EftShot_TestBits(c, p_objId_1, 16i32)) != 0)) as i32)) != 0) && ((((!((F::EftShot_TestBits(c, p_objId_1, 32i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                l_fire_4 = (((F::EftShot_TestBits(c, p_objId_1, 64i32)) != (0i32)) as i32);
            }
        }
    }
    if (((((c.m.r32(l_w_3) as i32)) & (512i32))) != 0) {
        if ((l_fire_4) != 0) {
            F::EftMulti_FirePiece(c, p_objId_1, p_task_2);
        }
        l_done_7 = 0i32;
        if ((l_release_5) != 0) {
            {
                l_i_8 = 0i32;
                'l1: loop {
                    if !(((((l_i_8) < (10i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if ((((!((l_done_7) != 0)) as i32)) != 0) {
                            l_p_9 = ((l_w_3).wrapping_add(1476)).wrapping_add((((l_i_8) as u32)).wrapping_mul(8));
                            if (((((({ let a0_ = c.m.r32((l_p_9).wrapping_add(4)); F::EftDisc_IsAlive(c, a0_) }) != 0) && ((((!(((((c.m.r32(l_p_9) as i32)) & (2i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                                { let a0_ = c.m.r32((l_p_9).wrapping_add(4)); F::EftDisc_Release(c, a0_, (l_w_3).wrapping_add(48)) };
                                l_done_7 = 1i32;
                                { let a0_ = c.m.r32((l_p_9).wrapping_add(4)); let a1_ = F32(c.m.r32((l_w_3).wrapping_add(24))); F::EftDisc_SetScale(c, a0_, a1_) };
                                { let r_ = 2i32; let a_ = l_p_9; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                    }
                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                }
            }
        }
    } else {
        if ((l_fire_4) != 0) {
            F::BtlCharApi_GetNodePos(c, p_objId_1, 21i32, (l_w_3).wrapping_add(64));
        }
        if ((l_release_5) != 0) {
            F::BtlCharApi_GetNodePos(c, p_objId_1, 21i32, (l_w_3).wrapping_add(80));
            F::EftMulti_FirePiece(c, p_objId_1, p_task_2);
        }
    }
    {
        l_i_8 = 0i32;
        'l3: loop {
            if !(((((l_i_8) < (10i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                l_p_9 = ((l_w_3).wrapping_add(1476)).wrapping_add((((l_i_8) as u32)).wrapping_mul(8));
                if (({ let a0_ = c.m.r32((l_p_9).wrapping_add(4)); F::EftDisc_IsAlive(c, a0_) }) != 0) {
                    if (((((c.m.r32(l_w_3) as i32)) & (4i32))) != 0) {
                        { let a0_ = c.m.r32((l_p_9).wrapping_add(4)); F::EftDisc_SetFlag200(c, a0_) };
                    }
                    if (((((c.m.r32(l_w_3) as i32)) & (1i32))) != 0) {
                        { let a0_ = c.m.r32((l_p_9).wrapping_add(4)); F::EftDisc_Kill(c, a0_) };
                    }
                }
            }
            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
        }
    }
    {
        l_i_8 = 0i32;
        'l5: loop {
            if !(((((l_i_8) < (10i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                l_p_9 = ((l_w_3).wrapping_add(1476)).wrapping_add((((l_i_8) as u32)).wrapping_mul(8));
                if (({ let a0_ = c.m.r32((l_p_9).wrapping_add(4)); F::EftDisc_IsAlive(c, a0_) }) != 0) {
                    l_alive_6 = 1i32;
                } else {
                    { let v_ = ((0i32) as u32); let a_ = (l_p_9).wrapping_add(4); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
        }
    }
    break 'f l_alive_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_UpdatePieces", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_IsPartKind5(c: &mut Ctx, mut p_set_1: u32, mut p_group_2: i32, mut p_part_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_IsPartKind5", &PROBE_N, &[(p_set_1) as u32, (p_group_2) as u32, (p_part_3) as u32], false); }
    let r_ = 'f: {
    break 'f ((((({ let a_ = ((c.m.r32((p_set_1).wrapping_add(8))).wrapping_add(((((((c.m.r8((((p_set_1).wrapping_add(12)).wrapping_add((((p_group_2) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(p_part_3)) as u32)).wrapping_mul(64))).wrapping_add(10); c.m.r8(a_) }) as i32)) == (5i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_IsPartKind5", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_UpdateParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_mode_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_UpdateParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_mode_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_g_6: i32 = 0;
    let mut l_info_7: u32 = 0;
    let mut l_j_8: i32 = 0;
    let mut l_res_9: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_g_6 = 0i32;
        'l1: loop {
            if !(((((l_g_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_g_6) as u32)) as u32)))) != 0) {
                    l_info_7 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_g_6) as u32)).wrapping_mul(16)));
                    {
                        l_j_8 = 0i32;
                        'l3: loop {
                            if !(((((l_j_8) < (((c.m.r8((l_info_7).wrapping_add(((1i32) as u32)))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if ((((!(({ let a1_ = ((c.m.r8((l_info_7).wrapping_add(((0i32) as u32)))) as i32); F::EftMulti_IsPartKind5(c, p_set_3, a1_, l_j_8) }) != 0)) as i32)) != 0) {
                                    if (((((p_mode_4) == (0i32)) as i32)) != 0) {
                                        l_res_9 = { let a5_ = (((c.m.r32(l_w_5) as i32)) & (1i32)); let a6_ = (((c.m.r32(l_w_5) as i32)) & (4i32)); F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_5).wrapping_add(164), p_objId_1, l_g_6, l_j_8, a5_, a6_) };
                                    } else {
                                        l_res_9 = F::EftEmit_GetResetFlags(c, p_set_3, (l_w_5).wrapping_add(164), l_g_6, l_j_8);
                                    }
                                    if (((((c.m.r32(l_w_5) as i32)) & (128i32))) != 0) {
                                        l_res_9 = 2i32;
                                    }
                                    if (((((l_res_9) != (0i32)) as i32)) != 0) {
                                        { let a8_ = F32(c.m.r32((l_w_5).wrapping_add(16))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_5).wrapping_add(164), ((l_w_5).wrapping_add(880)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_w_5).wrapping_add(112), (l_w_5).wrapping_add(48), l_g_6, l_j_8, l_res_9, a8_) };
                                    }
                                }
                            }
                            { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_g_6; let n_ = (o_).wrapping_add(1); l_g_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_parent_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_mgr_5: u32 = 0;
    let mut l_def_6: u32 = 0;
    let r_ = 'f: {
    l_parent_3 = F::BtlTask_GetParent(c, p_task_1);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_mgr_5 = c.m.r32((l_parent_3).wrapping_add(56));
    { let _ = F::memset(c, l_w_4, 0i32, ((1568i32) as u32)); };
    { let v_ = p_src_2; let a_ = (l_w_4).wrapping_add(160); c.m.w32(a_, v_); };
    l_def_6 = c.m.r32((p_src_2).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(52))); let a_ = (l_w_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(56))); let a_ = (l_w_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(12))); let a_ = (l_w_4).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = (l_mgr_5).wrapping_add(528); let a_ = (l_w_4).wrapping_add(1472); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, (l_mgr_5).wrapping_add(528), (l_w_4).wrapping_add(164));
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(1472)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(36); c.m.w32(a_, (v_).0); };
    if (((({ let a_ = (c.m.r32((p_src_2).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (32768i32))) != 0) {
        { let r_ = 512i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = 1024i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a1_ = (if ((((((c.m.r32(p_src_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32((l_w_2).wrapping_add(160));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(1472)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(164)) };
    if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_advance_4: i32 = 0;
    let mut l_alive_5: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32((l_w_2).wrapping_add(160));
    l_advance_4 = 0i32;
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
        break 'f;
    }
    F::EftEmit_UpdateNodesReq(c, l_src_3, (l_w_2).wrapping_add(880));
    if ((((!(((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0)) as i32)) != 0) {
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
            { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                { let v_ = ((2i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if (((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0) {
        's1: {
            let sv6: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
            match sv6 {
                0i32 => {
                    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
                        { let _ = { let a2_ = (c.m.r32(l_src_3) as i32); F::EftAim_GetDir(c, (l_w_2).wrapping_add(48), (((((l_w_2).wrapping_add(912)) as u64)) as i32), a2_) }; };
                        { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                2i32 => {
                    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                        F::Vec4_Copy(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(976));
                        F::Vec4_Copy(c, (l_w_2).wrapping_add(112), (l_w_2).wrapping_add(96));
                        { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_2).wrapping_add(128)) };
                        { let a3_ = (c.m.r32(l_src_3) as i32); { let _ = F::EftAim_GetDirKeep(c, l_src_3, (l_w_2).wrapping_add(48), (((((l_w_2).wrapping_add(112)) as u64)) as i32), a3_); } };
                        { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(4))); F::Vec3_Scale(c, (l_w_2).wrapping_add(144), (l_w_2).wrapping_add(48), a2_) };
                        { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                4i32 => {
                    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                        { let r_ = 9i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = ((5i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                1i32 | 3i32 | 5i32 => {
                    l_advance_4 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    l_alive_5 = { let a0_ = (c.m.r32(l_src_3) as i32); F::EftMulti_UpdatePieces(c, a0_, p_task_1) };
    if ((((((((!((l_alive_5) != 0)) as i32)) != 0) && (((((c.m.r32(l_w_2) as i32)) & (256i32))) != 0)) as i32)) != 0) {
        { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
        if ((((!(((((c.m.r32(l_w_2) as i32)) & (8i32))) != 0)) as i32)) != 0) {
            { let r_ = 4i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a0_ = (c.m.r32(l_src_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(1472)); F::EftMulti_UpdateParts(c, a0_, p_task_1, a2_, 0i32) };
    if ((l_advance_4) != 0) {
        { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
    }
    if (((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if ((((((((!((l_alive_5) != 0)) as i32)) != 0) && (((((c.m.r32(l_w_2) as i32)) & (2i32))) != 0)) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        if (((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) {
            if (((((c.m.r32(l_w_2) as i32)) & (4i32))) != 0) {
                { let r_ = 2i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(32))), F32(c.m.r32((l_w_2).wrapping_add(36)))) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        } else {
            if (((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                if (((((c.m.r32(l_w_2) as i32)) & (16i32))) != 0) {
                    { let a0_ = c.m.r32((l_w_2).wrapping_add(1472)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_2).wrapping_add(164)) };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        if (({ let a0_ = { let a_ = c.m.r32((l_w_2).wrapping_add(160)); (c.m.r32(a_) as i32) }; F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1472)); { let _ = F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(164)); } };
        if ((((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (1i32))) as u16)) != 0) {
            F::Vec3_Sub(c, fp, (p_task_1).wrapping_add(16), (l_w_2).wrapping_add(112));
            F::Vec3_Copy(c, (l_w_2).wrapping_add(112), (p_task_1).wrapping_add(16));
            F::Vec3_Add(c, (l_w_2).wrapping_add(128), (l_w_2).wrapping_add(128), fp);
            if ((((!(((((c.m.r32(l_w_2) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                { let r_ = 32i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            { let r_ = (!(32i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (4i32))) != 0) {
            { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32(l_w_2) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        { let r_ = 128i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1472)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(164)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftMulti_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMulti_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftMultiMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMultiMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_m_3: u32 = 0;
    let r_ = 'f: {
    l_m_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = 1328i32; F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_m_3, 0i32, ((1328i32) as u32)); };
    if ((((((((((({ let a_ = c.m.r32((p_src_2).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (344i32)) as i32)) != 0) || ((((((({ let a_ = c.m.r32((p_src_2).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (514i32)) as i32)) != 0)) as i32)) != 0) {
        { let a1_ = { let a0_ = c.m.r32((p_src_2).wrapping_add(28)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; F::EftTexSet_Load32(c, (l_m_3).wrapping_add(8), a1_) };
    }
    { let a3_ = c.m.r32((p_src_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_src_2, (l_m_3).wrapping_add(528), ((((0i32) as i64 as u64)) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((1568i32) as i64 as u64)) as u32)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMultiMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftMultiMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMultiMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(528); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMultiMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftMultiMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMultiMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_m_2: u32 = 0;
    let r_ = 'f: {
    l_m_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::EftEmit_BeginFrame(c, (l_m_2).wrapping_add(528));
    { let v_ = 0i32; let a_ = (l_m_2).wrapping_add(524); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMultiMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftMultiMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMultiMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMultiMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_InitProp(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_InitProp", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_parent_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_mgr_4: u32 = 0;
    let mut l_prop_5: u32 = 0;
    let r_ = 'f: {
    l_parent_2 = F::BtlTask_GetParent(c, p_task_1);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_mgr_4 = c.m.r32((l_parent_2).wrapping_add(56));
    l_prop_5 = (l_w_3).wrapping_add(1456);
    { let _ = F::memset(c, l_prop_5, 0i32, 112u32); };
    's1: {
        let sv6: i32 = (({ let a_ = { let a_ = (c.m.r32((l_w_3).wrapping_add(128))).wrapping_add(36); c.m.r32(a_) }; (c.m.r16(a_) as i16) }) as i32);
        match sv6 {
            357i32 => {
                { let v_ = F32(0x3ecccccc); let a_ = (l_prop_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fadd(rt::fmul(F::BtlScene_RandF(c), F32(0x3dcccccc)), F32(0x3d4ccccc)), F32(0x40490fda)); let a_ = (l_prop_5).wrapping_add(96); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(rt::fmul(F::BtlScene_RandF(c), F32(0x3ecccccc)), F32(0x3ecccccc)); let a_ = (l_prop_5).wrapping_add(88); c.m.w32(a_, (v_).0); };
                { let r_ = 48i32; let a_ = (l_prop_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            358i32 => {
                { let v_ = F32(0x3f199999); let a_ = (l_prop_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let r_ = 16i32; let a_ = (l_prop_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            410i32 => {
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let r_ = 8i32; let a_ = (l_prop_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            503i32 => {
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let r_ = 16i32; let a_ = (l_prop_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            504i32 => {
                { let v_ = F32(0x40400000); let a_ = (l_prop_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let r_ = 8i32; let a_ = (l_prop_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            505i32 => {
                { let v_ = F32(0x40000000); let a_ = (l_prop_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let r_ = 8i32; let a_ = (l_prop_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            705i32 => {
                { let v_ = F32(0x3f800000); let a_ = (l_prop_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x40a00000); let a_ = (l_prop_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let r_ = 24i32; let a_ = (l_prop_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = { let a1_ = c.m.r32((l_mgr_4).wrapping_add(804)); F::EftObj_Create(c, l_prop_5, a1_) }; let a_ = (l_prop_5).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
    { let a0_ = (c.m.r32((l_prop_5).wrapping_add(100)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_InitProp", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_UpdateProp(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_UpdateProp", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_prop_4: u32 = 0;
    let mut l_pitch_5: F32 = F32(0);
    let mut l_yaw_6: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(128));
        { let _ = F::memset(c, fp, 0i32, 64u32); };
        l_prop_4 = (l_w_2).wrapping_add(1456);
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
            if (((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & (16i32))) != 0) {
                { let r_ = 64i32; let a_ = (l_prop_4).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            F::Vec4_Copy(c, ((l_w_2).wrapping_add(1456)).wrapping_add(48), (l_w_2).wrapping_add(880));
            { let r_ = 1i32; let a_ = (l_prop_4).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
            if ((((!(((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                { let r_ = 64i32; let a_ = (l_prop_4).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let v_ = (((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & ((!(1i32))))) | (2i32)); let a_ = (l_prop_4).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        }
        if (((((c.m.r32(l_w_2) as i32)) & (5i32))) != 0) {
            { let r_ = (!(64i32)); let a_ = (l_prop_4).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & (1i32))) != 0) {
            if (((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & (16i32))) != 0) {
                F::Vec4_Copy(c, (l_prop_4).wrapping_add(48), (l_w_2).wrapping_add(880));
            }
            if (((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & (32i32))) != 0) {
                { let r_ = F32(c.m.r32((l_prop_4).wrapping_add(96))); let a_ = (l_prop_4).wrapping_add(92); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul({ let a0_ = { let a0_ = F32(c.m.r32((l_prop_4).wrapping_add(92))); F::Mathf_Cos(c, a0_) }; F::EftMath_WrapAngle(c, a0_) }, F32(c.m.r32((l_prop_4).wrapping_add(88)))); let a_ = ((l_prop_4).wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
        if (((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & (2i32))) != 0) {
            F::Vec4_Copy(c, (l_prop_4).wrapping_add(48), (l_w_2).wrapping_add(80));
            { let v_ = F32(c.m.r32((l_prop_4).wrapping_add(84))); let a_ = (l_prop_4).wrapping_add(80); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & (64i32))) != 0) {
            l_pitch_5 = { let a0_ = { let a0_ = rt::fneg(F32(c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) };
            l_yaw_6 = { let a0_ = { let a0_ = F32(c.m.r32((l_w_2).wrapping_add(48))); let a1_ = F32(c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) };
            F::Mtx_StoreIdentity(c, fp);
            { let a2_ = F32(c.m.r32((l_prop_4).wrapping_add(80))); F::Mtx_ScaleDiagUniform(c, fp, fp, a2_) };
            if (((((c.m.r32((l_prop_4).wrapping_add(104)) as i32)) & (8i32))) != 0) {
                F::Mtx_RotateY(c, fp, fp, F32(0x40490fda));
            }
            F::Mtx_RotateX(c, fp, fp, l_pitch_5);
            F::Mtx_RotateY(c, fp, fp, l_yaw_6);
            F::Mtx_Translate(c, fp, fp, (l_prop_4).wrapping_add(64));
            F::Mtx_Translate(c, fp, fp, (l_prop_4).wrapping_add(48));
            { let a0_ = (c.m.r32((l_prop_4).wrapping_add(100)) as i32); F::EftObj_SetMtx(c, a0_, fp) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_UpdateProp", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_ShowProp(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_ShowProp", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_prop_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_prop_3 = (l_w_2).wrapping_add(1456);
    l_src_4 = c.m.r32((l_w_2).wrapping_add(128));
    if (((((c.m.r32((l_prop_3).wrapping_add(104)) as i32)) & (64i32))) != 0) {
        if ((((!(({ let a0_ = (c.m.r32(l_src_4) as i32); F::BtlScene_IsEffectHidden(c, a0_, 1i32) }) != 0)) as i32)) != 0) {
            { let a0_ = (c.m.r32((l_prop_3).wrapping_add(100)) as i32); F::EftObj_SetVisible(c, a0_, 1i32) };
        } else {
            { let a0_ = (c.m.r32((l_prop_3).wrapping_add(100)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
        }
    } else {
        { let a0_ = (c.m.r32((l_prop_3).wrapping_add(100)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_ShowProp", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_FreeProp(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_FreeProp", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = ((c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(1456)).wrapping_add(100); (c.m.r32(a_) as i32) }; { let _ = F::EftObj_Destroy(c, a0_); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_FreeProp", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_AddHit(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_AddHit", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_r_4: F32 = F32(0);
    let mut l_a_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let mut l_a_7: u32 = 0;
    let mut l_b_8: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    { let s_ = (l_w_2).wrapping_add(64); let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (l_w_2).wrapping_add(80); let d_ = (l_rec_3).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (l_w_2).wrapping_add(96); let d_ = (l_rec_3).wrapping_add(48); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (l_w_2).wrapping_add(112); let d_ = (l_rec_3).wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
    l_r_4 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(16))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(132)));
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_w_2).wrapping_add(128)); let a_ = (l_rec_3).wrapping_add(100); c.m.w32(a_, v_); };
    's1: {
        let sv9: i32 = (({ let a_ = ({ let a_ = (c.m.r32((l_w_2).wrapping_add(128))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(8); (c.m.r8(a_) as i8) }) as i32);
        match sv9 {
            1i32 => {
                {
                    l_a_5 = F::EftHitArena_AllocBox(c);
                    l_b_6 = F::EftHitArena_AllocBox(c);
                    F::ColCapsule_Set(c, l_a_5, (l_w_2).wrapping_add(944), (l_w_2).wrapping_add(80), l_r_4);
                    F::ColCapsule_Set(c, l_b_6, (l_w_2).wrapping_add(944), (l_w_2).wrapping_add(96), l_r_4);
                    F::EftHit_SetShapeBoxes(c, l_rec_3, l_a_5, l_b_6);
                    break 's1;
                }
            }
            0i32 => {
                {
                    l_a_7 = F::EftHitArena_AllocSphere(c);
                    l_b_8 = F::EftHitArena_AllocSphere(c);
                    F::ColSphere_Set(c, l_a_7, (l_w_2).wrapping_add(80), l_r_4);
                    F::ColSphere_Set(c, l_b_8, (l_w_2).wrapping_add(96), l_r_4);
                    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_7, l_b_8);
                    break 's1;
                }
            }
            _ => {
                break 'f;
            }
        }
    }
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_AddHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_UpdateBlur(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_UpdateBlur", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_on_4: i32 = 0;
    let mut l_off_5: i32 = 0;
    let mut l_t_6: F32 = F32(0);
    let mut l_a_7: i32 = 0;
    let mut l_b_8: i32 = 0;
    let mut l_c_9: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(128));
        if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(72); (c.m.r16(a_) as i16) }) as i32)) != (2i32)) as i32)) != 0) {
            rt::memset(c, fp.wrapping_add(16), 0, 24);
            { let v_ = 2i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
            { let v_ = 8i32; c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_) as u32); }
            { let v_ = 16i32; c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_) as u32); }
            { let v_ = 32i32; c.m.w32(fp.wrapping_add(16).wrapping_add(16), (v_) as u32); }
            { let v_ = 64i32; c.m.w32(fp.wrapping_add(16).wrapping_add(20), (v_) as u32); }
            l_on_4 = { let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(72); (c.m.r16(a_) as i16) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            l_off_5 = { let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(74); (c.m.r16(a_) as i16) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, l_on_4) }) != 0) {
                { let r_ = 1024i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let _ = { let a2_ = (c.m.r32(l_src_3) as i32); F::EftAim_GetDir(c, (l_w_2).wrapping_add(48), ((((fp) as u64)) as i32), a2_) }; };
            }
            if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, l_off_5) }) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let r_ = (!(1024i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((c.m.r32(l_w_2) as i32)) & (1024i32))) != 0) {
                { let r_ = F32(0x3dcccccc); let a_ = (l_w_2).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(32))), F32(0x3f800000)) as i32)) != 0) {
                    { let v_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                }
            }
            l_t_6 = F32(c.m.r32((l_w_2).wrapping_add(32)));
            l_a_7 = rt::f2i(rt::fmul(l_t_6, F32(0x42000000)));
            l_b_8 = rt::f2i(rt::fmul(l_t_6, F32(0x42800000)));
            l_c_9 = rt::f2i(rt::fmul(l_t_6, F32(0x43000000)));
            F::StgBlur_SetColor0Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_a_7) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor1Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_b_8) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor2Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_c_9) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor3Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((64i32) as i64 as u64)) as u8));
            F::StgBlur_SetCenter(c, 0i32, (l_w_2).wrapping_add(48), 0i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_UpdateBlur", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_UpdateParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_mode_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_UpdateParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_mode_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_g_6: i32 = 0;
    let mut l_info_7: u32 = 0;
    let mut l_j_8: i32 = 0;
    let mut l_res_9: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_g_6 = 0i32;
        'l1: loop {
            if !(((((l_g_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_g_6) as u32)) as u32)))) != 0) {
                    l_info_7 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_g_6) as u32)).wrapping_mul(16)));
                    {
                        l_j_8 = 0i32;
                        'l3: loop {
                            if !(((((l_j_8) < (((c.m.r8((l_info_7).wrapping_add(((1i32) as u32)))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((p_mode_4) == (0i32)) as i32)) != 0) {
                                    l_res_9 = { let a5_ = (((c.m.r32(l_w_5) as i32)) & (1i32)); let a6_ = (((c.m.r32(l_w_5) as i32)) & (4i32)); F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_5).wrapping_add(132), p_objId_1, l_g_6, l_j_8, a5_, a6_) };
                                } else {
                                    l_res_9 = F::EftEmit_GetResetFlags(c, p_set_3, (l_w_5).wrapping_add(132), l_g_6, l_j_8);
                                }
                                if (((((c.m.r32(l_w_5) as i32)) & (128i32))) != 0) {
                                    l_res_9 = 2i32;
                                }
                                if (((((l_res_9) != (0i32)) as i32)) != 0) {
                                    { let a8_ = F32(c.m.r32((l_w_5).wrapping_add(12))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_5).wrapping_add(132), ((l_w_5).wrapping_add(848)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_w_5).wrapping_add(80), (l_w_5).wrapping_add(48), l_g_6, l_j_8, l_res_9, a8_) };
                                }
                            }
                            { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_g_6; let n_ = (o_).wrapping_add(1); l_g_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_parent_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_mgr_5: u32 = 0;
    let mut l_def_6: u32 = 0;
    let r_ = 'f: {
    l_parent_3 = F::BtlTask_GetParent(c, p_task_1);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_mgr_5 = c.m.r32((l_parent_3).wrapping_add(56));
    { let _ = F::memset(c, l_w_4, 0i32, ((1584i32) as u32)); };
    { let v_ = p_src_2; let a_ = (l_w_4).wrapping_add(128); c.m.w32(a_, v_); };
    l_def_6 = c.m.r32((p_src_2).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(52))); let a_ = (l_w_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(8))); let a_ = (l_w_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = (l_mgr_5).wrapping_add(4); let a_ = (l_w_4).wrapping_add(1440); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, (l_mgr_5).wrapping_add(4), (l_w_4).wrapping_add(132));
    if (({ let a0_ = c.m.r32((l_w_4).wrapping_add(1440)); F::EftEmit_HasWidth2(c, a0_) }) != 0) {
        { let v_ = F::EftEmit_GetWidth2(c, (l_w_4).wrapping_add(132)); let a_ = (l_w_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let r_ = 512i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(1440)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(28); c.m.w32(a_, (v_).0); };
    F::EftPropShot_InitProp(c, p_task_1);
    if ((((((({ let a_ = { let a_ = (c.m.r32((l_w_4).wrapping_add(128))).wrapping_add(36); c.m.r32(a_) }; (c.m.r16(a_) as i16) }) as i32)) == (410i32)) as i32)) != 0) {
        { let r_ = 2048i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a1_ = (if ((((((c.m.r32(p_src_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32((l_w_2).wrapping_add(128));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(132)) };
    F::EftPropShot_FreeProp(c, p_task_1);
    if ((((((((!(((((c.m.r32(l_w_2) as i32)) & (2048i32))) != 0)) as i32)) != 0) && ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_advance_4: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(128));
        l_advance_4 = 0i32;
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) == (0i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_PlayFireSound(c, a0_, l_src_3) };
        }
        F::EftEmit_UpdateNodesReq(c, l_src_3, (l_w_2).wrapping_add(848));
        if ((((!(((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
                { let _ = { let a2_ = (c.m.r32(l_src_3) as i32); F::EftAim_GetDir(c, (l_w_2).wrapping_add(48), (((((l_w_2).wrapping_add(880)) as u64)) as i32), a2_) }; };
                if (((((c.m.r32(l_w_2) as i32)) & (256i32))) != 0) {
                    { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 48i32, fp) };
                    F::Vec3_Sub(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(880), fp);
                    F::Vec3_Normalize(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(48));
                }
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                    { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0) {
            's1: {
                let sv5: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
                match sv5 {
                    0i32 => {
                        if (((((c.m.r32(l_w_2) as i32)) & (256i32))) != 0) {
                            { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 48i32, fp) };
                            F::Vec3_Sub(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(880), fp);
                            F::Vec3_Normalize(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(48));
                        }
                        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                            { let r_ = 16i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            F::Vec4_Copy(c, (l_w_2).wrapping_add(64), (l_w_2).wrapping_add(944));
                            F::Vec4_Copy(c, (l_w_2).wrapping_add(80), (l_w_2).wrapping_add(64));
                            { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_2).wrapping_add(96)) };
                            { let a3_ = (c.m.r32(l_src_3) as i32); { let _ = F::EftAim_GetDirKeep(c, l_src_3, (l_w_2).wrapping_add(48), (((((l_w_2).wrapping_add(80)) as u64)) as i32), a3_); } };
                            { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(4))); F::Vec3_Scale(c, (l_w_2).wrapping_add(112), (l_w_2).wrapping_add(48), a2_) };
                            if (((((c.m.r32(l_w_2) as i32)) & (2048i32))) != 0) {
                                { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjSetFlag100(c, a0_, 1i32) };
                            }
                            { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        }
                        break 's1;
                    }
                    2i32 => {
                        if ((((!(((((c.m.r32(l_w_2) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                            F::Vec4_Copy(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(80));
                        }
                        if ((((!(((({ let a_ = ({ let a_ = (c.m.r32((l_w_2).wrapping_add(128))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
                            if ((((!(((((c.m.r32(l_w_2) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                                if (((rt::fgt({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
                                    { let a3_ = (c.m.r32(l_src_3) as i32); let a4_ = F32(c.m.r32((l_w_2).wrapping_add(4))); let a5_ = { let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }; F::EftAim_Home(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(80), (l_w_2).wrapping_add(48), a3_, a4_, a5_) };
                                    { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(4))); F::Vec3_Scale(c, (l_w_2).wrapping_add(112), (l_w_2).wrapping_add(48), a2_) };
                                }
                                F::Vec3_Add(c, (l_w_2).wrapping_add(80), (l_w_2).wrapping_add(80), (l_w_2).wrapping_add(112));
                            } else {
                                F::Vec3_Copy(c, (l_w_2).wrapping_add(80), (p_task_1).wrapping_add(16));
                            }
                        }
                        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                            { let r_ = 9i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        }
                        break 's1;
                    }
                    1i32 | 3i32 => {
                        l_advance_4 = 1i32;
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
            if ((((!(((((c.m.r32(l_w_2) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                { let r_ = 4i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a0_ = (c.m.r32(l_src_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftPropShot_UpdateParts(c, a0_, p_task_1, a2_, 0i32) };
        F::EftPropShot_UpdateProp(c, p_task_1);
        if (((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (512i32))) != 0) {
            F::EftPropShot_UpdateBlur(c, p_task_1);
        }
        if ((l_advance_4) != 0) {
            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        if (((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((((((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) && ((((!(((((c.m.r32(l_w_2) as i32)) & (2i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            if (((((((((c.m.r32(l_w_2) as i32)) & (2048i32))) != 0) && (((((((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(24))), F32(c.m.r32((l_w_2).wrapping_add(28)))) as i32)) != 0) || (((((c.m.r32(l_w_2) as i32)) & (4i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                if ((((!(((((c.m.r32(l_w_2) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
                    if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
                        { let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
                    }
                    { let r_ = 4096i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((!(({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjHasFlags42(c, a0_) }) != 0)) as i32)) != 0) {
                        { let r_ = (!(4096i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
        if (((((c.m.r32(l_w_2) as i32)) & (2i32))) != 0) {
            if (((((c.m.r32(l_w_2) as i32)) & (2048i32))) != 0) {
                { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjSetFlag100(c, a0_, 0i32) };
            }
            F::BtlTask_SetDead(c, p_task_1);
        } else {
            if (((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) {
                if ((((!(((((c.m.r32(l_w_2) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
                    if (((((c.m.r32(l_w_2) as i32)) & (4i32))) != 0) {
                        { let r_ = 2i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(24))), F32(c.m.r32((l_w_2).wrapping_add(28)))) as i32)) != 0) {
                            { let r_ = 2i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
            } else {
                if (((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                    if (((((c.m.r32(l_w_2) as i32)) & (16i32))) != 0) {
                        { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_2).wrapping_add(132)) };
                        F::EftPropShot_AddHit(c, p_task_1);
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(128));
        F::EftPropShot_ShowProp(c, p_task_1);
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); { let _ = F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(132)); } };
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (1i32))) != 0) {
            { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_MarkKind6(c, a0_, (l_w_2).wrapping_add(132)) };
        }
        if ((((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (1i32))) as u16)) != 0) {
            F::Vec3_Sub(c, fp, (p_task_1).wrapping_add(16), (l_w_2).wrapping_add(80));
            F::Vec3_Copy(c, (l_w_2).wrapping_add(80), (p_task_1).wrapping_add(16));
            F::Vec3_Add(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(96), fp);
            if ((((!(((((c.m.r32(l_w_2) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                { let r_ = 32i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            { let r_ = (!(32i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (4i32))) != 0) {
            { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32(l_w_2) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        { let r_ = 128i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(132)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShot_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShot_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShotMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShotMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_m_3: u32 = 0;
    let mut l_costume_4: u8 = 0;
    let r_ = 'f: {
    l_m_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = 808i32; F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_m_3, 0i32, ((808i32) as u32)); };
    if ((((((({ let a_ = c.m.r32((p_src_2).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (410i32)) as i32)) != 0) {
        l_costume_4 = (({ let a0_ = (c.m.r32((p_src_2).wrapping_add(32)) as i32); F::BtlCharApi_GetCostume(c, a0_) }) as u8);
        { let v_ = { let a0_ = { let a0_ = c.m.r32((p_src_2).wrapping_add(28)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; F::BtlScene_GetPackEntry(c, a0_, (((l_costume_4) as i32)).wrapping_add(1i32)) }; let a_ = (l_m_3).wrapping_add(804); c.m.w32(a_, v_); };
    } else {
        { let v_ = { let a0_ = c.m.r32((p_src_2).wrapping_add(28)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; let a_ = (l_m_3).wrapping_add(804); c.m.w32(a_, v_); };
    }
    { let a3_ = c.m.r32((p_src_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_src_2, (l_m_3).wrapping_add(4), ((((0i32) as i64 as u64)) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((1584i32) as i64 as u64)) as u32)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShotMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShotMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShotMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShotMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShotMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShotMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); F::EftEmit_BeginFrame(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShotMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftPropShotMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPropShotMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPropShotMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_AddHit(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_AddHit", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_r_4: F32 = F32(0);
    let mut l_a_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let mut l_a_7: u32 = 0;
    let mut l_b_8: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    l_r_4 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(16))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(148)));
    { let s_ = (l_w_2).wrapping_add(80); let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (l_w_2).wrapping_add(96); let d_ = (l_rec_3).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (l_w_2).wrapping_add(112); let d_ = (l_rec_3).wrapping_add(48); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (l_w_2).wrapping_add(128); let d_ = (l_rec_3).wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_w_2).wrapping_add(144)); let a_ = (l_rec_3).wrapping_add(100); c.m.w32(a_, v_); };
    's1: {
        let sv9: i32 = (({ let a_ = ({ let a_ = (c.m.r32((l_w_2).wrapping_add(144))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(8); (c.m.r8(a_) as i8) }) as i32);
        match sv9 {
            1i32 => {
                {
                    l_a_5 = F::EftHitArena_AllocBox(c);
                    l_b_6 = F::EftHitArena_AllocBox(c);
                    F::ColCapsule_Set(c, l_a_5, (l_w_2).wrapping_add(960), (l_w_2).wrapping_add(96), l_r_4);
                    F::ColCapsule_Set(c, l_b_6, (l_w_2).wrapping_add(960), (l_w_2).wrapping_add(112), l_r_4);
                    F::EftHit_SetShapeBoxes(c, l_rec_3, l_a_5, l_b_6);
                    break 's1;
                }
            }
            0i32 => {
                {
                    l_a_7 = F::EftHitArena_AllocSphere(c);
                    l_b_8 = F::EftHitArena_AllocSphere(c);
                    F::ColSphere_Set(c, l_a_7, (l_w_2).wrapping_add(96), l_r_4);
                    F::ColSphere_Set(c, l_b_8, (l_w_2).wrapping_add(112), l_r_4);
                    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_7, l_b_8);
                    break 's1;
                }
            }
            _ => {
                break 'f;
            }
        }
    }
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_AddHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_UpdateBlur(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_UpdateBlur", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_on_4: i32 = 0;
    let mut l_off_5: i32 = 0;
    let mut l_t_6: F32 = F32(0);
    let mut l_a_7: i32 = 0;
    let mut l_b_8: i32 = 0;
    let mut l_c_9: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(144));
        if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(72); (c.m.r16(a_) as i16) }) as i32)) != (2i32)) as i32)) != 0) {
            rt::memset(c, fp.wrapping_add(16), 0, 24);
            { let v_ = 2i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
            { let v_ = 8i32; c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_) as u32); }
            { let v_ = 16i32; c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_) as u32); }
            { let v_ = 32i32; c.m.w32(fp.wrapping_add(16).wrapping_add(16), (v_) as u32); }
            { let v_ = 64i32; c.m.w32(fp.wrapping_add(16).wrapping_add(20), (v_) as u32); }
            l_on_4 = { let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(72); (c.m.r16(a_) as i16) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            l_off_5 = { let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(74); (c.m.r16(a_) as i16) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, l_on_4) }) != 0) {
                { let r_ = 1024i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let _ = { let a2_ = (c.m.r32(l_src_3) as i32); F::EftAim_GetDir(c, (l_w_2).wrapping_add(64), ((((fp) as u64)) as i32), a2_) }; };
            }
            if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, l_off_5) }) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let r_ = (!(1024i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((c.m.r32(l_w_2) as i32)) & (1024i32))) != 0) {
                { let r_ = F32(0x3dcccccc); let a_ = (l_w_2).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(32))), F32(0x3f800000)) as i32)) != 0) {
                    { let v_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                }
            }
            l_t_6 = F32(c.m.r32((l_w_2).wrapping_add(32)));
            l_a_7 = rt::f2i(rt::fmul(l_t_6, F32(0x42000000)));
            l_b_8 = rt::f2i(rt::fmul(l_t_6, F32(0x42800000)));
            l_c_9 = rt::f2i(rt::fmul(l_t_6, F32(0x43000000)));
            F::StgBlur_SetColor0Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_a_7) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor1Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_b_8) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor2Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_c_9) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor3Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((64i32) as i64 as u64)) as u8));
            F::StgBlur_SetCenter(c, 0i32, (l_w_2).wrapping_add(64), 0i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_UpdateBlur", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_UpdateParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_mode_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_UpdateParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_mode_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_g_6: i32 = 0;
    let mut l_info_7: u32 = 0;
    let mut l_j_8: i32 = 0;
    let mut l_res_9: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_g_6 = 0i32;
        'l1: loop {
            if !(((((l_g_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_g_6) as u32)) as u32)))) != 0) {
                    l_info_7 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_g_6) as u32)).wrapping_mul(16)));
                    {
                        l_j_8 = 0i32;
                        'l3: loop {
                            if !(((((l_j_8) < (((c.m.r8((l_info_7).wrapping_add(((1i32) as u32)))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((p_mode_4) == (0i32)) as i32)) != 0) {
                                    l_res_9 = { let a5_ = (((c.m.r32(l_w_5) as i32)) & (1i32)); let a6_ = (((c.m.r32(l_w_5) as i32)) & (4i32)); F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_5).wrapping_add(148), p_objId_1, l_g_6, l_j_8, a5_, a6_) };
                                } else {
                                    l_res_9 = F::EftEmit_GetResetFlags(c, p_set_3, (l_w_5).wrapping_add(148), l_g_6, l_j_8);
                                }
                                if (((((c.m.r32(l_w_5) as i32)) & (128i32))) != 0) {
                                    l_res_9 = 2i32;
                                }
                                if (((((l_res_9) != (0i32)) as i32)) != 0) {
                                    { let a8_ = F32(c.m.r32((l_w_5).wrapping_add(12))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_5).wrapping_add(148), ((l_w_5).wrapping_add(864)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_w_5).wrapping_add(96), (l_w_5).wrapping_add(48), l_g_6, l_j_8, l_res_9, a8_) };
                                }
                            }
                            { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_g_6; let n_ = (o_).wrapping_add(1); l_g_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_parent_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_src_5: u32 = 0;
    let mut l_mgr_6: u32 = 0;
    let mut l_def_7: u32 = 0;
    let mut l_ratio_8: F32 = F32(0);
    let mut l_ratio_9: F32 = F32(0);
    let mut l_add_10: F32 = F32(0);
    let r_ = 'f: {
    l_parent_3 = F::BtlTask_GetParent(c, p_task_1);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_5 = p_arg_2;
    l_mgr_6 = c.m.r32((l_parent_3).wrapping_add(56));
    { let _ = F::memset(c, l_w_4, 0i32, ((1472i32) as u32)); };
    { let v_ = l_src_5; let a_ = (l_w_4).wrapping_add(144); c.m.w32(a_, v_); };
    l_def_7 = c.m.r32((l_src_5).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_def_7).wrapping_add(52))); let a_ = (l_w_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_7).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
    if ((((((({ let a_ = c.m.r32((l_src_5).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (616i32)) as i32)) != 0) {
        l_ratio_8 = { let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_GetUnkE5CRatio(c, a0_) };
        { let v_ = l_ratio_8; let a_ = (l_w_4).wrapping_add(20); c.m.w32(a_, (v_).0); };
        { let r_ = rt::fmul(l_ratio_8, F32(0x3e999999)); let a_ = (l_w_4).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = 6144i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((((({ let a_ = c.m.r32((l_src_5).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (717i32)) as i32)) != 0) {
        l_ratio_9 = { let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_GetUnkE60Ratio(c, a0_) };
        { let v_ = l_ratio_9; let a_ = (l_w_4).wrapping_add(20); c.m.w32(a_, (v_).0); };
        l_add_10 = l_ratio_9;
        if (((rt::fgt(l_add_10, F32(0x3f333333)) as i32)) != 0) {
            l_add_10 = F32(0x3f333333);
        }
        { let r_ = 2048i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((l_w_4).wrapping_add(8))), F32(0x3f4ccccc)), l_add_10); let a_ = (l_w_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
    }
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(8))); let a_ = (l_w_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(8))); let a_ = (l_w_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
    if ((((((({ let a_ = c.m.r32((l_src_5).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (331i32)) as i32)) != 0) {
        { let r_ = 2048i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = (l_mgr_6).wrapping_add(4); let a_ = (l_w_4).wrapping_add(1456); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, (l_mgr_6).wrapping_add(4), (l_w_4).wrapping_add(148));
    if (({ let a0_ = c.m.r32((l_w_4).wrapping_add(1456)); F::EftEmit_HasWidth2(c, a0_) }) != 0) {
        { let v_ = rt::fmul(F::EftEmit_GetWidth2(c, (l_w_4).wrapping_add(148)), F32(c.m.r32((l_def_7).wrapping_add(48)))); let a_ = (l_w_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let r_ = 512i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(1456)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let a1_ = (if ((((((c.m.r32(l_src_5) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32((l_w_2).wrapping_add(144));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(148)) };
    if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_advance_4: i32 = 0;
    let mut l_ratio_5: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(144));
        l_advance_4 = 0i32;
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) == (0i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_PlayFireSound(c, a0_, l_src_3) };
        }
        F::EftEmit_UpdateNodesReq(c, l_src_3, (l_w_2).wrapping_add(864));
        if ((((!(((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
                { let _ = { let a2_ = (c.m.r32(l_src_3) as i32); F::EftAim_GetDir(c, (l_w_2).wrapping_add(48), (((((l_w_2).wrapping_add(896)) as u64)) as i32), a2_) }; };
                if (((((c.m.r32(l_w_2) as i32)) & (256i32))) != 0) {
                    { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 48i32, fp) };
                    F::Vec3_Sub(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(896), fp);
                    F::Vec3_Normalize(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(48));
                }
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                    { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0) {
            's1: {
                let sv6: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
                match sv6 {
                    0i32 => {
                        if ((((!(((((c.m.r32(l_w_2) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
                            l_ratio_5 = { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetUnkE44(c, a0_) };
                            { let v_ = l_ratio_5; let a_ = (l_w_2).wrapping_add(20); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(8))), rt::fadd(rt::fmul(l_ratio_5, F32(0x3f000000)), F32(0x3f800000))); let a_ = (l_w_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(8))), rt::fadd(rt::fmul(l_ratio_5, F32(0x00000000)), F32(0x3f800000))); let a_ = (l_w_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        }
                        if (((((c.m.r32(l_w_2) as i32)) & (256i32))) != 0) {
                            { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 48i32, fp) };
                            F::Vec3_Sub(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(896), fp);
                            F::Vec3_Normalize(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(48));
                        }
                        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                            { let r_ = 16i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            F::Vec4_Copy(c, (l_w_2).wrapping_add(80), (l_w_2).wrapping_add(960));
                            F::Vec4_Copy(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(80));
                            { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_2).wrapping_add(112)) };
                            { let a3_ = (c.m.r32(l_src_3) as i32); { let _ = F::EftAim_GetDirKeep(c, l_src_3, (l_w_2).wrapping_add(48), (((((l_w_2).wrapping_add(96)) as u64)) as i32), a3_); } };
                            { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(4))); F::Vec3_Scale(c, (l_w_2).wrapping_add(128), (l_w_2).wrapping_add(48), a2_) };
                            if (((((c.m.r32(l_w_2) as i32)) & (4096i32))) != 0) {
                                if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
                                    { let v_ = rt::fadd({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(48); F32(c.m.r32(a_)) }, rt::fmul(rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(20))), F32(0x3e999999)), F32(0x40800000))); let a_ = (l_w_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                }
                            }
                            { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        }
                        break 's1;
                    }
                    1i32 => {
                        l_advance_4 = 1i32;
                        break 's1;
                    }
                    2i32 => {
                        if (((((((((((((c.m.r32(l_w_2) as i32)) & (4096i32))) != 0) && (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(20))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) && (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(12))), F32(c.m.r32((l_w_2).wrapping_add(8)))) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = F32(0x3dcccccc); let a_ = (l_w_2).wrapping_add(12); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(12))), F32(c.m.r32((l_w_2).wrapping_add(8)))) as i32)) != 0) {
                                { let v_ = F32(c.m.r32((l_w_2).wrapping_add(8))); let a_ = (l_w_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            }
                        }
                        if ((((!(((((c.m.r32(l_w_2) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                            F::Vec4_Copy(c, (l_w_2).wrapping_add(112), (l_w_2).wrapping_add(96));
                        }
                        if ((((!(((({ let a_ = ({ let a_ = (c.m.r32((l_w_2).wrapping_add(144))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
                            if ((((!(((((c.m.r32(l_w_2) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                                if (((rt::fgt({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
                                    { let a3_ = (c.m.r32(l_src_3) as i32); let a4_ = F32(c.m.r32((l_w_2).wrapping_add(4))); let a5_ = { let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }; F::EftAim_Home(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(48), a3_, a4_, a5_) };
                                    { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(4))); F::Vec3_Scale(c, (l_w_2).wrapping_add(128), (l_w_2).wrapping_add(48), a2_) };
                                }
                                F::Vec3_Add(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(128));
                            } else {
                                F::Vec3_Copy(c, (l_w_2).wrapping_add(96), (p_task_1).wrapping_add(16));
                            }
                        }
                        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                            { let r_ = 9i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        }
                        break 's1;
                    }
                    3i32 => {
                        if (((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                            F::Vec3_Add(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(128));
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
            if ((((!(((((c.m.r32(l_w_2) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                { let r_ = 4i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a0_ = (c.m.r32(l_src_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftBlast_UpdateParts(c, a0_, p_task_1, a2_, 0i32) };
        if (((((c.m.r32(l_w_2) as i32)) & (16i32))) != 0) {
            if (((((c.m.r32(l_w_2) as i32)) & (512i32))) != 0) {
                { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_UpdateWidth2(c, a0_, (l_w_2).wrapping_add(148)) };
                { let v_ = rt::fmul(F::EftEmit_GetWidth2(c, (l_w_2).wrapping_add(148)), { let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(48); F32(c.m.r32(a_)) }); let a_ = (l_w_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(4))); F::Vec3_Scale(c, (l_w_2).wrapping_add(128), (l_w_2).wrapping_add(48), a2_) };
            }
        }
        if (((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (512i32))) != 0) {
            F::EftBlast_UpdateBlur(c, p_task_1);
        }
        if ((l_advance_4) != 0) {
            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        if (((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((((c.m.r32(l_w_2) as i32)) & (2i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        } else {
            if (((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) {
                if (((((c.m.r32(l_w_2) as i32)) & (4i32))) != 0) {
                    { let r_ = 2i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(24))), F32(c.m.r32((l_w_2).wrapping_add(28)))) as i32)) != 0) {
                        { let r_ = 2i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            } else {
                if (((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                    if (((((c.m.r32(l_w_2) as i32)) & (16i32))) != 0) {
                        { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_2).wrapping_add(148)) };
                        F::EftBlast_AddHit(c, p_task_1);
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(144));
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); { let _ = F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(148)); } };
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (1i32))) != 0) {
            { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_MarkKind6(c, a0_, (l_w_2).wrapping_add(148)) };
            if (((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (65536i32))) != 0) {
                { let r_ = (!(16i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let r_ = 8192i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (4i32))) != 0) {
                { let r_ = 16384i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if ((((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (1i32))) as u16)) != 0) {
            F::Vec3_Sub(c, fp, (p_task_1).wrapping_add(16), (l_w_2).wrapping_add(96));
            F::Vec3_Copy(c, (l_w_2).wrapping_add(96), (p_task_1).wrapping_add(16));
            F::Vec3_Add(c, (l_w_2).wrapping_add(112), (l_w_2).wrapping_add(112), fp);
            if ((((!(((((c.m.r32(l_w_2) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                { let r_ = 32i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            { let r_ = (!(32i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (4i32))) != 0) {
            { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32(l_w_2) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        { let r_ = 128i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(148)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlast_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlast_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_m_3: u32 = 0;
    let r_ = 'f: {
    l_m_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = 804i32; F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_m_3, 0i32, ((804i32) as u32)); };
    { let a3_ = c.m.r32((p_src_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_src_2, (l_m_3).wrapping_add(4), ((((0i32) as i64 as u64)) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((1472i32) as i64 as u64)) as u32)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); F::EftEmit_BeginFrame(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_InitModel(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_InitModel", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_parent_2: u32 = 0;
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_model_5: u32 = 0;
    let r_ = 'f: {
    l_parent_2 = F::BtlTask_GetParent(c, p_task_1);
    l_mgr_3 = c.m.r32((l_parent_2).wrapping_add(56));
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_model_5 = (l_w_4).wrapping_add(2240);
    { let v_ = c.m.r32((l_mgr_3).wrapping_add(812)); let a_ = l_model_5; c.m.w32(a_, v_); };
    { let v_ = (({ let a1_ = c.m.r32(l_model_5); F::EftObj_Create(c, ((l_w_4).wrapping_add(2240)).wrapping_add(4), a1_) }) as i8); let a_ = (l_model_5).wrapping_add(53); c.m.w8(a_, (v_) as u8); };
    { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_model_5).wrapping_add(54); c.m.w8(a_, (v_) as u8); };
    { let a0_ = (((c.m.r8((l_model_5).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
    { let a0_ = (((c.m.r8((l_model_5).wrapping_add(53)) as i8)) as i32); let _ = a0_; let _ = 0i32; F::EftObj_Nop(c) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_InitModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_SetModelPose(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32, mut p_angles_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_SetModelPose", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32, (p_angles_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_4: u32 = 0;
    let mut l_model_5: u32 = 0;
    let r_ = 'f: {
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        F::Mtx_StoreIdentity(c, fp);
        { let a2_ = F32(c.m.r32((l_w_4).wrapping_add(1432))); F::Mtx_ScaleDiagUniform(c, fp, fp, a2_) };
        l_model_5 = (l_w_4).wrapping_add(2240);
        F::Mtx_RotateZXY(c, fp, fp, p_angles_3);
        F::Mtx_Translate(c, fp, fp, p_pos_2);
        { let a0_ = (((c.m.r8((l_model_5).wrapping_add(53)) as i8)) as i32); F::EftObj_SetMtx(c, a0_, fp) };
        { let a0_ = (((c.m.r8((l_model_5).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 1i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_SetModelPose", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_FreeModel(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_FreeModel", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (({ let a_ = ((c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(2240)).wrapping_add(53); (c.m.r8(a_) as i8) }) as i32); { let _ = F::EftObj_Destroy(c, a0_); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_FreeModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_CalcDir(c: &mut Ctx, mut p_task_1: u32, mut p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_CalcDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_opp_5: i32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_4 = c.m.r32((l_w_3).wrapping_add(1504));
        l_opp_5 = { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        's1: {
            let sv6: i32 = { let a0_ = c.m.r32(l_w_3); F::EftEmit_GetHead25(c, a0_) };
            match sv6 {
                0i32 => {
                    F::BtlCharApi_GetNodePos(c, l_opp_5, 3i32, fp);
                    F::Vec3_Sub(c, p_dir_2, fp, (l_w_3).wrapping_add(1456));
                    { let v_ = F32(0x3f800000); let a_ = (p_dir_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, p_dir_2, p_dir_2);
                    break 's1;
                }
                1i32 => {
                    { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 46i32, p_dir_2) };
                    { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
                    F::Vec3_Sub(c, p_dir_2, fp, p_dir_2);
                    { let v_ = F32(0x3f800000); let a_ = (p_dir_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, p_dir_2, p_dir_2);
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = (c.m.r32(l_src_4) as i32); F::EftAim_GetDir(c, p_dir_2, (((((l_w_3).wrapping_add(1456)) as u64)) as i32), a2_) }; };
                    break 's1;
                }
                3i32 => {
                    { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 34i32, p_dir_2) };
                    { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 35i32, fp) };
                    F::Vec3_Sub(c, p_dir_2, fp, p_dir_2);
                    { let v_ = F32(0x3f800000); let a_ = (p_dir_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, p_dir_2, p_dir_2);
                    break 's1;
                }
                4i32 => {
                    { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 20i32, p_dir_2) };
                    { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 21i32, fp) };
                    F::Vec3_Sub(c, p_dir_2, fp, p_dir_2);
                    { let v_ = F32(0x3f800000); let a_ = (p_dir_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, p_dir_2, p_dir_2);
                    break 's1;
                }
                5i32 => {
                    { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodeMtx(c, a0_, 54i32, fp.wrapping_add(16)) };
                    { let a1_ = rt::fneg(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::fneg(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a3_ = rt::fneg(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Vec4_Set(c, p_dir_2, a1_, a2_, a3_, F32(0x3f800000)) };
                    F::Vec3_Normalize(c, p_dir_2, p_dir_2);
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1428))); F::Vec3_Scale(c, (l_w_3).wrapping_add(1488), p_dir_2, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_CalcDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_AllocShot(c: &mut Ctx, mut p_w_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_AllocShot", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_slot_2: u32 = 0;
    let mut l_res_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_slot_2 = (p_w_1).wrapping_add(1520);
    l_res_3 = ((0i32) as u32);
    l_i_4 = 0i32;
    'l1: while (((((l_i_4) < (14i32)) as i32)) != 0) {
        { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        if (((((c.m.r32((l_slot_2).wrapping_add(32))) == (((0i32) as u32))) as i32)) != 0) {
            l_res_3 = l_slot_2;
            { let _ = F::memset(c, l_slot_2, 0i32, 48u32); };
            break 'l1;
        }
        { let o_ = l_slot_2; let n_ = (o_).wrapping_add(48); l_slot_2 = n_; };
    }
    break 'f l_res_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_AllocShot", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_FireShots(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_evt_3: i32, mut p_kind_4: i32, mut p_evtIdx_5: i32, mut p_scale_6: F32, mut p_speed_7: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_FireShots", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_evt_3) as u32, (p_kind_4) as u32, (p_evtIdx_5) as u32, (p_scale_6).0, (p_speed_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_w_8: u32 = 0;
    let mut l_src_9: u32 = 0;
    let mut l_n_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_slot_12: u32 = 0;
    let mut l_alt_13: i32 = 0;
    let mut l_count_14: i32 = 0;
    let r_ = 'f: {
        l_w_8 = c.m.r32((p_task_2).wrapping_add(56));
        l_src_9 = c.m.r32((l_w_8).wrapping_add(1504));
        l_n_10 = 1i32;
        if (((((p_kind_4) == (1i32)) as i32)) != 0) {
            l_n_10 = (({ let a_ = (c.m.r32((l_src_9).wrapping_add(36))).wrapping_add(68); (c.m.r16(a_) as i16) }) as i32);
            if (((((l_n_10) <= (0i32)) as i32)) != 0) {
                l_n_10 = p_kind_4;
            }
        }
        {
            l_i_11 = 0i32;
            'l1: loop {
                if !(((((l_i_11) < (l_n_10)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_slot_12 = F::EftShotTech_AllocShot(c, l_w_8);
                    if (((((l_slot_12) != (((0i32) as u32))) as i32)) != 0) {
                        l_alt_13 = 0i32;
                        if (((((((({ let a0_ = c.m.r32(l_w_8); ((((F::EftEmit_GetHead26(c, a0_)) as i64 as u64)) as u8) }) as i32)) & (1i32))) as u8)) != 0) {
                            l_alt_13 = (((p_kind_4) == (4i32)) as i32);
                        }
                        { let a0_ = (c.m.r32(l_src_9) as i32); let a1_ = { let a0_ = (c.m.r32(l_src_9) as i32); F::EftShot_GetAttrKind(c, a0_, ((p_evt_3) as u64)) }; F::BtlCharApi_GetNodePos(c, a0_, a1_, fp) };
                        l_count_14 = (({ let a_ = (c.m.r32((l_src_9).wrapping_add(36))).wrapping_add(9); (c.m.r8(a_) as i8) }) as i32);
                        { let s_ = { rt::memset(c, fp.wrapping_add(64), 0, 48);; { let v_ = l_src_9; c.m.w32(fp.wrapping_add(64).wrapping_add(0), v_); }; { let v_ = c.m.r32(l_w_8); c.m.w32(fp.wrapping_add(64).wrapping_add(4), v_); }; { let v_ = (l_w_8).wrapping_add(720); c.m.w32(fp.wrapping_add(64).wrapping_add(8), v_); }; { let v_ = fp; c.m.w32(fp.wrapping_add(64).wrapping_add(12), v_); }; { let v_ = (l_w_8).wrapping_add(1328); c.m.w32(fp.wrapping_add(64).wrapping_add(16), v_); }; { let v_ = (c.m.r32((l_w_8).wrapping_add(2192)) as i32); c.m.w32(fp.wrapping_add(64).wrapping_add(20), (v_) as u32); }; { let v_ = l_count_14; c.m.w32(fp.wrapping_add(64).wrapping_add(24), (v_) as u32); }; { let v_ = p_kind_4; c.m.w32(fp.wrapping_add(64).wrapping_add(28), (v_) as u32); }; { let v_ = p_evtIdx_5; c.m.w32(fp.wrapping_add(64).wrapping_add(32), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(64).wrapping_add(36), (v_) as u32); }; { let v_ = p_scale_6; c.m.w32(fp.wrapping_add(64).wrapping_add(40), (v_).0); }; { let v_ = p_speed_7; c.m.w32(fp.wrapping_add(64).wrapping_add(44), (v_).0); }; fp.wrapping_add(64) }; let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 48); };
                        if ((l_alt_13) != 0) {
                            { let v_ = (l_w_8).wrapping_add(1392); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, v_); };
                            { let v_ = (l_w_8).wrapping_add(1376); let a_ = (fp.wrapping_add(16)).wrapping_add(16); c.m.w32(a_, v_); };
                        }
                        { let v_ = F::EftBlastObj_Create(c, fp.wrapping_add(16)); let a_ = (l_slot_12).wrapping_add(32); c.m.w32(a_, v_); };
                        { let r_ = 1i32; let a_ = (l_slot_12).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let a0_ = c.m.r32((l_slot_12).wrapping_add(32)); { let _ = F::EftBlastObj_SetNoHit(c, a0_, 1i32); } };
                        if ((l_alt_13) != 0) {
                            { let a0_ = c.m.r32((l_slot_12).wrapping_add(32)); { let _ = F::EftBlastObj_SetFrozen(c, a0_, 1i32); } };
                        }
                        { let r_ = 262144i32; let a_ = (l_w_8).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let a_ = (l_w_8).wrapping_add(2192); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_FireShots", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_ReapShots(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_ReapShots", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_alive_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_slot_5: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_alive_3 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_slot_5 = ((l_w_2).wrapping_add(1520)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48));
                if (({ let a0_ = c.m.r32((l_slot_5).wrapping_add(32)); F::EftBlastObj_IsAlive(c, a0_) }) != 0) {
                    l_alive_3 = 1i32;
                } else {
                    { let v_ = ((0i32) as u32); let a_ = (l_slot_5).wrapping_add(32); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_alive_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_ReapShots", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateShots(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateShots", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_kind_3: i32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_evt_5: i32 = 0;
    let mut l_evtIdx_6: i32 = 0;
    let mut l_def_7: u32 = 0;
    let mut l_scale_8: F32 = F32(0);
    let mut l_speed_9: F32 = F32(0);
    let mut l_i_10: i32 = 0;
    let mut l_slot_11: u32 = 0;
    let r_ = 'f: {
    l_kind_3 = (1i32).wrapping_neg();
    l_evt_5 = 0i32;
    l_evtIdx_6 = 0i32;
    l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
    l_def_7 = { let a_ = (c.m.r32((l_w_4).wrapping_add(1504))).wrapping_add(36); c.m.r32(a_) };
    l_scale_8 = F32(c.m.r32((l_w_4).wrapping_add(1432)));
    l_speed_9 = F32(c.m.r32((l_w_4).wrapping_add(1428)));
    if ((F::EftShot_TestBits(c, p_objId_1, 2i32)) != 0) {
        l_kind_3 = 0i32;
        l_evt_5 = 2i32;
        F::EftShotTech_CalcDir(c, p_task_2, (l_w_4).wrapping_add(1328));
    } else {
        if ((F::EftShot_TestBits(c, p_objId_1, 4i32)) != 0) {
            l_kind_3 = 1i32;
            l_evtIdx_6 = 1i32;
            l_evt_5 = 4i32;
            if ((F::EftShot_TestBits(c, p_objId_1, 64i32)) != 0) {
                { let r_ = F32(0x3f999999); let n_ = rt::fmul(l_scale_8, r_); l_scale_8 = n_; };
                { let r_ = F32(0x3f4ccccc); let n_ = rt::fmul(l_speed_9, r_); l_speed_9 = n_; };
            }
            F::EftShotTech_CalcDir(c, p_task_2, (l_w_4).wrapping_add(1328));
        } else {
            if ((F::EftShot_TestBits(c, p_objId_1, 16i32)) != 0) {
                l_kind_3 = 3i32;
                l_evtIdx_6 = 2i32;
                l_evt_5 = 16i32;
                F::EftShotTech_CalcDir(c, p_task_2, (l_w_4).wrapping_add(1328));
            } else {
                if ((F::EftShot_TestBits(c, p_objId_1, 32i32)) != 0) {
                    l_kind_3 = 4i32;
                    l_evtIdx_6 = 3i32;
                    l_evt_5 = 32i32;
                    F::EftShotTech_CalcDir(c, p_task_2, (l_w_4).wrapping_add(1328));
                } else {
                    if ((F::EftShot_TestBits(c, p_objId_1, 64i32)) != 0) {
                        l_kind_3 = 5i32;
                        l_evtIdx_6 = 4i32;
                        l_evt_5 = 64i32;
                        F::EftShotTech_CalcDir(c, p_task_2, (l_w_4).wrapping_add(1328));
                    }
                }
            }
        }
    }
    if (((((l_kind_3) >= (0i32)) as i32)) != 0) {
        if ((((({ let a0_ = c.m.r32(l_w_4); F::EftEmit_GetPhaseMask(c, a0_) }).wrapping_shr((l_kind_3) as u32)) & (1i32))) != 0) {
            F::EftShotTech_FireShots(c, p_objId_1, p_task_2, l_evt_5, l_kind_3, l_evtIdx_6, l_scale_8, l_speed_9);
        }
    }
    {
        l_i_10 = 0i32;
        'l1: loop {
            if !(((((l_i_10) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_slot_11 = ((l_w_4).wrapping_add(1520)).wrapping_add((((l_i_10) as u32)).wrapping_mul(48));
                if (({ let a0_ = c.m.r32((l_slot_11).wrapping_add(32)); F::EftBlastObj_IsAlive(c, a0_) }) != 0) {
                    if ((((!(((((c.m.r32((l_slot_11).wrapping_add(16)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                        if (((((({ let a0_ = c.m.r32(l_w_4); ((((F::EftEmit_GetHead26(c, a0_)) as i64 as u64)) as u8) }) as i32)) & (16i32))) != 0) {
                            { let a0_ = c.m.r32((l_slot_11).wrapping_add(32)); { let _ = F::EftBlastObj_SetTarget(c, a0_, 1i32, (l_w_4).wrapping_add(1456)); } };
                        }
                        if (((rt::fge(F32(c.m.r32((l_slot_11).wrapping_add(24))), F32(c.m.r32((l_def_7).wrapping_add(44)))) as i32)) != 0) {
                            { let r_ = 8i32; let a_ = (l_slot_11).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let a0_ = c.m.r32((l_slot_11).wrapping_add(32)); { let _ = F::EftBlastObj_Stop(c, a0_); } };
                        } else {
                            { let r_ = F32(0x3f800000); let a_ = (l_slot_11).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                    if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (1i32))) != 0) {
                        { let a0_ = c.m.r32((l_slot_11).wrapping_add(32)); { let _ = F::EftBlastObj_Stop(c, a0_); } };
                    }
                }
            }
            { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
        }
    }
    break 'f F::EftShotTech_ReapShots(c, p_task_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateShots", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_g_5: i32 = 0;
    let mut l_info_6: u32 = 0;
    let mut l_j_7: i32 = 0;
    let mut l_res_8: i32 = 0;
    let mut l_alt_9: i32 = 0;
    let r_ = 'f: {
    l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_g_5 = 0i32;
        'l1: loop {
            if !(((((l_g_5) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_g_5) as u32)) as u32)))) != 0) {
                    l_info_6 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_g_5) as u32)).wrapping_mul(16)));
                    {
                        l_j_7 = 0i32;
                        'l3: loop {
                            if !(((((l_j_7) < (((c.m.r8((l_info_6).wrapping_add(((1i32) as u32)))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (131072i32))) != 0) {
                                    if (({ let a1_ = ((c.m.r8((l_info_6).wrapping_add(((0i32) as u32)))) as i32); F::EftEmit_IsPartDeferred(c, p_set_3, a1_, l_j_7) }) != 0) {
                                        break 'c4;
                                    }
                                }
                                l_res_8 = { let a5_ = (((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (1i32)); let a6_ = (((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (4i32)); F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_4).wrapping_add(4), p_objId_1, l_g_5, l_j_7, a5_, a6_) };
                                if (((((l_res_8) != (0i32)) as i32)) != 0) {
                                    l_alt_9 = 0i32;
                                    if (((((((({ let a0_ = c.m.r32(l_w_4); ((((F::EftEmit_GetHead26(c, a0_)) as i64 as u64)) as u8) }) as i32)) & (1i32))) as u8)) != 0) {
                                        l_alt_9 = ((((({ let a_ = ((c.m.r32((p_set_3).wrapping_add(8))).wrapping_add((((((({ let a_ = (((p_set_3).wrapping_add(12)).wrapping_add((((c.m.r8((l_info_6).wrapping_add(((0i32) as u32)))) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.r8(a_) }) as i32)).wrapping_add(l_j_7)) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.r8(a_) }) as i32)) == (4i32)) as i32);
                                    }
                                    if ((l_alt_9) != 0) {
                                        { let a8_ = F32(c.m.r32((l_w_4).wrapping_add(1432))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_4).wrapping_add(4), (l_w_4).wrapping_add(720), (l_w_4).wrapping_add(1392), (l_w_4).wrapping_add(1376), l_g_5, l_j_7, l_res_8, a8_) };
                                    } else {
                                        { let a8_ = F32(c.m.r32((l_w_4).wrapping_add(1432))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_4).wrapping_add(4), (l_w_4).wrapping_add(720), (l_w_4).wrapping_add(1456), (l_w_4).wrapping_add(1312), l_g_5, l_j_7, l_res_8, a8_) };
                                    }
                                }
                            }
                            { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_g_5; let n_ = (o_).wrapping_add(1); l_g_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_Start(c: &mut Ctx, mut p_task_1: u32, mut p_evt_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Start", &PROBE_N, &[(p_task_1) as u32, (p_evt_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_4 = c.m.r32((l_w_3).wrapping_add(1504));
    if (((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (8192i32))) != 0) {
        if (({ let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_TestBits(c, a0_, 16i32) }) != 0) {
            { let r_ = 16384i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = 21i32; let a_ = (l_w_3).wrapping_add(1412); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = { let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_GetAttrKind(c, a0_, ((p_evt_2) as u64)) }; let a_ = (l_w_3).wrapping_add(1412); c.m.w32(a_, (v_) as u32); };
    }
    { let a0_ = (c.m.r32(l_src_4) as i32); let a1_ = (c.m.r32((l_w_3).wrapping_add(1412)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_3).wrapping_add(1360)) };
    F::Vec4_Copy(c, (l_w_3).wrapping_add(1440), (l_w_3).wrapping_add(1360));
    F::Vec4_Copy(c, (l_w_3).wrapping_add(1456), (l_w_3).wrapping_add(1440));
    F::Vec4_Copy(c, (l_w_3).wrapping_add(1472), (l_w_3).wrapping_add(1456));
    F::EftShotTech_CalcDir(c, p_task_1, (l_w_3).wrapping_add(1312));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateFlash(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateFlash", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_src_2: u32 = 0;
    let mut l_opp_3: i32 = 0;
    let r_ = 'f: {
        l_src_2 = { let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(1504); c.m.r32(a_) };
        l_opp_3 = { let a0_ = (c.m.r32(l_src_2) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        if ((((((((!(((({ let a_ = (c.m.r32((l_src_2).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (4194304i32))) != 0)) as i32)) != 0) && ((F::EftShot_TestBits(c, l_opp_3, 64i32)) != 0)) as i32)) != 0) {
            { let s_ = { rt::memset(c, fp.wrapping_add(48), 0, 48);; rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);; { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }; { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }; { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }; { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }; { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16), (v_).0); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(20), (v_) as u32); }; { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(24), (v_).0); }; { let v_ = (c.m.r32(l_src_2) as i32); c.m.w32(fp.wrapping_add(48).wrapping_add(28), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(32), (v_) as u32); }; fp.wrapping_add(48) }; let d_ = fp; rt::memcpy(c, d_, s_, 48); };
            if ((((((({ let a_ = c.m.r32((l_src_2).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (636i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
            { let _ = F::EftFlash_Start(c, fp); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateFlash", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateNodeFx(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateNodeFx", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_src_2: u32 = 0;
    let mut l_opp_3: i32 = 0;
    let r_ = 'f: {
        l_src_2 = { let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(1504); c.m.r32(a_) };
        l_opp_3 = { let a0_ = (c.m.r32(l_src_2) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        if ((F::EftShot_TestBits(c, l_opp_3, 64i32)) != 0) {
            rt::memset(c, fp, 0, 40);
            { let v_ = 71i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
            { let v_ = 0i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
            { let v_ = 71i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
            { let v_ = 3i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
            { let v_ = 71i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
            { let v_ = 10i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
            { let v_ = 71i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
            { let v_ = 14i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
            { let v_ = 71i32; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
            { let v_ = 21i32; c.m.w32(fp.wrapping_add(36), (v_) as u32); }
            { let a0_ = (c.m.r32(l_src_2) as i32); F::EftDelaySe_Start(c, a0_, fp, 5i32) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateNodeFx", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateBlur(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateBlur", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let mut l_t_5: F32 = F32(0);
    let mut l_a_6: i32 = 0;
    let mut l_b_7: i32 = 0;
    let mut l_c_8: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(1504));
        l_opp_4 = { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        if (((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (512i32))) != 0) {
            if ((F::EftShot_TestBits(c, l_opp_4, 16i32)) != 0) {
                { let r_ = 1024i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let _ = { let a2_ = (c.m.r32(l_src_3) as i32); F::EftAim_GetDir(c, (l_w_2).wrapping_add(1344), ((((fp) as u64)) as i32), a2_) }; };
            }
            if ((F::EftShot_TestBits(c, l_opp_4, 32i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(1436); c.m.w32(a_, (v_).0); };
                { let r_ = (!(1024i32)); let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (1024i32))) != 0) {
                { let r_ = F32(0x3dcccccc); let a_ = (l_w_2).wrapping_add(1436); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(1436))), F32(0x3f800000)) as i32)) != 0) {
                    { let v_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1436); c.m.w32(a_, (v_).0); };
                }
            }
        }
        l_t_5 = F32(c.m.r32((l_w_2).wrapping_add(1436)));
        l_a_6 = rt::f2i(rt::fmul(l_t_5, F32(0x41000000)));
        l_b_7 = rt::f2i(rt::fmul(l_t_5, F32(0x42000000)));
        l_c_8 = rt::f2i(rt::fmul(l_t_5, F32(0x43000000)));
        F::StgBlur_SetColor0Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_a_6) as u8)) as i32)) as i64 as u64)) as u8));
        F::StgBlur_SetColor1Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_b_7) as u8)) as i32)) as i64 as u64)) as u8));
        F::StgBlur_SetColor2Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_c_8) as u8)) as i32)) as i64 as u64)) as u8));
        F::StgBlur_SetColor3Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((64i32) as i64 as u64)) as u8));
        F::StgBlur_SetCenter(c, 0i32, (l_w_2).wrapping_add(1344), 0i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateBlur", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateSubs(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateSubs", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_def_6: u32 = 0;
    let mut l_kind_7: i32 = 0;
    let mut l_subArg_8: i32 = 0;
    let mut l_angle_9: F32 = F32(0);
    let mut l_i_10: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32((l_w_2).wrapping_add(1504));
    l_opp_4 = { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
    if ((F::EftShot_TestBits(c, l_opp_4, 128i32)) != 0) {
        l_n_5 = (c.m.r32((l_w_2).wrapping_add(2232)) as i32);
        l_def_6 = c.m.r32((l_src_3).wrapping_add(36));
        l_kind_7 = (((c.m.r8(((l_def_6).wrapping_add(88)).wrapping_add(((l_n_5) as u32))) as i8)) as i32);
        l_subArg_8 = (((c.m.r16((l_def_6).wrapping_add(128)) as i16)) as i32);
        l_angle_9 = { let a0_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_def_6).wrapping_add(96)).wrapping_add((((l_n_5) as u32)).wrapping_mul(4)))), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) };
        if (((((l_kind_7) >= (0i32)) as i32)) != 0) {
            { let v_ = { let a0_ = (c.m.r32(l_src_3) as i32); F::EftStreak_Start(c, a0_, l_kind_7, l_subArg_8, l_angle_9) }; let a_ = ((l_w_2).wrapping_add(2200)).wrapping_add((((l_n_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let r_ = 524288i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a_ = (l_w_2).wrapping_add(2232); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if ((F::EftShot_TestBits(c, l_opp_4, 256i32)) != 0) {
        l_i_10 = ((c.m.r32((l_w_2).wrapping_add(2232)) as i32)).wrapping_sub(1i32);
        if (((((l_i_10) < (0i32)) as i32)) != 0) {
            l_i_10 = 0i32;
        }
        if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (524288i32))) != 0) {
            { let a0_ = c.m.r32(((l_w_2).wrapping_add(2200)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))); F::EftStreak_Stop(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = ((l_w_2).wrapping_add(2200)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let r_ = (!(524288i32)); let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateSubs", &PROBE_N, &[], true); }
    r_
}
