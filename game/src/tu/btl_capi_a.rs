#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlCharApi_GetChara(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetChara", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_obj_2).wrapping_add(12)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetChara", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetHeight(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetHeight", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_obj_2).wrapping_add(4084)));
    }
    break 'f F32(0x41200000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetHeight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBodyUnkFF8(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBodyUnkFF8", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_obj_2).wrapping_add(4088)));
    }
    break 'f F32(0x41200000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBodyUnkFF8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetCenterHeight(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCenterHeight", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_obj_2).wrapping_add(4092)));
    }
    break 'f F32(0x40a00000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCenterHeight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBodyUnk1000(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBodyUnk1000", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_obj_2).wrapping_add(4096)));
    }
    break 'f F32(0x40a00000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBodyUnk1000", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjGetParamFlags0(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetParamFlags0", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (({ let a_ = c.m.r32((l_obj_2).wrapping_add(2332)); c.m.r16(a_) }) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetParamFlags0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjGetParamFlags(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetParamFlags", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32((l_obj_2).wrapping_add(2332))).wrapping_add(16); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetParamFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetAuraType(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAuraType", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((({ let a_ = (F::BtlChar_GetObj(c, l_chr_2)).wrapping_add(2624); (c.m.r32(a_) as i32) }) & (262144i32))) != 0) {
            break 'f 3i32;
        }
        break 'f F::BtlParam_GetAuraKind(c, l_chr_2);
    }
    l_obj_3 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32((l_obj_3).wrapping_add(2332))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((c.m.r32((l_obj_3).wrapping_add(2624)) as i32)) & (262144i32))) != 0) {
            break 'f 3i32;
        }
        break 'f (({ let a_ = (c.m.r32((l_obj_3).wrapping_add(2332))).wrapping_add(3); (c.m.r8(a_) as i8) }) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAuraType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamUnkC8(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamUnkC8", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_obj_2).wrapping_add(2332))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32((l_obj_2).wrapping_add(2332))).wrapping_add(200); F32(c.m.r32(a_)) };
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamUnkC8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetCostume(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCostume", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActive(c, l_chr_2)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCostume", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_HasKiBlastType2(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasKiBlastType2", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_e_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_e_3 = c.m.r32((l_obj_2).wrapping_add(2340));
    if (((((l_e_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (13i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((c.m.r8(((l_e_3).wrapping_add((((l_i_4) as u32)).wrapping_mul(52))).wrapping_add(2)) as i8)) as i32)) == (2i32)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasKiBlastType2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_HasKiBlastType3(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasKiBlastType3", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_e_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_e_3 = c.m.r32((l_obj_2).wrapping_add(2340));
    if (((((l_e_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (13i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((c.m.r8(((l_e_3).wrapping_add((((l_i_4) as u32)).wrapping_mul(52))).wrapping_add(2)) as i8)) as i32)) == (3i32)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasKiBlastType3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsFighter(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsFighter", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::BtlChar_FindByObjId(c, p_objId_1)) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsFighter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetObjIdOfPlayer(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetObjIdOfPlayer", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(12)) as i32);
    }
    break 'f p_player_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetObjIdOfPlayer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPlayer(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayer", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32(l_chr_2) as i32);
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetOpponentObjId(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetOpponentObjId", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlOpp_GetObjId(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetOpponentObjId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPartnerObjId(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPartnerObjId", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f p_objId_1;
    }
    if ((((((c.m.r32((l_chr_2).wrapping_add(4912)) as i32)) == (0i32)) as i32)) != 0) {
        break 'f p_objId_1;
    }
    break 'f (c.m.r32((l_chr_2).wrapping_add(4920)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPartnerObjId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPlayerObjUnk9C(c: &mut Ctx, mut p_player_1: i32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayerObjUnk9C", &PROBE_N, &[(p_player_1) as u32, (p_n_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        l_obj_4 = F::BtlChar_GetObj(c, l_chr_3);
        if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
            if (((((p_n_2) >= (((5i32) as u32))) as i32)) != 0) {
                break 'f 0i32;
            }
            break 'f (c.m.r32(((l_obj_4).wrapping_add(156)).wrapping_add((p_n_2).wrapping_mul(4))) as i32);
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayerObjUnk9C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPlayerSuperData(c: &mut Ctx, mut p_player_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayerSuperData", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, l_chr_2);
        if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
            break 'f c.m.r32((l_obj_3).wrapping_add(2348));
        }
        break 'f ((0i32) as u32);
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayerSuperData", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPlayerSkillData(c: &mut Ctx, mut p_player_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayerSkillData", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, l_chr_2);
        if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
            break 'f c.m.r32((l_obj_3).wrapping_add(2352));
        }
        break 'f ((0i32) as u32);
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayerSkillData", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPlayerObjUnk58(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayerObjUnk58", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, l_chr_2);
        if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
            break 'f (c.m.r32((l_obj_3).wrapping_add(88)) as i32);
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPlayerObjUnk58", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPos(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPos", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = F::BtlChar_GetPos(c, l_chr_3); let a2_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(32); F::Vec4_Add(c, p_out_2, a1_, a2_) };
        break 'f;
    }
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_2, (l_obj_4).wrapping_add(2416));
        break 'f;
    }
    F::Vec4_SetZeroW1(c, p_out_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBasePos(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBasePos", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = F::BtlChar_GetPos(c, l_chr_3); F::Vec4_Copy(c, p_out_2, a1_) };
        break 'f;
    }
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_2, (l_obj_4).wrapping_add(2416));
        break 'f;
    }
    F::Vec4_SetZeroW1(c, p_out_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBasePos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetRot(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetRot", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(16); F::Vec4_Copy(c, p_out_2, a1_) };
        break 'f;
    }
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_2, (l_obj_4).wrapping_add(2400));
        break 'f;
    }
    F::Vec4_SetZero(c, p_out_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetModelRot(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetModelRot", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_chr_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let mut l_node_5: u32 = 0;
    let r_ = 'f: {
        l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
        if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(16); F::Vec4_Copy(c, p_out_2, a1_) };
            { let a2_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(96); F::Vec4_Add(c, p_out_2, p_out_2, a2_) };
            F::BtlUtil_WrapAngles(c, p_out_2, p_out_2);
        } else {
            l_obj_4 = F::BtlObj_Get(c, p_objId_1);
            if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
                F::Vec4_Copy(c, p_out_2, (l_obj_4).wrapping_add(2400));
                l_node_5 = F::BtlObj_GetNode(c, l_obj_4, 0i32);
                if (((((l_node_5) != (((0i32) as u32))) as i32)) != 0) {
                    F::Vec4_Copy(c, fp, (l_node_5).wrapping_add(160));
                    F::Quat_ToEuler(c, fp.wrapping_add(16), fp);
                    F::Vec4_Add(c, p_out_2, p_out_2, fp.wrapping_add(16));
                    F::BtlUtil_WrapAngles(c, p_out_2, p_out_2);
                }
            } else {
                F::Vec4_SetZero(c, p_out_2);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetModelRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetYaw(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetYaw", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlChar_GetPos(c, l_chr_2)).wrapping_add(148); F32(c.m.r32(a_)) };
    }
    l_obj_3 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32(((l_obj_3).wrapping_add(2400)).wrapping_add(4)));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetVelocity(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetVelocity", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(48); F::Vec4_Copy(c, p_out_2, a1_) };
        break 'f;
    }
    F::Vec4_SetZero(c, p_out_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetVelocity", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetFrameMove(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetFrameMove", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(64); F::Vec4_Copy(c, p_out_2, a1_) };
        break 'f;
    }
    F::Vec4_SetZero(c, p_out_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetFrameMove", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetDir(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetDir", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(128); F::Vec4_Copy(c, p_out_2, a1_) };
        break 'f;
    }
    F::Vec4_SetZero(c, p_out_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetSpeed(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSpeed", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlChar_GetPos(c, l_chr_2)).wrapping_add(152); F32(c.m.r32(a_)) };
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetFallSpeed(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetFallSpeed", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlChar_GetPos(c, l_chr_2)).wrapping_add(156); F32(c.m.r32(a_)) };
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetFallSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetGroundY(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetGroundY", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = ((F::BtlChar_GetPos(c, l_chr_2)).wrapping_add(176)).wrapping_add(4); F32(c.m.r32(a_)) };
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetGroundY", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetAltitude(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAltitude", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::BtlCharApi_GetPos(c, p_objId_1, fp);
        break 'f rt::fsub(F::BtlCharApi_GetGroundY(c, p_objId_1), F32(c.m.r32((fp).wrapping_add(4))));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAltitude", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetNodePos(c: &mut Ctx, mut p_objId_1: i32, mut p_node_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodePos", &PROBE_N, &[(p_objId_1) as u32, (p_node_2) as u32, (p_out_3) as u32], false); }
    let mut l_obj_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        l_n_5 = F::BtlObj_GetNode(c, l_obj_4, p_node_2);
        if (((((l_n_5) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_Copy(c, p_out_3, ((l_n_5).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
            break 'f;
        }
        F::Vec4_Copy(c, p_out_3, (l_obj_4).wrapping_add(2416));
    }
    if (((((l_obj_4) == (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_SetZeroW1(c, p_out_3);
        break 'f;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodePos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetNodeUnk90(c: &mut Ctx, mut p_objId_1: i32, mut p_node_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeUnk90", &PROBE_N, &[(p_objId_1) as u32, (p_node_2) as u32, (p_out_3) as u32], false); }
    let mut l_obj_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        l_n_5 = F::BtlObj_GetNode(c, l_obj_4, p_node_2);
        if (((((l_n_5) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_Copy(c, p_out_3, (l_n_5).wrapping_add(144));
            break 'f;
        }
        F::Vec4_Copy(c, p_out_3, (l_obj_4).wrapping_add(2416));
    }
    if (((((l_obj_4) == (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_SetZeroW1(c, p_out_3);
        break 'f;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeUnk90", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetNodeMtx(c: &mut Ctx, mut p_objId_1: i32, mut p_node_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeMtx", &PROBE_N, &[(p_objId_1) as u32, (p_node_2) as u32, (p_out_3) as u32], false); }
    let mut l_obj_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        l_n_5 = F::BtlObj_GetNode(c, l_obj_4, p_node_2);
        if (((((l_n_5) != (((0i32) as u32))) as i32)) != 0) {
            F::Mtx_Copy(c, p_out_3, (l_n_5).wrapping_add(16));
            break 'f;
        }
        F::Mtx_StoreIdentity(c, p_out_3);
    }
    if (((((l_obj_4) == (((0i32) as u32))) as i32)) != 0) {
        F::Mtx_StoreIdentity(c, p_out_3);
        break 'f;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeMtx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetNodeMtx50(c: &mut Ctx, mut p_objId_1: i32, mut p_node_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeMtx50", &PROBE_N, &[(p_objId_1) as u32, (p_node_2) as u32, (p_out_3) as u32], false); }
    let mut l_obj_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        l_n_5 = F::BtlObj_GetNode(c, l_obj_4, p_node_2);
        if (((((l_n_5) != (((0i32) as u32))) as i32)) != 0) {
            F::Mtx_Copy(c, p_out_3, (l_n_5).wrapping_add(80));
            break 'f;
        }
        F::Mtx_StoreIdentity(c, p_out_3);
    }
    if (((((l_obj_4) == (((0i32) as u32))) as i32)) != 0) {
        F::Mtx_StoreIdentity(c, p_out_3);
        break 'f;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeMtx50", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetNodeRot(c: &mut Ctx, mut p_objId_1: i32, mut p_node_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeRot", &PROBE_N, &[(p_objId_1) as u32, (p_node_2) as u32, (p_out_3) as u32], false); }
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlObj_GetNodeVelocity(c, l_obj_4, p_node_2, p_out_3);
        break 'f;
    }
    F::Vec4_SetZero(c, p_out_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetNodeQuat(c: &mut Ctx, mut p_objId_1: i32, mut p_node_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeQuat", &PROBE_N, &[(p_objId_1) as u32, (p_node_2) as u32, (p_out_3) as u32], false); }
    let mut l_obj_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        l_n_5 = F::BtlObj_GetNode(c, l_obj_4, p_node_2);
        if (((((l_n_5) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_Copy(c, p_out_3, (l_n_5).wrapping_add(160));
            break 'f;
        }
        F::Quat_SetIdentity(c, p_out_3);
    }
    if (((((l_obj_4) == (((0i32) as u32))) as i32)) != 0) {
        F::Quat_SetIdentity(c, p_out_3);
        break 'f;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetNodeQuat", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPartUnk5C(c: &mut Ctx, mut p_objId_1: i32, mut p_n_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPartUnk5C", &PROBE_N, &[(p_objId_1) as u32, (p_n_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
        l_part_4 = F::BtlObj_FindBound(c, l_obj_3, p_n_2);
        if (((((l_part_4) != (((0i32) as u32))) as i32)) != 0) {
            break 'f F32(c.m.r32((l_part_4).wrapping_add(92)));
        }
        break 'f F32(0x3f800000);
    }
    break 'f F32(0x3f800000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPartUnk5C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetCloseRange(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCloseRange", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f F::BtlChar_GetSpacing(c, l_chr_2, 0i32);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCloseRange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjExistsF(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjExistsF", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    if (((((F::BtlObj_Get(c, p_objId_1)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f F32(0x3f800000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjExistsF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsClose(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsClose", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((19i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsClose", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsSightBlocked(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsSightBlocked", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((186i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsSightBlocked", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsOnGround(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsOnGround", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((15i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsOnGround", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsInWater(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInWater", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((17i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInWater", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsHidden(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsHidden", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((!((((((c.m.r32((l_obj_2).wrapping_add(2624)) as i32)).wrapping_shr((1i32) as u32)) & (1i32))) != 0)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsHidden", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsFrozen(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsFrozen", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_IsFrozen(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsFrozen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_AnyFrozen(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_AnyFrozen", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) != (((0i32) as u32))) as i32)) != 0) {
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (({ let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_IsFrozen(c, a0_) }) != 0) {
                        break 'f 1i32;
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_AnyFrozen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestStageBreakA(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestStageBreakA", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((((((((F::BtlParam_GetFlags(c, l_chr_2)) as u64)) as i32)) & (2048i32))) != 0) && ((((!((F::BtlChar_TestFlag(c, l_chr_2, ((((10i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((9i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestStageBreakA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestStageBreakB(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestStageBreakB", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, l_chr_2, ((((33i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if ((F::BtlChar_TestFlag(c, l_chr_2, ((((34i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((10i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestStageBreakB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetChargeRate(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetChargeRate", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_t_3: F32 = F32(0);
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    l_t_3 = F32(0x00000000);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    if (((rt::fgt(F32(c.m.r32((l_chr_2).wrapping_add(3448))), l_t_3) as i32)) != 0) {
        l_t_3 = F32(c.m.r32((l_chr_2).wrapping_add(3448)));
    }
    if (((rt::fgt(F32(c.m.r32((l_chr_2).wrapping_add(3564))), l_t_3) as i32)) != 0) {
        l_t_3 = F32(c.m.r32((l_chr_2).wrapping_add(3564)));
    }
    break 'f l_t_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetChargeRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetActionFxKind(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetActionFxKind", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        's1: {
            let sv3: i32 = F::BtlAct_GetCurrent(c, l_chr_2);
            match sv3 {
                187i32 => {
                    break 'f 4i32;
                }
                23i32 => {
                    break 'f 3i32;
                }
                24i32 => {
                    break 'f 4i32;
                }
                243i32 => {
                    break 'f 3i32;
                }
                301i32 | 302i32 | 303i32 => {
                    break 'f 5i32;
                }
                _ => {
                    break 'f 1i32;
                }
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetActionFxKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetClashBias(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetClashBias", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(92); F32(c.m.r32(a_)) };
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetClashBias", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBlindRatio(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlindRatio", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f { let a0_ = rt::fdiv(rt::i2f((c.m.r32((l_chr_2).wrapping_add(4092)) as i32)), F32(0x42b40000)); F::BtlUtil_ClampF(c, a0_, F32(0x00000000), F32(0x3f800000)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlindRatio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetRushFinishPhase(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetRushFinishPhase", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_level_2: i32 = 0;
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_level_2 = (1i32).wrapping_neg();
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if ((((!((F::BtlCharApi_IsInRushSequence(c, p_objId_1)) != 0)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if ((((((c.m.r32(l_chr_3) as i32)) == ((c.m.r32((l_chr_3).wrapping_add(3732)) as i32))) as i32)) != 0) {
        if ((((((c.m.r32((l_chr_3).wrapping_add(3800)) as i32)) != (0i32)) as i32)) != 0) {
            l_level_2 = ((c.m.r32((l_chr_3).wrapping_add(3744)) as i32)).wrapping_sub(1i32);
        }
    } else {
        if ((((((c.m.r32((l_chr_3).wrapping_add(3804)) as i32)) != (0i32)) as i32)) != 0) {
            l_level_2 = (c.m.r32((l_chr_3).wrapping_add(3756)) as i32);
        }
    }
    if (((((l_level_2) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (if ((((({ let a1_ = F::BtlAnim_GetId(c, l_chr_3); F::BtlAct_GetMotionLevel(c, l_chr_3, a1_) }) >= (l_level_2)) as i32)) != 0) { 1i32 } else { 2i32 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetRushFinishPhase", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjPushYaw(c: &mut Ctx, mut p_objId_1: i32, mut p_yaw_2: F32, mut p_len_3: F32, mut p_arg_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjPushYaw", &PROBE_N, &[(p_objId_1) as u32, (p_yaw_2).0, (p_len_3).0, (p_arg_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_obj_5: u32 = 0;
    let r_ = 'f: {
        l_obj_5 = F::BtlObj_Get(c, p_objId_1);
        if (((((l_obj_5) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = rt::fmul(F::Mathf_Sin(c, p_yaw_2), p_len_3); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F::Mathf_Cos(c, p_yaw_2), p_len_3); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::BtlObj_AddPush(c, l_obj_5, fp, p_arg_4);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjPushYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjPushDir(c: &mut Ctx, mut p_objId_1: i32, mut p_dir_2: u32, mut p_len_3: F32, mut p_arg_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjPushDir", &PROBE_N, &[(p_objId_1) as u32, (p_dir_2) as u32, (p_len_3).0, (p_arg_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_obj_5: u32 = 0;
    let mut l_l_6: F32 = F32(0);
    let r_ = 'f: {
        l_obj_5 = F::BtlObj_Get(c, p_objId_1);
        if (((((l_obj_5) != (((0i32) as u32))) as i32)) != 0) {
            l_l_6 = F::Vec3_Length(c, p_dir_2);
            if (((rt::flt(l_l_6, F32(0x38d1b717)) as i32)) != 0) {
                break 'f;
            }
            F::Vec4_Scale(c, fp, p_dir_2, rt::fdiv(F32(0x3f800000), l_l_6));
            F::Vec4_Scale(c, fp, fp, p_len_3);
            F::BtlObj_AddPush(c, l_obj_5, fp, p_arg_4);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjPushDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetRadius(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetRadius", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_r_3: F32 = F32(0);
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    l_r_3 = F32(0xbf800000);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32((l_obj_2).wrapping_add(2332))) != (((0i32) as u32))) as i32)) != 0) {
            l_r_3 = { let a_ = (c.m.r32((l_obj_2).wrapping_add(2332))).wrapping_add(12); F32(c.m.r32(a_)) };
        }
        if (((rt::fle(l_r_3, F32(0x00000000)) as i32)) != 0) {
            l_r_3 = rt::fmul(F32(c.m.r32((l_obj_2).wrapping_add(4080))), F32(0x3fe66666));
        }
        break 'f l_r_3;
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetRadius", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBodyPos(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBodyPos", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = c.m.r32((l_obj_3).wrapping_add(4000)); F::Vec4_Copy(c, p_out_2, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBodyPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_CalcAimDir(c: &mut Ctx, mut p_objId_1: i32, mut p_node_2: i32, mut p_from_3: u32, mut p_out_4: u32, mut p_minPitch_5: F32, mut p_maxPitch_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CalcAimDir", &PROBE_N, &[(p_objId_1) as u32, (p_node_2) as u32, (p_from_3) as u32, (p_out_4) as u32, (p_minPitch_5).0, (p_maxPitch_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_chr_7: u32 = 0;
    let mut l_pose_8: u32 = 0;
    let mut l_len_9: F32 = F32(0);
    let mut l_lo_10: F32 = F32(0);
    let mut l_hi_11: F32 = F32(0);
    let mut l_xz_12: F32 = F32(0);
    let mut l_k_13: F32 = F32(0);
    let mut l_xz_14: F32 = F32(0);
    let mut l_s_15: F32 = F32(0);
    let mut l_xz_16: F32 = F32(0);
    let mut l_k_17: F32 = F32(0);
    let mut l_xz_18: F32 = F32(0);
    let mut l_s_19: F32 = F32(0);
    let r_ = 'f: {
        l_chr_7 = F::BtlChar_FindByObjId(c, p_objId_1);
        if (((((l_chr_7) == (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_SetZero(c, p_out_4);
            break 'f;
        }
        l_pose_8 = F::BtlChar_GetPos(c, l_chr_7);
        F::BtlOpp_GetNodePos(c, l_chr_7, p_node_2, fp);
        F::Vec4_Sub(c, p_out_4, fp, p_from_3);
        if ((F::BtlCharApi_IsInTechnique(c, p_objId_1)) != 0) {
            if (((({ let a1_ = F::BtlAct_GetCurrentClass(c, l_chr_7); ((((F::BtlSuper_GetFlags(c, l_chr_7, a1_)) as u64)) as i32) }) & (262144i32))) != 0) {
                l_len_9 = F::BtlUtil_LengthXZ(c, p_out_4);
                { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_pose_8).wrapping_add(148))); F::Mathf_Sin(c, a0_) }, l_len_9); let a_ = p_out_4; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_pose_8).wrapping_add(148))); F::Mathf_Cos(c, a0_) }, l_len_9); let a_ = (p_out_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
            }
        }
        l_len_9 = F::Vec3_Length(c, p_out_4);
        if (((rt::flt(l_len_9, F32(0x38d1b717)) as i32)) != 0) {
            { let v_ = { let a0_ = F32(c.m.r32(((l_pose_8).wrapping_add(16)).wrapping_add(4))); F::Mathf_Sin(c, a0_) }; let a_ = p_out_4; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_out_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((l_pose_8).wrapping_add(16)).wrapping_add(4))); F::Mathf_Cos(c, a0_) }; let a_ = (p_out_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_out_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
            break 'f;
        }
        F::Vec4_Scale(c, p_out_4, p_out_4, rt::fdiv(F32(0x3f800000), l_len_9));
        F::BtlOpp_GetDelta(c, l_chr_7, fp.wrapping_add(16));
        if (((rt::flt(rt::fadd(rt::fmul(F32(c.m.r32(p_out_4)), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F32(c.m.r32((p_out_4).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32(p_out_4))); let a_ = p_out_4; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((p_out_4).wrapping_add(8)))); let a_ = (p_out_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
        }
        l_lo_10 = F::Mathf_Sin(c, p_minPitch_5);
        l_hi_11 = F::Mathf_Sin(c, p_maxPitch_6);
        if (((rt::flt(F32(c.m.r32((p_out_4).wrapping_add(4))), l_lo_10) as i32)) != 0) {
            if (((rt::flt({ let a0_ = F32(c.m.r32((p_out_4).wrapping_add(4))); rt::ffabs(a0_) }, F32(0x3f7fbe76)) as i32)) != 0) {
                l_xz_12 = F::Port_sqrtf(c, rt::fsub(F32(0x3f800000), rt::fmul(l_lo_10, l_lo_10)));
                l_k_13 = rt::fdiv(l_xz_12, { let a0_ = rt::fsub(F32(0x3f800000), rt::fmul(F32(c.m.r32((p_out_4).wrapping_add(4))), F32(c.m.r32((p_out_4).wrapping_add(4))))); F::Port_sqrtf(c, a0_) });
                { let v_ = l_lo_10; let a_ = (p_out_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let r_ = l_k_13; let a_ = p_out_4; let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = l_k_13; let a_ = (p_out_4).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            } else {
                l_xz_14 = F::Port_sqrtf(c, rt::fsub(F32(0x3f800000), rt::fmul(l_lo_10, l_lo_10)));
                l_s_15 = { let a0_ = F32(c.m.r32(((l_pose_8).wrapping_add(16)).wrapping_add(4))); F::Mathf_Sin(c, a0_) };
                { let v_ = l_lo_10; let a_ = (p_out_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(l_xz_14, l_s_15); let a_ = p_out_4; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(l_xz_14, { let a0_ = F32(c.m.r32(((l_pose_8).wrapping_add(16)).wrapping_add(4))); F::Mathf_Cos(c, a0_) }); let a_ = (p_out_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
            }
        }
        if (((rt::flt(l_hi_11, F32(c.m.r32((p_out_4).wrapping_add(4)))) as i32)) != 0) {
            if (((rt::flt({ let a0_ = F32(c.m.r32((p_out_4).wrapping_add(4))); rt::ffabs(a0_) }, F32(0x3f7fbe76)) as i32)) != 0) {
                l_xz_16 = F::Port_sqrtf(c, rt::fsub(F32(0x3f800000), rt::fmul(l_hi_11, l_hi_11)));
                l_k_17 = rt::fdiv(l_xz_16, { let a0_ = rt::fsub(F32(0x3f800000), rt::fmul(F32(c.m.r32((p_out_4).wrapping_add(4))), F32(c.m.r32((p_out_4).wrapping_add(4))))); F::Port_sqrtf(c, a0_) });
                { let v_ = l_hi_11; let a_ = (p_out_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let r_ = l_k_17; let a_ = p_out_4; let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = l_k_17; let a_ = (p_out_4).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            } else {
                l_xz_18 = F::Port_sqrtf(c, rt::fsub(F32(0x3f800000), rt::fmul(l_hi_11, l_hi_11)));
                l_s_19 = { let a0_ = F32(c.m.r32(((l_pose_8).wrapping_add(16)).wrapping_add(4))); F::Mathf_Sin(c, a0_) };
                { let v_ = l_hi_11; let a_ = (p_out_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(l_xz_18, l_s_19); let a_ = p_out_4; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(l_xz_18, { let a0_ = F32(c.m.r32(((l_pose_8).wrapping_add(16)).wrapping_add(4))); F::Mathf_Cos(c, a0_) }); let a_ = (p_out_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
            }
        }
        { let v_ = F32(0x00000000); let a_ = (p_out_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CalcAimDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_CalcAimDir45(c: &mut Ctx, mut p_objId_1: i32, mut p_from_2: u32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CalcAimDir45", &PROBE_N, &[(p_objId_1) as u32, (p_from_2) as u32, (p_out_3) as u32], false); }
    let r_ = 'f: {
    F::BtlCharApi_CalcAimDir(c, p_objId_1, 17i32, p_from_2, p_out_3, F32(0xbf490fda), F32(0x3f490fda));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CalcAimDir45", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetDeflectDir(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetDeflectDir", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_chr_3: u32 = 0;
    let mut l_c_4: F32 = F32(0);
    let r_ = 'f: {
        l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
        if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_SetZero(c, p_out_2);
            break 'f;
        }
        { let v_ = { let a0_ = { let a_ = ((F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::Mathf_Sin(c, a0_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_c_4 = { let a0_ = { let a_ = ((F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::Mathf_Cos(c, a0_) };
        { let v_ = l_c_4; let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(32)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Copy(c, fp, fp.wrapping_add(32));
        's1: {
            let sv5: i32 = F::BtlAnim_GetId(c, l_chr_3);
            match sv5 {
                249i32 => {
                    { let a3_ = rt::fadd(rt::fmul(F::BtlChar_RandF(c), F32(0x3e999999)), F32(0x3e999999)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(16), a3_) };
                    { let a3_ = rt::fneg(rt::fadd(rt::fmul(F::BtlChar_RandF(c), F32(0x3f4ccccc)), F32(0x3f000000))); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(32), a3_) };
                    break 's1;
                }
                250i32 => {
                    { let a3_ = rt::fadd(rt::fmul(F::BtlChar_RandF(c), F32(0x3e999999)), F32(0x3e999999)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(16), a3_) };
                    { let a3_ = rt::fadd(rt::fmul(F::BtlChar_RandF(c), F32(0x3f4ccccc)), F32(0x3f000000)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(32), a3_) };
                    break 's1;
                }
                25i32 | 251i32 => {
                    { let a3_ = rt::fadd(rt::fmul(F::BtlChar_RandF(c), F32(0x3f000000)), F32(0x40200000)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(16), a3_) };
                    { let a3_ = rt::fsub(rt::fmul(F::BtlChar_RandF(c), F32(0x3fe66666)), F32(0x3f666666)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(32), a3_) };
                    break 's1;
                }
                252i32 => {
                    { let a3_ = rt::fadd(rt::fmul(F::BtlChar_RandF(c), F32(0x3e999999)), F32(0x3e999999)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(16), a3_) };
                    { let a3_ = rt::fsub(rt::fmul(F::BtlChar_RandF(c), F32(0x3f999999)), F32(0x3f199999)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(32), a3_) };
                    break 's1;
                }
                _ => {
                    F::BtlOpp_GetDelta(c, l_chr_3, fp.wrapping_add(32));
                    F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
                    { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(32)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    F::Vec4_Copy(c, fp, fp.wrapping_add(32));
                    { let a3_ = rt::fsub(rt::fmul(F::BtlChar_RandF(c), F32(0x3f199999)), F32(0x3e999999)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(16), a3_) };
                    { let a3_ = rt::fsub(rt::fmul(F::BtlChar_RandF(c), F32(0x40c90fda)), F32(0x40490fda)); F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(32), a3_) };
                    break 's1;
                }
            }
        }
        F::Vec4_Copy(c, p_out_2, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetDeflectDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsLockedOn(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsLockedOn", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((5i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsLockedOn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsChargingKiBlast(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsChargingKiBlast", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((142i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsChargingKiBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsInTechnique(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInTechnique", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a0_ = F::BtlAct_GetCurrent(c, l_chr_2); F::BtlAct_IsTechniqueId(c, a0_) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInTechnique", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_EnteredTechnique(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_EnteredTechnique", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((({ let a0_ = F::BtlAct_GetCurrent(c, l_chr_2); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) && ((((!(({ let a0_ = F::BtlAct_GetPrev(c, l_chr_2); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_EnteredTechnique", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsRushConnected(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsRushConnected", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((161i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsRushConnected", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsInRushSequence(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInRushSequence", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        's1: {
            let sv3: i32 = F::BtlAct_GetCurrent(c, l_chr_2);
            match sv3 {
                301i32 | 302i32 | 303i32 | 313i32 | 314i32 | 315i32 => {
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
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInRushSequence", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsInSkill(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInSkill", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((((((F::BtlAct_GetCurrent(c, l_chr_2)).wrapping_sub(253i32)) as u32)) < (((6i32) as u32))) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInSkill", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsSkillStart(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsSkillStart", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((F::BtlCharApi_IsInSkill(c, p_objId_1)) != 0) {
            break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((8i32) as i64 as u64)) as u32));
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsSkillStart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsSkillApplied(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsSkillApplied", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((154i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsSkillApplied", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsInClashA(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInClashA", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        's1: {
            let sv3: i32 = F::BtlAct_GetCurrent(c, l_chr_2);
            match sv3 {
                304i32 | 305i32 | 306i32 => {
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
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInClashA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsInClashBC(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInClashBC", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_action_3: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_action_3 = F::BtlAct_GetCurrent(c, l_chr_2);
        if (((((((((l_action_3) == (250i32)) as i32)) != 0) || (((((l_action_3) == (252i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInClashBC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsChargingKi(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsChargingKi", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((12i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsChargingKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsMaxPowerStart(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsMaxPowerStart", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((13i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsMaxPowerStart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsChanging(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsChanging", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        's1: {
            let sv3: i32 = F::BtlAct_GetCurrent(c, l_chr_2);
            match sv3 {
                236i32 | 237i32 | 238i32 | 239i32 | 240i32 | 241i32 | 242i32 | 243i32 | 244i32 | 245i32 | 246i32 | 247i32 | 248i32 | 259i32 | 260i32 => {
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
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsChanging", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestAnimFlag10(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestAnimFlag10", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((({ let a0_ = F::BtlAnim_GetId(c, l_chr_2); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) }).wrapping_shr((4i32) as u32)) & (1i32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestAnimFlag10", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsCircleDash(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsCircleDash", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((F::BtlAct_GetCurrent(c, l_chr_2)) == (51i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsCircleDash", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsModelNew(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsModelNew", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((299i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsModelNew", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_HasWeaponOut(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasWeaponOut", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32((l_obj_2).wrapping_add(2332))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((((({ let a_ = (c.m.r32((l_obj_2).wrapping_add(2332))).wrapping_add(16); (c.m.r32(a_) as i32) }) & (64i32))) != 0) && (((((((c.m.r64((l_obj_2).wrapping_add(2624))) & (((34i32) as u64)))) == (((34i32) as u64))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
        if (((((((({ let a_ = (c.m.r32((l_obj_2).wrapping_add(2332))).wrapping_add(16); (c.m.r32(a_) as i32) }) & (128i32))) != 0) && (((((((c.m.r64((l_obj_2).wrapping_add(2624))) & (((66i32) as u64)))) == (((66i32) as u64))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasWeaponOut", &PROBE_N, &[], true); }
    r_
}
