#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlCtrl_GetOppUnkD50(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetOppUnkD50", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
        if ((F::BtlChar_IsFrozen(c, l_opp_3)) != 0) {
            break 'f 0i32;
        }
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
            break 'f 0i32;
        }
        break 'f (c.m.r32((l_opp_3).wrapping_add(3408)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetOppUnkD50", &PROBE_N, &[], true); }
    r_
}

pub fn btl_tech_a__BtlCtrl_ObjOf(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_tech_a__BtlCtrl_ObjOf", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlChar_GetObj(c, p_chr_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_tech_a__BtlCtrl_ObjOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetObj(c: &mut Ctx, mut p_side_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetObj", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::btl_tech_a__BtlCtrl_ObjOf(c, l_chr_2);
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetObj", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetFlagDEtoE2(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetFlagDEtoE2", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if ((((!((F::BtlChar_IsFrozen(c, l_chr_2)) != 0)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, l_chr_2, ((((222i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if ((F::BtlChar_TestFlag(c, l_chr_2, ((((223i32) as i64 as u64)) as u32))) != 0) {
            break 'f 1i32;
        }
        if ((F::BtlChar_TestFlag(c, l_chr_2, ((((224i32) as i64 as u64)) as u32))) != 0) {
            break 'f 2i32;
        }
        if ((F::BtlChar_TestFlag(c, l_chr_2, ((((225i32) as i64 as u64)) as u32))) != 0) {
            break 'f 3i32;
        }
        if ((F::BtlChar_TestFlag(c, l_chr_2, ((((226i32) as i64 as u64)) as u32))) != 0) {
            break 'f 4i32;
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetFlagDEtoE2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetClashCount(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetClashCount", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(3664)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetClashCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsFlag6Raised(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsFlag6Raised", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
            break 'f 0i32;
        }
        break 'f F::BtlChar_IsFlagRaised(c, l_chr_2, ((((6i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsFlag6Raised", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestFlag71(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlag71", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
            break 'f 0i32;
        }
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((113i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlag71", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsFlag5RaisedInFight(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsFlag5RaisedInFight", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
            break 'f 0i32;
        }
        if (((((F::BtlSeq_GetState(c)) != (3i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f F::BtlChar_IsFlagRaised(c, l_chr_2, ((((5i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsFlag5RaisedInFight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestFlagE3(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE3", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
            break 'f 0i32;
        }
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((227i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsFlag9CRaised(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsFlag9CRaised", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
            break 'f 0i32;
        }
        break 'f F::BtlChar_IsFlagRaised(c, l_chr_2, ((((156i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsFlag9CRaised", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsFlag9ERaised(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsFlag9ERaised", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
            break 'f 0i32;
        }
        break 'f F::BtlChar_IsFlagRaised(c, l_chr_2, ((((158i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsFlag9ERaised", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsDead(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsDead", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_IsDead(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsDead", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestFlag12B(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlag12B", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((299i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlag12B", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsSwitching(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsSwitching", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        's1: {
            let sv3: i32 = F::BtlAct_GetCurrent(c, l_chr_2);
            match sv3 {
                243i32 | 244i32 | 245i32 | 246i32 | 247i32 | 248i32 => {
                    break 'f 1i32;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsSwitching", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestFlagE4(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE4", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((228i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestFlagE5(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE5", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((229i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE5", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestProgressFrameBit(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestProgressFrameBit", &PROBE_N, &[], false); }
    let mut l_chr_1: u32 = 0;
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) != (5i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32(sym::gProgress)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_chr_1 = F::BtlChar_Get(c, 0i32);
    if (((((l_chr_1) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((((c.m.r64((l_chr_1).wrapping_add(5432))).wrapping_shr(({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2036); (c.m.r32(a_) as i32) }) as u32)) & (((1i32) as u64)))) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestProgressFrameBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestFlagE6Pad(c: &mut Ctx, mut p_side_1: i32, mut p_padStatus_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE6Pad", &PROBE_N, &[(p_side_1) as u32, (p_padStatus_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        if ((((({ let a_ = (F::BtlChar_GetPad(c, l_chr_3)).wrapping_add(388); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((p_padStatus_2) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a_ = (F::BtlChar_GetPad(c, l_chr_3)).wrapping_add(388); (c.m.r32(a_) as i32) }; let a_ = p_padStatus_2; c.m.w32(a_, (v_) as u32); };
        }
        break 'f F::BtlChar_TestFlag(c, l_chr_3, ((((230i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE6Pad", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestFlagE6Pad1(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE6Pad1", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((({ let a_ = (F::BtlChar_GetPad(c, l_chr_2)).wrapping_add(388); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((230i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlagE6Pad1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetTechPromptPad1(c: &mut Ctx, mut p_side_1: i32, mut p_row_2: u32, mut p_kind_3: u32, mut p_idx_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetTechPromptPad1", &PROBE_N, &[(p_side_1) as u32, (p_row_2) as u32, (p_kind_3) as u32, (p_idx_4) as u32], false); }
    let mut l_n_5: i32 = 0;
    let mut l_col_6: i32 = 0;
    let mut l_chr_7: u32 = 0;
    let mut l_r_8: i32 = 0;
    let mut l_k_9: i32 = 0;
    let mut l_rowp_10: u32 = 0;
    let r_ = 'f: {
    l_n_5 = (1i32).wrapping_neg();
    l_col_6 = (1i32).wrapping_neg();
    l_chr_7 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_7) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((c.m.r32((l_chr_7).wrapping_add(4728)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (F::BtlChar_GetPad(c, l_chr_7)).wrapping_add(388); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((231i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((232i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((233i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((234i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((235i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((236i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((238i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_chr_7).wrapping_add(5520)) as i32)) >= (2i32)) as i32)) != 0) {
        l_n_5 = (c.m.r32((l_chr_7).wrapping_add(5520)) as i32);
        l_col_6 = 0i32;
        if ((((!((F::BtlAct_CanUseTechnique(c, l_chr_7, l_n_5)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((l_chr_7).wrapping_add(3648)) as i32)) > (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
    } else {
        if ((((((c.m.r32((l_chr_7).wrapping_add(5524)) as i32)) >= (2i32)) as i32)) != 0) {
            l_n_5 = (c.m.r32((l_chr_7).wrapping_add(5524)) as i32);
            l_col_6 = 1i32;
            if ((F::BtlChar_TestFlag(c, l_chr_7, ((((157i32) as i64 as u64)) as u32))) != 0) {
                break 'f 0i32;
            }
            if ((F::BtlChar_TestFlag(c, l_chr_7, ((((287i32) as i64 as u64)) as u32))) != 0) {
                break 'f 0i32;
            }
        }
    }
    if (((((l_n_5) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_col_6) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_r_8 = F::BtlSuper_GetPromptRowIndex(c, l_chr_7, l_n_5);
    l_rowp_10 = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(52); c.m.r32(a_) }).wrapping_add((((l_r_8) as u32)).wrapping_mul(20));
    l_k_9 = 0i32;
    's1: {
        let sv11: i32 = (((c.m.r8((l_rowp_10).wrapping_add(((l_col_6) as u32))) as i8)) as i32);
        match sv11 {
            0i32 => {
                break 's1;
            }
            1i32 => {
                l_k_9 = 1i32;
                break 's1;
            }
            2i32 => {
                l_k_9 = 2i32;
                break 's1;
            }
            3i32 => {
                l_k_9 = 3i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    if (((((p_row_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_r_8; let a_ = p_row_2; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_kind_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_k_9; let a_ = p_kind_3; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_idx_4) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_n_5; let a_ = p_idx_4; c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetTechPromptPad1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetSwitchPrompt(c: &mut Ctx, mut p_side_1: i32, mut p_buttons_2: u32, mut p_count_3: u32, mut p_kind_4: u32, mut p_padStatus_5: u32, mut p_idx_6: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetSwitchPrompt", &PROBE_N, &[(p_side_1) as u32, (p_buttons_2) as u32, (p_count_3) as u32, (p_kind_4) as u32, (p_padStatus_5) as u32, (p_idx_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_n_7: i32 = 0;
    let mut l_chr_8: u32 = 0;
    let mut l_row_9: u32 = 0;
    let mut l_cnt_10: i32 = 0;
    let mut l_k_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let r_ = 'f: {
        l_n_7 = (1i32).wrapping_neg();
        l_chr_8 = F::BtlChar_FindBySide(c, p_side_1);
        if (((((l_chr_8) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((c.m.r32((l_chr_8).wrapping_add(4728)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((({ let a_ = (F::BtlChar_GetPad(c, l_chr_8)).wrapping_add(388); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((l_chr_8).wrapping_add(5520)) as i32)) >= (2i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((l_chr_8).wrapping_add(5524)) as i32)) >= (2i32)) as i32)) != 0) {
            l_n_7 = (c.m.r32((l_chr_8).wrapping_add(5524)) as i32);
            if ((F::BtlChar_TestFlag(c, l_chr_8, ((((157i32) as i64 as u64)) as u32))) != 0) {
                break 'f 0i32;
            }
            if ((F::BtlChar_TestFlag(c, l_chr_8, ((((287i32) as i64 as u64)) as u32))) != 0) {
                break 'f 0i32;
            }
        }
        if (((((l_n_7) < (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_row_9 = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(52); c.m.r32(a_) }).wrapping_add((((F::BtlSuper_GetPromptRowIndex(c, l_chr_8, l_n_7)) as u32)).wrapping_mul(20));
        l_cnt_10 = 0i32;
        l_k_11 = 0i32;
        if ((F::BtlChar_TestFlag(c, l_chr_8, ((((162i32) as i64 as u64)) as u32))) != 0) {
            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((2i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            l_cnt_10 = 1i32;
            l_k_11 = 2i32;
        } else {
            if ((F::BtlChar_TestFlag(c, l_chr_8, ((((163i32) as i64 as u64)) as u32))) != 0) {
                { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((2i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                l_cnt_10 = 1i32;
                l_k_11 = 1i32;
            } else {
                's1: {
                    let sv13: i32 = (((c.m.r8((l_row_9).wrapping_add(2)) as i8)) as i32);
                    match sv13 {
                        0i32 => {
                            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((4i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            l_cnt_10 = 2i32;
                            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((3i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        1i32 => {
                            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((5i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            l_cnt_10 = 2i32;
                            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((3i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        2i32 => {
                            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((6i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            l_cnt_10 = 2i32;
                            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((3i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        3i32 => {
                            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((7i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            l_cnt_10 = 2i32;
                            { let v_ = F::BtlCharApi_GetButtonIcon(c, l_chr_8, ((((3i32) as i64 as u64)) as u32)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        _ => {
                        }
                    }
                }
            }
        }
        {
            l_i_12 = 0i32;
            'l2: loop {
                if !(((((l_i_12) < (l_cnt_10)) as i32)) != 0) { break 'l2; }
                'c3: {
                    if ((((((c.m.r32((fp).wrapping_add((((l_i_12) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                }
                { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
            }
        }
        if (((((p_count_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_cnt_10; let a_ = p_count_3; c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_kind_4) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_k_11; let a_ = p_kind_4; c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_padStatus_5) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a_ = (F::BtlChar_GetPad(c, l_chr_8)).wrapping_add(388); (c.m.r32(a_) as i32) }; let a_ = p_padStatus_5; c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_idx_6) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_n_7; let a_ = p_idx_6; c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_buttons_2) != (((0i32) as u32))) as i32)) != 0) {
            {
                l_i_12 = 0i32;
                'l4: loop {
                    if !(((((l_i_12) < (l_cnt_10)) as i32)) != 0) { break 'l4; }
                    'c5: {
                        { let v_ = (c.m.r32((fp).wrapping_add((((l_i_12) as u32)).wrapping_mul(4))) as i32); let a_ = (p_buttons_2).wrapping_add((((l_i_12) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetSwitchPrompt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetChangePrompt(c: &mut Ctx, mut p_side_1: i32, mut p_button_2: u32, mut p_kind_3: u32, mut p_padStatus_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetChangePrompt", &PROBE_N, &[(p_side_1) as u32, (p_button_2) as u32, (p_kind_3) as u32, (p_padStatus_4) as u32], false); }
    let mut l_k_5: i32 = 0;
    let mut l_btn_6: i32 = 0;
    let mut l_chr_7: u32 = 0;
    let r_ = 'f: {
    l_k_5 = (1i32).wrapping_neg();
    l_btn_6 = (1i32).wrapping_neg();
    l_chr_7 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_7) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((c.m.r32((l_chr_7).wrapping_add(4728)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((231i32) as i64 as u64)) as u32))) != 0) {
        l_btn_6 = F::BtlCharApi_GetButtonIcon(c, l_chr_7, ((((0i32) as i64 as u64)) as u32));
        l_k_5 = 1i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((232i32) as i64 as u64)) as u32))) != 0) {
        l_btn_6 = F::BtlCharApi_GetButtonIcon(c, l_chr_7, ((((2i32) as i64 as u64)) as u32));
        l_k_5 = 1i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((233i32) as i64 as u64)) as u32))) != 0) {
        l_btn_6 = F::BtlCharApi_GetButtonIcon(c, l_chr_7, ((((27i32) as i64 as u64)) as u32));
        l_k_5 = 2i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((234i32) as i64 as u64)) as u32))) != 0) {
        l_btn_6 = F::BtlCharApi_GetButtonIcon(c, l_chr_7, ((((28i32) as i64 as u64)) as u32));
        l_k_5 = 2i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((235i32) as i64 as u64)) as u32))) != 0) {
        l_btn_6 = F::BtlCharApi_GetButtonIcon(c, l_chr_7, ((((29i32) as i64 as u64)) as u32));
        l_k_5 = 2i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_7, ((((236i32) as i64 as u64)) as u32))) != 0) {
        l_btn_6 = F::BtlCharApi_GetButtonIcon(c, l_chr_7, ((((30i32) as i64 as u64)) as u32));
        l_k_5 = 2i32;
    }
    if (((((((((l_btn_6) < (0i32)) as i32)) != 0) || (((((l_k_5) < (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_button_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_btn_6; let a_ = p_button_2; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_kind_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_k_5; let a_ = p_kind_3; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_padStatus_4) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = (F::BtlChar_GetPad(c, l_chr_7)).wrapping_add(388); (c.m.r32(a_) as i32) }; let a_ = p_padStatus_4; c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetChangePrompt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetStatMod0(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetStatMod0", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlStat_GetMod(c, l_chr_2, 0i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetStatMod0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetStatMod2(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetStatMod2", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlStat_GetMod(c, l_chr_2, 2i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetStatMod2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetStatMod1(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetStatMod1", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlStat_GetMod(c, l_chr_2, 1i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetStatMod1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetStatMod3(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetStatMod3", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlStat_GetMod(c, l_chr_2, 3i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetStatMod3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_CanUseTechnique(c: &mut Ctx, mut p_player_1: i32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_CanUseTechnique", &PROBE_N, &[(p_player_1) as u32, (p_slot_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv4: i32 = p_slot_2;
        match sv4 {
            0i32 | 1i32 => {
                break 'f F::BtlAct_CanUseSkill(c, l_chr_3, ((((p_slot_2) as i64 as u64)) as u32));
            }
            2i32 | 3i32 | 4i32 => {
                break 'f F::BtlAct_CanUseTechnique(c, l_chr_3, p_slot_2);
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_CanUseTechnique", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetWork5A8Count(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetWork5A8Count", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let mut l_n_2: i32 = 0;
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) != (3i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_work_1 = F::Battle_GetWork5A8(c);
    if (((((l_work_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_n_2 = (c.m.r32(l_work_1) as i32);
    if ((((({ let a0_ = F::BtlChar_Get(c, 1i32); F::BtlMember_CountAlive(c, a0_) }) < (1i32)) as i32)) != 0) {
        { let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; };
    }
    if ((((((c.m.r32((l_work_1).wrapping_add(4)) as i32)) < (l_n_2)) as i32)) != 0) {
        break 'f (c.m.r32((l_work_1).wrapping_add(4)) as i32);
    }
    break 'f l_n_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetWork5A8Count", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetCpuLevel(c: &mut Ctx, mut p_player_1: i32, mut p_level_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetCpuLevel", &PROBE_N, &[(p_player_1) as u32, (p_level_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) && ((((((c.m.r32((l_chr_3).wrapping_add(2456)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((l_chr_3).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = p_level_2; let a_ = (F::BtlMember_Get(c, l_chr_3, l_i_4)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetCpuLevel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestMemberUnk70(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestMemberUnk70", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestMemberUnk70(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestMemberUnk70", &PROBE_N, &[], true); }
    r_
}

pub fn btl_tech_a__BtlAtk_GetActionRow(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_tech_a__BtlAtk_GetActionRow", &PROBE_N, &[(p_n_1) as u32], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(44); c.m.r32(a_) }).wrapping_add((((p_n_1) as u32)).wrapping_mul(24));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_tech_a__BtlAtk_GetActionRow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetId(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetId", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_charge_3: F32 = F32(0);
    let mut l_hit_4: u32 = 0;
    let mut l_alt_5: i32 = 0;
    let mut l_action_6: i32 = 0;
    let mut l_anim_7: i32 = 0;
    let mut l_mid_8: F32 = F32(0);
    let mut l_full_9: F32 = F32(0);
    let mut l_frame_10: F32 = F32(0);
    let mut l_last_11: F32 = F32(0);
    let mut l_b_12: i32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    l_charge_3 = F32(c.m.r32((p_chr_1).wrapping_add(3448)));
    l_hit_4 = (l_obj_2).wrapping_add(3216);
    l_alt_5 = (c.m.r32((p_chr_1).wrapping_add(3432)) as i32);
    l_action_6 = F::BtlAct_GetCurrent(c, p_chr_1);
    l_anim_7 = F::BtlAnim_GetId(c, p_chr_1);
    l_mid_8 = F32(0x3e999999);
    l_full_9 = F32(0x3f7d70a3);
    if ((F::BtlAct_IsAttackId(c, l_action_6)) != 0) {
        break 'f (({ let a_ = (F::btl_tech_a__BtlAtk_GetActionRow(c, (l_action_6).wrapping_sub(112i32))).wrapping_add(3); c.m.r8(a_) }) as i32);
    }
    l_frame_10 = F::BtlAnim_GetFrame(c, p_chr_1);
    l_last_11 = rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_2, ((1i32) as u64), 0i32, ((((2i32) as i64 as u64)) as u32)));
    's1: {
        let sv13: i32 = l_action_6;
        match sv13 {
            70i32 => {
                's2: {
                    let sv14: i32 = l_anim_7;
                    match sv14 {
                        60i32 => {
                            break 'f 78i32;
                        }
                        61i32 => {
                            break 'f 79i32;
                        }
                        62i32 => {
                            break 'f 80i32;
                        }
                        63i32 => {
                            break 'f 81i32;
                        }
                        64i32 => {
                            break 'f 82i32;
                        }
                        _ => {
                        }
                    }
                }
                break 'f (1i32).wrapping_neg();
            }
            180i32 | 181i32 | 183i32 | 187i32 => {
                break 'f 85i32;
            }
            182i32 | 185i32 => {
                break 'f 86i32;
            }
            96i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 11i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 12i32;
                }
                break 'f 13i32;
            }
            90i32 => {
                break 'f 67i32;
            }
            91i32 => {
                break 'f 68i32;
            }
            92i32 => {
                break 'f 69i32;
            }
            93i32 => {
                break 'f 70i32;
            }
            107i32 => {
                break 'f (if ((l_alt_5) != 0) { 62i32 } else { 52i32 });
            }
            108i32 => {
                break 'f (if ((l_alt_5) != 0) { 63i32 } else { 53i32 });
            }
            109i32 => {
                break 'f (if ((l_alt_5) != 0) { 64i32 } else { 54i32 });
            }
            110i32 => {
                break 'f (if ((l_alt_5) != 0) { 65i32 } else { 55i32 });
            }
            111i32 => {
                break 'f (if ((l_alt_5) != 0) { 66i32 } else { 56i32 });
            }
            100i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 89i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 91i32;
                }
                break 'f 93i32;
            }
            101i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 90i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 92i32;
                }
                break 'f 94i32;
            }
            97i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 46i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 47i32;
                }
                break 'f 48i32;
            }
            98i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 49i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 50i32;
                }
                break 'f 51i32;
            }
            55i32 | 66i32 => {
                break 'f 102i32;
            }
            79i32 => {
                break 'f 103i32;
            }
            80i32 => {
                break 'f 104i32;
            }
            77i32 => {
                break 'f 105i32;
            }
            78i32 => {
                break 'f 106i32;
            }
            81i32 | 82i32 => {
                break 'f 107i32;
            }
            _ => {
            }
        }
    }
    's3: {
        let sv15: i32 = l_anim_7;
        match sv15 {
            55i32 => {
                if ((((((((((((((((c.m.r8((l_hit_4).wrapping_add(((28i32) as u32))) as i8)) as i32)) >= (2i32)) as i32)) != 0) && ((((({ l_b_12 = (((c.m.r8((l_hit_4).wrapping_add(((29i32) as u32))) as i8)) as i32); l_b_12 }) != (0i32)) as i32)) != 0)) as i32)) != 0) && (((((l_b_12) != ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                    break 'f 1i32;
                }
                break 'f 0i32;
            }
            56i32 => {
                break 'f 2i32;
            }
            57i32 => {
                break 'f 3i32;
            }
            58i32 => {
                break 'f 4i32;
            }
            59i32 => {
                break 'f 5i32;
            }
            60i32 => {
                break 'f 6i32;
            }
            61i32 => {
                break 'f 7i32;
            }
            62i32 => {
                break 'f 8i32;
            }
            63i32 => {
                break 'f 9i32;
            }
            64i32 => {
                break 'f 10i32;
            }
            147i32 => {
                if (((rt::flt(l_frame_10, l_last_11) as i32)) != 0) {
                    break 'f 17i32;
                }
                break 'f 18i32;
            }
            88i32 => {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((132i32) as i64 as u64)) as u32))) != 0) {
                    break 'f 103i32;
                }
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 19i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 24i32;
                }
                break 'f 29i32;
            }
            91i32 => {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((132i32) as i64 as u64)) as u32))) != 0) {
                    break 'f 104i32;
                }
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 20i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 25i32;
                }
                break 'f 30i32;
            }
            82i32 => {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((132i32) as i64 as u64)) as u32))) != 0) {
                    break 'f 105i32;
                }
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 21i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 26i32;
                }
                break 'f 31i32;
            }
            85i32 => {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((132i32) as i64 as u64)) as u32))) != 0) {
                    break 'f 106i32;
                }
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 22i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 27i32;
                }
                break 'f 32i32;
            }
            76i32 | 79i32 => {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((132i32) as i64 as u64)) as u32))) != 0) {
                    break 'f 107i32;
                }
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 23i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 28i32;
                }
                break 'f 33i32;
            }
            103i32 => {
                if ((l_alt_5) != 0) {
                    break 'f 62i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 52i32;
                }
                break 'f 57i32;
            }
            106i32 => {
                if ((l_alt_5) != 0) {
                    break 'f 63i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 53i32;
                }
                break 'f 58i32;
            }
            97i32 => {
                if ((l_alt_5) != 0) {
                    break 'f 64i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 54i32;
                }
                break 'f 59i32;
            }
            100i32 => {
                if ((l_alt_5) != 0) {
                    break 'f 65i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 55i32;
                }
                break 'f 60i32;
            }
            94i32 => {
                if ((l_alt_5) != 0) {
                    break 'f 66i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 56i32;
                }
                break 'f 61i32;
            }
            67i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 72i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 73i32;
                }
                break 'f 74i32;
            }
            70i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 75i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 76i32;
                }
                break 'f 77i32;
            }
            109i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f (if (((rt::flt(l_frame_10, l_last_11) as i32)) != 0) { 34i32 } else { 35i32 });
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f (if (((rt::flt(l_frame_10, l_last_11) as i32)) != 0) { 36i32 } else { 37i32 });
                }
                break 'f (if (((rt::flt(l_frame_10, l_last_11) as i32)) != 0) { 38i32 } else { 39i32 });
            }
            112i32 => {
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f (if (((rt::flt(l_frame_10, l_last_11) as i32)) != 0) { 40i32 } else { 41i32 });
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f (if (((rt::flt(l_frame_10, l_last_11) as i32)) != 0) { 42i32 } else { 43i32 });
                }
                break 'f (if (((rt::flt(l_frame_10, l_last_11) as i32)) != 0) { 44i32 } else { 45i32 });
            }
            248i32 => {
                break 'f 87i32;
            }
            411i32 => {
                if ((((((((c.m.r8((l_obj_2).wrapping_add(3245)) as i8)) as i32)) == (0i32)) as i32)) != 0) {
                    if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                        break 'f 126i32;
                    }
                    if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                        break 'f 129i32;
                    }
                    break 'f 132i32;
                }
                if ((((((((c.m.r8((l_obj_2).wrapping_add(3245)) as i8)) as i32)) < (((((c.m.r8((l_obj_2).wrapping_add(3244)) as i8)) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
                    if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                        break 'f 127i32;
                    }
                    if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                        break 'f 130i32;
                    }
                    break 'f 133i32;
                }
                if (((rt::flt(l_charge_3, l_mid_8) as i32)) != 0) {
                    break 'f 128i32;
                }
                if (((rt::flt(l_charge_3, l_full_9) as i32)) != 0) {
                    break 'f 131i32;
                }
                break 'f 134i32;
            }
            _ => {
            }
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetRecord(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetRecord", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let mut l_n_3: u32 = 0;
    let r_ = 'f: {
    l_n_3 = ((F::BtlAtk_GetId(c, p_chr_1)) as u32);
    if (((((l_n_3) >= (((163i32) as u32))) as i32)) != 0) {
        l_n_3 = ((0i32) as u32);
    }
    if (((((p_id_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = ((l_n_3) as i32); let a_ = p_id_2; c.m.w32(a_, (v_) as u32); };
    }
    break 'f ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2336); c.m.r32(a_) }).wrapping_add((l_n_3).wrapping_mul(48));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetRecord", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetRecordOf(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetRecordOf", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_id_2) >= (((163i32) as u32))) as i32)) != 0) {
        p_id_2 = ((0i32) as u32);
    }
    break 'f ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2336); c.m.r32(a_) }).wrapping_add((p_id_2).wrapping_mul(48));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetRecordOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_ApplyAbilities(c: &mut Ctx, mut p_chr_1: u32, mut p_val_2: i32, mut p_id_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_ApplyAbilities", &PROBE_N, &[(p_chr_1) as u32, (p_val_2) as u32, (p_id_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_id_3) < (((10i32) as u32))) as i32)) != 0) {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 4i32)) != 0) {
            { let r_ = rt::div_i32(p_val_2, 10i32); let n_ = (p_val_2).wrapping_add(r_); p_val_2 = n_; };
        }
        if ((F::BtlMember_HasAbility(c, p_chr_1, 5i32)) != 0) {
            { let r_ = rt::div_i32(p_val_2, 10i32); let n_ = (p_val_2).wrapping_sub(r_); p_val_2 = n_; };
        }
    }
    if ((((((p_id_3).wrapping_sub(((19i32) as u32))) < (((15i32) as u32))) as i32)) != 0) {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 6i32)) != 0) {
            { let r_ = rt::div_i32(p_val_2, 10i32); let n_ = (p_val_2).wrapping_add(r_); p_val_2 = n_; };
        }
        if ((F::BtlMember_HasAbility(c, p_chr_1, 7i32)) != 0) {
            { let r_ = rt::div_i32(p_val_2, 10i32); let n_ = (p_val_2).wrapping_sub(r_); p_val_2 = n_; };
        }
    }
    if ((((((p_id_3).wrapping_sub(((85i32) as u32))) < (((2i32) as u32))) as i32)) != 0) {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 8i32)) != 0) {
            { let r_ = 2i32; let n_ = (p_val_2).wrapping_mul(r_); p_val_2 = n_; };
        }
        if ((F::BtlMember_HasAbility(c, p_chr_1, 9i32)) != 0) {
            { let r_ = rt::div_i32(p_val_2, 2i32); let n_ = (p_val_2).wrapping_sub(r_); p_val_2 = n_; };
        }
    }
    break 'f p_val_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_ApplyAbilities", &PROBE_N, &[], true); }
    r_
}

pub fn btl_tech_a__BtlAtk_IsId13to21(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_tech_a__BtlAtk_IsId13to21", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((p_id_1).wrapping_sub(19i32)) as u32)) < (((15i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_tech_a__BtlAtk_IsId13to21", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetPriority(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetPriority", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(38); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetPriority", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetDamage(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetDamage", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_obj_2: u32 = 0;
    let mut l_val_3: i32 = 0;
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
        l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
        l_val_3 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, fp)).wrapping_add(4); c.m.r16(a_) }) as i32);
        l_val_3 = rt::f2i(rt::fmul(rt::i2f(l_val_3), F::BtlStat_GetMeleeDamageScale(c, p_chr_1)));
        l_val_3 = { let a2_ = (((c.m.r32(fp) as i32)) as u32); F::BtlAtk_ApplyAbilities(c, p_chr_1, l_val_3, a2_) };
        l_hits_4 = F::BtlObjAnim_QueryEvent(c, l_obj_2, ((1i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32));
        if (((((l_hits_4) >= (2i32)) as i32)) != 0) {
            { let r_ = l_hits_4; let n_ = rt::div_i32(l_val_3, r_); l_val_3 = n_; };
        }
        break 'f l_val_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetGuardDamage(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardDamage", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_obj_2: u32 = 0;
    let mut l_val_3: i32 = 0;
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
        l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
        l_val_3 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, fp)).wrapping_add(6); c.m.r16(a_) }) as i32);
        l_val_3 = rt::f2i(rt::fmul(rt::i2f(l_val_3), F::BtlStat_GetMeleeDamageScale(c, p_chr_1)));
        if ((F::BtlAct_TestPoweredSkill(c, p_chr_1, ((((16i32) as i64 as u64)) as u32))) != 0) {
            if ((((((((c.m.r32(fp) as i32)) as u32)) < (((11i32) as u32))) as i32)) != 0) {
                { let r_ = rt::div_i32((l_val_3).wrapping_mul(3i32), 10i32); let n_ = (l_val_3).wrapping_sub(r_); l_val_3 = n_; };
            }
        }
        l_val_3 = { let a2_ = (((c.m.r32(fp) as i32)) as u32); F::BtlAtk_ApplyAbilities(c, p_chr_1, l_val_3, a2_) };
        l_hits_4 = F::BtlObjAnim_QueryEvent(c, l_obj_2, ((1i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32));
        if (((((l_hits_4) >= (2i32)) as i32)) != 0) {
            { let r_ = l_hits_4; let n_ = rt::div_i32(l_val_3, r_); l_val_3 = n_; };
        }
        break 'f l_val_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetThrowParamC(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetThrowParamC", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(12); c.m.r16(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetThrowParamC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetThrowParamE(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetThrowParamE", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(14); c.m.r16(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetThrowParamE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetKiGain(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetKiGain", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_val_3: i32 = 0;
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    l_val_3 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(10); c.m.r16(a_) }) as i32);
    if ((F::BtlMember_HasAbility(c, p_chr_1, 110i32)) != 0) {
        { let r_ = rt::div_u32(((l_val_3) as u32), ((2i32) as u32)); let n_ = (((((l_val_3) as u32)).wrapping_add(r_)) as i32); l_val_3 = n_; };
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 32i32)) != 0) {
            { let r_ = rt::div_i32(l_val_3, 4i32); let n_ = (l_val_3).wrapping_add(r_); l_val_3 = n_; };
        }
    }
    l_hits_4 = F::BtlObjAnim_QueryEvent(c, l_obj_2, ((1i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32));
    if (((((l_hits_4) > (0i32)) as i32)) != 0) {
        { let r_ = l_hits_4; let n_ = rt::div_i32(l_val_3, r_); l_val_3 = n_; };
    }
    break 'f l_val_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetKiGain", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetGuardKiCost(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKiCost", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_val_2: i32 = 0;
    let r_ = 'f: {
    l_val_2 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(8); c.m.r16(a_) }) as i32);
    l_val_2 = rt::f2i(rt::fmul(rt::i2f(l_val_2), F::BtlStat_GetGuardKiCostScale(c, p_chr_1)));
    if ((F::BtlMember_HasAbility(c, p_chr_1, 68i32)) != 0) {
        { let r_ = 2i32; let n_ = (l_val_2).wrapping_mul(r_); l_val_2 = n_; };
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 109i32)) != 0) {
            { let r_ = rt::div_i32(l_val_2, 2i32); let n_ = (l_val_2).wrapping_add(r_); l_val_2 = n_; };
        }
    }
    break 'f l_val_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKiCost", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetUnk10(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetUnk10", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_val_3: i32 = 0;
    let mut l_hits_4: i32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    l_val_3 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(16); c.m.r16(a_) }) as i32);
    l_hits_4 = F::BtlObjAnim_QueryEvent(c, l_obj_2, ((1i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32));
    if (((((l_hits_4) > (0i32)) as i32)) != 0) {
        { let r_ = l_hits_4; let n_ = rt::div_i32(l_val_3, r_); l_val_3 = n_; };
    }
    break 'f l_val_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetUnk10", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetReaction(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReaction", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(27); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReaction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetReactionB(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(28)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(28)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(27)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetReactionC(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionC", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(31)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(31)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(27)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetReactionD(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionD", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(29)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(29)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(27)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionD", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetReactionE(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionE", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(30)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(30)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(27)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetReactionSub(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionSub", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(32); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionSub", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetReactionF(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionF", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(33)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(33)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(27)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetReactionG(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionG", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32));
    break 'f (if ((((((((c.m.r8((l_rec_2).wrapping_add(34)) as i8)) as i32)) != (0i32)) as i32)) != 0) { (((c.m.r8((l_rec_2).wrapping_add(34)) as i8)) as i32) } else { (((c.m.r8((l_rec_2).wrapping_add(27)) as i8)) as i32) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetReactionG", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetFlags(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32)); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetPushOnHit(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetPushOnHit", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(18); c.m.r8(a_) }) as i32)), F32(0x41200000)), F32(0x3c17b425));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetPushOnHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetPushOnGuard(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetPushOnGuard", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(19); c.m.r8(a_) }) as i32)), F32(0x41200000)), F32(0x3c17b425));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetPushOnGuard", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetLaunchSpeed(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchSpeed", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: F32 = F32(0);
    let r_ = 'f: {
    l_v_2 = rt::fmul(rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(20); c.m.r8(a_) }) as i32)), F32(0x41200000)), F32(0x3c17b425));
    break 'f rt::fmul(l_v_2, F::BtlStat_GetSpeedScale(c, p_chr_1));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetLaunchAngleA(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchAngleA", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(21); (c.m.r8(a_) as i8) }) as i32)), F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchAngleA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetLaunchAngleAOf(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchAngleAOf", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecordOf(c, p_chr_1, p_id_2)).wrapping_add(21); (c.m.r8(a_) as i8) }) as i32)), F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchAngleAOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetLaunchAngleB(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchAngleB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(22); (c.m.r8(a_) as i8) }) as i32)), F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchAngleB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetLaunchAngleBOf(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchAngleBOf", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecordOf(c, p_chr_1, p_id_2)).wrapping_add(22); (c.m.r8(a_) as i8) }) as i32)), F32(0x40490fda)), F32(0x43340000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetLaunchAngleBOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetShakePower(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetShakePower", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::i2f(({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(23); c.m.r8(a_) }) as i32);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetShakePower", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetShakeTime(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetShakeTime", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(24); c.m.r8(a_) }) as i32)), F32(0x3dcccccc));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetShakeTime", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetShakePowerB(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetShakePowerB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::i2f(({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(25); c.m.r8(a_) }) as i32);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetShakePowerB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetShakeTimeB(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetShakeTimeB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fmul(rt::i2f((({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(26); c.m.r8(a_) }) as i32)), F32(0x3dcccccc));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetShakeTimeB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetHitFxKind(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetHitFxKind", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(35); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetHitFxKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetHitSoundLevel(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetHitSoundLevel", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(36); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetHitSoundLevel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetVoiceKind(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetVoiceKind", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(37); (c.m.r8(a_) as i8) }) as i32);
        match sv2 {
            0i32 => {
                break 'f (1i32).wrapping_neg();
            }
            1i32 => {
                break 'f 5i32;
            }
            2i32 => {
                break 'f 6i32;
            }
            3i32 => {
                break 'f 7i32;
            }
            _ => {
            }
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetVoiceKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetGuardKindA(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKindA", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
        l_v_2 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, fp)).wrapping_add(41); (c.m.r8(a_) as i8) }) as i32);
        if (((((l_v_2) == (1i32)) as i32)) != 0) {
            l_v_2 = ((!((F::BtlOpp_HasAbility(c, p_chr_1, 64i32)) != 0)) as i32);
        }
        break 'f l_v_2;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKindA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetGuardKindB(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKindB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
        l_v_2 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, fp)).wrapping_add(39); (c.m.r8(a_) as i8) }) as i32);
        if (((((((((l_v_2) == (1i32)) as i32)) != 0) && ((F::BtlOpp_HasAbility(c, p_chr_1, 64i32)) != 0)) as i32)) != 0) {
            l_v_2 = ((!(({ let a0_ = (c.m.r32(fp) as i32); F::btl_tech_a__BtlAtk_IsId13to21(c, a0_) }) != 0)) as i32);
        }
        break 'f l_v_2;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKindB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetGuardKindC(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKindC", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
        l_v_2 = (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, fp)).wrapping_add(40); (c.m.r8(a_) as i8) }) as i32);
        if (((((((((l_v_2) == (1i32)) as i32)) != 0) && ((F::BtlOpp_HasAbility(c, p_chr_1, 64i32)) != 0)) as i32)) != 0) {
            l_v_2 = ((!(({ let a0_ = (c.m.r32(fp) as i32); F::btl_tech_a__BtlAtk_IsId13to21(c, a0_) }) != 0)) as i32);
        }
        break 'f l_v_2;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKindC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_RankGuardKind(c: &mut Ctx, mut p_kind_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_RankGuardKind", &PROBE_N, &[(p_kind_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: u32 = p_kind_1;
        match sv2 {
            4u32 => {
                break 'f 5i32;
            }
            0u32 => {
                break 'f 4i32;
            }
            1u32 => {
                break 'f 3i32;
            }
            2u32 => {
                break 'f 2i32;
            }
            3u32 => {
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_RankGuardKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetBestGuardKind(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetBestGuardKind", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_v_3: i32 = 0;
    let mut l_bestRank_4: i32 = 0;
    let mut l_best_5: i32 = 0;
    let mut l_rec_6: u32 = 0;
    let mut l_rank_7: i32 = 0;
    let r_ = 'f: {
    l_i_2 = 0i32;
    l_v_3 = 0i32;
    l_bestRank_4 = (1i32).wrapping_neg();
    l_best_5 = (1i32).wrapping_neg();
    l_rec_6 = F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32));
    {
        'l1: loop {
            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                's3: {
                    let sv8: i32 = l_i_2;
                    match sv8 {
                        0i32 => {
                            l_v_3 = (((c.m.r8((l_rec_6).wrapping_add(41)) as i8)) as i32);
                            break 's3;
                        }
                        1i32 => {
                            l_v_3 = (((c.m.r8((l_rec_6).wrapping_add(39)) as i8)) as i32);
                            break 's3;
                        }
                        2i32 => {
                            l_v_3 = (((c.m.r8((l_rec_6).wrapping_add(40)) as i8)) as i32);
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
                l_rank_7 = F::BtlAtk_RankGuardKind(c, ((l_v_3) as u32));
                if (((((l_bestRank_4) < (l_rank_7)) as i32)) != 0) {
                    l_bestRank_4 = l_rank_7;
                    l_best_5 = l_v_3;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f l_best_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetBestGuardKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetGuardKindE(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKindE", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(42); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardKindE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetGuardFlagSel(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardFlagSel", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(43); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetGuardFlagSel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetUnk2C(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetUnk2C", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecord(c, p_chr_1, ((0i32) as u32))).wrapping_add(44); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetUnk2C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAtk_GetUnk2COf(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetUnk2COf", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::BtlAtk_GetRecordOf(c, p_chr_1, p_id_2)).wrapping_add(44); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAtk_GetUnk2COf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_ScaleKiCharge(c: &mut Ctx, mut p_chr_1: u32, mut p_val_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_ScaleKiCharge", &PROBE_N, &[(p_chr_1) as u32, (p_val_2) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlMember_HasAbility(c, p_chr_1, 49i32)) != 0) {
        p_val_2 = rt::div_i32((p_val_2).wrapping_mul(4i32), 10i32);
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 48i32)) != 0) {
            p_val_2 = rt::div_i32((p_val_2).wrapping_mul(6i32), 10i32);
        } else {
            if ((F::BtlMember_HasAbility(c, p_chr_1, 47i32)) != 0) {
                p_val_2 = rt::div_i32((p_val_2).wrapping_mul(8i32), 10i32);
            }
        }
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 117i32)) != 0) {
        p_val_2 = rt::div_i32((p_val_2).wrapping_mul(125i32), 100i32);
    }
    break 'f p_val_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_ScaleKiCharge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetFlags(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetFlags2(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetFlags2", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(20); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetFlags2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetFlags3(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetFlags3", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(24); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetFlags3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk0(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk0", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }; c.m.r16(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk2(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk2", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(2); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkAC(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkAC", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(172); (c.m.r8(a_) as i8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkAC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetSlotId(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotId", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(152)).wrapping_add(((p_slot_2) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetSlotCost(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotCost", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(156)).wrapping_add(((p_slot_2) as u32)); c.m.r8(a_) }) as i32)).wrapping_mul(100000i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotCost", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetSlotA0(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotA0", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(160)).wrapping_add(((p_slot_2) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotA0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetSlotA4(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotA4", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(164)).wrapping_add(((p_slot_2) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotA4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetSlotA8(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotA8", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(168)).wrapping_add(((p_slot_2) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetSlotA8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkAD(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkAD", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(173); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkAD", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_CountSlots(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_CountSlots", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_param_2: u32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_param_2 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) };
    l_n_3 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r8(((l_param_2).wrapping_add(152)).wrapping_add(((l_i_4) as u32)))) as i32)) != (255i32)) as i32)) != 0) {
                    { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_CountSlots", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_TestUnkADBit(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_TestUnkADBit", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(173); c.m.r8(a_) }) as i32)) & ((16i32).wrapping_shl((p_n_2) as u32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_TestUnkADBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkB4(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkB4", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(180)).wrapping_add(((p_n_2) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkB4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkB7(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkB7", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(183)).wrapping_add(((p_n_2) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkB7", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkBA(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32, mut p_m_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkBA", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32, (p_m_3) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ((({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(186)).wrapping_add((((p_n_2) as u32)).wrapping_mul(4))).wrapping_add(((p_m_3) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkBA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetCostAE(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetCostAE", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(174)).wrapping_add(((p_n_2) as u32)); c.m.r8(a_) }) as i32)).wrapping_mul(100000i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetCostAE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkC6(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkC6", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(198); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkC6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnkB1(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkB1", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(177)).wrapping_add(((p_n_2) as u32)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnkB1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetAuraKind(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetAuraKind", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(3); (c.m.r8(a_) as i8) }) as i32);
    if ((F::BtlMember_HasAbility(c, p_chr_1, 77i32)) != 0) {
        l_v_2 = 0i32;
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 78i32)) != 0) {
            l_v_2 = 1i32;
        } else {
            if ((F::BtlMember_HasAbility(c, p_chr_1, 79i32)) != 0) {
                l_v_2 = 2i32;
            } else {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 80i32)) != 0) {
                    l_v_2 = 3i32;
                } else {
                    if ((F::BtlMember_HasAbility(c, p_chr_1, 81i32)) != 0) {
                        l_v_2 = 4i32;
                    } else {
                        if ((F::BtlMember_HasAbility(c, p_chr_1, 82i32)) != 0) {
                            l_v_2 = 5i32;
                        } else {
                            if ((F::BtlMember_HasAbility(c, p_chr_1, 83i32)) != 0) {
                                l_v_2 = 6i32;
                            } else {
                                if ((F::BtlMember_HasAbility(c, p_chr_1, 116i32)) != 0) {
                                    l_v_2 = 7i32;
                                } else {
                                    if ((F::BtlMember_HasAbility(c, p_chr_1, 84i32)) != 0) {
                                        l_v_2 = 8i32;
                                    } else {
                                        if ((F::BtlMember_HasAbility(c, p_chr_1, 85i32)) != 0) {
                                            l_v_2 = 9i32;
                                        } else {
                                            if ((F::BtlMember_HasAbility(c, p_chr_1, 86i32)) != 0) {
                                                l_v_2 = 10i32;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    break 'f l_v_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetAuraKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetType(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetType", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_param_2: u32 = 0;
    let mut l_costume_3: u32 = 0;
    let mut l_type_4: i32 = 0;
    let r_ = 'f: {
    l_param_2 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) };
    l_costume_3 = { let a_ = (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_costume_3) >= (((4i32) as u32))) as i32)) != 0) {
        l_costume_3 = ((0i32) as u32);
    }
    l_type_4 = (((c.m.r8(((l_param_2).wrapping_add(88)).wrapping_add(l_costume_3)) as i8)) as i32);
    's1: {
        let sv5: i32 = l_type_4;
        match sv5 {
            2i32 | 3i32 | 4i32 => {
                if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(32); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    if (((((l_type_4) == (4i32)) as i32)) != 0) {
                        l_type_4 = 8i32;
                    } else {
                        l_type_4 = 7i32;
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_type_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetTypeValueA(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetTypeValueA", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_v_2: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 40);
        { let v_ = F32(0x3cb8f4e6); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3ce16a79); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3d80cf20); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3de16a79); c.m.w32(fp.wrapping_add(12), (v_).0); }
        { let v_ = F32(0x3e9646fb); c.m.w32(fp.wrapping_add(16), (v_).0); }
        { let v_ = F32(0x3cb4552c); c.m.w32(fp.wrapping_add(20), (v_).0); }
        { let v_ = F32(0x3cf8bc1b); c.m.w32(fp.wrapping_add(24), (v_).0); }
        { let v_ = F32(0x3c8ab7ad); c.m.w32(fp.wrapping_add(28), (v_).0); }
        { let v_ = F32(0x3ca04bb6); c.m.w32(fp.wrapping_add(32), (v_).0); }
        { let v_ = F32(0x3cce1833); c.m.w32(fp.wrapping_add(36), (v_).0); }
        l_v_2 = { let a_ = (fp).wrapping_add((((F::BtlParam_GetType(c, p_chr_1)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
        if ((F::BtlMember_HasAbility(c, p_chr_1, 107i32)) != 0) {
            { let r_ = l_v_2; let n_ = rt::fadd(l_v_2, r_); l_v_2 = n_; };
        }
        break 'f l_v_2;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetTypeValueA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetTypeValueB(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetTypeValueB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_v_2: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 40);
        { let v_ = F32(0x41de38e2); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x4201a12e); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x43392f67); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x438ae38c); c.m.w32(fp.wrapping_add(12), (v_).0); }
        { let v_ = F32(0x43b92f67); c.m.w32(fp.wrapping_add(16), (v_).0); }
        { let v_ = F32(0x41de38e2); c.m.w32(fp.wrapping_add(20), (v_).0); }
        { let v_ = F32(0x421425eb); c.m.w32(fp.wrapping_add(24), (v_).0); }
        { let v_ = F32(0x419425eb); c.m.w32(fp.wrapping_add(28), (v_).0); }
        { let v_ = F32(0x41b92f67); c.m.w32(fp.wrapping_add(32), (v_).0); }
        { let v_ = F32(0x41de38e2); c.m.w32(fp.wrapping_add(36), (v_).0); }
        l_v_2 = { let a_ = (fp).wrapping_add((((F::BtlParam_GetType(c, p_chr_1)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
        if ((F::BtlMember_HasAbility(c, p_chr_1, 107i32)) != 0) {
            { let r_ = l_v_2; let n_ = rt::fadd(l_v_2, r_); l_v_2 = n_; };
        }
        break 'f l_v_2;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetTypeValueB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetTypeValueC(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetTypeValueC", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 40);
        { let v_ = F32(0x3e199999); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3e4ccccc); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3ecccccc); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3ee66666); c.m.w32(fp.wrapping_add(12), (v_).0); }
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(16), (v_).0); }
        { let v_ = F32(0x3e4ccccc); c.m.w32(fp.wrapping_add(20), (v_).0); }
        { let v_ = F32(0x3eb33333); c.m.w32(fp.wrapping_add(24), (v_).0); }
        { let v_ = F32(0x3dcccccc); c.m.w32(fp.wrapping_add(28), (v_).0); }
        { let v_ = F32(0x3e199999); c.m.w32(fp.wrapping_add(32), (v_).0); }
        { let v_ = F32(0x3e4ccccc); c.m.w32(fp.wrapping_add(36), (v_).0); }
        break 'f { let a_ = (fp).wrapping_add((((F::BtlParam_GetType(c, p_chr_1)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetTypeValueC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_IsType2to4(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_IsType2to4", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = F::BtlParam_GetType(c, p_chr_1);
        match sv2 {
            2i32 | 3i32 | 4i32 => {
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_IsType2to4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_IsTypeSetA(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_IsTypeSetA", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = F::BtlParam_GetType(c, p_chr_1);
        match sv2 {
            2i32 | 3i32 | 4i32 | 6i32 | 9i32 => {
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_IsTypeSetA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_IsTypeSetB(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_IsTypeSetB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = F::BtlParam_GetType(c, p_chr_1);
        match sv2 {
            0i32 | 1i32 | 2i32 | 3i32 | 4i32 | 6i32 | 9i32 => {
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_IsTypeSetB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetCount80(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetCount80", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_param_2: u32 = 0;
    let mut l_v_3: i32 = 0;
    let r_ = 'f: {
    l_param_2 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) };
    l_v_3 = (((c.m.r16((l_param_2).wrapping_add(128)) as i16)) as i32);
    if ((F::BtlMember_HasAbility(c, p_chr_1, 17i32)) != 0) {
        { let r_ = 3i32; let n_ = (l_v_3).wrapping_sub(r_); l_v_3 = n_; };
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 16i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_v_3).wrapping_sub(r_); l_v_3 = n_; };
        } else {
            if ((F::BtlMember_HasAbility(c, p_chr_1, 15i32)) != 0) {
                { let r_ = 1i32; let n_ = (l_v_3).wrapping_sub(r_); l_v_3 = n_; };
            }
        }
    }
    if ((((((((c.m.r16((l_param_2).wrapping_add(128)) as i16)) as i32)) > (0i32)) as i32)) != 0) {
        if (((((l_v_3) <= (0i32)) as i32)) != 0) {
            l_v_3 = 1i32;
        }
    }
    break 'f l_v_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetCount80", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk82(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk82", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(130); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk82", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk84(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk84", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_param_3: u32 = 0;
    let r_ = 'f: {
    l_param_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) };
    if (((((p_n_2) >= (((4i32) as u32))) as i32)) != 0) {
        p_n_2 = ((0i32) as u32);
    }
    break 'f ((c.m.r8(((l_param_3).wrapping_add(132)).wrapping_add(p_n_2))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk84", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk88(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk88", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_param_3: u32 = 0;
    let r_ = 'f: {
    l_param_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) };
    if (((((p_n_2) >= (((2i32) as u32))) as i32)) != 0) {
        p_n_2 = ((0i32) as u32);
    }
    break 'f ((c.m.r8(((l_param_3).wrapping_add(136)).wrapping_add(p_n_2))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk88", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk8A(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk8A", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_param_3: u32 = 0;
    let r_ = 'f: {
    l_param_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) };
    if (((((p_n_2) >= (((5i32) as u32))) as i32)) != 0) {
        p_n_2 = ((0i32) as u32);
    }
    break 'f ((c.m.r8(((l_param_3).wrapping_add(138)).wrapping_add(p_n_2))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk8A", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk1C(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk1C", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(28); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk1C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk20(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk20", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(32); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk20", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk24(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk24", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(36); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk24", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetKiRegenLimit(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetKiRegenLimit", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(40); (c.m.r32(a_) as i32) };
    if ((F::BtlMember_HasAbility(c, p_chr_1, 46i32)) != 0) {
        { let r_ = 60000i32; let n_ = (l_v_2).wrapping_add(r_); l_v_2 = n_; };
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 45i32)) != 0) {
            { let r_ = 40000i32; let n_ = (l_v_2).wrapping_add(r_); l_v_2 = n_; };
        } else {
            if ((F::BtlMember_HasAbility(c, p_chr_1, 44i32)) != 0) {
                { let r_ = 20000i32; let n_ = (l_v_2).wrapping_add(r_); l_v_2 = n_; };
            }
        }
    }
    if (((((l_v_2) > (100000i32)) as i32)) != 0) {
        l_v_2 = 100000i32;
    }
    break 'f l_v_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetKiRegenLimit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetGaugeB(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetGaugeB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(44); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetGaugeB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetUnk40(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk40", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(64); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetUnk40", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetRateA(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetRateA", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = rt::div_i32({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(48); (c.m.r32(a_) as i32) }, 30i32);
    { let r_ = F::BtlStat_GetKiChargeBonus(c, p_chr_1); let n_ = (l_v_2).wrapping_add(r_); l_v_2 = n_; };
    break 'f { let a0_ = { let a1_ = (l_v_2).wrapping_add(rt::div_i32((c.m.r32((p_chr_1).wrapping_add(3624)) as i32), 30i32)); F::BtlParam_ScaleKiCharge(c, p_chr_1, a1_) }; F::BtlUtil_Max(c, a0_, 200i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetRateA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetRateB(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetRateB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = rt::div_i32({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(52); (c.m.r32(a_) as i32) }, 30i32);
    { let r_ = F::BtlStat_GetKiChargeBonus(c, p_chr_1); let n_ = (l_v_2).wrapping_add(r_); l_v_2 = n_; };
    break 'f { let a0_ = { let a1_ = (l_v_2).wrapping_add(rt::div_i32((c.m.r32((p_chr_1).wrapping_add(3624)) as i32), 30i32)); F::BtlParam_ScaleKiCharge(c, p_chr_1, a1_) }; F::BtlUtil_Max(c, a0_, 200i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetRateB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetKiRegenRate(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetKiRegenRate", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = rt::div_i32({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(56); (c.m.r32(a_) as i32) }, 30i32);
    break 'f { let a0_ = (l_v_2).wrapping_add(F::BtlStat_GetKiRegenBonus(c, p_chr_1)); F::BtlUtil_Max(c, a0_, 0i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetKiRegenRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetKiRecoverRate(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetKiRecoverRate", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = rt::div_i32({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }, 30i32);
    { let r_ = F::BtlStat_GetKiRecoverBonus(c, p_chr_1); let n_ = (l_v_2).wrapping_add(r_); l_v_2 = n_; };
    if ((F::BtlMember_HasAbility(c, p_chr_1, 60i32)) != 0) {
        { let r_ = 2i32; let n_ = (l_v_2).wrapping_mul(r_); l_v_2 = n_; };
    }
    break 'f F::BtlUtil_Max(c, l_v_2, 10i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetKiRecoverRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetKiDrain67(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetKiDrain67", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = rt::div_i32({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }, 30i32);
    if ((F::BtlMember_HasAbility(c, p_chr_1, 40i32)) != 0) {
        { let r_ = 2i32; let n_ = rt::div_i32(l_v_2, r_); l_v_2 = n_; };
    }
    break 'f l_v_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetKiDrain67", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetRecoverKiCost(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetRecoverKiCost", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(72); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetRecoverKiCost", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetAmountA(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetAmountA", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = 10000i32;
    if ((F::BtlMember_HasAbility(c, p_chr_1, 36i32)) != 0) {
        l_v_2 = 0i32;
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 35i32)) != 0) {
            l_v_2 = 5000i32;
        }
    }
    break 'f l_v_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetAmountA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetAmountB(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetAmountB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = 20000i32;
    if ((F::BtlMember_HasAbility(c, p_chr_1, 36i32)) != 0) {
        l_v_2 = 0i32;
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 35i32)) != 0) {
            l_v_2 = 10000i32;
        }
    }
    break 'f l_v_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetAmountB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetBlastGainRate(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetBlastGainRate", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = rt::div_i32({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(68); (c.m.r32(a_) as i32) }, 30i32);
    { let r_ = F::BtlStat_GetBlastGainBonus(c, p_chr_1); let n_ = (l_v_2).wrapping_add(r_); l_v_2 = n_; };
    if ((F::BtlMember_HasAbility(c, p_chr_1, 50i32)) != 0) {
        l_v_2 = rt::div_i32((l_v_2).wrapping_mul(12i32), 10i32);
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 26i32)) != 0) {
        { let r_ = 2i32; let n_ = rt::div_i32(l_v_2, r_); l_v_2 = n_; };
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 51i32)) != 0) {
        l_v_2 = rt::div_i32((l_v_2).wrapping_mul(8i32), 10i32);
    }
    break 'f F::BtlUtil_Max(c, l_v_2, 20i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetBlastGainRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetStepA(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetStepA", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_t_2: F32 = F32(0);
    let r_ = 'f: {
    l_t_2 = { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(80); F32(c.m.r32(a_)) };
    { let r_ = F::BtlStat_GetMaxPowerChargeScale(c, p_chr_1); let n_ = rt::fdiv(l_t_2, r_); l_t_2 = n_; };
    break 'f rt::div_i32(30000i32, rt::f2i(rt::fmul(l_t_2, F32(0x41f00000))));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetStepA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlParam_GetMaxPowerDrain(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetMaxPowerDrain", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_t_2: F32 = F32(0);
    let r_ = 'f: {
    l_t_2 = { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(84); F32(c.m.r32(a_)) };
    { let r_ = F::BtlStat_GetMaxPowerExtraTime(c, p_chr_1); let n_ = rt::fadd(l_t_2, r_); l_t_2 = n_; };
    if (((rt::flt(l_t_2, F32(0x3d088888)) as i32)) != 0) {
        l_t_2 = F32(0x3d088888);
    }
    break 'f rt::div_i32(30000i32, rt::f2i(rt::fmul(l_t_2, F32(0x41f00000))));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlParam_GetMaxPowerDrain", &PROBE_N, &[], true); }
    r_
}
