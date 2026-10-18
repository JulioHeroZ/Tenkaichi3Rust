#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftHit_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((25616u32) as i32)) }; let a_ = sym::gEftHitList; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftHitList); F::memset(c, a0_, 0i32, 25616u32) }; };
    F::EftHitArena_Init(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftHitArena_Term(c);
    if (((((c.m.r32(sym::gEftHitList)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftHitList); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftHitList; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetNew(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetNew", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) }) >= (64i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); c.m.w32(a_, (v_) as u32); };
    }
    { let _ = F::BtlScene_IsTimeStopped(c); };
    { let _ = { let a0_ = (c.m.r32(sym::gEftHitList)).wrapping_add(((({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(400)); F::memset(c, a0_, 0i32, 400u32) }; };
    { let v_ = F32(0x3f800000); let a_ = (((c.m.r32(sym::gEftHitList)).wrapping_add(((({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    break 'f (c.m.r32(sym::gEftHitList)).wrapping_add(((({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(400));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetNew", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_Add(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Add", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlScene_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) }) >= (64i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); c.m.w32(a_, (v_) as u32); };
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = c.m.r32((p_rec_1).wrapping_add(100)); (c.m.r32(a_) as i32) }; let a_ = p_rec_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a_ = (p_rec_1).wrapping_add(240); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(52); F32(c.m.r32(a_)) }; let a_ = ((p_rec_1).wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = (({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(6); (c.m.r8(a_) as i8) }) as i32); let a_ = (p_rec_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(2); (c.m.r16(a_) as i16) }) as i32); let a_ = (p_rec_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (p_rec_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }).wrapping_shr((1i32) as u32)) & (1i32)); let a_ = (p_rec_1).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_rec_1).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::EftHit_GetScale(c, p_rec_1); let a_ = ((p_rec_1).wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        if ((((((({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(5); (c.m.r8(a_) as i8) }) as i32)) == (9i32)) as i32)) != 0) {
            { let r_ = 8i32; let a_ = (p_rec_1).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        F::EftHit_InitMultiHit(c, p_rec_1);
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = (({ let a_ = (c.m.r32((p_rec_1).wrapping_add(104))).wrapping_add(18); (c.m.r16(a_) as i16) }) as i32); let a_ = p_rec_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (c.m.r32((p_rec_1).wrapping_add(104))).wrapping_add(22); (c.m.r16(a_) as i16) }) as i32); let a_ = (p_rec_1).wrapping_add(240); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32((p_rec_1).wrapping_add(104))).wrapping_add(32); F32(c.m.r32(a_)) }; let a_ = ((p_rec_1).wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = (({ let a_ = (c.m.r32((p_rec_1).wrapping_add(104))).wrapping_add(27); c.m.r8(a_) }) as i32); let a_ = (p_rec_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32((p_rec_1).wrapping_add(104))).wrapping_add(48); (c.m.r32(a_) as i32) }; let a_ = (p_rec_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_rec_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_rec_1).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_rec_1).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::EftHit_GetScale(c, p_rec_1); let a_ = ((p_rec_1).wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    }
    { let s_ = p_rec_1; let d_ = (c.m.r32(sym::gEftHitList)).wrapping_add(((({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(400)); rt::memcpy(c, d_, s_, 400); };
    { let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Add", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_BeginFrame(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_BeginFrame", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!((F::BtlScene_IsTimeStopped(c)) != 0)) as i32)) != 0) {
        F::EftHitArena_Reset(c);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_BeginFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_Clear(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftHitArena_Reset(c);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetList(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetList", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gEftHitList);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetList", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetTaskFlags(c: &mut Ctx, mut p_idx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetTaskFlags", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_list_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let r_ = 'f: {
    l_list_2 = c.m.r32(sym::gEftHitList);
    if (((((p_idx_1) >= ((((c.m.r32((l_list_2).wrapping_add(25600)) as i32)) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_rec_3 = (l_list_2).wrapping_add((p_idx_1).wrapping_mul(400));
    if (((((c.m.r32((l_rec_3).wrapping_add(96))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f { let a_ = (c.m.r32((l_rec_3).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetTaskFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetTaskFlag(c: &mut Ctx, mut p_idx_1: u32, mut p_bits_2: i32, p_pos_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetTaskFlag", &PROBE_N, &[(p_idx_1) as u32, (p_bits_2) as u32, (p_pos_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rec_4: u32 = 0;
    rt::memcpy(c, fp, p_pos_3, 16);
    let r_ = 'f: {
        if (((((p_idx_1) < ((({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) }) as u32))) as i32)) != 0) {
            l_rec_4 = (c.m.r32(sym::gEftHitList)).wrapping_add((p_idx_1).wrapping_mul(400));
            if (((((c.m.r32((l_rec_4).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
                { let a0_ = c.m.r32((l_rec_4).wrapping_add(96)); F::BtlTask_SetTagBits(c, a0_, p_bits_2) };
                { let a0_ = c.m.r32((l_rec_4).wrapping_add(96)); F::BtlTask_SetPos(c, a0_, fp) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetTaskFlag", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SpawnImpacts(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SpawnImpacts", &PROBE_N, &[], false); }
    let mut l_rec_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_flags_3: i32 = 0;
    let r_ = 'f: {
    if ((F::BtlScene_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rec_1 = (c.m.r32(sym::gEftHitList)).wrapping_add((((l_i_2) as u32)).wrapping_mul(400));
                if (((((c.m.r32((l_rec_1).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
                    l_flags_3 = { let a_ = (c.m.r32((l_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) };
                    if ((((((((((((((((!((((l_flags_3) & (512i32))) != 0)) as i32)) != 0) && ((((!(((((c.m.r32((l_rec_1).wrapping_add(80)) as i32)) & (2i32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((l_flags_3) & (32i32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((l_flags_3) & (63i32))) != 0)) as i32)) != 0) {
                        if (((((c.m.r32((l_rec_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
                            F::EftHit_SpawnBlastImpact(c, l_rec_1);
                        }
                        if (((((c.m.r32((l_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
                            F::EftHit_SpawnTechImpact(c, l_rec_1);
                        }
                        { let r_ = 512i32; let a_ = (c.m.r32((l_rec_1).wrapping_add(96))).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SpawnImpacts", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_CanHit(c: &mut Ctx, mut p_rec_1: u32, mut p_mode_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_CanHit", &PROBE_N, &[(p_rec_1) as u32, (p_mode_2) as u32], false); }
    let mut l_ok_3: i32 = 0;
    let mut l_flags_4: i32 = 0;
    let mut l_def_5: u32 = 0;
    let r_ = 'f: {
    l_ok_3 = 1i32;
    if (((((c.m.r32((p_rec_1).wrapping_add(96))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_flags_4 = { let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) };
    if ((((((c.m.r32((p_rec_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
        if (((((p_mode_2) == (0i32)) as i32)) != 0) {
            if (((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (32i32))) != 0) {
            } else {
                if (((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (64i32))) != 0) {
                    l_ok_3 = 0i32;
                } else {
                    if ((((l_flags_4) & (65536i32))) != 0) {
                        l_ok_3 = 0i32;
                    } else {
                        if ((((l_flags_4) & (4i32))) != 0) {
                            l_ok_3 = 0i32;
                        }
                    }
                }
            }
        } else {
            l_ok_3 = (((((l_flags_4) & (9i32))) == (0i32)) as i32);
        }
        if ((((l_flags_4) & (64i32))) != 0) {
            l_ok_3 = 0i32;
        }
    } else {
        l_def_5 = { let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) };
        's1: {
            let sv6: i32 = p_mode_2;
            match sv6 {
                0i32 => {
                    if (((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (32i32))) != 0) {
                    } else {
                        if (((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (64i32))) != 0) {
                            l_ok_3 = 0i32;
                        } else {
                            if ((((l_flags_4) & (65536i32))) != 0) {
                                l_ok_3 = 0i32;
                            } else {
                                if ((((((((c.m.r8((l_def_5).wrapping_add(4)) as i8)) as i32)) != (0i32)) as i32)) != 0) {
                                    if ((((l_flags_4) & (260i32))) != 0) {
                                        l_ok_3 = 0i32;
                                    }
                                } else {
                                    if ((((((((c.m.r8((l_def_5).wrapping_add(5)) as i8)) as i32)) != (4i32)) as i32)) != 0) {
                                        l_ok_3 = 0i32;
                                    }
                                }
                            }
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    if (((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (128i32))) != 0) {
                        l_ok_3 = 0i32;
                    } else {
                        if ((((l_flags_4) & (131072i32))) != 0) {
                            l_ok_3 = 0i32;
                        } else {
                            if ((((l_flags_4) & (16384i32))) != 0) {
                            } else {
                                if ((((l_flags_4) & (265i32))) != 0) {
                                    l_ok_3 = 0i32;
                                }
                            }
                        }
                    }
                    break 's1;
                }
                2i32 => {
                    if ((((l_flags_4) & (262144i32))) != 0) {
                        l_ok_3 = 0i32;
                    } else {
                        l_ok_3 = (((((l_flags_4) & (269i32))) == (0i32)) as i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    break 'f l_ok_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_CanHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_IsStoppedByHit(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IsStoppedByHit", &PROBE_N, &[(p_rec_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let mut l_def_3: u32 = 0;
    let r_ = 'f: {
    l_result_2 = 1i32;
    if ((((((c.m.r32((p_rec_1).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) {
        l_def_3 = { let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) };
        if ((((((((c.m.r8((l_def_3).wrapping_add(4)) as i8)) as i32)) == (0i32)) as i32)) != 0) {
            if ((((((((c.m.r8((l_def_3).wrapping_add(5)) as i8)) as i32)) == (4i32)) as i32)) != 0) {
                l_result_2 = 0i32;
            }
        } else {
            if ((((((((((((c.m.r8((l_def_3).wrapping_add(5)) as i8)) as i32)) == (9i32)) as i32)) != 0) || ((((((((c.m.r8((l_def_3).wrapping_add(5)) as i8)) as i32)) == (7i32)) as i32)) != 0)) as i32)) != 0) {
                l_result_2 = 0i32;
            }
        }
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IsStoppedByHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_RearmImpact(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_RearmImpact", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_rec_1).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) {
        if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (16384i32))) != 0) {
            { let r_ = (!(512i32)); let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_RearmImpact", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_ClashTech(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_ClashTech", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_defA_3: u32 = 0;
    let mut l_defB_4: u32 = 0;
    let mut l_t_5: F32 = F32(0);
    let mut l_result_6: i32 = 0;
    let r_ = 'f: {
        l_defA_3 = { let a_ = (c.m.r32((p_a_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) };
        l_defB_4 = { let a_ = (c.m.r32((p_b_2).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) };
        if ((((((((((((c.m.r8((l_defA_3).wrapping_add(4)) as i8)) as i32)) == (0i32)) as i32)) != 0) || ((((((((c.m.r8((l_defB_4).wrapping_add(4)) as i8)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 3i32;
        }
        if ((((!((((((((((((((((c.m.r8((l_defA_3).wrapping_add(5)) as i8)) as i32)) >= (0i32)) as i32)) != 0) && ((((((((c.m.r8((l_defA_3).wrapping_add(5)) as i8)) as i32)) <= (2i32)) as i32)) != 0)) as i32)) != 0) || ((((((((c.m.r8((l_defA_3).wrapping_add(5)) as i8)) as i32)) == (8i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            break 'f 3i32;
        }
        if (((((((((((((((((c.m.r32((l_defA_3).wrapping_add(60)) as i32)) & (256i32))) != 0) || (((((c.m.r32((l_defB_4).wrapping_add(60)) as i32)) & (256i32))) != 0)) as i32)) != 0) || ((((((((c.m.r8((l_defA_3).wrapping_add(9)) as i8)) as i32)) >= (2i32)) as i32)) != 0)) as i32)) != 0) || ((((((((c.m.r8((l_defB_4).wrapping_add(9)) as i8)) as i32)) >= (2i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 3i32;
        }
        F::Vec4_Sub(c, fp, (p_a_1).wrapping_add(32), (p_a_1).wrapping_add(48));
        F::Vec4_Sub(c, fp.wrapping_add(16), (p_b_2).wrapping_add(32), (p_b_2).wrapping_add(48));
        F::Vec4_Sub(c, fp.wrapping_add(32), (p_b_2).wrapping_add(48), (p_a_1).wrapping_add(48));
        if (((((((rt::fgt(F::Vec3_Dot(c, fp, fp.wrapping_add(32)), F32(0x00000000)) as i32)) != 0) && (((rt::flt(F::Vec3_Dot(c, fp, fp.wrapping_add(16)), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            l_result_6 = 4i32;
            F::EftTechEvt_RequestStop(c, 0i32);
            F::EftTechEvt_RequestStop(c, 1i32);
        } else {
            F::Vec4_Sub(c, fp.wrapping_add(32), (p_a_1).wrapping_add(32), (p_b_2).wrapping_add(32));
            l_result_6 = 2i32;
            F::Vec3_Normalize(c, fp, fp);
            F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
            l_t_5 = F::Vec3_Dot(c, fp.wrapping_add(32), fp);
            F::Vec4_Sub(c, fp.wrapping_add(32), (p_b_2).wrapping_add(32), (p_a_1).wrapping_add(32));
            F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
            F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
            if ((((!(((rt::flt(l_t_5, F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(16))) as i32)) != 0)) as i32)) != 0) {
                l_result_6 = 1i32;
            }
        }
        break 'f l_result_6;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftHit_ClashTech", &PROBE_N, &[], true); }
    r_
}

pub fn eft_a__EftHit_CompareLevel(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_a__EftHit_CompareLevel", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_a_1).wrapping_add(8)) as i32)) > ((c.m.r32((p_b_2).wrapping_add(8)) as i32))) as i32)) != 0) {
        break 'f 2i32;
    }
    if ((((((c.m.r32((p_a_1).wrapping_add(8)) as i32)) < ((c.m.r32((p_b_2).wrapping_add(8)) as i32))) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 3i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_a__EftHit_CompareLevel", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_Clash(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Clash", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_result_3: i32 = 0;
    let r_ = 'f: {
    l_result_3 = (((c.m.r32((p_a_1).wrapping_add(104))) != (((0i32) as u32))) as i32);
    if (((((((((c.m.r32((p_a_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((p_b_2).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_result_3 = F::eft_a__EftHit_CompareLevel(c, p_a_1, p_b_2);
    } else {
        if (((((c.m.r32((p_b_2).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
            l_result_3 = 2i32;
        }
        if (((((((((c.m.r32((p_a_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((p_b_2).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            l_result_3 = F::EftHit_ClashTech(c, p_a_1, p_b_2);
            if (((((l_result_3) == (3i32)) as i32)) != 0) {
                l_result_3 = F::eft_a__EftHit_CompareLevel(c, p_a_1, p_b_2);
            }
        }
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_Clash", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_IsMultiHit(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IsMultiHit", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(96))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (16384i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IsMultiHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_IncHitCount(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IncHitCount", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    { let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(11); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IncHitCount", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetHitCount(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetHitCount", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(11); (c.m.r8(a_) as i8) }) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetHitCount", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetMaxHits(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetMaxHits", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(10); (c.m.r8(a_) as i8) }) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetMaxHits", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetDefUnkB(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetDefUnkB", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(11); (c.m.r8(a_) as i8) }) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetDefUnkB", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_IncCountA(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IncCountA", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
        { let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(10); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IncCountA", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetCountA(c: &mut Ctx, mut p_rec_1: u32, mut p_value_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetCountA", &PROBE_N, &[(p_rec_1) as u32, (p_value_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = ((p_value_2) as i8); let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetCountA", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetCountA(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetCountA", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(10); (c.m.r8(a_) as i8) }) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetCountA", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_HasDefFlag20(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_HasDefFlag20", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        if (((({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_HasDefFlag20", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_MarkLastHit(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_MarkLastHit", &PROBE_N, &[(p_rec_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (2048i32))) != 0) {
        break 'f;
    }
    if ((F::EftHit_IsMultiHit(c, p_rec_1)) != 0) {
        if (((((F::EftHit_GetHitCount(c, p_rec_1)) > (0i32)) as i32)) != 0) {
            if ((F::EftHit_HasDefFlag20(c, p_rec_1)) != 0) {
                { let r_ = 16i32; let a_ = (p_rec_1).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                l_count_2 = F::EftHit_GetHitCount(c, p_rec_1);
                if ((((!(((((l_count_2) < (F::EftHit_GetMaxHits(c, p_rec_1))) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = 16i32; let a_ = (p_rec_1).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    } else {
        if ((((((({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(5); (c.m.r8(a_) as i8) }) as i32)) != (4i32)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = (p_rec_1).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_MarkLastHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_HasDefFlag200000(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_HasDefFlag200000", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        if (((({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) & (2097152i32))) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_HasDefFlag200000", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_NotifyBlastTask(c: &mut Ctx, mut p_idx_1: u32, mut p_mtx_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_NotifyBlastTask", &PROBE_N, &[(p_idx_1) as u32, (p_mtx_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftHitList)) != (((0i32) as u32))) as i32)) != 0) {
        if (((((p_idx_1) < ((({ let a_ = (c.m.r32(sym::gEftHitList)).wrapping_add(25600); (c.m.r32(a_) as i32) }) as u32))) as i32)) != 0) {
            l_rec_3 = (c.m.r32(sym::gEftHitList)).wrapping_add((p_idx_1).wrapping_mul(400));
            if (((((c.m.r32((l_rec_3).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
                if ((((((c.m.r32((l_rec_3).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                    { let a0_ = c.m.r32((l_rec_3).wrapping_add(96)); F::EftKiBomb_SetContact(c, a0_, p_mtx_2) };
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_NotifyBlastTask", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_IsRushHit(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IsRushHit", &PROBE_N, &[(p_rec_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let mut l_def_3: u32 = 0;
    let r_ = 'f: {
    l_result_2 = 0i32;
    if (((((c.m.r32((p_rec_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        l_def_3 = { let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) };
        if ((((((((c.m.r8((l_def_3).wrapping_add(4)) as i8)) as i32)) == (0i32)) as i32)) != 0) {
            if ((((((((c.m.r8((l_def_3).wrapping_add(5)) as i8)) as i32)) == (4i32)) as i32)) != 0) {
                l_result_2 = 1i32;
            }
        } else {
            l_result_2 = ((((((c.m.r8((l_def_3).wrapping_add(5)) as i8)) as i32)) == (9i32)) as i32);
        }
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IsRushHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetRadiusA(c: &mut Ctx, mut p_rec_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetRadiusA", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_rec_1).wrapping_add(112)) as i32);
        match sv2 {
            0i32 => {
                break 'f { let a_ = (c.m.r32(((p_rec_1).wrapping_add(112)).wrapping_add(12))).wrapping_add(16); F32(c.m.r32(a_)) };
            }
            1i32 => {
                break 'f { let a_ = (c.m.r32(((p_rec_1).wrapping_add(112)).wrapping_add(12))).wrapping_add(32); F32(c.m.r32(a_)) };
            }
            _ => {
            }
        }
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetRadiusA", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetRadiusB(c: &mut Ctx, mut p_rec_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetRadiusB", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_rec_1).wrapping_add(112)) as i32);
        match sv2 {
            0i32 => {
                break 'f { let a_ = (c.m.r32(((p_rec_1).wrapping_add(112)).wrapping_add(16))).wrapping_add(16); F32(c.m.r32(a_)) };
            }
            1i32 => {
                break 'f { let a_ = (c.m.r32(((p_rec_1).wrapping_add(112)).wrapping_add(16))).wrapping_add(32); F32(c.m.r32(a_)) };
            }
            _ => {
            }
        }
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetRadiusB", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetRadiusA(c: &mut Ctx, mut p_rec_1: u32, mut p_radius_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetRadiusA", &PROBE_N, &[(p_rec_1) as u32, (p_radius_2).0], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = (c.m.r32((p_rec_1).wrapping_add(112)) as i32);
        match sv3 {
            0i32 => {
                { let v_ = p_radius_2; let a_ = (c.m.r32(((p_rec_1).wrapping_add(112)).wrapping_add(12))).wrapping_add(16); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            1i32 => {
                { let v_ = p_radius_2; let a_ = (c.m.r32(((p_rec_1).wrapping_add(112)).wrapping_add(12))).wrapping_add(32); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetRadiusA", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetRadiusB(c: &mut Ctx, mut p_rec_1: u32, mut p_radius_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetRadiusB", &PROBE_N, &[(p_rec_1) as u32, (p_radius_2).0], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = (c.m.r32((p_rec_1).wrapping_add(112)) as i32);
        match sv3 {
            0i32 => {
                { let v_ = p_radius_2; let a_ = (c.m.r32(((p_rec_1).wrapping_add(112)).wrapping_add(16))).wrapping_add(16); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            1i32 => {
                { let v_ = p_radius_2; let a_ = (c.m.r32(((p_rec_1).wrapping_add(112)).wrapping_add(16))).wrapping_add(32); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetRadiusB", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_ClampToStage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_ClampToStage", &PROBE_N, &[], false); }
    let mut l_list_1: u32 = 0;
    let mut l_rec_2: u32 = 0;
    let mut l_shape_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_radius_5: F32 = F32(0);
    let mut l_len_6: F32 = F32(0);
    let r_ = 'f: {
    l_list_1 = F::EftHit_GetList(c);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((l_list_1).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rec_2 = (l_list_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(400));
                if (((((c.m.r32((l_rec_2).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
                    l_radius_5 = rt::fadd(F::BtlStage_GetInnerRadius(c), F32(0x00000000));
                    if (((rt::flt(l_radius_5, { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32((l_rec_2).wrapping_add(32))), F32(c.m.r32((l_rec_2).wrapping_add(32)))), rt::fmul(F32(c.m.r32(((l_rec_2).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32(((l_rec_2).wrapping_add(32)).wrapping_add(8))))); F::Port_sqrtf(c, a0_) }) as i32)) != 0) {
                        if ((((!(((({ let a_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
                            { let a0_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16); F::Vec4_Copy(c, a0_, (l_rec_2).wrapping_add(32)) };
                            l_radius_5 = { let a0_ = rt::fadd(rt::fmul({ let a_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16); F32(c.m.r32(a_)) }), rt::fmul({ let a_ = ((c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16)).wrapping_add(8); F32(c.m.r32(a_)) }, { let a_ = ((c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16)).wrapping_add(8); F32(c.m.r32(a_)) })); F::Port_sqrtf(c, a0_) };
                            { let r_ = rt::fdiv(F::BtlStage_GetInnerRadius(c), l_radius_5); let a_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = rt::fdiv(F::BtlStage_GetInnerRadius(c), l_radius_5); let a_ = ((c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            l_shape_3 = (l_rec_2).wrapping_add(112);
                            's3: {
                                let sv7: i32 = (c.m.r32(l_shape_3) as i32);
                                match sv7 {
                                    0i32 => {
                                        { let a0_ = c.m.r32((l_shape_3).wrapping_add(12)); let a1_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16); F::Vec4_Copy(c, a0_, a1_) };
                                        break 's3;
                                    }
                                    1i32 => {
                                        { let a0_ = (c.m.r32((l_shape_3).wrapping_add(12))).wrapping_add(16); let a1_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(16); F::Vec4_Copy(c, a0_, a1_) };
                                        break 's3;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            { let r_ = 16i32; let a_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_ClampToStage", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_UpdateResults(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_UpdateResults", &PROBE_N, &[], false); }
    let mut l_list_1: u32 = 0;
    let mut l_rec_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_list_1 = F::EftHit_GetList(c);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((l_list_1).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rec_2 = (l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400));
                if (((((c.m.r32((l_rec_2).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = ((F::EftHit_CalcResult(c, l_rec_2)) as i16); let a_ = (c.m.r32((l_rec_2).wrapping_add(96))).wrapping_add(8); c.m.w16(a_, (v_) as u16); };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_UpdateResults", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_CalcResult(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_CalcResult", &PROBE_N, &[(p_rec_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let mut l_flags_3: i32 = 0;
    let mut l_count_4: i32 = 0;
    let r_ = 'f: {
    l_result_2 = 0i32;
    if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        l_result_2 = 5i32;
    }
    if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (32768i32))) != 0) {
        l_result_2 = 1i32;
        { let r_ = (!(32768i32)); let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (16384i32))) != 0) {
            l_count_4 = F::EftHit_GetHitCount(c, p_rec_1);
            if (((((F::EftHit_GetMaxHits(c, p_rec_1)) < (l_count_4)) as i32)) != 0) {
                if ((((!((F::EftHit_HasDefFlag20(c, p_rec_1)) != 0)) as i32)) != 0) {
                    l_result_2 = 5i32;
                }
            }
        } else {
            if ((F::EftHit_HasDefFlag20(c, p_rec_1)) != 0) {
                { let r_ = 65536i32; let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                l_result_2 = 4i32;
            }
        }
    }
    l_flags_3 = { let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) };
    if ((((l_flags_3) & (8i32))) != 0) {
        l_result_2 = 4i32;
    }
    if ((((l_flags_3) & (2i32))) != 0) {
        if ((((!((((l_flags_3) & (16384i32))) != 0)) as i32)) != 0) {
            l_result_2 = 5i32;
        }
    }
    if ((((l_flags_3) & (32i32))) != 0) {
        l_result_2 = 5i32;
    }
    if ((((l_flags_3) & (16i32))) != 0) {
        l_result_2 = 5i32;
    }
    if ((((l_flags_3) & (256i32))) != 0) {
        l_result_2 = 3i32;
    }
    if ((F::EftHit_IsTechClass(c, p_rec_1, 0i32)) != 0) {
        if ((((!(((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
            { let r_ = (!(5i32)); let n_ = ((l_result_2) & (r_)); l_result_2 = n_; };
        }
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_CalcResult", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((12u32) as i32)) }; let a_ = sym::gEftHitArena; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftHitArena); F::memset(c, a0_, 0i32, 12u32) }; };
    { let v_ = 6144i32; let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(8); (c.m.r32(a_) as i32) }; F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(4); c.m.r32(a_) }; let a2_ = (({ let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = { let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(4); c.m.r32(a_) }; let a_ = c.m.r32(sym::gEftHitArena); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(4); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gEftHitArena)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftHitArena); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftHitArena; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = { let a_ = (c.m.r32(sym::gEftHitArena)).wrapping_add(4); c.m.r32(a_) }; let a_ = c.m.r32(sym::gEftHitArena); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_Alloc20A(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Alloc20A", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gEftHitArena); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(32); c.m.w32(a_, n_); o_ };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Alloc20A", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_Alloc20B(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Alloc20B", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gEftHitArena); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(32); c.m.w32(a_, n_); o_ };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Alloc20B", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_Alloc40(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Alloc40", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gEftHitArena); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(64); c.m.w32(a_, n_); o_ };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Alloc40", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_AllocSphere(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_AllocSphere", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gEftHitArena); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(32); c.m.w32(a_, n_); o_ };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_AllocSphere", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_Alloc20C(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Alloc20C", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gEftHitArena); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(32); c.m.w32(a_, n_); o_ };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_Alloc20C", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_AllocRec(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_AllocRec", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gEftHitArena); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(400); c.m.w32(a_, n_); o_ };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_AllocRec", &PROBE_N, &[], true); }
    r_
}

pub fn EftHitArena_AllocBox(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_AllocBox", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gEftHitArena); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(48); c.m.w32(a_, n_); o_ };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHitArena_AllocBox", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetShape3(c: &mut Ctx, mut p_rec_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape3", &PROBE_N, &[(p_rec_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let mut l_shape_4: u32 = 0;
    let r_ = 'f: {
    l_shape_4 = (p_rec_1).wrapping_add(112);
    { let v_ = 3i32; let a_ = l_shape_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_2; let a_ = (l_shape_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = p_b_3; let a_ = (l_shape_4).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape3", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetShape4(c: &mut Ctx, mut p_rec_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape4", &PROBE_N, &[(p_rec_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let mut l_shape_4: u32 = 0;
    let r_ = 'f: {
    l_shape_4 = (p_rec_1).wrapping_add(112);
    { let v_ = 4i32; let a_ = l_shape_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_2; let a_ = (l_shape_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = p_b_3; let a_ = (l_shape_4).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape4", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetShape5(c: &mut Ctx, mut p_rec_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape5", &PROBE_N, &[(p_rec_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let mut l_shape_4: u32 = 0;
    let r_ = 'f: {
    l_shape_4 = (p_rec_1).wrapping_add(112);
    { let v_ = 5i32; let a_ = l_shape_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_2; let a_ = (l_shape_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = p_b_3; let a_ = (l_shape_4).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape5", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetShapeSpheres(c: &mut Ctx, mut p_rec_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShapeSpheres", &PROBE_N, &[(p_rec_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let mut l_shape_4: u32 = 0;
    let r_ = 'f: {
    l_shape_4 = (p_rec_1).wrapping_add(112);
    { let v_ = 0i32; let a_ = l_shape_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_2; let a_ = (l_shape_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = p_b_3; let a_ = (l_shape_4).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShapeSpheres", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetShape6(c: &mut Ctx, mut p_rec_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape6", &PROBE_N, &[(p_rec_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let mut l_shape_4: u32 = 0;
    let r_ = 'f: {
    l_shape_4 = (p_rec_1).wrapping_add(112);
    { let v_ = 6i32; let a_ = l_shape_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_2; let a_ = (l_shape_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = p_b_3; let a_ = (l_shape_4).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape6", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetShape2(c: &mut Ctx, mut p_rec_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape2", &PROBE_N, &[(p_rec_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let mut l_shape_4: u32 = 0;
    let r_ = 'f: {
    l_shape_4 = (p_rec_1).wrapping_add(112);
    { let v_ = 2i32; let a_ = l_shape_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_2; let a_ = (l_shape_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = p_b_3; let a_ = (l_shape_4).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShape2", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SetShapeBoxes(c: &mut Ctx, mut p_rec_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShapeBoxes", &PROBE_N, &[(p_rec_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let mut l_shape_4: u32 = 0;
    let r_ = 'f: {
    l_shape_4 = (p_rec_1).wrapping_add(112);
    { let v_ = 1i32; let a_ = l_shape_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_2; let a_ = (l_shape_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = p_b_3; let a_ = (l_shape_4).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SetShapeBoxes", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_IsTechClass(c: &mut Ctx, mut p_rec_1: u32, mut p_cls_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IsTechClass", &PROBE_N, &[(p_rec_1) as u32, (p_cls_2) as u32], false); }
    let mut l_result_3: i32 = 0;
    let r_ = 'f: {
    l_result_3 = 0i32;
    if ((((((c.m.r32((p_rec_1).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) {
        l_result_3 = ((((({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) == (p_cls_2)) as i32);
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_IsTechClass", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_GetScale(c: &mut Ctx, mut p_rec_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetScale", &PROBE_N, &[(p_rec_1) as u32], false); }
    let mut l_scale_2: F32 = F32(0);
    let r_ = 'f: {
    l_scale_2 = F32(0x3f800000);
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        l_scale_2 = { let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(132); F32(c.m.r32(a_)) };
    }
    if (((((c.m.r32((p_rec_1).wrapping_add(104))) != (((0i32) as u32))) as i32)) != 0) {
        l_scale_2 = F32(0x3dcccccc);
    }
    break 'f l_scale_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_GetScale", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_InitMultiHit(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_InitMultiHit", &PROBE_N, &[(p_rec_1) as u32], false); }
    let mut l_n_2: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        l_n_2 = (({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(10); (c.m.r8(a_) as i8) }) as i32);
        if (((((l_n_2) > (0i32)) as i32)) != 0) {
            { let v_ = { let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(10); (c.m.r8(a_) as i8) }; let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
            { let r_ = 16384i32; let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                { let r_ = (!(1i32)); let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftHit_InitMultiHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SpawnBlastImpact(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SpawnBlastImpact", &PROBE_N, &[(p_rec_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_atk_2: u32 = 0;
    let mut l_scale_3: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = F32(0x3f800000); let a_ = ((c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_scale_3 = F32(0x3f800000);
        l_atk_2 = c.m.r32((p_rec_1).wrapping_add(104));
        { let s_ = { rt::memset(c, fp.wrapping_add(48), 0, 48);; { let s_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16); rt::memcpy(c, fp.wrapping_add(48).wrapping_add(0), s_, 16); }; rt::memset(c, fp.wrapping_add(48).wrapping_add(16), 0, 16);; rt::memset(c, fp.wrapping_add(48).wrapping_add(16).wrapping_add(0), 0, 16);; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(32), (v_) as u32); }; { let v_ = (c.m.r32(p_rec_1) as i32); c.m.w32(fp.wrapping_add(48).wrapping_add(36), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(40), (v_) as u32); }; fp.wrapping_add(48) }; let d_ = fp; rt::memcpy(c, d_, s_, 48); };
        F::Vec4_Sub(c, (fp).wrapping_add(16), (p_rec_1).wrapping_add(32), (p_rec_1).wrapping_add(48));
        F::Vec3_Normalize(c, (fp).wrapping_add(16), (fp).wrapping_add(16));
        if (((((((c.m.r8((l_atk_2).wrapping_add(26))) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            l_scale_3 = F32(0x3f199999);
        }
        if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            if (((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (1024i32))) != 0) {
                { let a0_ = (c.m.r32(p_rec_1) as i32); let a1_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16); F::EftGndDust_SpawnImpact(c, a0_, a1_, F32(0x3f800000)) };
            } else {
                { let _ = F::EftImpact_SpawnBlast(c, fp, l_scale_3, F32(0x3e4ccccc)); };
            }
        } else {
            if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                if ((((!(((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (1024i32))) != 0)) as i32)) != 0) {
                    { let _ = F::EftImpact_SpawnBlast(c, fp, l_scale_3, F32(0x3f800000)); };
                } else {
                    { let a0_ = (c.m.r32(p_rec_1) as i32); let a1_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16); F::EftGndDust_SpawnImpact(c, a0_, a1_, F32(0x3f800000)) };
                }
            } else {
                { let _ = F::EftImpact_SpawnBlast(c, fp, F32(0x3f800000), F32(0x3f000000)); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SpawnBlastImpact", &PROBE_N, &[], true); }
    r_
}

pub fn EftHit_SpawnTechImpact(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SpawnTechImpact", &PROBE_N, &[(p_rec_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_src_2: u32 = 0;
    let mut l_spawn_3: i32 = 0;
    let r_ = 'f: {
        l_spawn_3 = 0i32;
        { let v_ = F32(0x3f800000); let a_ = ((c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_src_2 = c.m.r32((p_rec_1).wrapping_add(100));
        F::Vec4_Sub(c, fp, (p_rec_1).wrapping_add(48), (p_rec_1).wrapping_add(32));
        F::Vec3_Normalize(c, fp, fp);
        if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (3i32))) != 0) {
            if ((((((({ let a_ = (c.m.r32((l_src_2).wrapping_add(36))).wrapping_add(64); (c.m.r16(a_) as i16) }) as i32)) >= (0i32)) as i32)) != 0) {
                { let a1_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16); F::Vec4_Copy(c, fp.wrapping_add(16), a1_) };
                if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (16384i32))) != 0) {
                    if ((F::EftHit_IsRushHit(c, p_rec_1)) != 0) {
                        l_spawn_3 = 1i32;
                        { let a0_ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp.wrapping_add(16)) };
                    } else {
                        if (((((F::EftHit_GetHitCount(c, p_rec_1)) == (1i32)) as i32)) != 0) {
                            l_spawn_3 = 1i32;
                        }
                    }
                } else {
                    l_spawn_3 = 1i32;
                }
                if ((l_spawn_3) != 0) {
                    { let s_ = { rt::memset(c, fp.wrapping_add(128), 0, 48);; { let s_ = fp.wrapping_add(16); rt::memcpy(c, fp.wrapping_add(128).wrapping_add(0), s_, 16); }; { let s_ = fp; rt::memcpy(c, fp.wrapping_add(128).wrapping_add(16), s_, 16); }; { let v_ = (({ let a_ = (c.m.r32((l_src_2).wrapping_add(36))).wrapping_add(64); (c.m.r16(a_) as i16) }) as i32); c.m.w32(fp.wrapping_add(128).wrapping_add(32), (v_) as u32); }; { let v_ = (c.m.r32(p_rec_1) as i32); c.m.w32(fp.wrapping_add(128).wrapping_add(36), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(128).wrapping_add(40), (v_) as u32); }; fp.wrapping_add(128) }; let d_ = fp.wrapping_add(32); rt::memcpy(c, d_, s_, 48); };
                    { let _ = { let a1_ = { let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(80); F32(c.m.r32(a_)) }; F::EftImpact_SpawnBlast(c, fp.wrapping_add(32), a1_, F32(0x3f000000)) }; };
                }
            }
        } else {
            if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                if ((((((((((({ let a_ = (c.m.r32((l_src_2).wrapping_add(36))).wrapping_add(66); (c.m.r16(a_) as i16) }) as i32)) >= (0i32)) as i32)) != 0) && ((((!(({ let a0_ = (c.m.r32(p_rec_1) as i32); F::BtlCharApi_TestFlagA4(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let _ = F::BtlStage_HasFeature(c, ((((7i32) as i64 as u64)) as u32)); };
                    l_spawn_3 = 1i32;
                }
                if ((l_spawn_3) != 0) {
                    { let s_ = { rt::memset(c, fp.wrapping_add(176), 0, 48);; { let s_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16); rt::memcpy(c, fp.wrapping_add(176).wrapping_add(0), s_, 16); }; { let s_ = fp; rt::memcpy(c, fp.wrapping_add(176).wrapping_add(16), s_, 16); }; { let v_ = (({ let a_ = (c.m.r32((l_src_2).wrapping_add(36))).wrapping_add(66); (c.m.r16(a_) as i16) }) as i32); c.m.w32(fp.wrapping_add(176).wrapping_add(32), (v_) as u32); }; { let v_ = (c.m.r32(p_rec_1) as i32); c.m.w32(fp.wrapping_add(176).wrapping_add(36), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(176).wrapping_add(40), (v_) as u32); }; fp.wrapping_add(176) }; let d_ = fp.wrapping_add(80); rt::memcpy(c, d_, s_, 48); };
                    { let _ = { let a1_ = { let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(84); F32(c.m.r32(a_)) }; F::EftImpact_SpawnBlast(c, fp.wrapping_add(80), a1_, F32(0x3f800000)) }; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftHit_SpawnTechImpact", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((28u32) as i32)) }; let a_ = sym::gEftCam; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftCam); F::memset(c, a0_, 0i32, 28u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftCam); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftCam; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_Clear(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftCam)) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32(sym::gEftCam); F::memset(c, a0_, 0i32, 28u32) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_Start(c: &mut Ctx, mut p_arg_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_arg_1; let d_ = c.m.r32(sym::gEftCam); rt::memcpy(c, d_, s_, 12); };
    { let a0_ = c.m.r32(p_arg_1); F::DemoCam_SetAnim(c, a0_) };
    if ((((((c.m.r32((p_arg_1).wrapping_add(4)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a0_ = { let a0_ = (c.m.r32((p_arg_1).wrapping_add(4)) as i32); F::BtlObj_Get(c, a0_) }; F::DemoCam_SetChr(c, a0_) };
        { let a0_ = { let a0_ = (c.m.r32((p_arg_1).wrapping_add(4)) as i32); F::BtlCharApi_GetChargedUnkC78(c, a0_) }; F::DemoCam_SetTime(c, a0_) };
    } else {
        F::DemoCam_SetTime(c, F32(0x00000000));
    }
    F::DemoCam_Start(c);
    { let v_ = (c.m.r32((p_arg_1).wrapping_add(4)) as i32); let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_SetHold(c: &mut Ctx, mut p_hold_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_SetHold", &PROBE_N, &[(p_hold_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_hold_1; let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_SetHold", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_SetPaused(c: &mut Ctx, mut p_paused_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_SetPaused", &PROBE_N, &[(p_paused_1) as u32], false); }
    let r_ = 'f: {
    if (({ let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(12); (c.m.r32(a_) as i32) }) != 0) {
        { let v_ = p_paused_1; let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_SetPaused", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_IsPaused(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_IsPaused", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(16); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_IsPaused", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_IsActive(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_IsActive", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(12); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_Stop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Stop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (({ let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(12); (c.m.r32(a_) as i32) }) != 0) {
        F::DemoCam_Stop(c);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftCam_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Update", &PROBE_N, &[], false); }
    let mut l_time_1: F32 = F32(0);
    let r_ = 'f: {
    if ((((!(({ let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(12); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (({ let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(16); (c.m.r32(a_) as i32) }) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(24); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
        if ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(24); (c.m.r32(a_) as i32) }; F::BtlCharApi_IsInTechnique(c, a0_) }) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_time_1 = { let a0_ = { let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(24); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetChargedUnkC78(c, a0_) };
    } else {
        l_time_1 = rt::fadd(F::DemoCam_GetTime(c), F32(0x40000000));
    }
    if (((rt::flt(F::DemoCam_GetLength(c), l_time_1) as i32)) != 0) {
        { let a0_ = F::DemoCam_GetLength(c); F::DemoCam_SetTime(c, a0_) };
        if ((((!(({ let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(20); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
            F::DemoCam_Stop(c);
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftCam)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        F::DemoCam_SetTime(c, l_time_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCam_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_LightClutDiffuse(c: &mut Ctx, mut p_dst_1: u32, mut p_nrm_2: u32, p_light_3: u32, mut p_r_4: u8, mut p_g_5: u8, mut p_b_6: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_LightClutDiffuse", &PROBE_N, &[(p_dst_1) as u32, (p_nrm_2) as u32, (p_light_3) as u32, (p_r_4) as u32, (p_g_5) as u32, (p_b_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_7: i32 = 0;
    let mut l_v_8: i32 = 0;
    rt::memcpy(c, fp, p_light_3, 16);
    let r_ = 'f: {
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (256i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fsub(rt::i2f(((c.m.r8((p_nrm_2).wrapping_add((((l_i_7).wrapping_mul(4i32)) as u32)))) as i32)), F32(0x43000000)); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::i2f(((c.m.r8((p_nrm_2).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(1i32)) as u32)))) as i32)), F32(0x43000000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::i2f(((c.m.r8((p_nrm_2).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)))) as i32)), F32(0x43000000)); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    l_v_8 = rt::f2i(rt::fmul(F::Vec3_Dot(c, fp, fp.wrapping_add(16)), F32(0x43000000)));
                    { let r_ = ((p_r_4) as i32); let a_ = (p_dst_1).wrapping_add((((l_i_7).wrapping_mul(4i32)) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
                    { let r_ = ((p_g_5) as i32); let a_ = (p_dst_1).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(1i32)) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
                    { let r_ = ((p_b_6) as i32); let a_ = (p_dst_1).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
                    if (((((l_v_8) < (0i32)) as i32)) != 0) {
                        l_v_8 = 0i32;
                    }
                    l_v_8 = (rt::div_i32(l_v_8, 2i32)).wrapping_add(32i32);
                    if (((((l_v_8) > (128i32)) as i32)) != 0) {
                        l_v_8 = 128i32;
                    }
                    { let r_ = l_v_8; let a_ = (p_dst_1).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(3i32)) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_LightClutDiffuse", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_LightClutSpecular(c: &mut Ctx, mut p_dst_1: u32, mut p_nrm_2: u32, p_view_3: u32, p_light_4: u32, p_eye_5: u32, mut p_power_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_LightClutSpecular", &PROBE_N, &[(p_dst_1) as u32, (p_nrm_2) as u32, (p_view_3) as u32, (p_light_4) as u32, (p_eye_5) as u32, (p_power_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_i_7: i32 = 0;
    let mut l_v_8: i32 = 0;
    let mut l_d_9: F32 = F32(0);
    rt::memcpy(c, fp, p_view_3, 64);
    rt::memcpy(c, fp.wrapping_add(64), p_light_4, 64);
    rt::memcpy(c, fp.wrapping_add(128), p_eye_5, 16);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(208), 0, 16);
        rt::memset(c, fp.wrapping_add(208).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(208).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(208).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(208).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(208).wrapping_add(0).wrapping_add(12), (v_).0); }
        l_d_9 = F32(0x00000000);
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(208)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Mtx_InverseRT(c, fp.wrapping_add(224), fp);
        { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16))))); let a_ = fp.wrapping_add(208); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32(((fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); let a_ = (fp.wrapping_add(208)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32(((fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)))); let a_ = (fp.wrapping_add(208)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let v_ = { let v_ = l_d_9; let a_ = ((fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); v_ }; let a_ = (fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, fp.wrapping_add(208), (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let a_ = fp.wrapping_add(192); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, fp.wrapping_add(208), (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let a_ = (fp.wrapping_add(192)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, fp.wrapping_add(208), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let a_ = (fp.wrapping_add(192)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(192)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (256i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fsub(rt::i2f(((c.m.r8((p_nrm_2).wrapping_add((((l_i_7).wrapping_mul(4i32)) as u32)))) as i32)), F32(0x43000000)); let a_ = fp.wrapping_add(176); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::i2f(((c.m.r8((p_nrm_2).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(1i32)) as u32)))) as i32)), F32(0x43000000)); let a_ = (fp.wrapping_add(176)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::i2f(((c.m.r8((p_nrm_2).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)))) as i32)), F32(0x43000000)); let a_ = (fp.wrapping_add(176)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(176)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, fp.wrapping_add(176), fp.wrapping_add(176));
                    l_d_9 = F::Vec3_Dot(c, fp.wrapping_add(128), fp.wrapping_add(176));
                    l_d_9 = rt::fadd(l_d_9, l_d_9);
                    { let v_ = rt::fsub(rt::fmul(F32(c.m.r32(fp.wrapping_add(176))), l_d_9), F32(c.m.r32(fp.wrapping_add(128)))); let a_ = fp.wrapping_add(160); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(4))), l_d_9), F32(c.m.r32((fp.wrapping_add(128)).wrapping_add(4)))); let a_ = (fp.wrapping_add(160)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(8))), l_d_9), F32(c.m.r32((fp.wrapping_add(128)).wrapping_add(8)))); let a_ = (fp.wrapping_add(160)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(160)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Mtx_MulVec4(c, fp.wrapping_add(144), fp.wrapping_add(224), fp.wrapping_add(160));
                    l_v_8 = rt::f2i(rt::fmul(F::Vec3_Dot(c, fp.wrapping_add(192), fp.wrapping_add(144)), rt::fmul(p_power_6, F32(0x437f0000))));
                    if (((((l_v_8) < (0i32)) as i32)) != 0) {
                        l_v_8 = 0i32;
                    }
                    if (((((l_v_8) > (255i32)) as i32)) != 0) {
                        l_v_8 = 255i32;
                    }
                    { let r_ = l_v_8; let a_ = (p_dst_1).wrapping_add((((l_i_7).wrapping_mul(4i32)) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
                    { let r_ = l_v_8; let a_ = (p_dst_1).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(1i32)) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
                    { let r_ = l_v_8; let a_ = (p_dst_1).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
                    { let r_ = 128i32; let a_ = (p_dst_1).wrapping_add(((((l_i_7).wrapping_mul(4i32)).wrapping_add(3i32)) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_LightClutSpecular", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_LerpClut(c: &mut Ctx, mut p_dst_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_t_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_LerpClut", &PROBE_N, &[(p_dst_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_t_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        if (((rt::flt(p_t_4, F32(0x00000000)) as i32)) != 0) {
            p_t_4 = F32(0x00000000);
        }
        if (((rt::fgt(p_t_4, F32(0x3f800000)) as i32)) != 0) {
            p_t_4 = F32(0x3f800000);
        }
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (256i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = rt::i2f((c.m.r8((p_a_2).wrapping_add((((l_i_5).wrapping_mul(4i32)) as u32)))) as i32); let a2_ = rt::i2f((c.m.r8((p_a_2).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(1i32)) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((p_a_2).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)))) as i32); let a4_ = rt::i2f((c.m.r8((p_a_2).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(3i32)) as u32)))) as i32); F::Vec4_Set(c, fp, a1_, a2_, a3_, a4_) };
                    { let a1_ = rt::i2f((c.m.r8((p_b_3).wrapping_add((((l_i_5).wrapping_mul(4i32)) as u32)))) as i32); let a2_ = rt::i2f((c.m.r8((p_b_3).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(1i32)) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((p_b_3).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)))) as i32); let a4_ = rt::i2f((c.m.r8((p_b_3).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(3i32)) as u32)))) as i32); F::Vec4_Set(c, fp.wrapping_add(16), a1_, a2_, a3_, a4_) };
                    F::Vec4_Lerp(c, fp.wrapping_add(32), fp, fp.wrapping_add(16), p_t_4);
                    { let v_ = ((rt::f2u(F32(c.m.r32(fp.wrapping_add(32))))) as u8); let a_ = (p_dst_1).wrapping_add((((l_i_5).wrapping_mul(4i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2u(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))))) as u8); let a_ = (p_dst_1).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2u(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))))) as u8); let a_ = (p_dst_1).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2u(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(12))))) as u8); let a_ = (p_dst_1).wrapping_add(((((l_i_5).wrapping_mul(4i32)).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_LerpClut", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_CalcTangentFrame(c: &mut Ctx, mut p_out_1: u32, mut p_pos_2: u32, mut p_uv_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_CalcTangentFrame", &PROBE_N, &[(p_out_1) as u32, (p_pos_2) as u32, (p_uv_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let a1_ = rt::fsub(F32(c.m.r32((p_pos_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_pos_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a2_ = rt::fsub(F32(c.m.r32((p_uv_3).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a3_ = rt::fsub(F32(c.m.r32(((p_uv_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); F::Vec4_Set(c, fp.wrapping_add(32), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = rt::fsub(F32(c.m.r32((p_pos_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_pos_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a2_ = rt::fsub(F32(c.m.r32((p_uv_3).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a3_ = rt::fsub(F32(c.m.r32(((p_uv_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); F::Vec4_Set(c, fp.wrapping_add(48), a1_, a2_, a3_, F32(0x3f800000)) };
        F::Vec3_Cross(c, fp.wrapping_add(16), fp.wrapping_add(32), fp.wrapping_add(48));
        { let v_ = rt::fdiv(rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), F32(c.m.r32(fp.wrapping_add(16)))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))), F32(c.m.r32(fp.wrapping_add(16)))); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let a1_ = rt::fsub(F32(c.m.r32(((p_pos_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_pos_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); let a2_ = rt::fsub(F32(c.m.r32((p_uv_3).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a3_ = rt::fsub(F32(c.m.r32(((p_uv_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); F::Vec4_Set(c, fp.wrapping_add(32), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = rt::fsub(F32(c.m.r32(((p_pos_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_pos_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); let a2_ = rt::fsub(F32(c.m.r32((p_uv_3).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a3_ = rt::fsub(F32(c.m.r32(((p_uv_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); F::Vec4_Set(c, fp.wrapping_add(48), a1_, a2_, a3_, F32(0x3f800000)) };
        F::Vec3_Cross(c, fp.wrapping_add(16), fp.wrapping_add(32), fp.wrapping_add(48));
        { let v_ = rt::fdiv(rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), F32(c.m.r32(fp.wrapping_add(16)))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))), F32(c.m.r32(fp.wrapping_add(16)))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let a1_ = rt::fsub(F32(c.m.r32(((p_pos_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_pos_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))); let a2_ = rt::fsub(F32(c.m.r32((p_uv_3).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a3_ = rt::fsub(F32(c.m.r32(((p_uv_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); F::Vec4_Set(c, fp.wrapping_add(32), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = rt::fsub(F32(c.m.r32(((p_pos_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_pos_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))); let a2_ = rt::fsub(F32(c.m.r32((p_uv_3).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a3_ = rt::fsub(F32(c.m.r32(((p_uv_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_uv_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); F::Vec4_Set(c, fp.wrapping_add(48), a1_, a2_, a3_, F32(0x3f800000)) };
        F::Vec3_Cross(c, fp.wrapping_add(16), fp.wrapping_add(32), fp.wrapping_add(48));
        { let v_ = rt::fdiv(rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), F32(c.m.r32(fp.wrapping_add(16)))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))), F32(c.m.r32(fp.wrapping_add(16)))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec3_Cross(c, fp, (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec3_Cross(c, (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp, (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec3_Normalize(c, (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec3_Normalize(c, (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec3_Normalize(c, (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp);
        { let v_ = F32(0x3f800000); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_CalcTangentFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_MtxFromDir(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32, mut p_roll_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_MtxFromDir", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32, (p_roll_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
        { let v_ = F32(0xbf800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(32), p_dir_2);
        if (((((((((((rt::feq(F32(c.m.r32(fp.wrapping_add(32))), F32(0x00000000)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::Vec3_Cross(c, fp.wrapping_add(16), fp, fp.wrapping_add(32));
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp, fp.wrapping_add(16), fp.wrapping_add(32));
        F::Vec3_Normalize(c, fp, fp);
        F::Mtx_StoreIdentity(c, p_out_1);
        { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp)); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(32))); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        if (((rt::fne(p_roll_3, F32(0x00000000)) as i32)) != 0) {
            F::Mtx_StoreIdentity(c, fp.wrapping_add(48));
            F::Mtx_RotateZ(c, fp.wrapping_add(48), fp.wrapping_add(48), p_roll_3);
            F::Mtx_Mul(c, p_out_1, p_out_1, fp.wrapping_add(48));
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_MtxFromDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_WrapAngle(c: &mut Ctx, mut p_angle_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_WrapAngle", &PROBE_N, &[(p_angle_1).0], false); }
    let mut l_n_2: i32 = 0;
    let r_ = 'f: {
    if (((rt::fge(p_angle_1, F32(0x40490fda)) as i32)) != 0) {
        l_n_2 = rt::f2i(rt::fdiv(rt::fsub(p_angle_1, F32(0x40490fda)), F32(0x40c90fda)));
        l_n_2 = F::abs(c, l_n_2);
        { let r_ = rt::fadd(rt::fmul(rt::i2f(l_n_2), F32(0x40c90fda)), F32(0x40c90fda)); let n_ = rt::fsub(p_angle_1, r_); p_angle_1 = n_; };
    } else {
        if (((rt::fle(p_angle_1, F32(0xc0490fda)) as i32)) != 0) {
            l_n_2 = rt::f2i(rt::fdiv(rt::fadd(p_angle_1, F32(0x40490fda)), F32(0x40c90fda)));
            l_n_2 = F::abs(c, l_n_2);
            { let r_ = rt::fadd(rt::fmul(rt::i2f(l_n_2), F32(0x40c90fda)), F32(0x40c90fda)); let n_ = rt::fadd(p_angle_1, r_); p_angle_1 = n_; };
        }
    }
    break 'f p_angle_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMath_WrapAngle", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_CatmullRom(c: &mut Ctx, p_out_1: u32, mut p_p_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_CatmullRom", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    rt::memcpy(c, fp, p_out_1, 16);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x40400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0xc0400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x40000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xc0a00000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x40800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x40000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        F::Vec4_Set(c, fp.wrapping_add(80), rt::fmul(rt::fmul(p_t_3, p_t_3), p_t_3), rt::fmul(p_t_3, p_t_3), p_t_3, F32(0x3f800000));
        F::Mtx_MulVec4(c, fp.wrapping_add(80), fp.wrapping_add(16), fp.wrapping_add(80));
        F::Mtx_MulVec4(c, fp.wrapping_add(80), fp.wrapping_add(96), fp.wrapping_add(80));
        F::Vec4_Div(c, fp, fp.wrapping_add(80), F32(0x40000000));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_CatmullRom", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_Spline3(c: &mut Ctx, mut p_out_1: u32, mut p_p_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_Spline3", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 64);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(0).wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0xc0000000); c.m.w32(fp.wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(0).wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xc0400000); c.m.w32(fp.wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x40800000); c.m.w32(fp.wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(0).wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x40000000); c.m.w32(fp.wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        F::Vec4_Set(c, fp.wrapping_add(128), rt::fmul(rt::fmul(p_t_3, p_t_3), p_t_3), rt::fmul(p_t_3, p_t_3), p_t_3, F32(0x3f800000));
        F::Mtx_MulVec4(c, fp.wrapping_add(128), fp, fp.wrapping_add(128));
        F::Mtx_MulVec4(c, fp.wrapping_add(128), fp.wrapping_add(64), fp.wrapping_add(128));
        F::Vec4_Div(c, p_out_1, fp.wrapping_add(128), F32(0x40000000));
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_Spline3", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_Spline3B(c: &mut Ctx, p_out_1: u32, mut p_p_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_Spline3B", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    rt::memcpy(c, fp, p_out_1, 16);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xc0000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x40000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        F::Vec4_Set(c, fp.wrapping_add(144), rt::fmul(rt::fmul(p_t_3, p_t_3), p_t_3), rt::fmul(p_t_3, p_t_3), p_t_3, F32(0x3f800000));
        F::Mtx_MulVec4(c, fp.wrapping_add(144), fp.wrapping_add(16), fp.wrapping_add(144));
        F::Mtx_MulVec4(c, fp.wrapping_add(144), fp.wrapping_add(80), fp.wrapping_add(144));
        F::Vec4_Div(c, fp, fp.wrapping_add(144), F32(0x40000000));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_Spline3B", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_Bezier(c: &mut Ctx, p_out_1: u32, mut p_p_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_Bezier", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    rt::memcpy(c, fp, p_out_1, 16);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x40400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0xc0400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x40400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xc0c00000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x40400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0xc0400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x40400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        F::Vec4_Set(c, fp.wrapping_add(144), rt::fmul(rt::fmul(p_t_3, p_t_3), p_t_3), rt::fmul(p_t_3, p_t_3), p_t_3, F32(0x3f800000));
        F::Mtx_MulVec4(c, fp.wrapping_add(144), fp.wrapping_add(16), fp.wrapping_add(144));
        F::Mtx_MulVec4(c, fp.wrapping_add(144), fp.wrapping_add(80), fp.wrapping_add(144));
        F::Vec4_Copy(c, fp, fp.wrapping_add(144));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_Bezier", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_Lerp(c: &mut Ctx, p_out_1: u32, mut p_p_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_Lerp", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_4: i32 = 0;
    rt::memcpy(c, fp, p_out_1, 16);
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fadd(F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))))), p_t_3)); let a_ = (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_Lerp", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_TcbSpline(c: &mut Ctx, p_out_1: u32, mut p_p_2: u32, mut p_t_3: F32, mut p_tension_4: F32, mut p_bias_5: F32, mut p_continuity_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_TcbSpline", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32, (p_t_3).0, (p_tension_4).0, (p_bias_5).0, (p_continuity_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(208) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 208);
    rt::memcpy(c, fp, p_out_1, 16);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x40000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xc0000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0xc0400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x40400000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0xc0000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        F::Vec4_Set(c, fp.wrapping_add(80), rt::fmul(rt::fmul(p_t_3, p_t_3), p_t_3), rt::fmul(p_t_3, p_t_3), p_t_3, F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(96), rt::fmul(rt::fmul(rt::fmul(rt::fsub(F32(0x3f800000), p_tension_4), rt::fadd(p_bias_5, F32(0x3f800000))), rt::fadd(p_continuity_6, F32(0x3f800000))), F32(0x3f000000)), rt::fmul(rt::fmul(rt::fmul(rt::fsub(F32(0x3f800000), p_tension_4), rt::fsub(F32(0x3f800000), p_bias_5)), rt::fsub(F32(0x3f800000), p_continuity_6)), F32(0x3f000000)), rt::fmul(rt::fmul(rt::fmul(rt::fsub(F32(0x3f800000), p_tension_4), rt::fsub(F32(0x3f800000), p_bias_5)), rt::fadd(p_continuity_6, F32(0x3f800000))), F32(0x3f000000)), rt::fmul(rt::fmul(rt::fmul(rt::fsub(F32(0x3f800000), p_tension_4), rt::fadd(p_bias_5, F32(0x3f800000))), rt::fsub(F32(0x3f800000), p_continuity_6)), F32(0x3f000000)));
        F::Mtx_MulVec4(c, fp.wrapping_add(176), fp.wrapping_add(16), fp.wrapping_add(80));
        F::Vec4_Sub(c, (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        { let a2_ = F32(c.m.r32(fp.wrapping_add(96))); F::Vec4_Scale(c, fp.wrapping_add(192), (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a2_) };
        { let a2_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4))); F::Vec4_Scale(c, (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a2_) };
        F::Vec4_Add(c, (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(192));
        { let a2_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))); F::Vec4_Scale(c, fp.wrapping_add(192), (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a2_) };
        { let a2_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(12))); F::Vec4_Scale(c, (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a2_) };
        F::Vec4_Add(c, (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(192));
        F::Vec4_Copy(c, (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Mtx_MulVec4(c, fp, fp.wrapping_add(112), fp.wrapping_add(176));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_TcbSpline", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_GetClipPlanes(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_GetClipPlanes", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gEftClipPlanes;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_GetClipPlanes", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_UpdateClipPlanes(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_UpdateClipPlanes", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_plane_1: u32 = 0;
    let mut l_angle_2: F32 = F32(0);
    let mut l_aspect_3: F32 = F32(0);
    let mut l_one_4: F32 = F32(0);
    let mut l_angle2_5: F32 = F32(0);
    let mut l_s_6: F32 = F32(0);
    let mut l_c_7: F32 = F32(0);
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
        l_plane_1 = sym::gEftClipPlanes;
        l_angle_2 = F32(0x3f060a91);
        l_aspect_3 = rt::fdiv(F32(0x3f400000), F::Port_WideFactor(c));
        l_one_4 = F32(0x3f800000);
        l_s_6 = F::Port_sinf(c, l_angle_2);
        l_c_7 = F::Port_cosf(c, l_angle_2);
        { let v_ = F32(0x00000000); let a_ = (l_plane_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_7; let a_ = ((l_plane_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_6; let a_ = ((l_plane_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_4; let a_ = ((l_plane_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_plane_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_c_7); let a_ = ((l_plane_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_6; let a_ = ((l_plane_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_4; let a_ = ((l_plane_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_angle2_5 = F::Port_atanf(c, rt::fdiv(l_s_6, rt::fmul(l_c_7, l_aspect_3)));
        l_s_6 = F::Port_sinf(c, l_angle2_5);
        l_c_7 = F::Port_cosf(c, l_angle2_5);
        { let v_ = rt::fneg(l_c_7); let a_ = (l_plane_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_plane_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_6; let a_ = ((l_plane_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_4; let a_ = ((l_plane_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_7; let a_ = (l_plane_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_plane_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_6; let a_ = ((l_plane_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_4; let a_ = ((l_plane_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_plane_1).wrapping_add((((4i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_plane_1).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_4; let a_ = ((l_plane_1).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_one_4); let a_ = ((l_plane_1).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(((64i32) as u32)); F::Mtx_InverseRT(c, fp, a1_) };
        F::Vec4_Copy(c, fp.wrapping_add(64), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); v_ }; let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (l_plane_1).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)), fp, (l_plane_1).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
                    { let v_ = rt::fneg(F::Vec3_Dot(c, (l_plane_1).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)), fp.wrapping_add(64))); let a_ = ((l_plane_1).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        { let r_ = F32(0x3e999999); let a_ = ((l_plane_1).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(12); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_UpdateClipPlanes", &PROBE_N, &[], true); }
    r_
}

pub fn EftAim_GetDir(c: &mut Ctx, mut p_out_1: u32, mut p_arg_2: i32, mut p_objId_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAim_GetDir", &PROBE_N, &[(p_out_1) as u32, (p_arg_2) as u32, (p_objId_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((F::BtlCharApi_IsLockedOn(c, p_objId_3)) != 0) {
            F::BtlCharApi_CalcAimDir45(c, p_objId_3, ((((p_arg_2) as i64 as u64)) as u32), p_out_1);
        } else {
            F::BtlCharApi_GetRot(c, p_objId_3, fp);
            { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_sinf(c, a0_) }; let a_ = p_out_1; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_cosf(c, a0_) }; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        F::Vec3_Normalize(c, p_out_1, p_out_1);
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAim_GetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftAim_GetDirKeep(c: &mut Ctx, mut p_src_1: u32, mut p_out_2: u32, mut p_arg_3: i32, mut p_objId_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAim_GetDirKeep", &PROBE_N, &[(p_src_1) as u32, (p_out_2) as u32, (p_arg_3) as u32, (p_objId_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        F::BtlCharApi_GetRot(c, p_objId_4, fp);
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_sinf(c, a0_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_cosf(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((F::BtlCharApi_IsLockedOn(c, p_objId_4)) != 0) {
            if (((((p_src_1) != (((0i32) as u32))) as i32)) != 0) {
                if ((((((({ let a_ = (c.m.r32((p_src_1).wrapping_add(36))).wrapping_add(12); (c.m.r8(a_) as i8) }) as i32)) == (0i32)) as i32)) != 0) {
                    F::BtlCharApi_CalcAimDir45(c, p_objId_4, ((((p_arg_3) as i64 as u64)) as u32), p_out_2);
                } else {
                    if ((((!(((((c.m.r32((p_src_1).wrapping_add(64)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        F::BtlCharApi_CalcAimDir45(c, p_objId_4, ((((p_arg_3) as i64 as u64)) as u32), p_out_2);
                        F::Vec4_Copy(c, (p_src_1).wrapping_add(48), p_out_2);
                        { let r_ = 2i32; let a_ = (p_src_1).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        F::Vec4_Copy(c, p_out_2, (p_src_1).wrapping_add(48));
                    }
                }
                F::Vec3_Normalize(c, p_out_2, p_out_2);
                F::Vec4_Copy(c, fp.wrapping_add(32), p_out_2);
                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(16)), F32(0x3f000000)) as i32)) != 0) {
                    F::Vec4_Copy(c, p_out_2, fp.wrapping_add(16));
                }
            }
        } else {
            F::Vec4_Copy(c, p_out_2, fp.wrapping_add(16));
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAim_GetDirKeep", &PROBE_N, &[], true); }
    r_
}

pub fn EftMath_RotateAboutAxis(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_axis_3: u32, mut p_angle_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMath_RotateAboutAxis", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_axis_3) as u32, (p_angle_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_cs_5: F32 = F32(0);
    let mut l_sn_6: F32 = F32(0);
    let r_ = 'f: {
        l_cs_5 = F::Port_cosf(c, p_angle_4);
        l_sn_6 = F::Port_sinf(c, p_angle_4);
        F::Vec3_Scale(c, fp, p_v_2, l_cs_5);
        F::Vec3_Cross(c, fp.wrapping_add(16), p_axis_3, p_v_2);
        F::Vec3_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), l_sn_6);
        { let a2_ = rt::fmul(F::Vec3_Dot(c, p_axis_3, p_v_2), rt::fsub(F32(0x3f800000), l_cs_5)); F::Vec3_Scale(c, fp.wrapping_add(32), p_axis_3, a2_) };
        F::Vec3_Add(c, p_out_1, fp, fp.wrapping_add(16));
        F::Vec3_Add(c, p_out_1, p_out_1, fp.wrapping_add(32));
        { let v_ = F32(c.m.r32((p_v_2).wrapping_add(12))); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMath_RotateAboutAxis", &PROBE_N, &[], true); }
    r_
}

pub fn EftAim_Home(c: &mut Ctx, mut p_out_1: u32, mut p_pos_2: u32, mut p_dir_3: u32, mut p_objId_4: i32, mut p_speed_5: F32, mut p_maxTurn_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAim_Home", &PROBE_N, &[(p_out_1) as u32, (p_pos_2) as u32, (p_dir_3) as u32, (p_objId_4) as u32, (p_speed_5).0, (p_maxTurn_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_opp_7: i32 = 0;
    let mut l_frames_8: F32 = F32(0);
    let mut l_t_9: F32 = F32(0);
    let mut l_d_10: F32 = F32(0);
    let mut l_angle_11: F32 = F32(0);
    let r_ = 'f: {
        l_opp_7 = F::BtlCharApi_GetOpponentObjId(c, p_objId_4);
        F::Vec3_Normalize(c, p_dir_3, p_dir_3);
        if ((((!((F::BtlCharApi_IsLockedOn(c, p_objId_4)) != 0)) as i32)) != 0) {
            F::Vec4_Copy(c, p_out_1, p_dir_3);
            break 'f;
        }
        F::BtlCharApi_GetNodePos(c, l_opp_7, 17i32, fp);
        F::Vec4_Sub(c, fp.wrapping_add(80), fp, p_pos_2);
        l_frames_8 = rt::fdiv({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(80)); F::Port_sqrtf(c, a0_) }, p_speed_5);
        p_speed_5 = l_frames_8;
        if (((rt::fgt(p_speed_5, F32(0x41700000)) as i32)) != 0) {
            p_speed_5 = F32(0x41700000);
        }
        F::BtlCharApi_GetFrameMove(c, l_opp_7, fp.wrapping_add(16));
        F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(16), rt::fmul(p_speed_5, F32(0x3f000000)));
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(32));
        F::Vec4_Sub(c, fp.wrapping_add(80), fp, p_pos_2);
        F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
        F::Vec3_Normalize(c, fp.wrapping_add(64), p_dir_3);
        l_d_10 = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(64));
        if (((rt::fgt(l_d_10, F32(0x00000000)) as i32)) != 0) {
            if (((rt::fgt(l_d_10, F32(0x3f800000)) as i32)) != 0) {
                l_d_10 = F32(0x3f800000);
            }
            l_angle_11 = F::Port_acosf(c, l_d_10);
            if (((rt::fgt(l_frames_8, F32(0x40000000)) as i32)) != 0) {
                { let r_ = rt::fmul(l_frames_8, F32(0x3f000000)); let n_ = rt::fdiv(l_angle_11, r_); l_angle_11 = n_; };
            }
            l_t_9 = p_maxTurn_6;
            if ((((!(((rt::flt(l_t_9, l_angle_11) as i32)) != 0)) as i32)) != 0) {
                l_t_9 = l_angle_11;
            }
            F::Vec3_Cross(c, fp.wrapping_add(96), fp.wrapping_add(64), fp.wrapping_add(80));
            F::Vec3_Normalize(c, fp.wrapping_add(96), fp.wrapping_add(96));
            F::EftMath_RotateAboutAxis(c, fp.wrapping_add(48), fp.wrapping_add(64), fp.wrapping_add(96), l_t_9);
            F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
        } else {
            F::Vec4_Copy(c, fp.wrapping_add(48), fp.wrapping_add(64));
        }
        F::Vec4_Copy(c, p_out_1, fp.wrapping_add(48));
        F::Vec3_Normalize(c, p_out_1, p_out_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAim_Home", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_DrawPolyAvgZ(c: &mut Ctx, mut p_verts_1: u32, mut p_arg1_2: i32, mut p_arg2_3: i32, mut p_arg3_4: i32, mut p_flip_5: i32, mut p_tex_6: u64, mut p_zOfs_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawPolyAvgZ", &PROBE_N, &[(p_verts_1) as u32, (p_arg1_2) as u32, (p_arg2_3) as u32, (p_arg3_4) as u32, (p_flip_5) as u32, (p_tex_6) as u32, (p_zOfs_7) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_8: u32 = 0;
    let mut l_count_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_z_11: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_verts_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_verts_1 = fp;
        l_count_9 = 3i32;
        l_plane_8 = F::EftGfx_GetClipPlanes(c);
        {
            l_i_10 = 0i32;
            'l1: loop {
                if !(((((l_i_10) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_count_9 = F::ClipPoly_ClipPlane(c, p_verts_1, l_plane_8, l_count_9);
                    { let o_ = l_plane_8; let n_ = (o_).wrapping_add(16); l_plane_8 = n_; };
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        if (((((l_count_9) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_verts_1, l_count_9);
            {
                l_i_10 = 2i32;
                'l3: loop {
                    if !(((((l_i_10) < (l_count_9)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_z_11 = (rt::div_i32((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_10).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)), 3i32)).wrapping_shr((8i32) as u32);
                        if ((p_flip_5) != 0) {
                            l_z_11 = (4096i32).wrapping_sub(l_z_11);
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_10).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_10).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        F::EftPrim_DrawTriangle(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_10).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)), ((p_verts_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_verts_1).wrapping_add(((((l_i_10).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_verts_1).wrapping_add((((l_i_10) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_10).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)), p_arg2_3, p_arg3_4, p_arg1_2, (l_z_11).wrapping_add(p_zOfs_7), p_tex_6);
                    }
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawPolyAvgZ", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_DrawPolyFixedZ(c: &mut Ctx, mut p_verts_1: u32, mut p_arg1_2: i32, mut p_arg2_3: i32, mut p_arg3_4: i32, mut p_front_5: i32, mut p_flip_6: i32, mut p_tex_7: u64, mut p_z_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawPolyFixedZ", &PROBE_N, &[(p_verts_1) as u32, (p_arg1_2) as u32, (p_arg2_3) as u32, (p_arg3_4) as u32, (p_front_5) as u32, (p_flip_6) as u32, (p_tex_7) as u32, (p_z_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_9: u32 = 0;
    let mut l_count_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_verts_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_verts_1 = fp;
        l_count_10 = 3i32;
        l_plane_9 = F::EftGfx_GetClipPlanes(c);
        {
            l_i_11 = 0i32;
            'l1: loop {
                if !(((((l_i_11) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_count_10 = F::ClipPoly_ClipPlane(c, p_verts_1, l_plane_9, l_count_10);
                    { let o_ = l_plane_9; let n_ = (o_).wrapping_add(16); l_plane_9 = n_; };
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
        if (((((l_count_10) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_verts_1, l_count_10);
            {
                l_i_11 = 2i32;
                'l3: loop {
                    if !(((((l_i_11) < (l_count_10)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if ((p_flip_6) != 0) {
                            p_z_8 = (4096i32).wrapping_sub(p_z_8);
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((p_front_5) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        F::EftPrim_DrawTriangle(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), ((p_verts_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_verts_1).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_verts_1).wrapping_add((((l_i_11) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), p_arg2_3, p_arg3_4, p_arg1_2, p_z_8, p_tex_7);
                    }
                    { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawPolyFixedZ", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_DrawPolyAvgZFront(c: &mut Ctx, mut p_verts_1: u32, mut p_arg1_2: i32, mut p_arg2_3: i32, mut p_arg3_4: i32, mut p_front_5: i32, mut p_flip_6: i32, mut p_tex_7: u64, mut p_zOfs_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawPolyAvgZFront", &PROBE_N, &[(p_verts_1) as u32, (p_arg1_2) as u32, (p_arg2_3) as u32, (p_arg3_4) as u32, (p_front_5) as u32, (p_flip_6) as u32, (p_tex_7) as u32, (p_zOfs_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_9: u32 = 0;
    let mut l_count_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_z_12: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_verts_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_verts_1 = fp;
        l_count_10 = 3i32;
        l_plane_9 = F::EftGfx_GetClipPlanes(c);
        {
            l_i_11 = 0i32;
            'l1: loop {
                if !(((((l_i_11) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_count_10 = F::ClipPoly_ClipPlane(c, p_verts_1, l_plane_9, l_count_10);
                    { let o_ = l_plane_9; let n_ = (o_).wrapping_add(16); l_plane_9 = n_; };
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
        if (((((l_count_10) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_verts_1, l_count_10);
            {
                l_i_11 = 2i32;
                'l3: loop {
                    if !(((((l_i_11) < (l_count_10)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_z_12 = (rt::div_i32((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)), 3i32)).wrapping_shr((8i32) as u32);
                        if ((p_flip_6) != 0) {
                            l_z_12 = (4096i32).wrapping_sub(l_z_12);
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((p_front_5) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        F::EftPrim_DrawTriangle(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), ((p_verts_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_verts_1).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_verts_1).wrapping_add((((l_i_11) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), p_arg2_3, p_arg3_4, p_arg1_2, (l_z_12).wrapping_add(p_zOfs_8), p_tex_7);
                    }
                    { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawPolyAvgZFront", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_DrawPolyScaledZ(c: &mut Ctx, mut p_verts_1: u32, mut p_arg1_2: i32, mut p_arg2_3: i32, mut p_arg3_4: i32, mut p_front_5: i32, mut p_flip_6: i32, mut p_tex_7: u64, mut p_zScale_8: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawPolyScaledZ", &PROBE_N, &[(p_verts_1) as u32, (p_arg1_2) as u32, (p_arg2_3) as u32, (p_arg3_4) as u32, (p_front_5) as u32, (p_flip_6) as u32, (p_tex_7) as u32, (p_zScale_8).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_9: u32 = 0;
    let mut l_count_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_z_12: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_verts_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_verts_1 = fp;
        l_count_10 = 3i32;
        l_plane_9 = F::EftGfx_GetClipPlanes(c);
        {
            l_i_11 = 0i32;
            'l1: loop {
                if !(((((l_i_11) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_count_10 = F::ClipPoly_ClipPlane(c, p_verts_1, l_plane_9, l_count_10);
                    { let o_ = l_plane_9; let n_ = (o_).wrapping_add(16); l_plane_9 = n_; };
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
        if (((((l_count_10) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_verts_1, l_count_10);
            {
                l_i_11 = 2i32;
                'l3: loop {
                    if !(((((l_i_11) < (l_count_10)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_z_12 = (rt::div_i32((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)), 3i32)).wrapping_shr((8i32) as u32);
                        if ((p_flip_6) != 0) {
                            l_z_12 = (4096i32).wrapping_sub(l_z_12);
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((p_front_5) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        F::EftPrim_DrawTriangle(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), ((p_verts_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_verts_1).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_verts_1).wrapping_add((((l_i_11) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), p_arg2_3, p_arg3_4, p_arg1_2, rt::f2i(rt::fmul(rt::i2f(l_z_12), p_zScale_8)), p_tex_7);
                    }
                    { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawPolyScaledZ", &PROBE_N, &[], true); }
    r_
}

pub fn EftGfx_DrawSprite(c: &mut Ctx, mut p_pos_1: u32, mut p_color_2: u32, mut p_w_3: F32, mut p_h_4: F32, mut p_u0_5: F32, mut p_v0_6: F32, mut p_u1_7: F32, mut p_v1_8: F32, mut p_rot_9: F32, mut p_layer_10: i32, mut p_front_11: i32, mut p_tex0_12: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawSprite", &PROBE_N, &[(p_pos_1) as u32, (p_color_2) as u32, (p_w_3).0, (p_h_4).0, (p_u0_5).0, (p_v0_6).0, (p_u1_7).0, (p_v1_8).0, (p_rot_9).0, (p_layer_10) as u32, (p_front_11) as u32, (p_tex0_12) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_scale_13: F32 = F32(0);
    let mut l_sw_14: i32 = 0;
    let mut l_sh_15: i32 = 0;
    let mut l_z_16: i32 = 0;
    let mut l_x0_17: i32 = 0;
    let mut l_y0_18: i32 = 0;
    let mut l_x1_19: i32 = 0;
    let mut l_y1_20: i32 = 0;
    let mut l_x2_21: i32 = 0;
    let mut l_y2_22: i32 = 0;
    let mut l_x3_23: i32 = 0;
    let mut l_y3_24: i32 = 0;
    let mut l_zz_25: i32 = 0;
    let mut l_abe_26: i32 = 0;
    let mut l_ctx_27: i32 = 0;
    let mut l_l_28: i32 = 0;
    let mut l_p_29: u32 = 0;
    let mut l_e_30: u32 = 0;
    let r_ = 'f: {
        l_scale_13 = { let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(((564i32) as u32)); F32(c.m.r32(a_)) };
        l_abe_26 = 1i32;
        { let a2_ = rt::fmul(F32(0x3f955555), F::Port_WideFactor(c)); F::Vec4_Set(c, fp.wrapping_add(64), F32(0x3f800000), a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let v_ = F32(0x3f800000); let a_ = (p_pos_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(112), p_pos_1); };
        l_sw_14 = rt::f2i(p_w_3);
        l_sh_15 = rt::f2i(p_h_4);
        { let r_ = F32(0x45800000); let n_ = rt::fmul(l_scale_13, r_); l_scale_13 = n_; };
        l_z_16 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        { let r_ = rt::i2f((c.m.r32((fp.wrapping_add(112)).wrapping_add(12)) as i32)); let n_ = rt::fdiv(l_scale_13, r_); l_scale_13 = n_; };
        l_sw_14 = ((l_sw_14).wrapping_mul(rt::f2i(l_scale_13))).wrapping_shr((12i32) as u32);
        l_sh_15 = ((l_sh_15).wrapping_mul(rt::f2i(l_scale_13))).wrapping_shr((12i32) as u32);
        if (((((((((l_sw_14) < (2i32)) as i32)) != 0) || (((((l_sh_15) < (2i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        p_w_3 = rt::i2f(l_sw_14);
        p_h_4 = rt::i2f(l_sh_15);
        F::Vec4_Set(c, fp.wrapping_add(80), p_w_3, p_h_4, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(96), rt::fneg(p_w_3), p_h_4, F32(0x00000000), F32(0x3f800000));
        F::Mtx_StoreIdentity(c, fp);
        F::Mtx_RotateZ(c, fp, fp, p_rot_9);
        F::Mtx_MulVec4(c, fp.wrapping_add(80), fp, fp.wrapping_add(80));
        F::Mtx_MulVec4(c, fp.wrapping_add(96), fp, fp.wrapping_add(96));
        F::Vec4_Mul(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(64));
        F::Vec4_Mul(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(64));
        F::Vec4_ToInt(c, fp.wrapping_add(128), fp.wrapping_add(80));
        F::Vec4_ToInt(c, fp.wrapping_add(144), fp.wrapping_add(96));
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_sub((c.m.r32(fp.wrapping_add(128)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(128)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32); F::EftPrim_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(144)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32); F::EftPrim_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_sub((c.m.r32(fp.wrapping_add(144)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32); F::EftPrim_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(128)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(128)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32); F::EftPrim_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if ((p_front_11) != 0) {
            { let v_ = 16777215i32; let a_ = (fp.wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        l_p_29 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_29).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        l_x0_17 = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_sub((c.m.r32(fp.wrapping_add(128)) as i32));
        l_y0_18 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(128)).wrapping_add(4)) as i32));
        l_x1_19 = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(144)) as i32));
        l_y1_20 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32));
        l_x2_21 = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_sub((c.m.r32(fp.wrapping_add(144)) as i32));
        l_y2_22 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32));
        l_x3_23 = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(128)) as i32));
        l_y3_24 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(128)).wrapping_add(4)) as i32));
        l_zz_25 = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32);
        if (((((l_p_29) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_ctx_27 = (((p_layer_10) >= (2i32)) as i32);
        { let v_ = (((((((l_abe_26) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_27) as u64)).wrapping_shl((9i32) as u32)))) | (((28i32) as u64))); let a_ = (l_p_29).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870920i32) as u32); let a_ = l_p_29; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_29).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177288i32) as u32); let a_ = (l_p_29).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 16429131440647602177u64; let a_ = (l_p_29).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = (((18599481537995104i64).wrapping_add((((l_ctx_27).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_29).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_29).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = (l_p_29).wrapping_add(48); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = ((l_p_29).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = ((l_p_29).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8); let a_ = ((l_p_29).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_29).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = (l_p_29).wrapping_add(72); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = ((l_p_29).wrapping_add(72)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = ((l_p_29).wrapping_add(72)).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8); let a_ = ((l_p_29).wrapping_add(72)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_29).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = (l_p_29).wrapping_add(96); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = ((l_p_29).wrapping_add(96)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = ((l_p_29).wrapping_add(96)).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8); let a_ = ((l_p_29).wrapping_add(96)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_29).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = (l_p_29).wrapping_add(120); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = ((l_p_29).wrapping_add(120)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = ((l_p_29).wrapping_add(120)).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8); let a_ = ((l_p_29).wrapping_add(120)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_29).wrapping_add(120)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_5; let a_ = (l_p_29).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_6; let a_ = ((l_p_29).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_5; let a_ = (l_p_29).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_8; let a_ = ((l_p_29).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_7; let a_ = (l_p_29).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_6; let a_ = ((l_p_29).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_7; let a_ = (l_p_29).wrapping_add(128); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_8; let a_ = ((l_p_29).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((l_x0_17) as u64); let a_ = (l_p_29).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y0_18) as u64); let a_ = ((l_p_29).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_zz_25) as u64); let a_ = ((l_p_29).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_29).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((l_x1_19) as u64); let a_ = (l_p_29).wrapping_add(88); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y1_20) as u64); let a_ = ((l_p_29).wrapping_add(88)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_zz_25) as u64); let a_ = ((l_p_29).wrapping_add(88)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_29).wrapping_add(88)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((l_x2_21) as u64); let a_ = (l_p_29).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y2_22) as u64); let a_ = ((l_p_29).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_zz_25) as u64); let a_ = ((l_p_29).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_29).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((l_x3_23) as u64); let a_ = (l_p_29).wrapping_add(136); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y3_24) as u64); let a_ = ((l_p_29).wrapping_add(136)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_zz_25) as u64); let a_ = ((l_p_29).wrapping_add(136)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_29).wrapping_add(136)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = p_tex0_12; let a_ = (l_p_29).wrapping_add(40); c.m.w64(a_, v_); };
        l_l_28 = p_layer_10;
        if (((((l_l_28) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_28).wrapping_sub(r_); l_l_28 = n_; };
        }
        if (((((l_z_16) < (0i32)) as i32)) != 0) {
            l_e_30 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_28) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_16) >= (4096i32)) as i32)) != 0) {
                l_e_30 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_28) as u32)).wrapping_mul(8));
            } else {
                l_e_30 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_16) as u32)).wrapping_mul(16))).wrapping_add((((l_l_28) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_29; let a_ = (c.m.r32((l_e_30).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_29; let a_ = (l_e_30).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGfx_DrawSprite", &PROBE_N, &[], true); }
    r_
}
