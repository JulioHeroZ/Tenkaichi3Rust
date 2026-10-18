#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlInput_StickToByte(c: &mut Ctx, mut p_v_1: F32) -> u8 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_StickToByte", &PROBE_N, &[(p_v_1).0], false); }
    let mut l_n_2: i32 = 0;
    let r_ = 'f: {
    l_n_2 = rt::f2i(rt::fmul(rt::fadd(rt::fmul(p_v_1, F32(0x3f000000)), F32(0x3f000000)), F32(0x437e0000)));
    if (((((l_n_2) < (0i32)) as i32)) != 0) {
        l_n_2 = 0i32;
    }
    if (((((l_n_2) > (254i32)) as i32)) != 0) {
        l_n_2 = 254i32;
    }
    break 'f ((l_n_2) as u8);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_StickToByte", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_ByteToStick(c: &mut Ctx, mut p_b_1: u8) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_ByteToStick", &PROBE_N, &[(p_b_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fsub(rt::fmul(rt::fmul(rt::i2f(((p_b_1) as i32)), F32(0x3b810204)), F32(0x40000000)), F32(0x3f800000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_ByteToStick", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_Get(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Get", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_chr_1).wrapping_add(1392);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_RemapButtons(c: &mut Ctx, mut p_rawHeld_1: u32, mut p_mask_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_RemapButtons", &PROBE_N, &[(p_rawHeld_1) as u32, (p_mask_2) as u32], false); }
    let mut l_out_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_bit_5: u32 = 0;
    let r_ = 'f: {
    l_out_3 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (31i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_bit_5 = (((1i32).wrapping_shl((l_i_4) as u32)) as u32);
                if ((((l_bit_5) & (((32768i32) as u32)))) != 0) {
                    if (((((((p_rawHeld_1) & (c.m.r32(p_mask_2)))) == (c.m.r32(p_mask_2))) as i32)) != 0) {
                        { let r_ = l_bit_5; let n_ = ((l_out_3) | (r_)); l_out_3 = n_; };
                    }
                } else {
                    if ((((p_rawHeld_1) & (c.m.r32(p_mask_2)))) != 0) {
                        { let r_ = l_bit_5; let n_ = ((l_out_3) | (r_)); l_out_3 = n_; };
                    }
                }
                { let o_ = p_mask_2; let n_ = (o_).wrapping_add(4); p_mask_2 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_out_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_RemapButtons", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_TickCounter(c: &mut Ctx, mut p_cond_1: i32, mut p_count_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TickCounter", &PROBE_N, &[(p_cond_1) as u32, (p_count_2) as u32], false); }
    let r_ = 'f: {
    if ((p_cond_1) != 0) {
        if ((((((((c.m.r8(p_count_2) as i8)) as i32)) < (100i32)) as i32)) != 0) {
            { let a_ = p_count_2; let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
        }
    } else {
        { let v_ = ((0i32) as i8); let a_ = p_count_2; c.m.w8(a_, (v_) as u8); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TickCounter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_TickCounters(c: &mut Ctx, mut p_c_1: u32, mut p_held_2: u32, mut p_pressed_3: u32, mut p_released_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TickCounters", &PROBE_N, &[(p_c_1) as u32, (p_held_2) as u32, (p_pressed_3) as u32, (p_released_4) as u32], false); }
    let mut l_i_5: i32 = 0;
    let mut l_bit_6: u32 = 0;
    let r_ = 'f: {
    { let s_ = (p_c_1).wrapping_add(64); let d_ = (p_c_1).wrapping_add(128); rt::memcpy(c, d_, s_, 32); };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (32i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_bit_6 = (((1i32).wrapping_shl((l_i_5) as u32)) as u32);
                F::BtlInput_TickCounter(c, ((((p_held_2) & (l_bit_6))) as i32), (p_c_1).wrapping_add(((l_i_5) as u32)));
                F::BtlInput_TickCounter(c, (((((p_held_2) & (l_bit_6))) == (((0i32) as u32))) as i32), ((p_c_1).wrapping_add(32)).wrapping_add(((l_i_5) as u32)));
                F::BtlInput_TickCounter(c, (((((p_pressed_3) & (l_bit_6))) == (((0i32) as u32))) as i32), ((p_c_1).wrapping_add(64)).wrapping_add(((l_i_5) as u32)));
                F::BtlInput_TickCounter(c, (((((p_released_4) & (l_bit_6))) == (((0i32) as u32))) as i32), ((p_c_1).wrapping_add(96)).wrapping_add(((l_i_5) as u32)));
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TickCounters", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_RingPush(c: &mut Ctx, mut p_chr_1: u32, mut p_buttons_2: u32, mut p_commands_3: u32, mut p_stick_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_RingPush", &PROBE_N, &[(p_chr_1) as u32, (p_buttons_2) as u32, (p_commands_3) as u32, (p_stick_4) as u32], false); }
    let mut l_in_5: u32 = 0;
    let r_ = 'f: {
    l_in_5 = (p_chr_1).wrapping_add(1392);
    if (((((rt::rem_i32(((c.m.r32((l_in_5).wrapping_add(928)) as i32)).wrapping_add(1i32), 8i32)) != ((c.m.r32((l_in_5).wrapping_add(932)) as i32))) as i32)) != 0) {
        { let v_ = p_buttons_2; let a_ = ((l_in_5).wrapping_add(848)).wrapping_add(((((c.m.r32((l_in_5).wrapping_add(928)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = p_commands_3; let a_ = ((l_in_5).wrapping_add(880)).wrapping_add(((((c.m.r32((l_in_5).wrapping_add(928)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r8((p_stick_4).wrapping_add(((0i32) as u32))); let a_ = (((l_in_5).wrapping_add(912)).wrapping_add(((((c.m.r32((l_in_5).wrapping_add(928)) as i32)) as u32)).wrapping_mul(2))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = c.m.r8((p_stick_4).wrapping_add(((1i32) as u32))); let a_ = (((l_in_5).wrapping_add(912)).wrapping_add(((((c.m.r32((l_in_5).wrapping_add(928)) as i32)) as u32)).wrapping_mul(2))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = rt::rem_i32(((c.m.r32((l_in_5).wrapping_add(928)) as i32)).wrapping_add(1i32), 8i32); let a_ = (l_in_5).wrapping_add(928); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_RingPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_RingPop(c: &mut Ctx, mut p_chr_1: u32, mut p_buttons_2: u32, mut p_commands_3: u32, mut p_stick_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_RingPop", &PROBE_N, &[(p_chr_1) as u32, (p_buttons_2) as u32, (p_commands_3) as u32, (p_stick_4) as u32], false); }
    let mut l_in_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let mut l_c_7: u32 = 0;
    let r_ = 'f: {
    l_in_5 = (p_chr_1).wrapping_add(1392);
    if ((((((c.m.r32((l_in_5).wrapping_add(928)) as i32)) == ((c.m.r32((l_in_5).wrapping_add(932)) as i32))) as i32)) != 0) {
        if (((((p_buttons_2) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = p_buttons_2; c.m.w32(a_, v_); };
        }
        if (((((p_commands_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = p_commands_3; c.m.w32(a_, v_); };
        }
        { let v_ = { let v_ = ((127i32) as u8); let a_ = (p_stick_4).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); v_ }; let a_ = (p_stick_4).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        break 'f;
    }
    l_b_6 = { let a_ = ((l_in_5).wrapping_add(848)).wrapping_add(((((c.m.r32((l_in_5).wrapping_add(932)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_c_7 = { let a_ = ((l_in_5).wrapping_add(880)).wrapping_add(((((c.m.r32((l_in_5).wrapping_add(932)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    { let v_ = { let a_ = (((l_in_5).wrapping_add(912)).wrapping_add(((((c.m.r32((l_in_5).wrapping_add(932)) as i32)) as u32)).wrapping_mul(2))).wrapping_add(((0i32) as u32)); c.m.r8(a_) }; let a_ = (p_stick_4).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = { let a_ = (((l_in_5).wrapping_add(912)).wrapping_add(((((c.m.r32((l_in_5).wrapping_add(932)) as i32)) as u32)).wrapping_mul(2))).wrapping_add(((1i32) as u32)); c.m.r8(a_) }; let a_ = (p_stick_4).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    if (((((p_buttons_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_b_6; let a_ = p_buttons_2; c.m.w32(a_, v_); };
    }
    if (((((p_commands_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_c_7; let a_ = p_commands_3; c.m.w32(a_, v_); };
    }
    { let v_ = rt::rem_i32(((c.m.r32((l_in_5).wrapping_add(932)) as i32)).wrapping_add(1i32), 8i32); let a_ = (l_in_5).wrapping_add(932); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_RingPop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_AddDoubleTaps(c: &mut Ctx, mut p_chr_1: u32, mut p_buttons_2: u32, mut p_prev_3: u32, mut p_padStatus_4: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_AddDoubleTaps", &PROBE_N, &[(p_chr_1) as u32, (p_buttons_2) as u32, (p_prev_3) as u32, (p_padStatus_4) as u32], false); }
    let mut l_in_5: u32 = 0;
    let mut l_pressed_6: u32 = 0;
    let r_ = 'f: {
    l_in_5 = (p_chr_1).wrapping_add(1392);
    l_pressed_6 = ((p_buttons_2) & ((!(p_prev_3))));
    's1: {
        let sv7: i32 = p_padStatus_4;
        match sv7 {
            0i32 | 1i32 => {
                if ((((((((l_pressed_6) & (((2048i32) as u32)))) != 0) && ((((((((c.m.r8((((l_in_5).wrapping_add(140)).wrapping_add(128)).wrapping_add(((11i32) as u32))) as i8)) as i32)) < (7i32)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = ((2097152i32) as u32); let n_ = ((p_buttons_2) | (r_)); p_buttons_2 = n_; };
                }
                if ((((((((l_pressed_6) & (((4096i32) as u32)))) != 0) && ((((((((c.m.r8((((l_in_5).wrapping_add(140)).wrapping_add(128)).wrapping_add(((12i32) as u32))) as i8)) as i32)) < (7i32)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = ((4194304i32) as u32); let n_ = ((p_buttons_2) | (r_)); p_buttons_2 = n_; };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f p_buttons_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_AddDoubleTaps", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_TestSwitch(c: &mut Ctx, mut p_chr_1: u32, mut p_arg_2: i32, mut p_held_3: u32, mut p_pressed_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TestSwitch", &PROBE_N, &[(p_chr_1) as u32, (p_arg_2) as u32, (p_held_3) as u32, (p_pressed_4) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((162i32) as i64 as u64)) as u32))) != 0) {
        if ((((!((((p_held_3) & (((4i32) as u32)))) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    } else {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((163i32) as i64 as u64)) as u32))) != 0) {
            if ((((p_pressed_4) & (((4i32) as u32)))) != 0) {
                break 'f 1i32;
            }
        } else {
            's1: {
                let sv5: i32 = (({ let a_ = (F::BtlSuper_GetPromptRow(c, p_chr_1, p_arg_2)).wrapping_add(2); (c.m.r8(a_) as i8) }) as i32);
                match sv5 {
                    1i32 => {
                        if ((((((((p_held_3) & (((32i32) as u32)))) != 0) && ((((p_pressed_4) & (((8i32) as u32)))) != 0)) as i32)) != 0) {
                            break 'f 1i32;
                        }
                        break 's1;
                    }
                    0i32 => {
                        if ((((((((p_held_3) & (((16i32) as u32)))) != 0) && ((((p_pressed_4) & (((8i32) as u32)))) != 0)) as i32)) != 0) {
                            break 'f 1i32;
                        }
                        break 's1;
                    }
                    2i32 => {
                        if ((((((((p_held_3) & (((64i32) as u32)))) != 0) && ((((p_pressed_4) & (((8i32) as u32)))) != 0)) as i32)) != 0) {
                            break 'f 1i32;
                        }
                        break 's1;
                    }
                    3i32 => {
                        if ((((((((p_held_3) & (((128i32) as u32)))) != 0) && ((((p_pressed_4) & (((8i32) as u32)))) != 0)) as i32)) != 0) {
                            break 'f 1i32;
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TestSwitch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_MergeStick(c: &mut Ctx, mut p_chr_1: u32, mut p_buttons_2: u32, mut p_stick_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_MergeStick", &PROBE_N, &[(p_chr_1) as u32, (p_buttons_2) as u32, (p_stick_3) as u32], false); }
    let mut l_x_4: u8 = 0;
    let mut l_y_5: u8 = 0;
    let mut l_dx_6: i32 = 0;
    let mut l_dy_7: i32 = 0;
    let r_ = 'f: {
    if ((((p_buttons_2) & (((240i32) as u32)))) != 0) {
        if ((((p_buttons_2) & (((64i32) as u32)))) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (p_stick_3).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        }
        if ((((p_buttons_2) & (((128i32) as u32)))) != 0) {
            { let v_ = ((255i32) as u8); let a_ = (p_stick_3).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        }
        if ((((p_buttons_2) & (((16i32) as u32)))) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (p_stick_3).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        }
        if ((((p_buttons_2) & (((32i32) as u32)))) != 0) {
            { let v_ = ((255i32) as u8); let a_ = (p_stick_3).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        }
    } else {
        l_x_4 = c.m.r8((p_stick_3).wrapping_add(((0i32) as u32)));
        l_y_5 = c.m.r8((p_stick_3).wrapping_add(((1i32) as u32)));
        l_dx_6 = ((((l_x_4) as i32)).wrapping_sub(127i32)).wrapping_abs();
        l_dy_7 = ((((l_y_5) as i32)).wrapping_sub(127i32)).wrapping_abs();
        if (((((((((l_dx_6) >= (2i32)) as i32)) != 0) || (((((l_dy_7) >= (2i32)) as i32)) != 0)) as i32)) != 0) {
            if (((((l_dy_7) < (l_dx_6)) as i32)) != 0) {
                p_buttons_2 = (if (((((((l_x_4) as i32)) > (127i32)) as i32)) != 0) { ((p_buttons_2) | (((128i32) as u32))) } else { ((p_buttons_2) | (((64i32) as u32))) });
            } else {
                p_buttons_2 = (if (((((((l_y_5) as i32)) > (127i32)) as i32)) != 0) { ((p_buttons_2) | (((32i32) as u32))) } else { ((p_buttons_2) | (((16i32) as u32))) });
            }
        }
    }
    break 'f p_buttons_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_MergeStick", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_BuildCommands(c: &mut Ctx, mut p_chr_1: u32, mut p_held_2: u32, mut p_prev_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_BuildCommands", &PROBE_N, &[(p_chr_1) as u32, (p_held_2) as u32, (p_prev_3) as u32], false); }
    let mut l_in_4: u32 = 0;
    let mut l_cmd_5: u32 = 0;
    let mut l_released_6: u32 = 0;
    let mut l_pressed_7: u32 = 0;
    let mut l_charge_8: u32 = 0;
    let r_ = 'f: {
    l_in_4 = (p_chr_1).wrapping_add(1392);
    l_cmd_5 = ((0i32) as u32);
    l_released_6 = (((!(p_held_2))) & (p_prev_3));
    l_pressed_7 = ((p_held_2) & ((!(p_prev_3))));
    if ((((p_held_2) & (((2i32) as u32)))) != 0) {
        l_charge_8 = ((p_held_2) & (((512i32) as u32)));
        l_cmd_5 = ((8i32) as u32);
        if ((((!((l_charge_8) != 0)) as i32)) != 0) {
            l_cmd_5 = ((1i32) as u32);
        }
    }
    if ((((l_pressed_7) & (((2i32) as u32)))) != 0) {
        { let r_ = ((((262144i32) | (4i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        { let r_ = ((8388608i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        if ((((p_held_2) & (((512i32) as u32)))) != 0) {
            { let r_ = ((2i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
        if ((((p_held_2) & (((64i32) as u32)))) != 0) {
            { let r_ = ((((67108864i32) | (65536i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
        if ((((p_held_2) & (((128i32) as u32)))) != 0) {
            { let r_ = ((((134217728i32) | (65536i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
        if ((((p_held_2) & (((32i32) as u32)))) != 0) {
            { let r_ = ((((536870912i32) | (65536i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
        if ((((p_held_2) & (((16i32) as u32)))) != 0) {
            { let r_ = ((((268435456i32) | (65536i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
        if ((((!((((p_held_2) & (((240i32) as u32)))) != 0)) as i32)) != 0) {
            { let r_ = ((((268435456i32) | (65536i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
            if ((((((((c.m.r8((((l_in_4).wrapping_add(140)).wrapping_add(128)).wrapping_add(((1i32) as u32))) as i8)) as i32)) < (7i32)) as i32)) != 0) {
                { let r_ = ((131072i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
            }
        }
    }
    if ((((p_held_2) & (((1i32) as u32)))) != 0) {
        { let r_ = ((((1048576i32) | (16i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        if ((((l_pressed_7) & (((64i32) as u32)))) != 0) {
            { let r_ = ((524288i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
        if ((((l_pressed_7) & (((128i32) as u32)))) != 0) {
            { let r_ = ((524288i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
    }
    if ((((p_held_2) & (((512i32) as u32)))) != 0) {
        if ((((l_pressed_7) & (((4i32) as u32)))) != 0) {
            if ((((p_held_2) & (((32i32) as u32)))) != 0) {
                { let r_ = ((2048i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
            } else {
                if ((((p_held_2) & (((16i32) as u32)))) != 0) {
                    { let r_ = ((1024i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
                } else {
                    { let r_ = ((512i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
                }
            }
        }
        if ((((l_pressed_7) & (((1i32) as u32)))) != 0) {
            if ((((p_held_2) & (((16i32) as u32)))) != 0) {
                { let r_ = ((32768i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
            } else {
                { let r_ = ((16384i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
            }
        }
    }
    if ((((l_released_6) & (((8i32) as u32)))) != 0) {
        if ((((((((c.m.r8((((l_in_4).wrapping_add(140)).wrapping_add(64)).wrapping_add(((3i32) as u32))) as i8)) as i32)) < (7i32)) as i32)) != 0) {
            { let r_ = ((32i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
    }
    if ((((p_held_2) & (((8i32) as u32)))) != 0) {
        if ((((l_pressed_7) & (((8i32) as u32)))) != 0) {
            { let r_ = ((16777216i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
        if ((((((((c.m.r8((((l_in_4).wrapping_add(140)).wrapping_add(64)).wrapping_add(((3i32) as u32))) as i8)) as i32)) == (6i32)) as i32)) != 0) {
            { let r_ = ((64i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
        { let r_ = ((128i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        if ((((p_held_2) & (((16i32) as u32)))) != 0) {
            { let r_ = ((268435456i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        } else {
            if ((((p_held_2) & (((32i32) as u32)))) != 0) {
                { let r_ = ((536870912i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
            } else {
                if ((((p_held_2) & (((64i32) as u32)))) != 0) {
                    { let r_ = ((67108864i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
                } else {
                    if ((((p_held_2) & (((128i32) as u32)))) != 0) {
                        { let r_ = ((134217728i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
                    }
                }
            }
        }
    } else {
        { let r_ = ((256i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
    }
    if ((((((((l_pressed_7) & (((4i32) as u32)))) != 0) && ((((!((((p_held_2) & (((512i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
        if ((((p_held_2) & (((16i32) as u32)))) != 0) {
            { let r_ = ((((268435456i32) | (2097152i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        } else {
            if ((((p_held_2) & (((64i32) as u32)))) != 0) {
                { let r_ = ((((67108864i32) | (2097152i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
            } else {
                if ((((p_held_2) & (((128i32) as u32)))) != 0) {
                    { let r_ = ((((134217728i32) | (2097152i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
                } else {
                    { let r_ = ((((536870912i32) | (2097152i32))) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
                }
            }
        }
    }
    if ((((l_pressed_7) & (((16384i32) as u32)))) != 0) {
        { let r_ = ((4194304i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
    }
    if ((((l_pressed_7) & (((240i32) as u32)))) != 0) {
        { let r_ = ((33554432i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(5524)) as i32)) >= (2i32)) as i32)) != 0) {
        if (({ let a1_ = (c.m.r32((p_chr_1).wrapping_add(5524)) as i32); F::BtlInput_TestSwitch(c, p_chr_1, a1_, p_held_2, l_pressed_7) }) != 0) {
            { let r_ = ((4096i32) as u32); let n_ = ((l_cmd_5) | (r_)); l_cmd_5 = n_; };
        }
    }
    break 'f l_cmd_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_BuildCommands", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_BuildRecord(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_BuildRecord", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_in_2: u32 = 0;
    let mut l_commands_3: u32 = 0;
    let mut l_prevButtons_4: u32 = 0;
    let mut l_prevCommands_5: u32 = 0;
    let mut l_active_6: i32 = 0;
    let mut l_x_7: F32 = F32(0);
    let mut l_y_8: F32 = F32(0);
    let mut l_raw_9: u32 = 0;
    let r_ = 'f: {
        l_in_2 = (p_chr_1).wrapping_add(1392);
        { let v_ = ((0i32) as u32); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        l_prevButtons_4 = c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(8));
        l_prevCommands_5 = c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(12));
        { let v_ = { let v_ = ((127i32) as u8); let a_ = (fp).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); v_ }; let a_ = (fp).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        if (((c.m.r32((p_chr_1).wrapping_add(4728)) as i32)) != 0) {
            l_x_7 = F32(c.m.r32((p_chr_1).wrapping_add(4736)));
            l_y_8 = F32(c.m.r32((p_chr_1).wrapping_add(4740)));
            { let v_ = c.m.r32((p_chr_1).wrapping_add(4732)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
            { let v_ = l_x_7; let a_ = (fp.wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = l_y_8; let a_ = (fp.wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            if ((((c.m.r32(fp.wrapping_add(12))) & (((1i32) as u32)))) != 0) {
                { let r_ = ((1048576i32) as u32); let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
            if ((((c.m.r32(fp.wrapping_add(12))) & (((4i32) as u32)))) != 0) {
                { let r_ = ((16777216i32) as u32); let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
            if ((((c.m.r32(fp.wrapping_add(12))) & (((65536i32) as u32)))) != 0) {
                { let r_ = ((33554432i32) as u32); let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
            if ((((c.m.r32(fp.wrapping_add(12))) & (((131072i32) as u32)))) != 0) {
                { let r_ = ((67108864i32) as u32); let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
            { let v_ = F::BtlInput_StickToByte(c, l_x_7); let a_ = (fp).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = F::BtlInput_StickToByte(c, l_y_8); let a_ = (fp).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = { let a1_ = c.m.r32(fp.wrapping_add(12)); F::BtlInput_MergeStick(c, p_chr_1, a1_, fp) }; let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
            l_commands_3 = { let a1_ = c.m.r32(fp.wrapping_add(12)); F::BtlInput_BuildCommands(c, p_chr_1, a1_, l_prevButtons_4) };
        } else {
            l_raw_9 = F::BtlInput_Get(c, p_chr_1);
            if (((((l_raw_9) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = { let a0_ = c.m.r32(l_raw_9); F::BtlInput_RemapButtons(c, a0_, (l_raw_9).wrapping_add(8)) }; let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                { let v_ = c.m.r8((l_raw_9).wrapping_add(4)); let a_ = (fp).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                { let v_ = c.m.r8((l_raw_9).wrapping_add(5)); let a_ = (fp).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            }
            { let v_ = { let a1_ = c.m.r32(fp.wrapping_add(12)); F::BtlInput_MergeStick(c, p_chr_1, a1_, fp) }; let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
            { let v_ = { let a1_ = c.m.r32(fp.wrapping_add(12)); let a3_ = (c.m.r32((l_in_2).wrapping_add(132)) as i32); F::BtlInput_AddDoubleTaps(c, p_chr_1, a1_, l_prevButtons_4, a3_) }; let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
            l_commands_3 = { let a1_ = c.m.r32(fp.wrapping_add(12)); F::BtlInput_BuildCommands(c, p_chr_1, a1_, l_prevButtons_4) };
        }
        l_active_6 = 0i32;
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((2i32) as i64 as u64)) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((3i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((310i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                l_active_6 = (((F::BtlChars_IsTimeStopped(c)) == (0i32)) as i32);
            }
        }
        if ((l_active_6) != 0) {
            if ((F::BattleReplay_IsActive(c)) != 0) {
                F::BtlReplay_Play(c, p_chr_1, fp.wrapping_add(12), fp);
                l_commands_3 = { let a1_ = c.m.r32(fp.wrapping_add(12)); F::BtlInput_BuildCommands(c, p_chr_1, a1_, l_prevButtons_4) };
            } else {
                { let a1_ = c.m.r32(fp.wrapping_add(12)); F::BtlReplay_Record(c, p_chr_1, a1_, l_commands_3, fp) };
            }
        } else {
            { let v_ = ((0i32) as u32); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
            { let v_ = { let v_ = ((127i32) as u8); let a_ = (fp).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); v_ }; let a_ = (fp).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            l_commands_3 = ((0i32) as u32);
        }
        { let v_ = ((79i32) as u8); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((80i32) as u8); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((82i32) as u8); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((84i32) as u8); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32(p_chr_1) as i32)) as u8); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(4); c.m.w8(a_, v_); };
        { let v_ = c.m.r32(fp.wrapping_add(12)); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = c.m.r8((fp).wrapping_add(((0i32) as u32))); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(6); c.m.w8(a_, v_); };
        { let v_ = c.m.r8((fp).wrapping_add(((1i32) as u32))); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(7); c.m.w8(a_, v_); };
        { let v_ = l_commands_3; let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u8); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(5); c.m.w8(a_, v_); };
        if ((((!((F::BtlChars_IsTimeStopped(c)) != 0)) as i32)) != 0) {
            { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = ((c.m.r32(fp.wrapping_add(12))) & ((!(l_prevButtons_4)))); let a3_ = ((l_prevButtons_4) & ((!(c.m.r32(fp.wrapping_add(12)))))); F::BtlInput_TickCounters(c, ((p_chr_1).wrapping_add(1392)).wrapping_add(140), a1_, a2_, a3_) };
            F::BtlInput_TickCounters(c, ((p_chr_1).wrapping_add(1392)).wrapping_add(300), l_commands_3, ((l_commands_3) & ((!(l_prevCommands_5)))), ((l_prevCommands_5) & ((!(l_commands_3)))));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_BuildRecord", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_Fetch(c: &mut Ctx, mut p_chr_1: u32, mut p_outButtons_2: u32, mut p_outCommands_3: u32, mut p_stick_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Fetch", &PROBE_N, &[(p_chr_1) as u32, (p_outButtons_2) as u32, (p_outCommands_3) as u32, (p_stick_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_block_5: i32 = 0;
    let mut l_filter_6: i32 = 0;
    let r_ = 'f: {
        l_block_5 = 0i32;
        l_filter_6 = 0i32;
        { let v_ = { let v_ = ((127i32) as u8); let a_ = (p_stick_4).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); v_ }; let a_ = (p_stick_4).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = fp.wrapping_add(4); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(8)); let a2_ = c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(12)); F::BtlInput_RingPush(c, p_chr_1, a1_, a2_, (((p_chr_1).wrapping_add(1392)).wrapping_add(968)).wrapping_add(6)) };
        F::BtlInput_RingPop(c, p_chr_1, fp, fp.wrapping_add(4), p_stick_4);
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((1024i32) as u64)))) != 0) {
            if ((F::BtlChar_IsStage4Or27(c)) != 0) {
                l_block_5 = 1i32;
            } else {
                l_filter_6 = 1i32;
            }
        }
        if (((((((((((((((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((512i32) as u64)))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((286i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((287i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((177i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            l_block_5 = 1i32;
        }
        if ((l_filter_6) != 0) {
            { let r_ = ((9378035i32) as u32); let a_ = fp; let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            { let r_ = ((1006698513i32) as u32); let a_ = fp.wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        }
        if ((l_block_5) != 0) {
            { let v_ = ((0i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = fp.wrapping_add(4); c.m.w32(a_, v_); };
            { let v_ = { let v_ = ((127i32) as u8); let a_ = (p_stick_4).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); v_ }; let a_ = (p_stick_4).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        }
        if (((((p_outButtons_2) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r32(fp); let a_ = p_outButtons_2; c.m.w32(a_, v_); };
        }
        if (((((p_outCommands_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r32(fp.wrapping_add(4)); let a_ = p_outCommands_3; c.m.w32(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Fetch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetKeyMask(c: &mut Ctx, mut p_chr_1: u32, mut p_action_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetKeyMask", &PROBE_N, &[(p_chr_1) as u32, (p_action_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 32);
        { let v_ = 8192i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 16384i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 32768i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 4096i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 1024i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 256i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 2048i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 512i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (p_action_2)) as i32)) != 0) {
                        break 'f (((c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)) as u32);
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        break 'f ((0i32) as u32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetKeyMask", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_BuildMaskTable(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_BuildMaskTable", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_in_2: u32 = 0;
    let r_ = 'f: {
    l_in_2 = (p_chr_1).wrapping_add(1392);
    { let v_ = F::BtlInput_GetKeyMask(c, p_chr_1, 0i32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::BtlInput_GetKeyMask(c, p_chr_1, 3i32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::BtlInput_GetKeyMask(c, p_chr_1, 2i32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::BtlInput_GetKeyMask(c, p_chr_1, 1i32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::BtlInput_GetKeyMask(c, p_chr_1, 6i32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::BtlInput_GetKeyMask(c, p_chr_1, 7i32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::BtlInput_GetKeyMask(c, p_chr_1, 5i32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::BtlInput_GetKeyMask(c, p_chr_1, 4i32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((4i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((2i32) | (4i32))) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((16i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((64i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((128i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((32i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((c.m.r32(((l_in_2).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))) | (c.m.r32(((l_in_2).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) | (c.m.r32(((l_in_2).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) | (c.m.r32(((l_in_2).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((1i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((8388608i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((4194304i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((1048576i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((2097152i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((8388608i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((4194304i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((1048576i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32(((l_in_2).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((2097152i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((8192i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((16384i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((4096i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((32768i32) as u32); let a_ = ((l_in_2).wrapping_add(8)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_BuildMaskTable", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_Init(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Init", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlInput_BuildMaskTable(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_Sample(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Sample", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pad_2: u32 = 0;
    let mut l_in_3: u32 = 0;
    let r_ = 'f: {
    l_pad_2 = F::BtlChar_GetPad(c, p_chr_1);
    l_in_3 = (p_chr_1).wrapping_add(1392);
    { let v_ = (c.m.r32((l_pad_2).wrapping_add(384)) as i32); let a_ = (l_in_3).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((l_pad_2).wrapping_add(388)) as i32); let a_ = (l_in_3).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    { let v_ = c.m.r32((l_pad_2).wrapping_add(328)); let a_ = l_in_3; c.m.w32(a_, v_); };
    { let v_ = { let a0_ = F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::BtlInput_StickToByte(c, a0_) }; let a_ = (l_in_3).wrapping_add(4); c.m.w8(a_, v_); };
    { let v_ = { let a0_ = F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::BtlInput_StickToByte(c, a0_) }; let a_ = (l_in_3).wrapping_add(5); c.m.w8(a_, v_); };
    F::BtlInput_BuildRecord(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Sample", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_Update(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Update", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_in_2: u32 = 0;
    let mut l_len_3: F32 = F32(0);
    let mut l_rawX_4: F32 = F32(0);
    let mut l_rawY_5: F32 = F32(0);
    let mut l_stickX_6: F32 = F32(0);
    let mut l_stickY_7: F32 = F32(0);
    let mut l_pressed_8: u32 = 0;
    let mut l_released_9: u32 = 0;
    let mut l_cmdPressed_10: u32 = 0;
    let mut l_cmdReleased_11: u32 = 0;
    let mut l_inv_12: F32 = F32(0);
    let r_ = 'f: {
        l_in_2 = (p_chr_1).wrapping_add(1392);
        { let _ = F::BtlChar_GetPad(c, p_chr_1); };
        F::BtlInput_Fetch(c, p_chr_1, fp.wrapping_add(4), fp.wrapping_add(8), fp);
        l_released_9 = ((c.m.r32((l_in_2).wrapping_add(460))) & ((!(c.m.r32(fp.wrapping_add(4))))));
        l_rawX_4 = F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((0i32) as u32)).wrapping_mul(4))));
        l_rawY_5 = F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((1i32) as u32)).wrapping_mul(4))));
        l_stickX_6 = F32(c.m.r32(((l_in_2).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4))));
        l_stickY_7 = F32(c.m.r32(((l_in_2).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4))));
        l_cmdReleased_11 = ((c.m.r32((l_in_2).wrapping_add(476))) & ((!(c.m.r32(fp.wrapping_add(8))))));
        l_pressed_8 = ((c.m.r32(fp.wrapping_add(4))) & ((!(c.m.r32((l_in_2).wrapping_add(460))))));
        /* asm: __asm__("" : "=r"(pressed) : "0"(pressed)) */
        l_cmdPressed_10 = ((c.m.r32(fp.wrapping_add(8))) & ((!(c.m.r32((l_in_2).wrapping_add(476))))));
        { let v_ = l_stickX_6; let a_ = ((l_in_2).wrapping_add(500)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_stickY_7; let a_ = ((l_in_2).wrapping_add(500)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_rawX_4; let a_ = ((l_in_2).wrapping_add(516)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_rawY_5; let a_ = ((l_in_2).wrapping_add(516)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_pressed_8; let a_ = (l_in_2).wrapping_add(468); c.m.w32(a_, v_); };
        { let v_ = l_released_9; let a_ = (l_in_2).wrapping_add(472); c.m.w32(a_, v_); };
        { let v_ = l_cmdPressed_10; let a_ = (l_in_2).wrapping_add(484); c.m.w32(a_, v_); };
        { let v_ = l_cmdReleased_11; let a_ = (l_in_2).wrapping_add(488); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((l_in_2).wrapping_add(460)); let a_ = (l_in_2).wrapping_add(464); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((l_in_2).wrapping_add(476)); let a_ = (l_in_2).wrapping_add(480); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(fp.wrapping_add(4)); let a_ = (l_in_2).wrapping_add(460); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(fp.wrapping_add(8)); let a_ = (l_in_2).wrapping_add(476); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r8((fp).wrapping_add(((0i32) as u32))); F::BtlInput_ByteToStick(c, a0_) }; let a_ = ((l_in_2).wrapping_add(508)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = c.m.r8((fp).wrapping_add(((1i32) as u32))); F::BtlInput_ByteToStick(c, a0_) }; let a_ = ((l_in_2).wrapping_add(508)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if ((F::BtlChars_IsTimeStopped(c)) != 0) {
            { let v_ = F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_in_2).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((l_in_2).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        } else {
            { let r_ = rt::fmul(rt::fsub(F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_in_2).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = ((l_in_2).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(rt::fsub(F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_in_2).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = ((l_in_2).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        l_len_3 = { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(((l_in_2).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_in_2).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((l_in_2).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_in_2).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); F::Port_sqrtf(c, a0_) };
        if (((rt::fgt(l_len_3, F32(0x3f800000)) as i32)) != 0) {
            l_inv_12 = rt::fdiv(F32(0x3f800000), l_len_3);
            { let r_ = l_inv_12; let a_ = ((l_in_2).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = l_inv_12; let a_ = ((l_in_2).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((((((((((rt::fgt({ let a0_ = F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }, F32(0x3c23d70a)) as i32)) != 0) || (((rt::fgt({ let a0_ = F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }, F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) && (((((((rt::fgt({ let a0_ = F32(c.m.r32(((l_in_2).wrapping_add(516)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }, F32(0x3c23d70a)) as i32)) != 0) || (((rt::fgt({ let a0_ = F32(c.m.r32(((l_in_2).wrapping_add(516)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }, F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = { let a0_ = rt::fsub({ let a0_ = F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32(((l_in_2).wrapping_add(508)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Port_atan2f(c, a0_, a1_) }, { let a0_ = F32(c.m.r32(((l_in_2).wrapping_add(516)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32(((l_in_2).wrapping_add(516)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Port_atan2f(c, a0_, a1_) }); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_in_2).wrapping_add(524); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(0x00000000); let a_ = (l_in_2).wrapping_add(524); c.m.w32(a_, (v_).0); };
        }
        if ((((!((F::BtlChars_IsTimeStopped(c)) != 0)) as i32)) != 0) {
            { let a1_ = c.m.r32((l_in_2).wrapping_add(460)); let a2_ = c.m.r32((l_in_2).wrapping_add(468)); let a3_ = c.m.r32((l_in_2).wrapping_add(472)); F::BtlInput_TickCounters(c, (l_in_2).wrapping_add(528), a1_, a2_, a3_) };
            { let a1_ = c.m.r32((l_in_2).wrapping_add(476)); let a2_ = c.m.r32((l_in_2).wrapping_add(484)); let a3_ = c.m.r32((l_in_2).wrapping_add(488)); F::BtlInput_TickCounters(c, (l_in_2).wrapping_add(688), a1_, a2_, a3_) };
        }
        F::BtlInput_ClearFrameBits(c, p_chr_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_Stub(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Stub", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Stub", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_Push(c: &mut Ctx, mut p_chr_1: u32, mut p_buttons_2: u32, mut p_commands_3: u32, mut p_stick_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Push", &PROBE_N, &[(p_chr_1) as u32, (p_buttons_2) as u32, (p_commands_3) as u32, (p_stick_4) as u32], false); }
    let r_ = 'f: {
    F::BtlInput_RingPush(c, p_chr_1, p_buttons_2, p_commands_3, p_stick_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Push", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_Pop(c: &mut Ctx, mut p_chr_1: u32, mut p_buttons_2: u32, mut p_commands_3: u32, mut p_stick_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Pop", &PROBE_N, &[(p_chr_1) as u32, (p_buttons_2) as u32, (p_commands_3) as u32, (p_stick_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::BtlInput_RingPop(c, p_chr_1, fp.wrapping_add(4), fp.wrapping_add(8), fp);
        if (((((p_buttons_2) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r32(fp.wrapping_add(4)); let a_ = p_buttons_2; c.m.w32(a_, v_); };
        }
        if (((((p_commands_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r32(fp.wrapping_add(8)); let a_ = p_commands_3; c.m.w32(a_, v_); };
        }
        if (((((p_stick_4) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r8((fp).wrapping_add(((0i32) as u32))); let a_ = (p_stick_4).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8((fp).wrapping_add(((1i32) as u32))); let a_ = (p_stick_4).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_Pop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetQueued(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetQueued", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_in_2: u32 = 0;
    let r_ = 'f: {
    l_in_2 = (p_chr_1).wrapping_add(1392);
    break 'f rt::rem_i32((((c.m.r32((l_in_2).wrapping_add(928)) as i32)).wrapping_sub((c.m.r32((l_in_2).wrapping_add(932)) as i32))).wrapping_add(8i32), 8i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetQueued", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_IsPressed(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsPressed", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(468))) & (p_mask_2))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsPressed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_IsHeldNotPrev(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsHeldNotPrev", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    if ((((c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(464))) & (p_mask_2))) != 0) {
        break 'f 0i32;
    }
    break 'f (((((c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(460))) & (p_mask_2))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsHeldNotPrev", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_IsHeld(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsHeld", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(460))) & (p_mask_2))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsHeld", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_IsReleased(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsReleased", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(472))) & (p_mask_2))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsReleased", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetHeldFrames(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetHeldFrames", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r8((((p_chr_1).wrapping_add(1392)).wrapping_add(528)).wrapping_add(((p_bit_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetHeldFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetNotHeldFrames(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetNotHeldFrames", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r8(((((p_chr_1).wrapping_add(1392)).wrapping_add(528)).wrapping_add(32)).wrapping_add(((p_bit_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetNotHeldFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetFramesSincePress(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetFramesSincePress", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r8(((((p_chr_1).wrapping_add(1392)).wrapping_add(528)).wrapping_add(64)).wrapping_add(((p_bit_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetFramesSincePress", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetFramesSinceRelease(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetFramesSinceRelease", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r8(((((p_chr_1).wrapping_add(1392)).wrapping_add(528)).wrapping_add(96)).wrapping_add(((p_bit_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetFramesSinceRelease", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetPrevFramesSincePress(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetPrevFramesSincePress", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r8(((((p_chr_1).wrapping_add(1392)).wrapping_add(528)).wrapping_add(128)).wrapping_add(((p_bit_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetPrevFramesSincePress", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_IsCmdPressed(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsCmdPressed", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(484))) & (p_mask_2))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsCmdPressed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_IsCmdHeld(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsCmdHeld", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(476))) & (p_mask_2))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsCmdHeld", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_IsCmdReleased(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsCmdReleased", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(488))) & (p_mask_2))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_IsCmdReleased", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetCmdHeldFrames(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetCmdHeldFrames", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r8((((p_chr_1).wrapping_add(1392)).wrapping_add(688)).wrapping_add(((p_bit_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetCmdHeldFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetCmdNotHeldFrames(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetCmdNotHeldFrames", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r8(((((p_chr_1).wrapping_add(1392)).wrapping_add(688)).wrapping_add(32)).wrapping_add(((p_bit_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetCmdNotHeldFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetCmdPrevFramesSincePress(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetCmdPrevFramesSincePress", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r8(((((p_chr_1).wrapping_add(1392)).wrapping_add(688)).wrapping_add(128)).wrapping_add(((p_bit_2) as u32))) as i8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetCmdPrevFramesSincePress", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_SetFrameBit(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32, mut p_set_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_SetFrameBit", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32, (p_set_3) as u32], false); }
    let mut l_byte_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let r_ = 'f: {
    l_byte_4 = (p_bit_2).wrapping_shr((3i32) as u32);
    l_n_5 = ((p_bit_2) & (7i32));
    p_set_3 = (((p_set_3) != (0i32)) as i32);
    { let r_ = (1i32).wrapping_shl((l_n_5) as u32); let a_ = (((p_chr_1).wrapping_add(1392)).wrapping_add(936)).wrapping_add((((p_set_3).wrapping_add((l_byte_4).wrapping_mul(2i32))) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_SetFrameBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_TestFrameBit(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32, mut p_set_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TestFrameBit", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32, (p_set_3) as u32], false); }
    let mut l_byte_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_mask_6: u8 = 0;
    let r_ = 'f: {
    l_byte_4 = (p_bit_2).wrapping_shr((3i32) as u32);
    l_n_5 = ((p_bit_2) & (7i32));
    p_set_3 = (((p_set_3) != (0i32)) as i32);
    l_mask_6 = (((1i32).wrapping_shl((l_n_5) as u32)) as u8);
    break 'f (((((((c.m.r8((((p_chr_1).wrapping_add(1392)).wrapping_add(936)).wrapping_add((((p_set_3).wrapping_add((l_byte_4).wrapping_mul(2i32))) as u32)))) as i32)) & (((l_mask_6) as i32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TestFrameBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_ClearFrameBits(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_ClearFrameBits", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, ((p_chr_1).wrapping_add(1392)).wrapping_add(936), 0i32, 30u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_ClearFrameBits", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_TestAction(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32, mut p_want_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TestAction", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32, (p_want_3) as u32], false); }
    let mut l_invert_4: i32 = 0;
    let mut l_result_5: i32 = 0;
    let r_ = 'f: {
    l_invert_4 = (((p_want_3) == (0i32)) as i32);
    F::BtlInput_SetFrameBit(c, p_chr_1, p_id_2, p_want_3);
    l_result_5 = 0i32;
    's1: {
        let sv6: i32 = p_id_2;
        match sv6 {
            1i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((8192i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32));
                }
                break 's1;
            }
            2i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 4i32)) < (3i32)) as i32)) != 0) {
                    break 's1;
                }
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 5i32)) < (3i32)) as i32)) != 0) {
                    break 's1;
                }
                if (((((((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 6i32)) >= (3i32)) as i32)) != 0) && (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 7i32)) >= (3i32)) as i32)) != 0)) as i32)) != 0) {
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            3i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((8192i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((1i32) as u32));
                }
                break 's1;
            }
            4i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) {
                    if (((rt::fgt({ let a0_ = F::BtlInput_GetStickTurnFromFacing(c, p_chr_1); F::Port_fabsf(c, a0_) }, F32(0x3f490fda)) as i32)) != 0) {
                        l_result_5 = F::BtlInput_IsHeldNotPrev(c, p_chr_1, ((240i32) as u32));
                    }
                    if (((rt::fgt({ let a0_ = F32(c.m.r32(((p_chr_1).wrapping_add(1392)).wrapping_add(524))); F::Port_fabsf(c, a0_) }, F32(0x3fbc430d)) as i32)) != 0) {
                        l_result_5 = 1i32;
                    }
                }
                break 's1;
            }
            5i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) {
                    if (((rt::fgt({ let a0_ = F::BtlInput_GetStickTurnFromFacing(c, p_chr_1); F::Port_fabsf(c, a0_) }, F32(0x3f490fda)) as i32)) != 0) {
                        l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((1i32) as u32));
                    }
                }
                break 's1;
            }
            6i32 => {
                if (((((F::BtlInput_GetCmdNotHeldFrames(c, p_chr_1, 0i32)) >= (6i32)) as i32)) != 0) {
                    l_result_5 = ((!((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32);
                }
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((8192i32) as u32))) != 0) {
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            7i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((512i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((2i32) as u32));
                }
                break 's1;
            }
            8i32 => {
                l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((2048i32) as u32));
                break 's1;
            }
            9i32 => {
                l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((2048i32) as u32));
                break 's1;
            }
            10i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((4096i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsHeld(c, p_chr_1, ((2048i32) as u32));
                }
                break 's1;
            }
            12i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((2048i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsHeld(c, p_chr_1, ((4096i32) as u32));
                }
                break 's1;
            }
            14i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((2048i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((4096i32) as u32));
                }
                break 's1;
            }
            11i32 => {
                l_result_5 = ((((((!((F::BtlInput_IsHeld(c, p_chr_1, ((2048i32) as u32))) != 0)) as i32)) != 0) || ((F::BtlInput_IsHeld(c, p_chr_1, ((4096i32) as u32))) != 0)) as i32);
                break 's1;
            }
            13i32 => {
                l_result_5 = ((((((!((F::BtlInput_IsHeld(c, p_chr_1, ((4096i32) as u32))) != 0)) as i32)) != 0) || ((F::BtlInput_IsHeld(c, p_chr_1, ((2048i32) as u32))) != 0)) as i32);
                break 's1;
            }
            15i32 => {
                l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((2097152i32) as u32));
                break 's1;
            }
            16i32 => {
                l_result_5 = ((!((F::BtlInput_IsHeld(c, p_chr_1, ((2048i32) as u32))) != 0)) as i32);
                break 's1;
            }
            17i32 => {
                l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((4194304i32) as u32));
                break 's1;
            }
            18i32 => {
                l_result_5 = ((!((F::BtlInput_IsHeld(c, p_chr_1, ((4096i32) as u32))) != 0)) as i32);
                break 's1;
            }
            20i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((2i32) as u32));
                }
                break 's1;
            }
            21i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) {
                    l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((2i32) as u32));
                }
                break 's1;
            }
            22i32 => {
                l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((4i32) as u32));
                break 's1;
            }
            23i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((8i32) as u32));
                }
                break 's1;
            }
            24i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) {
                    l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((8i32) as u32));
                }
                break 's1;
            }
            25i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((65536i32) as u32))) != 0) && ((F::BtlInput_IsCmdPressed(c, p_chr_1, ((536870912i32) as u32))) != 0)) as i32);
                break 's1;
            }
            26i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((65536i32) as u32))) != 0) && ((F::BtlInput_IsCmdPressed(c, p_chr_1, ((67108864i32) as u32))) != 0)) as i32);
                break 's1;
            }
            27i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((65536i32) as u32))) != 0) && ((F::BtlInput_IsCmdPressed(c, p_chr_1, ((134217728i32) as u32))) != 0)) as i32);
                break 's1;
            }
            28i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((65536i32) as u32))) != 0) && ((F::BtlInput_IsCmdPressed(c, p_chr_1, ((268435456i32) as u32))) != 0)) as i32);
                break 's1;
            }
            30i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((64i32) as u32))) != 0)) as i32);
                break 's1;
            }
            31i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((128i32) as u32))) != 0)) as i32);
                break 's1;
            }
            32i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0)) as i32);
                break 's1;
            }
            29i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0)) as i32);
                break 's1;
            }
            49i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) {
                    if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((512i32) as u32))) != 0)) as i32)) != 0) {
                        l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32));
                    }
                }
                break 's1;
            }
            50i32 => {
                l_result_5 = ((!((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16i32) as u32))) != 0)) as i32);
                break 's1;
            }
            51i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((33554432i32) as u32));
                break 's1;
            }
            97i32 => {
                l_result_5 = F::BtlInput_IsHeld(c, p_chr_1, ((256i32) as u32));
                break 's1;
            }
            98i32 => {
                l_result_5 = F::BtlInput_IsHeld(c, p_chr_1, ((1024i32) as u32));
                break 's1;
            }
            19i32 => {
                l_result_5 = F::BtlInput_IsHeld(c, p_chr_1, ((512i32) as u32));
                break 's1;
            }
            34i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((16i32) as u32));
                break 's1;
            }
            35i32 => {
                l_result_5 = ((((((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16i32) as u32))) != 0) && ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0)) as i32)) != 0)) as i32);
                break 's1;
            }
            36i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0)) as i32);
                break 's1;
            }
            37i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0)) as i32);
                break 's1;
            }
            38i32 => {
                l_result_5 = ((((F::BtlInput_IsPressed(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsPressed(c, p_chr_1, ((8i32) as u32))) != 0)) as i32);
                break 's1;
            }
            40i32 => {
                l_result_5 = F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32));
                break 's1;
            }
            41i32 => {
                l_result_5 = F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32));
                break 's1;
            }
            42i32 => {
                l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((262144i32) as u32));
                break 's1;
            }
            43i32 => {
                l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((524288i32) as u32));
                break 's1;
            }
            44i32 => {
                l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((1048576i32) as u32));
                break 's1;
            }
            45i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) {
                    if (((((((((F::BtlInput_GetHeldFrames(c, p_chr_1, 11i32)) >= (6i32)) as i32)) != 0) && (((((F::BtlInput_GetHeldFrames(c, p_chr_1, 12i32)) >= (6i32)) as i32)) != 0)) as i32)) != 0) {
                        l_result_5 = 1i32;
                    }
                }
                break 's1;
            }
            46i32 => {
                if ((F::BtlInput_IsCmdPressed(c, p_chr_1, ((4194304i32) as u32))) != 0) {
                    if (((((F::BtlInput_GetCmdPrevFramesSincePress(c, p_chr_1, 22i32)) < (7i32)) as i32)) != 0) {
                        l_result_5 = 1i32;
                    }
                }
                break 's1;
            }
            47i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) {
                    l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32));
                }
                break 's1;
            }
            48i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) {
                    l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32));
                }
                break 's1;
            }
            52i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((32i32) as u32));
                break 's1;
            }
            53i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((64i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((268435456i32) as u32))) != 0)) as i32);
                break 's1;
            }
            54i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((64i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((536870912i32) as u32))) != 0)) as i32);
                break 's1;
            }
            55i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((64i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((67108864i32) as u32))) != 0)) as i32);
                break 's1;
            }
            56i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((64i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((134217728i32) as u32))) != 0)) as i32);
                break 's1;
            }
            57i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdPressed(c, p_chr_1, ((64i32) as u32))) != 0) && ((((!((F::BtlInput_IsCmdHeld(c, p_chr_1, ((((((((67108864i32) | (134217728i32))) | (268435456i32))) | (536870912i32))) as u32))) != 0)) as i32)) != 0)) as i32);
                break 's1;
            }
            58i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((128i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((268435456i32) as u32))) != 0)) as i32);
                break 's1;
            }
            59i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((128i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((536870912i32) as u32))) != 0)) as i32);
                break 's1;
            }
            60i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((128i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((67108864i32) as u32))) != 0)) as i32);
                break 's1;
            }
            61i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((128i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((134217728i32) as u32))) != 0)) as i32);
                break 's1;
            }
            62i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((128i32) as u32))) != 0) && ((((!((F::BtlInput_IsCmdHeld(c, p_chr_1, ((((((((67108864i32) | (134217728i32))) | (268435456i32))) | (536870912i32))) as u32))) != 0)) as i32)) != 0)) as i32);
                break 's1;
            }
            63i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((256i32) as u32));
                break 's1;
            }
            64i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((4i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((8i32) as u32));
                }
                break 's1;
            }
            65i32 => {
                l_result_5 = ((!((F::BtlInput_IsHeld(c, p_chr_1, ((8i32) as u32))) != 0)) as i32);
                break 's1;
            }
            66i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((((240i32) | (8i32))) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((4i32) as u32));
                }
                break 's1;
            }
            77i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((8i32) as u32))) != 0) {
                    break 's1;
                }
                l_result_5 = ((((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsPressed(c, p_chr_1, ((4i32) as u32))) != 0)) as i32);
                break 's1;
            }
            78i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((8i32) as u32))) != 0) {
                    break 's1;
                }
                l_result_5 = ((((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) && ((F::BtlInput_IsPressed(c, p_chr_1, ((4i32) as u32))) != 0)) as i32);
                break 's1;
            }
            79i32 => {
                l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((8i32) as u32));
                break 's1;
            }
            80i32 => {
                l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((4i32) as u32));
                break 's1;
            }
            82i32 => {
                l_result_5 = ((((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsPressed(c, p_chr_1, ((8i32) as u32))) != 0)) as i32);
                break 's1;
            }
            83i32 => {
                l_result_5 = ((((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsPressed(c, p_chr_1, ((4i32) as u32))) != 0)) as i32);
                break 's1;
            }
            84i32 => {
                l_result_5 = ((((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) && ((F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32))) != 0)) as i32);
                break 's1;
            }
            85i32 => {
                l_result_5 = ((((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) && ((F::BtlInput_IsPressed(c, p_chr_1, ((8i32) as u32))) != 0)) as i32);
                break 's1;
            }
            86i32 => {
                l_result_5 = ((((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) && ((F::BtlInput_IsPressed(c, p_chr_1, ((4i32) as u32))) != 0)) as i32);
                break 's1;
            }
            87i32 => {
                l_result_5 = ((((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) && ((F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32))) != 0)) as i32);
                break 's1;
            }
            68i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((2097152i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((268435456i32) as u32))) != 0)) as i32);
                break 's1;
            }
            69i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((2097152i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((536870912i32) as u32))) != 0)) as i32);
                break 's1;
            }
            70i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((2097152i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((67108864i32) as u32))) != 0)) as i32);
                break 's1;
            }
            71i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((2097152i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((134217728i32) as u32))) != 0)) as i32);
                break 's1;
            }
            74i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16777216i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((268435456i32) as u32))) != 0)) as i32);
                break 's1;
            }
            75i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16777216i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((536870912i32) as u32))) != 0)) as i32);
                break 's1;
            }
            72i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16777216i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((67108864i32) as u32))) != 0)) as i32);
                break 's1;
            }
            73i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16777216i32) as u32))) != 0) && ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((134217728i32) as u32))) != 0)) as i32);
                break 's1;
            }
            76i32 => {
                l_result_5 = ((((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16777216i32) as u32))) != 0) && ((((!((F::BtlInput_IsCmdHeld(c, p_chr_1, ((((((((67108864i32) | (134217728i32))) | (268435456i32))) | (536870912i32))) as u32))) != 0)) as i32)) != 0)) as i32);
                break 's1;
            }
            92i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((512i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((131072i32) as u32));
                }
                break 's1;
            }
            93i32 => {
                l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((8388608i32) as u32));
                break 's1;
            }
            94i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((512i32) as u32))) != 0) {
                    break 's1;
                }
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) {
                    l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((4i32) as u32));
                }
                break 's1;
            }
            95i32 => {
                if ((F::BtlInput_IsCmdHeld(c, p_chr_1, ((16i32) as u32))) != 0) {
                    l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((16777216i32) as u32));
                }
                break 's1;
            }
            96i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0) {
                    l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32));
                }
                break 's1;
            }
            33i32 | 39i32 | 81i32 => {
                l_result_5 = F::BtlInput_IsCmdPressed(c, p_chr_1, ((16i32) as u32));
                break 's1;
            }
            88i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((((512i32) | (8i32))) as u32))) != 0) {
                    break 's1;
                }
                if ((F::BtlInput_IsReleased(c, p_chr_1, ((4i32) as u32))) != 0) {
                    l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 2i32)) < (7i32)) as i32);
                }
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((4i32) as u32))) != 0) {
                    l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 2i32)) == (6i32)) as i32);
                }
                break 's1;
            }
            89i32 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((((512i32) | (8i32))) as u32))) != 0) {
                    break 's1;
                }
                if ((F::BtlInput_IsReleased(c, p_chr_1, ((4i32) as u32))) != 0) {
                    l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 2i32)) < (7i32)) as i32);
                }
                break 's1;
            }
            90i32 => {
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((((512i32) | (8i32))) as u32))) != 0)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((4i32) as u32))) != 0) {
                        l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 2i32)) == (6i32)) as i32);
                    }
                }
                break 's1;
            }
            67i32 | 91i32 => {
                l_result_5 = ((!((F::BtlInput_IsHeld(c, p_chr_1, ((4i32) as u32))) != 0)) as i32);
                break 's1;
            }
            99i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 15i32)) < (30i32)) as i32)) != 0) {
                    break 's1;
                }
                if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0) {
                    if ((F::BtlInput_IsReleased(c, p_chr_1, ((8192i32) as u32))) != 0) {
                        l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 13i32)) < (13i32)) as i32);
                    }
                }
                break 's1;
            }
            100i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 15i32)) >= (30i32)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((64i32) as u32))) != 0) {
                        if ((F::BtlInput_IsReleased(c, p_chr_1, ((8192i32) as u32))) != 0) {
                            l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 13i32)) < (13i32)) as i32);
                        }
                    }
                }
                break 's1;
            }
            101i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 15i32)) >= (30i32)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) {
                        if ((F::BtlInput_IsReleased(c, p_chr_1, ((8192i32) as u32))) != 0) {
                            l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 13i32)) < (13i32)) as i32);
                        }
                    }
                }
                break 's1;
            }
            102i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 15i32)) >= (30i32)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((128i32) as u32))) != 0) {
                        if ((F::BtlInput_IsReleased(c, p_chr_1, ((8192i32) as u32))) != 0) {
                            l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 13i32)) < (13i32)) as i32);
                        }
                    }
                }
                break 's1;
            }
            103i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 15i32)) >= (30i32)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) {
                        if ((F::BtlInput_IsReleased(c, p_chr_1, ((8192i32) as u32))) != 0) {
                            l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 13i32)) < (13i32)) as i32);
                        }
                    }
                }
                break 's1;
            }
            104i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 15i32)) >= (30i32)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((64i32) as u32))) != 0) {
                        if ((F::BtlInput_IsHeld(c, p_chr_1, ((8192i32) as u32))) != 0) {
                            l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 13i32)) == (12i32)) as i32);
                        }
                    }
                }
                break 's1;
            }
            105i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 15i32)) >= (30i32)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) {
                        if ((F::BtlInput_IsHeld(c, p_chr_1, ((8192i32) as u32))) != 0) {
                            l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 13i32)) == (12i32)) as i32);
                        }
                    }
                }
                break 's1;
            }
            106i32 => {
                if (((((F::BtlInput_GetNotHeldFrames(c, p_chr_1, 15i32)) >= (30i32)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((128i32) as u32))) != 0) {
                        if ((F::BtlInput_IsHeld(c, p_chr_1, ((8192i32) as u32))) != 0) {
                            l_result_5 = (((F::BtlInput_GetFramesSincePress(c, p_chr_1, 13i32)) == (12i32)) as i32);
                        }
                    }
                }
                break 's1;
            }
            107i32 => {
                l_result_5 = F::BtlInput_IsPressed(c, p_chr_1, ((32768i32) as u32));
                break 's1;
            }
            108i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((512i32) as u32));
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((290i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((290i32) as i64 as u64)) as u32));
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((286i32) as i64 as u64)) as u32));
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            109i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((1024i32) as u32));
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((291i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((291i32) as i64 as u64)) as u32));
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((286i32) as i64 as u64)) as u32));
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            110i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((2048i32) as u32));
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((292i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((292i32) as i64 as u64)) as u32));
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((286i32) as i64 as u64)) as u32));
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            111i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((4096i32) as u32));
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((287i32) as i64 as u64)) as u32))) != 0) {
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            112i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((8192i32) as u32));
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((287i32) as i64 as u64)) as u32))) != 0) {
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            113i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((16384i32) as u32));
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((288i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((288i32) as i64 as u64)) as u32));
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((286i32) as i64 as u64)) as u32));
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            114i32 => {
                l_result_5 = F::BtlInput_IsCmdHeld(c, p_chr_1, ((32768i32) as u32));
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((289i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((289i32) as i64 as u64)) as u32));
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((286i32) as i64 as u64)) as u32));
                    l_result_5 = 1i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f ((l_result_5) ^ (l_invert_4));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_TestAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetStickX(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickX", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f F32(c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickX", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetStickY(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickY", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f F32(c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickY", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetStickLength(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickLength", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); F::Port_sqrtf(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickLength", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetStickAngle(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickAngle", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_x_2: F32 = F32(0);
    let mut l_y_3: F32 = F32(0);
    let r_ = 'f: {
    l_x_2 = F32(c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(508)).wrapping_add((((0i32) as u32)).wrapping_mul(4))));
    l_y_3 = F32(c.m.r32((((p_chr_1).wrapping_add(1392)).wrapping_add(508)).wrapping_add((((1i32) as u32)).wrapping_mul(4))));
    if (((((((rt::flt(F::Port_fabsf(c, l_x_2), F32(0x3c23d70a)) as i32)) != 0) && (((rt::flt(F::Port_fabsf(c, l_y_3), F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f F::Port_atan2f(c, l_x_2, rt::fneg(l_y_3));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickAngle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlInput_GetStickTurnFromFacing(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickTurnFromFacing", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = rt::fsub(F::BtlInput_GetStickAngle(c, p_chr_1), F::BtlAct_GetFacingRelCam(c, p_chr_1)); F::BtlUtil_WrapAngle(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlInput_GetStickTurnFromFacing", &PROBE_N, &[], true); }
    r_
}
