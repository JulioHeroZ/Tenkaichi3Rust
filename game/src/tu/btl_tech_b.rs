#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlParam_GetUnk5C(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk5C", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(92); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk5C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk68(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk68", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(104); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk68", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk6C(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk6C", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(108); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk6C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetHitReactScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetHitReactScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(96); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetHitReactScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk70(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk70", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(112); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk70", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk72(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk72", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(114); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk72", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetChargeRateA(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetChargeRateA", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: F32 = F32(0);
    let r_ = 'f: {
    l_v_2 = { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(116); F32(c.m.r32(a_)) };
    if ((F::BtlMember_HasAbility(c, p_chr_1, 74i32)) != 0) {
        { let r_ = F32(0x3f999999); let n_ = rt::fmul(l_v_2, r_); l_v_2 = n_; };
    }
    break 'f l_v_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetChargeRateA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetChargeRateB(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetChargeRateB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(120); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetChargeRateB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetDamageTakenScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetDamageTakenScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(124); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetDamageTakenScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetGauge99CTime(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetGauge99CTime", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: F32 = F32(0);
    let r_ = 'f: {
    l_v_2 = { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(100); F32(c.m.r32(a_)) };
    if ((F::BtlMember_HasAbility(c, p_chr_1, 52i32)) != 0) {
        { let r_ = F32(0x3f999999); let n_ = rt::fdiv(l_v_2, r_); l_v_2 = n_; };
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 27i32)) != 0) {
        { let r_ = l_v_2; let n_ = rt::fadd(l_v_2, r_); l_v_2 = n_; };
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 53i32)) != 0) {
            { let r_ = F32(0x3f4ccccc); let n_ = rt::fdiv(l_v_2, r_); l_v_2 = n_; };
        }
    }
    break 'f l_v_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetGauge99CTime", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkC7Scale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkC7Scale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: F32 = F32(0);
    let r_ = 'f: {
    l_v_2 = rt::fmul(rt::i2f((({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(199); c.m.r8(a_) }) as i32)), F32(0x3c23d70a));
    if ((F::BtlMember_HasAbility(c, p_chr_1, 72i32)) != 0) {
        { let r_ = F32(0x3f000000); let n_ = rt::fmul(l_v_2, r_); l_v_2 = n_; };
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 73i32)) != 0) {
        { let r_ = F32(0x3fc00000); let n_ = rt::fmul(l_v_2, r_); l_v_2 = n_; };
    }
    break 'f l_v_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkC7Scale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkC8(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkC8", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(200); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkC8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk8F(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk8F", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(143)).wrapping_add(((p_n_2) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk8F", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetDashSound(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetDashSound", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (16i32))) != 0) {
        break 'f 29i32;
    }
    break 'f 28i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetDashSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetChargeStartSound(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetChargeStartSound", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((F::BtlParam_GetUnk0(c, p_chr_1)) & (256i32))) != 0) {
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 62i32);
    }
    break 'f 21i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetChargeStartSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetChargeLoopSound(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetChargeLoopSound", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((F::BtlParam_GetUnk0(c, p_chr_1)) & (256i32))) != 0) {
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 63i32);
    }
    break 'f 22i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetChargeLoopSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetMaxPowerSound(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetMaxPowerSound", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((F::BtlParam_GetUnk0(c, p_chr_1)) & (256i32))) != 0) {
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 64i32);
    }
    break 'f 23i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetMaxPowerSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_CanFly(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_CanFly", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f (((F::BtlMember_HasAbility(c, p_chr_1, 54i32)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_CanFly", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_ScaleDamageByAbility(c: &mut Ctx, mut p_chr_1: u32, mut p_damage_2: i32, mut p_kind_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_ScaleDamageByAbility", &PROBE_N, &[(p_chr_1) as u32, (p_damage_2) as u32, (p_kind_3) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlMember_HasAbility(c, p_chr_1, 10i32)) != 0) {
        { let r_ = rt::div_i32(p_damage_2, 2i32); let n_ = (p_damage_2).wrapping_add(r_); p_damage_2 = n_; };
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 11i32)) != 0) {
        { let r_ = 2i32; let n_ = rt::div_i32(p_damage_2, r_); p_damage_2 = n_; };
    }
    break 'f p_damage_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_ScaleDamageByAbility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_ScaleSpeedByAbility(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: i32, mut p_speed_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_ScaleSpeedByAbility", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32, (p_speed_3).0], false); }
    let r_ = 'f: {
    's1: {
        let sv4: i32 = p_kind_2;
        match sv4 {
            0i32 | 4i32 | 8i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 14i32)) != 0) {
                    { let r_ = F32(0x3fcccccc); let n_ = rt::fmul(p_speed_3, r_); p_speed_3 = n_; };
                } else {
                    if ((F::BtlMember_HasAbility(c, p_chr_1, 13i32)) != 0) {
                        { let r_ = F32(0x3fb33333); let n_ = rt::fmul(p_speed_3, r_); p_speed_3 = n_; };
                    } else {
                        if ((F::BtlMember_HasAbility(c, p_chr_1, 12i32)) != 0) {
                            { let r_ = F32(0x3f999999); let n_ = rt::fmul(p_speed_3, r_); p_speed_3 = n_; };
                        }
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f p_speed_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_ScaleSpeedByAbility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_ScaleKiByAbility(c: &mut Ctx, mut p_chr_1: u32, mut p_ki_2: i32, mut p_kind_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_ScaleKiByAbility", &PROBE_N, &[(p_chr_1) as u32, (p_ki_2) as u32, (p_kind_3) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlMember_HasAbility(c, p_chr_1, 42i32)) != 0) {
        { let r_ = 2i32; let n_ = (p_ki_2).wrapping_mul(r_); p_ki_2 = n_; };
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 41i32)) != 0) {
        { let r_ = 2i32; let n_ = rt::div_i32(p_ki_2, r_); p_ki_2 = n_; };
    }
    break 'f p_ki_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_ScaleKiByAbility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_MapReact(c: &mut Ctx, mut p_chr_1: u32, mut p_react_2: i32, mut p_kind_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_MapReact", &PROBE_N, &[(p_chr_1) as u32, (p_react_2) as u32, (p_kind_3) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlMember_HasAbility(c, p_chr_1, 75i32)) != 0) {
        's1: {
            let sv4: i32 = F::BtlKiBlast_GetTypeOf(c, p_chr_1, ((p_kind_3) as u32));
            match sv4 {
                0i32 | 1i32 => {
                    's2: {
                        let sv5: i32 = p_kind_3;
                        match sv5 {
                            1i32 => {
                                p_react_2 = 23i32;
                                break 's2;
                            }
                            2i32 => {
                                p_react_2 = 24i32;
                                break 's2;
                            }
                            3i32 => {
                                p_react_2 = 25i32;
                                break 's2;
                            }
                            4i32 => {
                                break 's2;
                            }
                            5i32 => {
                                p_react_2 = 23i32;
                                break 's2;
                            }
                            6i32 => {
                                p_react_2 = 24i32;
                                break 's2;
                            }
                            7i32 => {
                                p_react_2 = 25i32;
                                break 's2;
                            }
                            8i32 => {
                                break 's2;
                            }
                            9i32 => {
                                p_react_2 = 23i32;
                                break 's2;
                            }
                            10i32 => {
                                p_react_2 = 24i32;
                                break 's2;
                            }
                            11i32 => {
                                p_react_2 = 25i32;
                                break 's2;
                            }
                            _ => {
                            }
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    break 'f p_react_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_MapReact", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_MapReactS800(c: &mut Ctx, mut p_chr_1: u32, mut p_react_2: i32, mut p_kind_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_MapReactS800", &PROBE_N, &[(p_chr_1) as u32, (p_react_2) as u32, (p_kind_3) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlMember_HasAbility(c, p_chr_1, 75i32)) != 0) {
        's1: {
            let sv4: i32 = F::BtlKiBlast_GetTypeOf(c, p_chr_1, ((p_kind_3) as u32));
            match sv4 {
                0i32 | 1i32 => {
                    's2: {
                        let sv5: i32 = p_kind_3;
                        match sv5 {
                            1i32 => {
                                p_react_2 = 26i32;
                                break 's2;
                            }
                            2i32 => {
                                p_react_2 = 27i32;
                                break 's2;
                            }
                            3i32 => {
                                p_react_2 = 28i32;
                                break 's2;
                            }
                            4i32 => {
                                break 's2;
                            }
                            5i32 => {
                                p_react_2 = 26i32;
                                break 's2;
                            }
                            6i32 => {
                                p_react_2 = 27i32;
                                break 's2;
                            }
                            7i32 => {
                                p_react_2 = 28i32;
                                break 's2;
                            }
                            8i32 => {
                                break 's2;
                            }
                            9i32 => {
                                p_react_2 = 26i32;
                                break 's2;
                            }
                            10i32 => {
                                p_react_2 = 27i32;
                                break 's2;
                            }
                            11i32 => {
                                p_react_2 = 28i32;
                                break 's2;
                            }
                            _ => {
                            }
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    break 'f p_react_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_MapReactS800", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetCurrentKind(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetCurrentKind", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_lo_2: F32 = F32(0);
    let mut l_hi_3: F32 = F32(0);
    let r_ = 'f: {
    l_lo_2 = F32(0x3e999999);
    l_hi_3 = F32(0x3f666666);
    if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (149i32)) as i32)) != 0) {
        break 'f 12i32;
    }
    's1: {
        let sv4: i32 = F::BtlAnim_GetId(c, p_chr_1);
        match sv4 {
            113i32 | 114i32 => {
                break 'f 0i32;
            }
            123i32 | 124i32 => {
                break 'f 1i32;
            }
            125i32 => {
                if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(3564))), l_lo_2) as i32)) != 0) {
                    break 'f 1i32;
                }
                if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(3564))), l_hi_3) as i32)) != 0) {
                    break 'f 2i32;
                }
                break 'f 3i32;
            }
            119i32 | 120i32 | 121i32 => {
                break 'f 4i32;
            }
            132i32 | 133i32 | 135i32 | 136i32 | 138i32 | 139i32 => {
                break 'f 5i32;
            }
            134i32 | 137i32 | 140i32 => {
                if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(3564))), l_lo_2) as i32)) != 0) {
                    break 'f 5i32;
                }
                if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(3564))), l_hi_3) as i32)) != 0) {
                    break 'f 6i32;
                }
                break 'f 7i32;
            }
            122i32 => {
                break 'f 8i32;
            }
            141i32 | 142i32 => {
                break 'f 9i32;
            }
            143i32 => {
                if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(3564))), l_lo_2) as i32)) != 0) {
                    break 'f 9i32;
                }
                if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(3564))), l_hi_3) as i32)) != 0) {
                    break 'f 10i32;
                }
                break 'f 11i32;
            }
            _ => {
            }
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetCurrentKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetCurrent(c: &mut Ctx, mut p_chr_1: u32, mut p_outKind_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetCurrent", &PROBE_N, &[(p_chr_1) as u32, (p_outKind_2) as u32], false); }
    let mut l_kind_3: i32 = 0;
    let r_ = 'f: {
    l_kind_3 = F::BtlKiBlast_GetCurrentKind(c, p_chr_1);
    if (((((l_kind_3) < (0i32)) as i32)) != 0) {
        l_kind_3 = 0i32;
    }
    if (((((p_outKind_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_kind_3; let a_ = p_outKind_2; c.m.w32(a_, (v_) as u32); };
    }
    break 'f ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2340); c.m.r32(a_) }).wrapping_add((((l_kind_3) as u32)).wrapping_mul(52));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetCurrent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetOfHit(c: &mut Ctx, mut p_hit_1: u32, mut p_outChr_2: u32, mut p_outKind_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetOfHit", &PROBE_N, &[(p_hit_1) as u32, (p_outChr_2) as u32, (p_outKind_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_kind_5: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_hit_1).wrapping_add(104))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_chr_4 = { let a0_ = (({ let a_ = (c.m.r32((p_hit_1).wrapping_add(104))).wrapping_add(16); (c.m.r16(a_) as i16) }) as i32); F::BtlChar_Get(c, a0_) };
    l_kind_5 = (({ let a_ = (c.m.r32((p_hit_1).wrapping_add(104))).wrapping_add(24); (c.m.r16(a_) as i16) }) as i32);
    if (((((p_outKind_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_kind_5; let a_ = p_outKind_3; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_outChr_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_chr_4; let a_ = p_outChr_2; c.m.w32(a_, v_); };
    }
    break 'f ({ let a_ = (F::BtlChar_GetObj(c, l_chr_4)).wrapping_add(2340); c.m.r32(a_) }).wrapping_add((((l_kind_5) as u32)).wrapping_mul(52));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_Get(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_Get", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_kind_2) >= (((13i32) as u32))) as i32)) != 0) {
        p_kind_2 = ((0i32) as u32);
    }
    break 'f ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2340); c.m.r32(a_) }).wrapping_add((p_kind_2).wrapping_mul(52));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetFlags(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_flags_2: i32 = 0;
    let r_ = 'f: {
    l_flags_2 = (({ let a_ = F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32)); c.m.r16(a_) }) as i32);
    's1: {
        let sv3: i32 = F::BtlChar_TestMemberUnk70(c, p_chr_1);
        match sv3 {
            0i32 => {
                break 's1;
            }
            _ => {
                { let r_ = (!(3i32)); let n_ = ((l_flags_2) & (r_)); l_flags_2 = n_; };
            }
        }
    }
    break 'f l_flags_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetFlagsOfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFlagsOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let mut l_flags_2: i32 = 0;
    let r_ = 'f: {
    l_flags_2 = (({ let a_ = F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32)); c.m.r16(a_) }) as i32);
    's1: {
        let sv3: i32 = { let a0_ = { let a0_ = (({ let a_ = (c.m.r32((p_hit_1).wrapping_add(104))).wrapping_add(16); (c.m.r16(a_) as i16) }) as i32); F::BtlChar_Get(c, a0_) }; F::BtlChar_TestMemberUnk70(c, a0_) };
        match sv3 {
            0i32 => {
                break 's1;
            }
            _ => {
                { let r_ = (!(3i32)); let n_ = ((l_flags_2) & (r_)); l_flags_2 = n_; };
            }
        }
    }
    break 'f l_flags_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFlagsOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetFlagsOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFlagsOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_flags_3: i32 = 0;
    let r_ = 'f: {
    l_flags_3 = (({ let a_ = F::BtlKiBlast_Get(c, p_chr_1, p_kind_2); c.m.r16(a_) }) as i32);
    's1: {
        let sv4: i32 = F::BtlChar_TestMemberUnk70(c, p_chr_1);
        match sv4 {
            0i32 => {
                break 's1;
            }
            _ => {
                { let r_ = (!(3i32)); let n_ = ((l_flags_3) & (r_)); l_flags_3 = n_; };
            }
        }
    }
    break 'f l_flags_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFlagsOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetDamage(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetDamage", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rec_2: u32 = 0;
    let mut l_damage_3: i32 = 0;
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
        l_rec_2 = F::BtlKiBlast_GetCurrent(c, p_chr_1, fp);
        l_damage_3 = (c.m.r32((l_rec_2).wrapping_add(4)) as i32);
        l_hits_4 = (((c.m.r8((l_rec_2).wrapping_add(48)) as i8)) as i32);
        l_damage_3 = { let a1_ = rt::f2i(rt::fmul(rt::i2f(l_damage_3), F::BtlStat_GetKiBlastDamageScale(c, p_chr_1))); let a2_ = (c.m.r32(fp) as i32); F::BtlKiBlast_ScaleDamageByAbility(c, p_chr_1, a1_, a2_) };
        if (((((l_hits_4) >= (2i32)) as i32)) != 0) {
            { let r_ = l_hits_4; let n_ = rt::div_i32(l_damage_3, r_); l_damage_3 = n_; };
        }
        break 'f l_damage_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetDamageOfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetDamageOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rec_2: u32 = 0;
    let mut l_damage_3: i32 = 0;
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
        l_rec_2 = F::BtlKiBlast_GetOfHit(c, p_hit_1, fp, fp.wrapping_add(4));
        l_damage_3 = (c.m.r32((l_rec_2).wrapping_add(4)) as i32);
        l_hits_4 = (((c.m.r8((l_rec_2).wrapping_add(48)) as i8)) as i32);
        l_damage_3 = { let a0_ = c.m.r32(fp); let a1_ = rt::f2i(rt::fmul(rt::i2f(l_damage_3), { let a0_ = c.m.r32(fp); F::BtlStat_GetKiBlastDamageScale(c, a0_) })); let a2_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::BtlKiBlast_ScaleDamageByAbility(c, a0_, a1_, a2_) };
        if (((((l_hits_4) >= (2i32)) as i32)) != 0) {
            { let r_ = l_hits_4; let n_ = rt::div_i32(l_damage_3, r_); l_damage_3 = n_; };
        }
        break 'f l_damage_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetDamageOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetDamageOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetDamageOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let mut l_damage_4: i32 = 0;
    let mut l_hits_5: i32 = 0;
    let r_ = 'f: {
    l_rec_3 = F::BtlKiBlast_Get(c, p_chr_1, p_kind_2);
    l_damage_4 = (c.m.r32((l_rec_3).wrapping_add(4)) as i32);
    l_hits_5 = (((c.m.r8((l_rec_3).wrapping_add(48)) as i8)) as i32);
    l_damage_4 = { let a1_ = rt::f2i(rt::fmul(rt::i2f(l_damage_4), F::BtlStat_GetKiBlastDamageScale(c, p_chr_1))); F::BtlKiBlast_ScaleDamageByAbility(c, p_chr_1, a1_, ((p_kind_2) as i32)) };
    if (((((l_hits_5) >= (2i32)) as i32)) != 0) {
        { let r_ = l_hits_5; let n_ = rt::div_i32(l_damage_4, r_); l_damage_4 = n_; };
    }
    break 'f l_damage_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetDamageOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetGuardDamage(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardDamage", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rec_2: u32 = 0;
    let mut l_damage_3: i32 = 0;
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
        l_rec_2 = F::BtlKiBlast_GetCurrent(c, p_chr_1, fp);
        l_damage_3 = (c.m.r32((l_rec_2).wrapping_add(8)) as i32);
        l_hits_4 = (((c.m.r8((l_rec_2).wrapping_add(48)) as i8)) as i32);
        l_damage_3 = { let a1_ = rt::f2i(rt::fmul(rt::i2f(l_damage_3), F::BtlStat_GetKiBlastDamageScale(c, p_chr_1))); let a2_ = (c.m.r32(fp) as i32); F::BtlKiBlast_ScaleDamageByAbility(c, p_chr_1, a1_, a2_) };
        if (((((l_hits_4) > (0i32)) as i32)) != 0) {
            { let r_ = l_hits_4; let n_ = rt::div_i32(l_damage_3, r_); l_damage_3 = n_; };
        }
        break 'f l_damage_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetGuardDamageOfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardDamageOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rec_2: u32 = 0;
    let mut l_damage_3: i32 = 0;
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
        l_rec_2 = F::BtlKiBlast_GetOfHit(c, p_hit_1, fp, fp.wrapping_add(4));
        l_damage_3 = (c.m.r32((l_rec_2).wrapping_add(8)) as i32);
        l_hits_4 = (((c.m.r8((l_rec_2).wrapping_add(48)) as i8)) as i32);
        l_damage_3 = { let a0_ = c.m.r32(fp); let a1_ = rt::f2i(rt::fmul(rt::i2f(l_damage_3), { let a0_ = c.m.r32(fp); F::BtlStat_GetKiBlastDamageScale(c, a0_) })); let a2_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::BtlKiBlast_ScaleDamageByAbility(c, a0_, a1_, a2_) };
        if (((((l_hits_4) > (0i32)) as i32)) != 0) {
            { let r_ = l_hits_4; let n_ = rt::div_i32(l_damage_3, r_); l_damage_3 = n_; };
        }
        break 'f l_damage_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardDamageOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetGuardDamageOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardDamageOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let mut l_damage_4: i32 = 0;
    let mut l_hits_5: i32 = 0;
    let r_ = 'f: {
    l_rec_3 = F::BtlKiBlast_Get(c, p_chr_1, p_kind_2);
    l_damage_4 = (c.m.r32((l_rec_3).wrapping_add(8)) as i32);
    l_hits_5 = (((c.m.r8((l_rec_3).wrapping_add(48)) as i8)) as i32);
    l_damage_4 = { let a1_ = rt::f2i(rt::fmul(rt::i2f(l_damage_4), F::BtlStat_GetKiBlastDamageScale(c, p_chr_1))); F::BtlKiBlast_ScaleDamageByAbility(c, p_chr_1, a1_, ((p_kind_2) as i32)) };
    if (((((l_hits_5) > (0i32)) as i32)) != 0) {
        { let r_ = l_hits_5; let n_ = rt::div_i32(l_damage_4, r_); l_damage_4 = n_; };
    }
    break 'f l_damage_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardDamageOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetKiCost(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetKiCost", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        break 'f { let a1_ = { let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, fp)).wrapping_add(12); (c.m.r32(a_) as i32) }; let a2_ = (c.m.r32(fp) as i32); F::BtlKiBlast_ScaleKiByAbility(c, p_chr_1, a1_, a2_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetKiCost", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetKiCostOfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetKiCostOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        break 'f { let a0_ = c.m.r32(fp); let a1_ = { let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, fp, fp.wrapping_add(4))).wrapping_add(12); (c.m.r32(a_) as i32) }; let a2_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::BtlKiBlast_ScaleKiByAbility(c, a0_, a1_, a2_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetKiCostOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetKiCostOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetKiCostOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = { let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlKiBlast_ScaleKiByAbility(c, p_chr_1, a1_, ((p_kind_2) as i32)) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetKiCostOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetSpeed(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetSpeed", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        break 'f { let a1_ = (c.m.r32(fp) as i32); let a2_ = rt::fmul(rt::fmul({ let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, fp)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425)); F::BtlKiBlast_ScaleSpeedByAbility(c, p_chr_1, a1_, a2_) };
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetSpeedOfHit(c: &mut Ctx, mut p_hit_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetSpeedOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        break 'f { let a0_ = c.m.r32(fp); let a1_ = (c.m.r32(fp.wrapping_add(4)) as i32); let a2_ = rt::fmul(rt::fmul({ let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, fp, fp.wrapping_add(4))).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425)); F::BtlKiBlast_ScaleSpeedByAbility(c, a0_, a1_, a2_) };
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetSpeedOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetSpeedOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetSpeedOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a2_ = rt::fmul(rt::fmul({ let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425)); F::BtlKiBlast_ScaleSpeedByAbility(c, p_chr_1, ((p_kind_2) as i32), a2_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetSpeedOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetTurnRate(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTurnRate", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul(rt::fdiv({ let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32))).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x41f00000)), F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTurnRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetTurnRateOfHit(c: &mut Ctx, mut p_hit_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTurnRateOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul(rt::fdiv({ let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32))).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x41f00000)), F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTurnRateOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetTurnRateOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTurnRateOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul(rt::fdiv({ let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x41f00000)), F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTurnRateOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetFrames(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFrames", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fmul({ let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32))).wrapping_add(24); F32(c.m.r32(a_)) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetFramesOfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFramesOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fmul({ let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32))).wrapping_add(24); F32(c.m.r32(a_)) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFramesOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetFramesOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFramesOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fmul({ let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(24); F32(c.m.r32(a_)) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetFramesOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk31(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk31", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32))).wrapping_add(49); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk31", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk31OfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk31OfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32))).wrapping_add(49); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk31OfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk31Of(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk31Of", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(49); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk31Of", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReact(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReact", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        break 'f { let a1_ = (({ let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, fp)).wrapping_add(36); (c.m.r8(a_) as i8) }) as i32); let a2_ = (c.m.r32(fp) as i32); F::BtlKiBlast_MapReact(c, p_chr_1, a1_, a2_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReact", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactOfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        break 'f { let a0_ = c.m.r32(fp); let a1_ = (({ let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, fp, fp.wrapping_add(4))).wrapping_add(36); (c.m.r8(a_) as i8) }) as i32); let a2_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::BtlKiBlast_MapReact(c, a0_, a1_, a2_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = (({ let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(36); (c.m.r8(a_) as i8) }) as i32); F::BtlKiBlast_MapReact(c, p_chr_1, a1_, ((p_kind_2) as i32)) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS800(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS800", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
        l_rec_2 = F::BtlKiBlast_GetCurrent(c, p_chr_1, fp);
        break 'f { let a1_ = (if ((((((((c.m.r8((l_rec_2).wrapping_add(37)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(37)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(36)) as i8)) as i32) }); let a2_ = (c.m.r32(fp) as i32); F::BtlKiBlast_MapReactS800(c, p_chr_1, a1_, a2_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS800", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS800OfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS800OfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rec_2: u32 = 0;
    let mut l_react_3: i32 = 0;
    let r_ = 'f: {
        l_rec_2 = F::BtlKiBlast_GetOfHit(c, p_hit_1, fp, fp.wrapping_add(4));
        l_react_3 = (if ((((((((c.m.r8((l_rec_2).wrapping_add(37)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(37)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(36)) as i8)) as i32) });
        break 'f { let a0_ = c.m.r32(fp); let a2_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::BtlKiBlast_MapReactS800(c, a0_, l_react_3, a2_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS800OfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS800Of(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS800Of", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let r_ = 'f: {
    l_rec_3 = F::BtlKiBlast_Get(c, p_chr_1, p_kind_2);
    break 'f { let a1_ = (if ((((((((c.m.r8((l_rec_3).wrapping_add(37)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_3).wrapping_add(37)) as i8)) as i32) } else { (((c.m.r8((l_rec_3).wrapping_add(36)) as i8)) as i32) }); F::BtlKiBlast_MapReactS800(c, p_chr_1, a1_, ((p_kind_2) as i32)) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS800Of", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS4000(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS4000", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(38)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(38)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(36)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS4000", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS4000OfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS4000OfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(38)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(38)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(36)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS4000OfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS4000Of(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS4000Of", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let r_ = 'f: {
    l_rec_3 = F::BtlKiBlast_Get(c, p_chr_1, p_kind_2);
    break 'f (if ((((((((c.m.r8((l_rec_3).wrapping_add(38)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_3).wrapping_add(38)) as i8)) as i32) } else { (((c.m.r8((l_rec_3).wrapping_add(36)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS4000Of", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS200(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS200", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(39)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(39)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(36)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS200", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS200OfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS200OfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(39)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(39)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(36)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS200OfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetReactS200Of(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS200Of", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let r_ = 'f: {
    l_rec_3 = F::BtlKiBlast_Get(c, p_chr_1, p_kind_2);
    break 'f (if ((((((((c.m.r8((l_rec_3).wrapping_add(39)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_3).wrapping_add(39)) as i8)) as i32) } else { (((c.m.r8((l_rec_3).wrapping_add(36)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetReactS200Of", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetType(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetType", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32))).wrapping_add(2); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetTypeOfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTypeOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32))).wrapping_add(2); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTypeOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetTypeOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTypeOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(2); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetTypeOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk3(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk3", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32))).wrapping_add(3); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk3OfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk3OfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32))).wrapping_add(3); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk3OfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk3Of(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk3Of", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(3); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk3Of", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetHits(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetHits", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32))).wrapping_add(48); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetHits", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetHitsOfHit(c: &mut Ctx, mut p_hit_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetHitsOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32))).wrapping_add(48); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetHitsOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetHitsOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetHitsOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(48); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetHitsOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk28(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk28", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32))).wrapping_add(40); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk28", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk28OfHit(c: &mut Ctx, mut p_hit_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk28OfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32))).wrapping_add(40); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk28OfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk28Of(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk28Of", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(40); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk28Of", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk2C(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk2C", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32))).wrapping_add(44); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk2C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk2COfHit(c: &mut Ctx, mut p_hit_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk2COfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32))).wrapping_add(44); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk2COfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetUnk2COf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk2COf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlKiBlast_Get(c, p_chr_1, p_kind_2)).wrapping_add(44); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetUnk2COf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetPush(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetPush", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let mut l_push_3: F32 = F32(0);
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32));
    l_push_3 = rt::fmul(rt::fmul(F32(c.m.r32((l_rec_2).wrapping_add(28))), F32(0x41200000)), F32(0x3c17b425));
    l_hits_4 = (((c.m.r8((l_rec_2).wrapping_add(48)) as i8)) as i32);
    if (((((l_hits_4) > (0i32)) as i32)) != 0) {
        { let r_ = rt::i2f(l_hits_4); let n_ = rt::fdiv(l_push_3, r_); l_push_3 = n_; };
    }
    break 'f l_push_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetPushOfHit(c: &mut Ctx, mut p_hit_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetPushOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let mut l_push_3: F32 = F32(0);
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32));
    l_push_3 = rt::fmul(rt::fmul(F32(c.m.r32((l_rec_2).wrapping_add(28))), F32(0x41200000)), F32(0x3c17b425));
    l_hits_4 = (((c.m.r8((l_rec_2).wrapping_add(48)) as i8)) as i32);
    if (((((l_hits_4) > (0i32)) as i32)) != 0) {
        { let r_ = rt::i2f(l_hits_4); let n_ = rt::fdiv(l_push_3, r_); l_push_3 = n_; };
    }
    break 'f l_push_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetPushOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetPushOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetPushOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let mut l_push_4: F32 = F32(0);
    let mut l_hits_5: i32 = 0;
    let r_ = 'f: {
    l_rec_3 = F::BtlKiBlast_Get(c, p_chr_1, p_kind_2);
    l_push_4 = rt::fmul(rt::fmul(F32(c.m.r32((l_rec_3).wrapping_add(28))), F32(0x41200000)), F32(0x3c17b425));
    l_hits_5 = (((c.m.r8((l_rec_3).wrapping_add(48)) as i8)) as i32);
    if (((((l_hits_5) > (0i32)) as i32)) != 0) {
        { let r_ = rt::i2f(l_hits_5); let n_ = rt::fdiv(l_push_4, r_); l_push_4 = n_; };
    }
    break 'f l_push_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetPushOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetGuardPush(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardPush", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let mut l_push_3: F32 = F32(0);
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlKiBlast_GetCurrent(c, p_chr_1, ((0i32) as u32));
    l_push_3 = rt::fmul(rt::fmul(F32(c.m.r32((l_rec_2).wrapping_add(32))), F32(0x41200000)), F32(0x3c17b425));
    l_hits_4 = (((c.m.r8((l_rec_2).wrapping_add(48)) as i8)) as i32);
    if (((((l_hits_4) > (0i32)) as i32)) != 0) {
        { let r_ = rt::i2f(l_hits_4); let n_ = rt::fdiv(l_push_3, r_); l_push_3 = n_; };
    }
    break 'f l_push_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetGuardPushOfHit(c: &mut Ctx, mut p_hit_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardPushOfHit", &PROBE_N, &[(p_hit_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let mut l_push_3: F32 = F32(0);
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlKiBlast_GetOfHit(c, p_hit_1, ((0i32) as u32), ((0i32) as u32));
    l_push_3 = rt::fmul(rt::fmul(F32(c.m.r32((l_rec_2).wrapping_add(32))), F32(0x41200000)), F32(0x3c17b425));
    l_hits_4 = (((c.m.r8((l_rec_2).wrapping_add(48)) as i8)) as i32);
    if (((((l_hits_4) > (0i32)) as i32)) != 0) {
        { let r_ = rt::i2f(l_hits_4); let n_ = rt::fdiv(l_push_3, r_); l_push_3 = n_; };
    }
    break 'f l_push_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardPushOfHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlKiBlast_GetGuardPushOf(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardPushOf", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let mut l_push_4: F32 = F32(0);
    let mut l_hits_5: i32 = 0;
    let r_ = 'f: {
    l_rec_3 = F::BtlKiBlast_Get(c, p_chr_1, p_kind_2);
    l_push_4 = rt::fmul(rt::fmul(F32(c.m.r32((l_rec_3).wrapping_add(32))), F32(0x41200000)), F32(0x3c17b425));
    l_hits_5 = (((c.m.r8((l_rec_3).wrapping_add(48)) as i8)) as i32);
    if (((((l_hits_5) > (0i32)) as i32)) != 0) {
        { let r_ = rt::i2f(l_hits_5); let n_ = rt::fdiv(l_push_4, r_); l_push_4 = n_; };
    }
    break 'f l_push_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlKiBlast_GetGuardPushOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMoveParam_GetSpeed(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetSpeed", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_v_3: F32 = F32(0);
    let r_ = 'f: {
    l_v_3 = rt::fmul(rt::fmul({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2344); c.m.r32(a_) }).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425));
    's1: {
        let sv4: i32 = p_n_2;
        match sv4 {
            15i32 | 19i32 => {
                break 's1;
            }
            4i32 | 5i32 => {
                { let r_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(172); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_v_3, r_); l_v_3 = n_; };
                { let r_ = F::BtlStat_GetSpeedScale(c, p_chr_1); let n_ = rt::fmul(l_v_3, r_); l_v_3 = n_; };
                break 's1;
            }
            _ => {
                { let r_ = F::BtlStat_GetSpeedScale(c, p_chr_1); let n_ = rt::fmul(l_v_3, r_); l_v_3 = n_; };
                break 's1;
            }
        }
    }
    if (((rt::flt(F32(0x421425ec), l_v_3) as i32)) != 0) {
        l_v_3 = F32(0x421425ec);
    }
    break 'f l_v_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMoveParam_GetAccel(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetAccel", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fmul(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2344); c.m.r32(a_) }).wrapping_add(80)).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3e8e38e3)), F32(0x3a91a2b3));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetAccel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMoveParam_GetAngle58(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetAngle58", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2344); c.m.r32(a_) }).wrapping_add(88); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetAngle58", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMoveParam_GetHeight(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetHeight", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2344); c.m.r32(a_) }).wrapping_add(92)).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41200000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetHeight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMoveParam_GetKiCost(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetKiCost", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_v_3: i32 = 0;
    let r_ = 'f: {
    l_v_3 = rt::div_i32({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2344); c.m.r32(a_) }).wrapping_add(100)).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 30i32);
    's1: {
        let sv4: i32 = p_n_2;
        match sv4 {
            0i32 | 1i32 | 2i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 38i32)) != 0) {
                    l_v_3 = 0i32;
                } else {
                    if ((F::BtlMember_HasAbility(c, p_chr_1, 37i32)) != 0) {
                        { let r_ = 2i32; let n_ = rt::div_i32(l_v_3, r_); l_v_3 = n_; };
                    }
                }
                break 's1;
            }
            3i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 39i32)) != 0) {
                    { let r_ = 2i32; let n_ = rt::div_i32(l_v_3, r_); l_v_3 = n_; };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_v_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetKiCost", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMoveParam_GetTurnRate(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetTurnRate", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_v_3: F32 = F32(0);
    let r_ = 'f: {
    l_v_3 = rt::fmul(rt::fdiv(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2344); c.m.r32(a_) }).wrapping_add(116)).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)), F32(0x3d088888));
    if (((((p_n_2) == (0i32)) as i32)) != 0) {
        { let r_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(172); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_v_3, r_); l_v_3 = n_; };
    }
    break 'f l_v_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetTurnRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMoveParam_GetTurnAccel(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetTurnAccel", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fdiv(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2344); c.m.r32(a_) }).wrapping_add(128)).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)), F32(0x3d088888));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetTurnAccel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMoveParam_GetTurnMax(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetTurnMax", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2344); c.m.r32(a_) }).wrapping_add(144)).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMoveParam_GetTurnMax", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetHitCount(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitCount", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = F::BtlSuper_GetShots(c, p_chr_1, p_slot_2);
    { let r_ = F::BtlSuper_GetHitsB(c, p_chr_1, p_slot_2); let n_ = (l_n_3).wrapping_mul(r_); l_n_3 = n_; };
    { let r_ = F::BtlSuper_GetHitsC(c, p_chr_1, p_slot_2); let n_ = (l_n_3).wrapping_mul(r_); l_n_3 = n_; };
    if (((((l_n_3) > (0i32)) as i32)) != 0) {
        break 'f l_n_3;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetFlagsA(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetFlagsA", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) };
    { let r_ = 2i32; let n_ = (p_slot_2).wrapping_sub(r_); p_slot_2 = n_; };
    break 'f c.m.r32((l_tbl_3).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetFlagsA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetFlags(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetFlags", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) };
    { let r_ = 2i32; let n_ = (p_slot_2).wrapping_sub(r_); p_slot_2 = n_; };
    break 'f c.m.r32(((l_tbl_3).wrapping_add(12)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetId(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetId", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(24)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetUnk1E(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk1E", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(30)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk1E", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetShotTime(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShotTime", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(36)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShotTime", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetShotSpeed(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShotSpeed", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(84)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShotSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetShotTurnRate(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShotTurnRate", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fdiv(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(96)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)), F32(0x3d088888));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShotTurnRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetShotUnk6C(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShotUnk6C", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(108)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShotUnk6C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetUnk90(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk90", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(144)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk90", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetShots(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShots", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(147)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_n_3) < (1i32)) as i32)) != 0) {
        l_n_3 = 1i32;
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetShots", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetHitsB(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitsB", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(150)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_n_3) < (1i32)) as i32)) != 0) {
        l_n_3 = 1i32;
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitsB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetHitsC(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitsC", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(156)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32)).wrapping_add(1i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitsC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetHitDirKind(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitDirKind", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(315)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitDirKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetType(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetType", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(318)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetReact(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetReact", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(321)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetReact", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetReactAlt(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetReactAlt", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) };
    break 'f (if ((((((((c.m.r8(((l_tbl_3).wrapping_add(324)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32))) as i8)) as i32)) > (0i32)) as i32)) != 0) { (((c.m.r8(((l_tbl_3).wrapping_add(324)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32))) as i8)) as i32) } else { (((c.m.r8(((l_tbl_3).wrapping_add(321)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32))) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetReactAlt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetReactS800(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetReactS800", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(327)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetReactS800", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetReactAltS800(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetReactAltS800", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) };
    break 'f (if ((((((((c.m.r8(((l_tbl_3).wrapping_add(330)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32))) as i8)) as i32)) > (0i32)) as i32)) != 0) { (((c.m.r8(((l_tbl_3).wrapping_add(330)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32))) as i8)) as i32) } else { (((c.m.r8(((l_tbl_3).wrapping_add(327)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32))) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetReactAltS800", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetUnk14D(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk14D", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(333)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk14D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetGuardKind(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetGuardKind", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(336)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetGuardKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetHitSound(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32, mut p_alt_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitSound", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_alt_3) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ((({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(339)).wrapping_add((((p_alt_3) as u32)).wrapping_mul(3))).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetHitSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetStepCount(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetStepCount", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(345)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_n_3) < (1i32)) as i32)) != 0) {
        l_n_3 = 1i32;
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetStepCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetPush(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetPush", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(364)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetGuardPush(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetGuardPush", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(376)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetGuardPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetBaseDamage(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32, mut p_total_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetBaseDamage", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_total_3) as u32], false); }
    let mut l_damage_4: i32 = 0;
    let r_ = 'f: {
    l_damage_4 = { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(388)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    's1: {
        let sv5: i32 = p_slot_2;
        match sv5 {
            2i32 | 3i32 => {
                { let r_ = F::BtlStat_GetBlast2DamageScale(c, p_chr_1); let n_ = rt::f2i(rt::fmul(rt::i2f(l_damage_4), r_)); l_damage_4 = n_; };
                break 's1;
            }
            4i32 => {
                { let r_ = F::BtlStat_GetUltimateDamageScale(c, p_chr_1); let n_ = rt::f2i(rt::fmul(rt::i2f(l_damage_4), r_)); l_damage_4 = n_; };
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((!((p_total_3) != 0)) as i32)) != 0) {
        { let r_ = F::BtlSuper_GetHitCount(c, p_chr_1, p_slot_2); let n_ = rt::div_i32(l_damage_4, r_); l_damage_4 = n_; };
    }
    break 'f l_damage_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetBaseDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetBaseGuardDamage(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32, mut p_total_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetBaseGuardDamage", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_total_3) as u32], false); }
    let mut l_damage_4: i32 = 0;
    let r_ = 'f: {
    l_damage_4 = { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(400)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    's1: {
        let sv5: i32 = p_slot_2;
        match sv5 {
            2i32 | 3i32 => {
                { let r_ = F::BtlStat_GetBlast2DamageScale(c, p_chr_1); let n_ = rt::f2i(rt::fmul(rt::i2f(l_damage_4), r_)); l_damage_4 = n_; };
                break 's1;
            }
            4i32 => {
                { let r_ = F::BtlStat_GetUltimateDamageScale(c, p_chr_1); let n_ = rt::f2i(rt::fmul(rt::i2f(l_damage_4), r_)); l_damage_4 = n_; };
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((!((p_total_3) != 0)) as i32)) != 0) {
        { let r_ = F::BtlSuper_GetHitCount(c, p_chr_1, p_slot_2); let n_ = rt::div_i32(l_damage_4, r_); l_damage_4 = n_; };
    }
    break 'f l_damage_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetBaseGuardDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetKiCost(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetKiCost", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_ki_3: i32 = 0;
    let r_ = 'f: {
    l_ki_3 = { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(412)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    if ((F::BtlMember_HasAbility(c, p_chr_1, 43i32)) != 0) {
        { let r_ = 2i32; let n_ = rt::div_i32(l_ki_3, r_); l_ki_3 = n_; };
    }
    break 'f l_ki_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetKiCost", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetRecoilDamage(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetRecoilDamage", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(424)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetRecoilDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetDrain(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetDrain", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(436)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetDrain", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetCooldownFrames(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetCooldownFrames", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(448)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetCooldownFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetChargeLimitFrames(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetChargeLimitFrames", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(460)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetChargeLimitFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetChargeScale(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetChargeScale", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(472)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetChargeScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetThrowAngleA(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowAngleA", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(484)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowAngleA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetThrowAngleB(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowAngleB", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(496)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowAngleB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetRushSpeed(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetRushSpeed", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(508)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetRushSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetThrowChara(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowChara", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_tbl_5: u32 = 0;
    let mut l_chara_6: i32 = 0;
    let mut l_alt_7: i32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
    l_i_4 = (p_slot_2).wrapping_sub(2i32);
    l_tbl_5 = c.m.r32((l_obj_3).wrapping_add(2348));
    l_chara_6 = (((c.m.r16(((l_tbl_5).wrapping_add(520)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2))) as i16)) as i32);
    l_alt_7 = (((c.m.r16(((l_tbl_5).wrapping_add(526)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2))) as i16)) as i32);
    if ((((((c.m.r32((l_obj_3).wrapping_add(12)) as i32)) != (l_chara_6)) as i32)) != 0) {
        if (((((((((l_chara_6) >= (256i32)) as i32)) != 0) && ((((c.m.r32(((l_tbl_5).wrapping_add(12)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))) & (((1073741824i32) as u32)))) != 0)) as i32)) != 0) {
            if ((((({ let a_ = (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                break 'f l_alt_7;
            }
        }
        break 'f l_chara_6;
    }
    break 'f l_alt_7;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowChara", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetThrowCostume(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowCostume", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) };
    { let r_ = 2i32; let n_ = (p_slot_2).wrapping_sub(r_); p_slot_2 = n_; };
    if ((((((((c.m.r16(((l_tbl_3).wrapping_add(520)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(2))) as i16)) as i32)) < (256i32)) as i32)) != 0) {
        if ((((c.m.r32(((l_tbl_3).wrapping_add(12)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)))) & (((1073741824i32) as u32)))) != 0) {
            break 'f { let a_ = (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(4); (c.m.r32(a_) as i32) };
        }
        if ((((((((c.m.r16(((l_tbl_3).wrapping_add(24)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(2))) as i16)) as i32)) == (639i32)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowCostume", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetThrowGauge20(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowGauge20", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) };
    { let r_ = 2i32; let n_ = (p_slot_2).wrapping_sub(r_); p_slot_2 = n_; };
    if ((((((((c.m.r16(((l_tbl_3).wrapping_add(520)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(2))) as i16)) as i32)) < (256i32)) as i32)) != 0) {
        if ((((c.m.r32(((l_tbl_3).wrapping_add(12)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)))) & (((1073741824i32) as u32)))) != 0) {
            break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(32); (c.m.r32(a_) as i32) };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetThrowGauge20", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetUnk220(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk220", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(544)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk220", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetUnk223(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk223", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(547)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_n_3) < (0i32)) as i32)) != 0) {
        l_n_3 = 999i32;
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk223", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetUnk226(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk226", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(550)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk226", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetPromptRowIndex(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetPromptRowIndex", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(553)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetPromptRowIndex", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetPromptRow(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetPromptRow", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(52); c.m.r32(a_) }).wrapping_add((((F::BtlSuper_GetPromptRowIndex(c, p_chr_1, p_slot_2)) as u32)).wrapping_mul(20));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetPromptRow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetUnk165(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk165", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(357)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetUnk165", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetLastStep(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetLastStep", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(360)).wrapping_add((((p_slot_2).wrapping_sub(2i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_n_3) < (0i32)) as i32)) != 0) {
        l_n_3 = (F::BtlSuper_GetStepCount(c, p_chr_1, p_slot_2)).wrapping_sub(1i32);
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetLastStep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetFrames22C(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetFrames22C", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(556)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetFrames22C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetFrames238(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetFrames238", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2348); c.m.r32(a_) }).wrapping_add(568)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetFrames238", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetDamage(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32, mut p_guard_3: i32, mut p_total_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetDamage", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_guard_3) as u32, (p_total_4) as u32], false); }
    let mut l_flags_5: u32 = 0;
    let mut l_damage_6: i32 = 0;
    let r_ = 'f: {
    l_flags_5 = F::BtlSuper_GetFlags(c, p_chr_1, p_slot_2);
    if ((p_guard_3) != 0) {
        l_damage_6 = F::BtlSuper_GetBaseGuardDamage(c, p_chr_1, p_slot_2, p_total_4);
    } else {
        l_damage_6 = F::BtlSuper_GetBaseDamage(c, p_chr_1, p_slot_2, p_total_4);
    }
    l_damage_6 = F::BtlUtil_RoundUp(c, l_damage_6, 10i32);
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((164i32) as i64 as u64)) as u32))) != 0) {
        l_damage_6 = F::BtlUtil_RoundUp(c, rt::div_i32((l_damage_6).wrapping_mul(3i32), 10i32), 10i32);
    }
    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0) && (((((p_slot_2) != (4i32)) as i32)) != 0)) as i32)) != 0) {
        l_damage_6 = F::BtlUtil_RoundUp(c, (l_damage_6).wrapping_add(rt::div_i32(l_damage_6, 5i32)), 10i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((156i32) as i64 as u64)) as u32))) != 0) {
        l_damage_6 = F::BtlUtil_RoundUp(c, (l_damage_6).wrapping_add(rt::div_i32(l_damage_6, 10i32)), 10i32);
    }
    if ((((((((l_flags_5) & (((32768i32) as u32)))) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((152i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        l_damage_6 = F::BtlUtil_RoundUp(c, (l_damage_6).wrapping_add(rt::div_i32((l_damage_6).wrapping_mul(3i32), 10i32)), 10i32);
    }
    l_damage_6 = rt::f2i(rt::fadd(rt::i2f(l_damage_6), rt::fmul(rt::i2f(l_damage_6), rt::fmul(F32(c.m.r32((p_chr_1).wrapping_add(3652))), rt::fsub(F::BtlSuper_GetChargeScale(c, p_chr_1, p_slot_2), F32(0x3f800000))))));
    if ((((l_flags_5) & (((16i32) as u32)))) != 0) {
        if ((F::BtlOpp_HasAbility(c, p_chr_1, 55i32)) != 0) {
            l_damage_6 = 1i32;
        }
    }
    's1: {
        let sv7: i32 = F::BtlSuper_GetId(c, p_chr_1, p_slot_2);
        match sv7 {
            616i32 => {
                's2: {
                    let sv8: i32 = (c.m.r32((p_chr_1).wrapping_add(3676)) as i32);
                    match sv8 {
                        1i32 => {
                            { let r_ = rt::div_i32(l_damage_6, 2i32); let n_ = (l_damage_6).wrapping_add(r_); l_damage_6 = n_; };
                            break 's2;
                        }
                        2i32 => {
                            { let r_ = rt::div_i32((l_damage_6).wrapping_mul(13i32), 10i32); let n_ = (l_damage_6).wrapping_add(r_); l_damage_6 = n_; };
                            break 's2;
                        }
                        3i32 => {
                            { let r_ = rt::div_i32((l_damage_6).wrapping_mul(25i32), 10i32); let n_ = (l_damage_6).wrapping_add(r_); l_damage_6 = n_; };
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
                l_damage_6 = F::BtlUtil_RoundUp(c, l_damage_6, 10i32);
                break 's1;
            }
            717i32 => {
                l_damage_6 = { let a0_ = (l_damage_6).wrapping_add(rt::div_i32((l_damage_6).wrapping_mul((c.m.r32((p_chr_1).wrapping_add(3680)) as i32)), 5i32)); F::BtlUtil_RoundUp(c, a0_, 10i32) };
                break 's1;
            }
            725i32 => {
                l_damage_6 = { let a0_ = rt::f2i(rt::fmul(rt::i2f(l_damage_6), F::BtlOpp_GetUnk20F318(c, p_chr_1))); F::BtlUtil_RoundUp(c, a0_, 10i32) };
                break 's1;
            }
            _ => {
            }
        }
    }
    if (((((p_slot_2) == (4i32)) as i32)) != 0) {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 111i32)) != 0) {
            l_damage_6 = F::BtlUtil_RoundUp(c, (l_damage_6).wrapping_add(rt::div_i32(l_damage_6, 5i32)), 10i32);
        }
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 119i32)) != 0) {
        l_damage_6 = rt::div_i32(999999i32, F::BtlSuper_GetHitCount(c, p_chr_1, p_slot_2));
    }
    break 'f l_damage_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_GetStagePlacement(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32, mut p_n_3: i32, mut p_pos_4: u32, mut p_rot_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetStagePlacement", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_n_3) as u32, (p_pos_4) as u32, (p_rot_5) as u32], false); }
    let mut l_stage_6: i32 = 0;
    let mut l_tbl_7: u32 = 0;
    let r_ = 'f: {
    l_stage_6 = F::Battle_GetStage(c);
    if ((((((((((((p_slot_2).wrapping_sub(2i32)) as u32)) < (((3i32) as u32))) as i32)) != 0) && ((((({ l_tbl_7 = c.m.r32(((p_chr_1).wrapping_add(5608)).wrapping_add(((((p_slot_2).wrapping_sub(2i32)) as u32)).wrapping_mul(4))); l_tbl_7 }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = F32(c.m.r32((l_tbl_7).wrapping_add((((((l_stage_6).wrapping_mul(5i32)).wrapping_add(p_n_3)) as u32)).wrapping_mul(16)))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_tbl_7).wrapping_add((((((l_stage_6).wrapping_mul(5i32)).wrapping_add(p_n_3)) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_tbl_7).wrapping_add((((((l_stage_6).wrapping_mul(5i32)).wrapping_add(p_n_3)) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_pos_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = p_rot_5; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_tbl_7).wrapping_add((((((l_stage_6).wrapping_mul(5i32)).wrapping_add(p_n_3)) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x40490fda)), F32(0x43340000)); let a_ = (p_rot_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_5).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_GetStagePlacement", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_IsThrow(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_IsThrow", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((((((F::BtlSuper_GetReact(c, p_chr_1, p_slot_2)) == (34i32)) as i32)) != 0) || (((((F::BtlSuper_GetReactAlt(c, p_chr_1, p_slot_2)) == (34i32)) as i32)) != 0)) as i32)) != 0) || (((((F::BtlSuper_GetReactS800(c, p_chr_1, p_slot_2)) == (34i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f (((F::BtlSuper_GetReactAltS800(c, p_chr_1, p_slot_2)) == (34i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_IsThrow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetHitCount(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitCount", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = F::BtlSkill_GetShots(c, p_chr_1, p_slot_2);
    { let r_ = F::BtlSkill_GetHitsB(c, p_chr_1, p_slot_2); let n_ = (l_n_3).wrapping_mul(r_); l_n_3 = n_; };
    { let r_ = F::BtlSkill_GetHitsC(c, p_chr_1, p_slot_2); let n_ = (l_n_3).wrapping_mul(r_); l_n_3 = n_; };
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetFlagsA(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetFlagsA", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) };
    break 'f c.m.r32((l_tbl_3).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetFlagsA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetFlags(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetFlags", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) };
    break 'f c.m.r32(((l_tbl_3).wrapping_add(8)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetId(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetId", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(16)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetUnk14(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetUnk14", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(20)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetUnk14", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetShotTime(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShotTime", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(24)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShotTime", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetShotSpeed(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShotSpeed", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(32)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41200000)), F32(0x3c17b425));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShotSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetShotTurnRate(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShotTurnRate", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fdiv(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(40)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)), F32(0x3d088888));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShotTurnRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetShotUnk30(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShotUnk30", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShotUnk30", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetUnk38(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetUnk38", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(56)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetUnk38", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetShots(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShots", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(58)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_n_3) < (1i32)) as i32)) != 0) {
        l_n_3 = 1i32;
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetShots", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetHitsB(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitsB", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(60)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_n_3) < (1i32)) as i32)) != 0) {
        l_n_3 = 1i32;
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitsB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetHitsC(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitsC", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(64)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32)).wrapping_add(1i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitsC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetHitDirKind(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitDirKind", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(98)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitDirKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetSequence(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetSequence", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(100)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetSequence", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetUnk9E(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetUnk9E", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(158)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetUnk9E", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetStatKind(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatKind", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(102)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetReact(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetReact", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(104)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetReact", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetReactS800(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetReactS800", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let mut l_react_4: i32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) };
    l_react_4 = (((c.m.r8(((l_tbl_3).wrapping_add(106)).wrapping_add(((p_slot_2) as u32))) as i8)) as i32);
    if (((((l_react_4) > (0i32)) as i32)) != 0) {
        break 'f l_react_4;
    }
    break 'f (((c.m.r8(((l_tbl_3).wrapping_add(104)).wrapping_add(((p_slot_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetReactS800", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetHitSound(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitSound", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(108)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHitSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetPush(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetPush", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) };
    break 'f rt::fdiv(F32(c.m.r32(((l_tbl_3).wrapping_add(116)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)))), rt::i2f(F::BtlSkill_GetHitCount(c, p_chr_1, p_slot_2)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetGuardPush(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetGuardPush", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) };
    break 'f rt::fdiv(F32(c.m.r32(((l_tbl_3).wrapping_add(124)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)))), rt::i2f(F::BtlSkill_GetHitCount(c, p_chr_1, p_slot_2)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetGuardPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetDamage(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32, mut p_total_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetDamage", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_total_3) as u32], false); }
    let mut l_damage_4: i32 = 0;
    let r_ = 'f: {
    l_damage_4 = { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(132)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    if ((((!((p_total_3) != 0)) as i32)) != 0) {
        { let r_ = F::BtlSkill_GetHitCount(c, p_chr_1, p_slot_2); let n_ = rt::div_i32(l_damage_4, r_); l_damage_4 = n_; };
    }
    break 'f l_damage_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetGuardDamage(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32, mut p_total_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetGuardDamage", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_total_3) as u32], false); }
    let mut l_damage_4: i32 = 0;
    let r_ = 'f: {
    l_damage_4 = { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(140)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    if ((((!((p_total_3) != 0)) as i32)) != 0) {
        { let r_ = F::BtlSkill_GetHitCount(c, p_chr_1, p_slot_2); let n_ = rt::div_i32(l_damage_4, r_); l_damage_4 = n_; };
    }
    break 'f l_damage_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetGuardDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetBlastCost(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetBlastCost", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(148)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if ((F::BtlMember_HasAbility(c, p_chr_1, 21i32)) != 0) {
        { let o_ = l_n_3; let n_ = (o_).wrapping_sub(1); l_n_3 = n_; };
    }
    if (((((l_n_3) < (1i32)) as i32)) != 0) {
        l_n_3 = 1i32;
    }
    break 'f (l_n_3).wrapping_mul(100000i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetBlastCost", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetStatLevel0(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatLevel0", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(150)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatLevel0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetStatLevel2(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatLevel2", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(152)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatLevel2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetStatLevel1(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatLevel1", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(154)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatLevel1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetStatLevel3(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatLevel3", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(156)).wrapping_add(((p_slot_2) as u32)); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetStatLevel3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetFrames(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetFrames", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fmul({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(160)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetHealthChange(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHealthChange", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) };
    break 'f rt::div_i32(({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul((c.m.r32(((l_tbl_3).wrapping_add(168)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4))) as i32)), 100i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetHealthChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetKiChange(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetKiChange", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(176)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetKiChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetValF(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetValF", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(184)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetValF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_GetValE(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetValE", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2352); c.m.r32(a_) }).wrapping_add(192)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_GetValE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlGame_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::PauseMenu_Reset(c);
    F::BtlPause_Reset(c);
    F::Hud_Reset(c);
    { let _ = F::BtlSeq_Reset(c); };
    if ((F::BattleReplay_IsActive(c)) != 0) {
        { let v_ = 1i32; let a_ = sym::gBtlGameReplayActive; c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = sym::gBtlGameReplayActive; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlGame_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Hud_Init(c);
    F::PauseMenu_Init(c);
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        F::BtlPause_Init(c, 2i32);
    } else {
        F::BtlPause_Init(c, 1i32);
    }
    { let _ = F::BtlSeq_Init(c); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlGame_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Hud_Term(c);
    F::PauseMenu_Term(c);
    F::BtlPause_Term(c);
    F::BtlSeq_Term(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlGame_PreUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_PreUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Hud_PreUpdate(c);
    { let _ = F::BtlSeq_PreUpdate(c); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlGame_Update(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::BtlSeq_Update(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Update", &PROBE_N, &[], true); }
    r_
}
