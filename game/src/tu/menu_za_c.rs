#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn menu_za_c__PassChk_Section(c: &mut Ctx, mut p_pack_1: u32, mut p_n_2: i32, mut p_host_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_c__PassChk_Section", &PROBE_N, &[(p_pack_1) as u32, (p_n_2) as u32, (p_host_3) as u32], false); }
    let r_ = 'f: {
    break 'f (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_za_c__PassChk_Section", &PROBE_N, &[], true); }
    r_
}

pub fn PassChk_Init(c: &mut Ctx, mut p_pack_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassChk_Init", &PROBE_N, &[(p_pack_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((20u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gPassChk; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gPassChk); F::memset(c, a0_, 0i32, 20u32) }; };
    { let v_ = F::menu_za_c__PassChk_Section(c, p_pack_1, 1i32, sym::menu_za_c__str0); let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = F::menu_za_c__PassChk_Section(c, p_pack_1, 2i32, sym::menu_za_c__str1); let a_ = c.m.r32(sym::gPassChk); c.m.w32(a_, v_); };
    { let v_ = F::menu_za_c__PassChk_Section(c, p_pack_1, 3i32, sym::menu_za_c__str2); let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = F::menu_za_c__PassChk_Section(c, p_pack_1, 5i32, sym::menu_za_c__str3); let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = F::menu_za_c__PassChk_Section(c, p_pack_1, 4i32, sym::menu_za_c__str4); let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PassChk_Init", &PROBE_N, &[], true); }
    r_
}

pub fn PassChk_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassChk_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gPassChk)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gPassChk); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gPassChk; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PassChk_Term", &PROBE_N, &[], true); }
    r_
}

pub fn PassChk_ConvertOld(c: &mut Ctx, sret: u32, mut p_old_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassChk_ConvertOld", &PROBE_N, &[(sret) as u32, (p_old_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_charaTbl_2: u32 = 0;
    let mut l_level_3: u32 = 0;
    let mut l_defense_4: u32 = 0;
    let mut l_attack_5: u32 = 0;
    let mut l_chara_6: i32 = 0;
    let mut l_cls_7: i32 = 0;
    let mut l_type_8: i32 = 0;
    let mut l_row_9: u32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_item_11: u32 = 0;
    let r_ = 'f: {
        l_charaTbl_2 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        l_level_3 = ((0i32) as u32);
        l_defense_4 = ((0i32) as u32);
        l_attack_5 = ((0i32) as u32);
        { let _ = F::memset(c, fp, 0i32, 44u32); };
        l_chara_6 = { let a_ = ({ let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((c.m.r32(p_old_1) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        { let v_ = (7i32).wrapping_sub(((c.m.r16(((l_charaTbl_2).wrapping_add((((l_chara_6) as u32)).wrapping_mul(60))).wrapping_add(14))) as i32)); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        {
            l_i_10 = 0i32;
            'l1: loop {
                if !(((((l_i_10) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((((c.m.r32(((p_old_1).wrapping_add(4)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(1i32)) as u32)) < (((265i32) as u32))) as i32)) != 0) {
                        l_item_11 = ({ let a_ = c.m.r32(sym::gPassChk); c.m.r32(a_) }).wrapping_add((((((c.m.r32(((p_old_1).wrapping_add(4)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4));
                        { let r_ = ((c.m.r8((l_item_11).wrapping_add(1))) as u32); let n_ = (l_level_3).wrapping_add(r_); l_level_3 = n_; };
                        { let r_ = ((c.m.r8((l_item_11).wrapping_add(2))) as u32); let n_ = (l_defense_4).wrapping_add(r_); l_defense_4 = n_; };
                        { let r_ = ((c.m.r8((l_item_11).wrapping_add(3))) as u32); let n_ = (l_attack_5).wrapping_add(r_); l_attack_5 = n_; };
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        if (((((l_level_3) == (((0i32) as u32))) as i32)) != 0) {
            l_cls_7 = 0i32;
        } else {
            if (((((((((l_level_3) >= (((1i32) as u32))) as i32)) != 0) && (((((l_level_3) <= (((7i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                l_cls_7 = 1i32;
            } else {
                if (((((((((l_level_3) >= (((8i32) as u32))) as i32)) != 0) && (((((l_level_3) <= (((14i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                    l_cls_7 = 2i32;
                } else {
                    if (((((((((l_level_3) >= (((15i32) as u32))) as i32)) != 0) && (((((l_level_3) <= (((21i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                        l_cls_7 = 3i32;
                    } else {
                        if (((((((((l_level_3) >= (((22i32) as u32))) as i32)) != 0) && (((((l_level_3) <= (((28i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                            l_cls_7 = 4i32;
                        } else {
                            if (((((((((l_level_3) >= (((29i32) as u32))) as i32)) != 0) && (((((l_level_3) <= (((35i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                l_cls_7 = 5i32;
                            } else {
                                l_cls_7 = 0i32;
                            }
                        }
                    }
                }
            }
        }
        l_row_9 = ({ let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_cls_7) as u32)).wrapping_mul(84));
        if (((((l_attack_5) < (l_defense_4)) as i32)) != 0) {
            l_type_8 = 0i32;
        } else {
            if (((((l_defense_4) < (l_attack_5)) as i32)) != 0) {
                l_type_8 = 1i32;
            } else {
                if (((((l_defense_4) == (l_attack_5)) as i32)) != 0) {
                    l_type_8 = 2i32;
                } else {
                    l_type_8 = 0i32;
                }
            }
        }
        { let v_ = l_chara_6; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        {
            l_i_10 = 0i32;
            'l3: loop {
                if !(((((l_i_10) < (7i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((((c.m.r32(((l_row_9).wrapping_add((((l_type_8) as u32)).wrapping_mul(28))).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))) as i32)) != (999i32)) as i32)) != 0) {
                        { let v_ = ((c.m.r32(((l_row_9).wrapping_add((((l_type_8) as u32)).wrapping_mul(28))).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))) as i32)).wrapping_add(1i32); let a_ = ((fp).wrapping_add(4)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 0i32; let a_ = ((fp).wrapping_add(4)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        break 'f { let s_ = fp; rt::memcpy(c, sret, s_, 44); sret };
       sret
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "PassChk_ConvertOld", &PROBE_N, &[], true); }
    r_
}

pub fn PassChk_IsOldValid(c: &mut Ctx, mut p_old_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassChk_IsOldValid", &PROBE_N, &[(p_old_1) as u32], false); }
    let mut l_chara_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_item_4: u32 = 0;
    let r_ = 'f: {
    l_chara_2 = (((c.m.r32(p_old_1) as i32)) as u32);
    if (((((l_chara_2) >= (((202i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((((l_chara_2) == (((130i32) as u32))) as i32)) != 0) || (((((l_chara_2) == (((132i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_chara_2) == (((200i32) as u32))) as i32)) != 0) {
        l_chara_2 = ((130i32) as u32);
    } else {
        if (((((l_chara_2) == (((202i32) as u32))) as i32)) != 0) {
            l_chara_2 = ((132i32) as u32);
        }
    }
    if ((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((l_chara_2).wrapping_mul(4))).wrapping_add(2); c.m.r16(a_) }) as i32)) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((c.m.r32((p_old_1).wrapping_add(32)) as i32)) as u32)) >= (((3i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(((p_old_1).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                    l_item_4 = ((((c.m.r32(((p_old_1).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(1i32)) as u32);
                    if (((((l_item_4) >= (((351i32) as u32))) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if (((((({ let a_ = ({ let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((l_item_4).wrapping_mul(2)); c.m.r8(a_) }) as i32)) & (1i32))) != 0) {
                        break 'f 0i32;
                    }
                    if ((((!(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((l_item_4).wrapping_mul(2)); c.m.r8(a_) }) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if (((((l_item_4) == (((162i32) as u32))) as i32)) != 0) {
                        if (((((((((((((((((((((l_chara_2) >= (((36i32) as u32))) as i32)) != 0) && (((((l_chara_2) <= (((38i32) as u32))) as i32)) != 0)) as i32)) != 0) || (((((l_chara_2) == (((83i32) as u32))) as i32)) != 0)) as i32)) != 0) || (((((l_chara_2) == (((118i32) as u32))) as i32)) != 0)) as i32)) != 0) || (((((l_chara_2) == (((119i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                    }
                    if ((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((l_item_4).wrapping_mul(2))).wrapping_add(1); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                        if ((((!(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((l_chara_2).wrapping_mul(4)); c.m.r16(a_) }) as i32)) & ((1i32).wrapping_shl((((({ let a_ = (({ let a_ = (c.m.r32(sym::gPassChk)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((l_item_4).wrapping_mul(2))).wrapping_add(1); c.m.r8(a_) }) as i32)).wrapping_sub(1i32)) as u32)))) != 0)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PassChk_IsOldValid", &PROBE_N, &[], true); }
    r_
}

pub fn PassChk_IsValid(c: &mut Ctx, mut p_pass_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassChk_IsValid", &PROBE_N, &[(p_pass_1) as u32], false); }
    let mut l_fill_2: i32 = 0;
    let mut l_cost_3: i32 = 0;
    let mut l_itemTbl_4: u32 = 0;
    let mut l_charaTbl_5: u32 = 0;
    let mut l_chara_6: u32 = 0;
    let mut l_extra_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_j_9: i32 = 0;
    let mut l_item_10: u32 = 0;
    let mut l_other_11: u32 = 0;
    let mut l_other_12: u32 = 0;
    let r_ = 'f: {
    l_fill_2 = 0i32;
    l_cost_3 = 0i32;
    l_itemTbl_4 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    l_charaTbl_5 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    l_chara_6 = (((c.m.r32(p_pass_1) as i32)) as u32);
    if (((((l_chara_6) > (((160i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_extra_7 = (c.m.r32((p_pass_1).wrapping_add(36)) as i32);
    if ((((((((c.m.r16(((l_charaTbl_5).wrapping_add((l_chara_6).wrapping_mul(60))).wrapping_add(14))) as i32)).wrapping_add(l_extra_7)) > (7i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_8 = 0i32;
        'l1: loop {
            if !(((((l_i_8) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(((p_pass_1).wrapping_add(4)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                    l_item_10 = ((((c.m.r32(((p_pass_1).wrapping_add(4)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(1i32)) as u32);
                    if (((((((c.m.r8((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40)))) as i32)) == (3i32)) as i32)) != 0) {
                        l_fill_2 = 1i32;
                    }
                    if (((((l_item_10) >= (((350i32) as u32))) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    {
                        l_j_9 = 0i32;
                        'l3: loop {
                            if !(((((l_j_9) < (7i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_other_11 = ((((c.m.r32(((p_pass_1).wrapping_add(4)).wrapping_add((((l_j_9) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(1i32)) as u32);
                                if (((((((((l_j_9) != (l_i_8)) as i32)) != 0) && (((((l_item_10) == (l_other_11)) as i32)) != 0)) as i32)) != 0) {
                                    break 'f 0i32;
                                }
                            }
                            { let o_ = l_j_9; let n_ = (o_).wrapping_add(1); l_j_9 = n_; };
                        }
                    }
                    {
                        l_j_9 = (l_i_8).wrapping_add(1i32);
                        'l5: loop {
                            if !(((((l_j_9) < (7i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                l_other_12 = ((((c.m.r32(((p_pass_1).wrapping_add(4)).wrapping_add((((l_j_9) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(1i32)) as u32);
                                if (((((((((l_other_12) < (((350i32) as u32))) as i32)) != 0) && (((((l_j_9) != (l_i_8)) as i32)) != 0)) as i32)) != 0) {
                                    if (((((((c.m.r8((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40)))) as i32)) == (((c.m.r8((l_itemTbl_4).wrapping_add((l_other_12).wrapping_mul(40)))) as i32))) as i32)) != 0) {
                                        if (((((((c.m.r8(((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40))).wrapping_add(1))) as i32)) == (((c.m.r8(((l_itemTbl_4).wrapping_add((l_other_12).wrapping_mul(40))).wrapping_add(1))) as i32))) as i32)) != 0) {
                                            break 'f 0i32;
                                        }
                                    }
                                }
                            }
                            { let o_ = l_j_9; let n_ = (o_).wrapping_add(1); l_j_9 = n_; };
                        }
                    }
                    if (((((l_i_8) == (7i32)) as i32)) != 0) {
                        if (((((((c.m.r8((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40)))) as i32)) != (2i32)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                    } else {
                        if (((((((c.m.r8((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40)))) as i32)) == (2i32)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                    }
                    if ((((c.m.r32(((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40))).wrapping_add(20))) & (((5i32) as u32)))) != 0) {
                        break 'f 0i32;
                    }
                    if (((((((c.m.r32(((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40))).wrapping_add(20))) & (((24i32) as u32)))) != (((24i32) as u32))) as i32)) != 0) {
                        if ((((((((((c.m.r16(((l_charaTbl_5).wrapping_add((l_chara_6).wrapping_mul(60))).wrapping_add(8))) as i32)) & (1i32))) != 0) && ((((c.m.r32(((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40))).wrapping_add(20))) & (((16i32) as u32)))) != 0)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                        if ((((((((((c.m.r16(((l_charaTbl_5).wrapping_add((l_chara_6).wrapping_mul(60))).wrapping_add(8))) as i32)) & (2i32))) != 0) && ((((c.m.r32(((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40))).wrapping_add(20))) & (((8i32) as u32)))) != 0)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                    }
                    { let r_ = ((c.m.r8(((l_itemTbl_4).wrapping_add((l_item_10).wrapping_mul(40))).wrapping_add(3))) as i32); let n_ = (l_cost_3).wrapping_add(r_); l_cost_3 = n_; };
                } else {
                    if (((((((((l_i_8) < (6i32)) as i32)) != 0) && ((((((c.m.r32(((p_pass_1).wrapping_add(4)).wrapping_add(((((l_i_8).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                }
            }
            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
        }
    }
    if ((l_fill_2) != 0) {
        l_extra_7 = { let v_ = (7i32).wrapping_sub(((c.m.r16(((l_charaTbl_5).wrapping_add((l_chara_6).wrapping_mul(60))).wrapping_add(14))) as i32)); let a_ = (p_pass_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); v_ };
    }
    if ((((((l_extra_7).wrapping_add(((c.m.r16(((l_charaTbl_5).wrapping_add((l_chara_6).wrapping_mul(60))).wrapping_add(14))) as i32))) < (l_cost_3)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PassChk_IsValid", &PROBE_N, &[], true); }
    r_
}
