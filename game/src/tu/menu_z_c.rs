#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn menu_z_c__DcMenu_SetUv(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_SetUv", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, p_parent_2, p_name_3, fp);
        F::Flash_ClipSetUv(c, p_flash_1, fp, p_uv_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_SetUv", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_c__DcMenu_Say(c: &mut Ctx, mut p_menu_1: u32, mut p_line_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_Say", &PROBE_N, &[(p_menu_1) as u32, (p_line_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_menu_1).wrapping_add(132)); F::Voice_PlayWithSubtitle(c, a0_, 33688i32, p_line_2) };
    { let v_ = p_line_2; let a_ = (p_menu_1).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_Say", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_c__DcMenu_VoiceEnded(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_VoiceEnded", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_VoiceEnded", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_Wrap(c: &mut Ctx, mut p_value_1: i32, mut p_lo_2: i32, mut p_hi_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Wrap", &PROBE_N, &[(p_value_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_value_1) < (p_lo_2)) as i32)) != 0) {
        p_value_1 = p_hi_3;
    } else {
        if (((((p_value_1) > (p_hi_3)) as i32)) != 0) {
            p_value_1 = p_lo_2;
        }
    }
    break 'f p_value_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Wrap", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_AnimGuide(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_AnimGuide", &PROBE_N, &[(p_menu_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = (p_menu_1).wrapping_add(8);
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_c__str0, fp);
        F::FlashAnim_Blink(c, l_flash_2, fp, ((p_menu_1).wrapping_add(8)).wrapping_add(112), 0i32);
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_c__str1, fp);
        if (((((((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0) && ((((!((F::DcSave_IsStarted(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::FlashAnim_Talk(c, l_flash_2, fp, ((p_menu_1).wrapping_add(8)).wrapping_add(116), 0i32);
        } else {
            F::FlashAnim_ShowNext2(c, l_flash_2, fp, 0i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_AnimGuide", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_AnimBg(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_AnimBg", &PROBE_N, &[(p_menu_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = (p_menu_1).wrapping_add(8);
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_c__str2, fp);
        F::FlashAnim_Scroll(c, l_flash_2, fp, fp.wrapping_add(8), (p_menu_1).wrapping_add(196), ((0i32) as u32), F32(0xbe1c09c0), F32(0x00000000));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_AnimBg", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_DrawPlates(c: &mut Ctx, mut p_flash_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_DrawPlates", &PROBE_N, &[(p_flash_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_i_2) == (1i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 512i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 96i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((((l_i_2) == (2i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                            { let v_ = 512i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 32i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 64i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                            { let v_ = 512i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (l_i_2).wrapping_shl((5i32) as u32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((l_i_2).wrapping_shl((5i32) as u32)).wrapping_add(32i32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(20), sym::menu_z_c__str3, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::menu_z_c__DcMenu_SetUv(c, p_flash_1, fp.wrapping_add(20), sym::menu_z_c__str4, fp);
                    F::menu_z_c__DcMenu_SetUv(c, p_flash_1, fp.wrapping_add(20), sym::menu_z_c__str5, fp);
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_DrawPlates", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_c__DcMenu_LightClip(c: &mut Ctx, mut p_flash_1: u32, mut p_name_2: u32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_LightClip", &PROBE_N, &[(p_flash_1) as u32, (p_name_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), p_name_2, fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_z_c__str6);
        } else {
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_z_c__str7);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_LightClip", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_c__DcMenu_FlashPlate(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_FlashPlate", &PROBE_N, &[(p_menu_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        { let _ = { let a2_ = ((c.m.r32((p_menu_1).wrapping_add(200)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_z_c__str8, &[((a2_) as i64 as u64)]) }; };
        l_flash_2 = (p_menu_1).wrapping_add(8);
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_z_c__str9);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_z_c__str10);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_FlashPlate", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_LightPlate(c: &mut Ctx, mut p_menu_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_LightPlate", &PROBE_N, &[(p_menu_1) as u32, (p_on_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        l_flash_3 = (p_menu_1).wrapping_add(8);
        { let v_ = { let a0_ = (c.m.r32((p_menu_1).wrapping_add(200)) as i32); F::DcMenu_Wrap(c, a0_, 0i32, (3i32).wrapping_sub(1i32)) }; let a_ = (p_menu_1).wrapping_add(200); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a2_ = ((c.m.r32((p_menu_1).wrapping_add(200)) as i32)).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_z_c__str11, &[((a2_) as i64 as u64)]) }; };
        F::menu_z_c__DcMenu_LightClip(c, l_flash_3, fp, p_on_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_LightPlate", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_ResetBlink(c: &mut Ctx, mut p_view_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_ResetBlink", &PROBE_N, &[(p_view_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = (p_view_1).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_ResetBlink", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_Start(c: &mut Ctx, mut p_menu_1: u32, mut p_visits_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Start", &PROBE_N, &[(p_menu_1) as u32, (p_visits_2) as u32], false); }
    let r_ = 'f: {
    F::DcMenu_ResetBlink(c, (p_menu_1).wrapping_add(8));
    if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4616); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4616); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (p_menu_1).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = p_visits_2; let a_ = (p_menu_1).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
    { let v_ = 30i32; let a_ = (p_menu_1).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1684); (c.m.r32(a_) as i32) }; let a_ = (p_menu_1).wrapping_add(200); c.m.w32(a_, (v_) as u32); };
    's1: {
        let sv3: i32 = (c.m.r32((p_menu_1).wrapping_add(200)) as i32);
        match sv3 {
            (-1i32) => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(200); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            0i32 => {
                { let v_ = 11i32; let a_ = (p_menu_1).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 22i32; let a_ = (p_menu_1).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 15i32; let a_ = (p_menu_1).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Start", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_Update(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Update", &PROBE_N, &[(p_menu_1) as u32], false); }
    let r_ = 'f: {
    F::Flash_Advance(c, (p_menu_1).wrapping_add(8));
    if ((((!(((((c.m.r32((p_menu_1).wrapping_add(220)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
        { let a1_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1680); (c.m.r32(a_) as i32) }; F::DcMenu_Start(c, p_menu_1, a1_) };
        { let r_ = 4i32; let a_ = (p_menu_1).wrapping_add(220); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((!(((((c.m.r32((p_menu_1).wrapping_add(220)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
        if ((((c.m.r32(((p_menu_1).wrapping_add(8)).wrapping_add(8))) & (((2i32) as u32)))) != 0) {
            F::DcMenu_LightPlate(c, p_menu_1, 1i32);
            { let a1_ = (c.m.r32((p_menu_1).wrapping_add(208)) as i32); F::menu_z_c__DcMenu_Say(c, p_menu_1, a1_) };
            { let r_ = 8i32; let a_ = (p_menu_1).wrapping_add(220); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if ((((!(((((c.m.r32((p_menu_1).wrapping_add(220)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
        if ((((((((((((((c.m.r32((p_menu_1).wrapping_add(208)) as i32)) == (11i32)) as i32)) != 0) || ((((((c.m.r32((p_menu_1).wrapping_add(208)) as i32)) == (15i32)) as i32)) != 0)) as i32)) != 0) || ((((((c.m.r32((p_menu_1).wrapping_add(208)) as i32)) == (22i32)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = (p_menu_1).wrapping_add(220); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_c__DcMenu_Section(c: &mut Ctx, mut p_menu_1: u32, mut p_n_2: i32, mut p_host_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_Section", &PROBE_N, &[(p_menu_1) as u32, (p_n_2) as u32, (p_host_3) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_menu_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_menu_1).wrapping_add(4))).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_Section", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_Init(c: &mut Ctx, mut p_menu_1: u32, mut p_section_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Init", &PROBE_N, &[(p_menu_1) as u32, (p_section_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = (c.m.r32(sym::gMenuArc8)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc8)).wrapping_add((((p_section_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = p_menu_1; c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(p_menu_1); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (p_menu_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 6i32, sym::menu_z_c__str12); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = ((p_menu_1).wrapping_add(8)).wrapping_add(44); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 11i32, sym::menu_z_c__str13); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 7i32, sym::menu_z_c__str14); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 10i32, sym::menu_z_c__str15); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 8i32, sym::menu_z_c__str16); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_menu_1).wrapping_add(8)).wrapping_add(48)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 5i32, sym::menu_z_c__str17); F::Flash_Create(c, (p_menu_1).wrapping_add(8), a1_, ((p_menu_1).wrapping_add(8)).wrapping_add(48)) };
        F::Flash_Play(c, (p_menu_1).wrapping_add(8), 1i32);
        { let v_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 1i32, sym::menu_z_c__str18); let a_ = (p_menu_1).wrapping_add(132); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 9i32, sym::menu_z_c__str19); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 3i32, sym::menu_z_c__str20); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 4i32, sym::menu_z_c__str21); let a_ = (p_menu_1).wrapping_add(128); c.m.w32(a_, v_); };
        { let a0_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 2i32, sym::menu_z_c__str22); let a1_ = c.m.r32((p_menu_1).wrapping_add(128)); let _ = (((p_menu_1).wrapping_add(136)) as i32); F::MsgWin_Init(c, a0_, a1_, 0i32) };
        F::MsgWin_Open(c);
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_menu_1).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
        { let a0_ = F::menu_z_c__DcMenu_Section(c, p_menu_1, 12i32, sym::menu_z_c__str23); F::DcSave_Init(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_Term(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Term", &PROBE_N, &[(p_menu_1) as u32], false); }
    let r_ = 'f: {
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    F::Flash_Destroy(c, (p_menu_1).wrapping_add(8));
    F::DcSave_Term(c);
    if (((((c.m.r32((p_menu_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_menu_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_menu_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_Draw(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Draw", &PROBE_N, &[(p_menu_1) as u32], false); }
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
    l_flash_2 = (p_menu_1).wrapping_add(8);
    F::DcMenu_AnimGuide(c, p_menu_1);
    F::DcMenu_AnimBg(c, p_menu_1);
    F::DcMenu_DrawPlates(c, l_flash_2);
    { let a0_ = c.m.r32(((p_menu_1).wrapping_add(8)).wrapping_add(44)); F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
    F::IconWin_Draw(c);
    F::Flash_Draw(c, l_flash_2);
    { let a2_ = (c.m.r32((p_menu_1).wrapping_add(208)) as i32); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
    F::DcSave_Update(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_UpdateVoice(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_UpdateVoice", &PROBE_N, &[(p_menu_1) as u32], false); }
    let mut l_n_2: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv3: i32 = (c.m.r32((p_menu_1).wrapping_add(208)) as i32);
        match sv3 {
            0i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 1i32);
                }
                break 's1;
            }
            1i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 2i32);
                }
                break 's1;
            }
            2i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 3i32);
                }
                break 's1;
            }
            3i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 4i32);
                }
                break 's1;
            }
            4i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 10i32);
                }
                break 's1;
            }
            5i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    if ((((((c.m.r32((p_menu_1).wrapping_add(216)) as i32)) < (6i32)) as i32)) != 0) {
                        F::menu_z_c__DcMenu_Say(c, p_menu_1, 6i32);
                    } else {
                        l_n_2 = ((c.m.r32((p_menu_1).wrapping_add(216)) as i32)).wrapping_sub(5i32);
                        if (((((l_n_2) >= (4i32)) as i32)) != 0) {
                            l_n_2 = 3i32;
                        }
                        l_n_2 = ((F::Rand_Range(c, (((l_n_2).wrapping_add(1i32)) as u32))) as i32);
                        's2: {
                            let sv4: i32 = l_n_2;
                            match sv4 {
                                0i32 => {
                                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 6i32);
                                    break 's2;
                                }
                                1i32 => {
                                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 7i32);
                                    break 's2;
                                }
                                2i32 => {
                                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 8i32);
                                    break 's2;
                                }
                                3i32 => {
                                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 9i32);
                                    break 's2;
                                }
                                _ => {
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            6i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 10i32);
                }
                break 's1;
            }
            7i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 10i32);
                }
                break 's1;
            }
            8i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 10i32);
                }
                break 's1;
            }
            9i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    F::menu_z_c__DcMenu_Say(c, p_menu_1, 10i32);
                }
                break 's1;
            }
            10i32 => {
                if ((((((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    if ((((((c.m.r32((p_menu_1).wrapping_add(200)) as i32)) == (0i32)) as i32)) != 0) {
                        F::menu_z_c__DcMenu_Say(c, p_menu_1, 11i32);
                    } else {
                        if ((((((c.m.r32((p_menu_1).wrapping_add(200)) as i32)) == (1i32)) as i32)) != 0) {
                            F::menu_z_c__DcMenu_Say(c, p_menu_1, 22i32);
                        } else {
                            if ((((((c.m.r32((p_menu_1).wrapping_add(200)) as i32)) == (2i32)) as i32)) != 0) {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 15i32);
                            }
                        }
                    }
                }
                break 's1;
            }
            11i32 => {
                if ((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) {
                    's3: {
                        let sv5: i32 = ((F::Rand_Range(c, ((3i32) as u32))) as i32);
                        match sv5 {
                            0i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 12i32);
                                break 's3;
                            }
                            1i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 13i32);
                                break 's3;
                            }
                            2i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 14i32);
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
                break 's1;
            }
            12i32 | 13i32 | 14i32 => {
                break 's1;
            }
            15i32 => {
                if ((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) {
                    's4: {
                        let sv6: i32 = ((F::Rand_Range(c, ((3i32) as u32))) as i32);
                        match sv6 {
                            0i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 16i32);
                                break 's4;
                            }
                            1i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 17i32);
                                break 's4;
                            }
                            2i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 18i32);
                                break 's4;
                            }
                            _ => {
                            }
                        }
                    }
                }
                break 's1;
            }
            16i32 | 17i32 | 18i32 | 19i32 | 20i32 | 21i32 => {
                break 's1;
            }
            22i32 => {
                if ((F::menu_z_c__DcMenu_VoiceEnded(c)) != 0) {
                    's5: {
                        let sv7: i32 = ((F::Rand_Range(c, ((3i32) as u32))) as i32);
                        match sv7 {
                            0i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 23i32);
                                break 's5;
                            }
                            1i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 24i32);
                                break 's5;
                            }
                            2i32 => {
                                F::menu_z_c__DcMenu_Say(c, p_menu_1, 25i32);
                                break 's5;
                            }
                            _ => {
                            }
                        }
                    }
                }
                break 's1;
            }
            23i32 | 24i32 | 25i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_UpdateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_c__DcMenu_SayPlate(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_SayPlate", &PROBE_N, &[(p_menu_1) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_menu_1).wrapping_add(200)) as i32)) == (0i32)) as i32)) != 0) {
        F::menu_z_c__DcMenu_Say(c, p_menu_1, 11i32);
    } else {
        if ((((((c.m.r32((p_menu_1).wrapping_add(200)) as i32)) == (1i32)) as i32)) != 0) {
            F::menu_z_c__DcMenu_Say(c, p_menu_1, 22i32);
        } else {
            F::menu_z_c__DcMenu_Say(c, p_menu_1, 15i32);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_c__DcMenu_SayPlate", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_Input(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Input", &PROBE_N, &[(p_menu_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((((c.m.r32(((p_menu_1).wrapping_add(8)).wrapping_add(8))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((p_menu_1).wrapping_add(220)) as i32)) & (1i32))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((F::DcSave_GetState(c)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
        F::DcMenu_LightPlate(c, p_menu_1, 0i32);
        { let a_ = (p_menu_1).wrapping_add(200); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        F::DcMenu_LightPlate(c, p_menu_1, 1i32);
        F::menu_z_c__DcMenu_SayPlate(c, p_menu_1);
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
            F::DcMenu_LightPlate(c, p_menu_1, 0i32);
            { let a_ = (p_menu_1).wrapping_add(200); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            F::DcMenu_LightPlate(c, p_menu_1, 1i32);
            F::menu_z_c__DcMenu_SayPlate(c, p_menu_1);
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                F::menu_z_c__DcMenu_FlashPlate(c, p_menu_1);
                { let r_ = 1i32; let a_ = (p_menu_1).wrapping_add(220); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_menu_1).wrapping_add(200)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 54i32; let a_ = (p_menu_1).wrapping_add(204); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((((c.m.r32((p_menu_1).wrapping_add(200)) as i32)) == (1i32)) as i32)) != 0) {
                        { let v_ = 55i32; let a_ = (p_menu_1).wrapping_add(204); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 56i32; let a_ = (p_menu_1).wrapping_add(204); c.m.w32(a_, (v_) as u32); };
                    }
                }
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::MsgWin_Close(c);
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(204); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 1i32; let a_ = (p_menu_1).wrapping_add(220); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        F::DcSave_Start(c);
                    }
                }
            }
        }
    }
    F::DcMenu_UpdateVoice(c, p_menu_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Input", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_CheckLeave(c: &mut Ctx, mut p_menu_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_CheckLeave", &PROBE_N, &[(p_menu_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_menu_1).wrapping_add(220)) as i32)) & (1i32))) != 0) {
        if ((((!(((((c.m.r32((p_menu_1).wrapping_add(220)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            if ((F::DcSave_IsDone(c)) != 0) {
                if ((((((c.m.r32((p_menu_1).wrapping_add(204)) as i32)) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
                { let r_ = 2i32; let a_ = (p_menu_1).wrapping_add(220); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                F::Flash_GotoLabel(c, (p_menu_1).wrapping_add(8), sym::menu_z_c__str24, ((((1i32) as i64 as u64)) as u8));
                F::IconWin_Close(c);
                F::MsgWin_Close(c);
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
            }
        }
        if ((F::DcSave_IsStarted(c)) != 0) {
            F::Voice_FadeOutStep(c, 0i32);
        }
        if (((((c.m.r32((p_menu_1).wrapping_add(220)) as i32)) & (2i32))) != 0) {
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
            } else {
                break 'f 1i32;
            }
        } else {
            break 'f 0i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_CheckLeave", &PROBE_N, &[], true); }
    r_
}

pub fn DcMenu_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 224u32); };
        F::DcMenu_Init(c, fp, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::ColorFade_Update(c);
            F::Snd_Update(c);
            F::DcMenu_Update(c, fp);
            F::DcMenu_Draw(c, fp);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::DcMenu_CheckLeave(c, fp)) != 0) {
                break 'l1;
            }
            F::DcMenu_Input(c, fp);
        }
        { let v_ = (c.m.r32((fp).wrapping_add(200)) as i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1684); c.m.w32(a_, (v_) as u32); };
        F::DcMenu_Term(c, fp);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32((fp).wrapping_add(204)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcMenu_Run", &PROBE_N, &[], true); }
    r_
}
