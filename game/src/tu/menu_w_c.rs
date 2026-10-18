#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EvoTop_PlayVoice(c: &mut Ctx, mut p_menu_1: u32, mut p_line_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_PlayVoice", &PROBE_N, &[(p_menu_1) as u32, (p_line_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_menu_1).wrapping_add(116)); F::Voice_PlayWithSubtitle(c, a0_, 34505i32, p_line_2) };
    { let v_ = p_line_2; let a_ = (p_menu_1).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_PlayVoice", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_Wrap(c: &mut Ctx, mut p_value_1: i32, mut p_min_2: i32, mut p_max_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Wrap", &PROBE_N, &[(p_value_1) as u32, (p_min_2) as u32, (p_max_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_value_1) < (p_min_2)) as i32)) != 0) {
        p_value_1 = p_max_3;
    } else {
        if (((((p_value_1) > (p_max_3)) as i32)) != 0) {
            p_value_1 = p_min_2;
        }
    }
    break 'f p_value_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Wrap", &PROBE_N, &[], true); }
    r_
}

pub fn menu_w_c__EvoTop_ClipOnOff(c: &mut Ctx, mut p_flash_1: u32, mut p_name_2: u32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_w_c__EvoTop_ClipOnOff", &PROBE_N, &[(p_flash_1) as u32, (p_name_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), p_name_2, fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_w_c__str0);
        } else {
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_w_c__str1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_w_c__EvoTop_ClipOnOff", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_SetPlate(c: &mut Ctx, mut p_menu_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_SetPlate", &PROBE_N, &[(p_menu_1) as u32, (p_on_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        l_flash_3 = ((p_menu_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ((c.m.r32((p_menu_1).wrapping_add(184)) as i32)).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_w_c__str2, &[((a2_) as i64 as u64)]) }; };
        F::menu_w_c__EvoTop_ClipOnOff(c, l_flash_3, fp, p_on_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_SetPlate", &PROBE_N, &[], true); }
    r_
}

pub fn menu_w_c__EvoTop_ClipSetUv(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_w_c__EvoTop_ClipSetUv", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "menu_w_c__EvoTop_ClipSetUv", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_SetPlateText(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_SetPlateText", &PROBE_N, &[(p_menu_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_flash_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_menu_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(20), sym::menu_w_c__str3, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    F::menu_w_c__EvoTop_ClipSetUv(c, l_flash_2, fp.wrapping_add(20), sym::menu_w_c__str4, fp);
                    F::menu_w_c__EvoTop_ClipSetUv(c, l_flash_2, fp.wrapping_add(20), sym::menu_w_c__str5, fp);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_SetPlateText", &PROBE_N, &[], true); }
    r_
}

pub fn menu_w_c__EvoTop_PlateOk(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_w_c__EvoTop_PlateOk", &PROBE_N, &[(p_m_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let r_ = 'f: {
        { let _ = { let a2_ = ((c.m.r32((p_m_1).wrapping_add(184)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_w_c__str6, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, sym::menu_w_c__str7);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_w_c__EvoTop_PlateOk", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_Advance(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Advance", &PROBE_N, &[(p_menu_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Flash_Advance(c, ((p_menu_1).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn menu_w_c__EvoTop_Section(c: &mut Ctx, mut p_menu_1: u32, mut p_n_2: i32, mut p_host_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_w_c__EvoTop_Section", &PROBE_N, &[(p_menu_1) as u32, (p_n_2) as u32, (p_host_3) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_menu_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_menu_1).wrapping_add(4))).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_w_c__EvoTop_Section", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_Init(c: &mut Ctx, mut p_menu_1: u32, mut p_section_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Init", &PROBE_N, &[(p_menu_1) as u32, (p_section_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        l_flash_3 = ((p_menu_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let v_ = (c.m.r32(sym::gMenuArc7)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc7)).wrapping_add((((p_section_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = p_menu_1; c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(p_menu_1); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (p_menu_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 4i32, sym::menu_w_c__str8); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 2i32, sym::menu_w_c__str9); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 5i32, sym::menu_w_c__str10); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_menu_1).wrapping_add(56)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 1i32, sym::menu_w_c__str11); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (p_menu_1).wrapping_add(52); c.m.w32(a_, v_); };
        { let v_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 3i32, sym::menu_w_c__str12); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 9i32, sym::menu_w_c__str13); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 7i32, sym::menu_w_c__str14); let a_ = (p_menu_1).wrapping_add(120); c.m.w32(a_, v_); };
        { let a0_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 8i32, sym::menu_w_c__str15); let a1_ = c.m.r32((p_menu_1).wrapping_add(120)); let _ = (p_menu_1).wrapping_add(124); F::MsgWin_Init(c, a0_, a1_, 1i32) };
        F::MsgWin_Open(c);
        { let v_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 10i32, sym::menu_w_c__str16); let a_ = (p_menu_1).wrapping_add(116); c.m.w32(a_, v_); };
        { let a1_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 6i32, sym::menu_w_c__str17); F::Flash_Create(c, l_flash_3, a1_, (p_menu_1).wrapping_add(56)) };
        F::Flash_Play(c, l_flash_3, 1i32);
        { let a0_ = F::menu_w_c__EvoTop_Section(c, p_menu_1, 11i32, sym::menu_w_c__str18); F::DcSave_Init(c, a0_) };
        { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = (p_menu_1).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(((2000i32) as u32)); (c.m.r32(a_) as i32) }; let a_ = (p_menu_1).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_menu_1).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
        F::StreamSe_PlayDefault(c, 0i32, 68576i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_Term(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Term", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::IconWin_Term(c);
    F::MsgWin_Term(c);
    F::Flash_Destroy(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)));
    if (((((c.m.r32((p_m_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_m_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_m_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
    F::DcSave_Term(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_ScissorCloud1(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_ScissorCloud1", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 256i32, 18i32, 274i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_ScissorCloud1", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_ScissorCloud2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_ScissorCloud2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 384i32, 512i32, 18i32, 274i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_ScissorCloud2", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_ScissorOff(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_ScissorOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_ScissorOff", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_SetCloudClips(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_SetCloudClips", &PROBE_N, &[(p_m_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), ((0i32) as u32), sym::menu_w_c__str19, fp);
        F::Flash_ClipSetCallbackA(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, fid::EvoTop_ScissorCloud1, ((0i32) as u32));
        F::Flash_ClipSetCallbackB(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, fid::EvoTop_ScissorOff, ((0i32) as u32));
        F::Flash_FindLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), ((0i32) as u32), sym::menu_w_c__str20, fp);
        F::Flash_ClipSetCallbackA(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, fid::EvoTop_ScissorCloud2, ((0i32) as u32));
        F::Flash_ClipSetCallbackB(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, fid::EvoTop_ScissorOff, ((0i32) as u32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_SetCloudClips", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_UpdateFace(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_UpdateFace", &PROBE_N, &[(p_m_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), ((0i32) as u32), sym::menu_w_c__str21, fp);
        F::FlashAnim_Blink(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, (p_m_1).wrapping_add(108), 0i32);
        F::Flash_FindLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), ((0i32) as u32), sym::menu_w_c__str22, fp);
        if (((((((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0) && ((((!((F::DcSave_IsStarted(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::FlashAnim_Talk(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, (p_m_1).wrapping_add(112), 0i32);
        } else {
            F::FlashAnim_ShowNext2(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, 0i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_UpdateFace", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_Draw(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Draw", &PROBE_N, &[(p_m_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_i_2: i32 = 0;
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        if ((((!(((((c.m.r32((p_m_1).wrapping_add(200)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
            F::Flash_GotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_w_c__str23, ((((1i32) as i64 as u64)) as u8));
            { let r_ = 4i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((!(((((c.m.r32((p_m_1).wrapping_add(200)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
            if ((((c.m.r32((((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((2i32) as u32)))) != 0) {
                's1: {
                    let sv4: i32 = (c.m.r32((p_m_1).wrapping_add(184)) as i32);
                    match sv4 {
                        (-1i32) => {
                            { let v_ = 0i32; let a_ = (p_m_1).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
                            F::EvoTop_PlayVoice(c, p_m_1, 0i32);
                            break 's1;
                        }
                        0i32 => {
                            F::EvoTop_PlayVoice(c, p_m_1, 1i32);
                            break 's1;
                        }
                        1i32 => {
                            F::EvoTop_PlayVoice(c, p_m_1, 2i32);
                            break 's1;
                        }
                        2i32 => {
                            F::EvoTop_PlayVoice(c, p_m_1, 3i32);
                            break 's1;
                        }
                        _ => {
                        }
                    }
                }
                F::EvoTop_SetPlate(c, p_m_1, 1i32);
                { let r_ = 8i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        F::EvoTop_UpdateFace(c, p_m_1);
        F::EvoTop_SetPlateText(c, p_m_1);
        F::EvoTop_SetCloudClips(c, p_m_1);
        l_flash_3 = ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l2: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_w_c__str24, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                    F::FlashAnim_Scroll(c, l_flash_3, fp, fp.wrapping_add(8), (p_m_1).wrapping_add(204), ((0i32) as u32), F32(0x3e91a2b3), F32(0x00000000));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = c.m.r32((p_m_1).wrapping_add(52)); F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        {
            l_i_2 = 0i32;
            'l4: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    F::Flash_Draw(c, ((p_m_1).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::IconWin_Draw(c);
        { let a2_ = (c.m.r32((p_m_1).wrapping_add(188)) as i32); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        F::DcSave_Update(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_Input(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Input", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((((c.m.r32((((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((p_m_1).wrapping_add(200)) as i32)) & (1i32))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((F::DcSave_GetState(c)) != 0) {
        break 'f;
    }
    if ((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_m_1).wrapping_add(188)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let a1_ = ((c.m.r32((p_m_1).wrapping_add(184)) as i32)).wrapping_add(1i32); F::EvoTop_PlayVoice(c, p_m_1, a1_) };
    }
    if ((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) != 0) || ((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_m_1).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
    }
    if ((((!(((((c.m.r32((p_m_1).wrapping_add(200)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
            F::EvoTop_SetPlate(c, p_m_1, 0i32);
            { let a_ = (p_m_1).wrapping_add(184); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            { let v_ = { let a0_ = (c.m.r32((p_m_1).wrapping_add(184)) as i32); F::EvoTop_Wrap(c, a0_, 0i32, 2i32) }; let a_ = (p_m_1).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
            F::EvoTop_SetPlate(c, p_m_1, 1i32);
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
            { let a1_ = ((c.m.r32((p_m_1).wrapping_add(184)) as i32)).wrapping_add(1i32); F::EvoTop_PlayVoice(c, p_m_1, a1_) };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                F::EvoTop_SetPlate(c, p_m_1, 0i32);
                { let a_ = (p_m_1).wrapping_add(184); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = { let a0_ = (c.m.r32((p_m_1).wrapping_add(184)) as i32); F::EvoTop_Wrap(c, a0_, 0i32, 2i32) }; let a_ = (p_m_1).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
                F::EvoTop_SetPlate(c, p_m_1, 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                { let a1_ = ((c.m.r32((p_m_1).wrapping_add(184)) as i32)).wrapping_add(1i32); F::EvoTop_PlayVoice(c, p_m_1, a1_) };
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    F::menu_w_c__EvoTop_PlateOk(c, p_m_1);
                    's1: {
                        let sv2: i32 = (c.m.r32((p_m_1).wrapping_add(184)) as i32);
                        match sv2 {
                            0i32 => {
                                F::Flash_GotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_w_c__str25, ((((1i32) as i64 as u64)) as u8));
                                { let r_ = 1i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 1i32; let a_ = (p_m_1).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                                break 's1;
                            }
                            1i32 => {
                                F::Flash_GotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_w_c__str26, ((((1i32) as i64 as u64)) as u8));
                                { let r_ = 1i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 2i32; let a_ = (p_m_1).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                                break 's1;
                            }
                            2i32 => {
                                F::Flash_GotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_w_c__str27, ((((1i32) as i64 as u64)) as u8));
                                { let r_ = 32i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                F::EvoTop_PlayVoice(c, p_m_1, 4i32);
                                break 's1;
                            }
                            _ => {
                            }
                        }
                    }
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (p_m_1).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                        { let r_ = 1i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                            F::DcSave_Start(c);
                        }
                    }
                }
            }
        }
        { let a_ = (p_m_1).wrapping_add(196); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    } else {
        if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) {
            if ((((((c.m.r32((p_m_1).wrapping_add(188)) as i32)) == (3i32)) as i32)) != 0) {
                F::menu_w_c__EvoTop_PlateOk(c, p_m_1);
            }
            if ((((((c.m.r32((p_m_1).wrapping_add(188)) as i32)) == (16i32)) as i32)) != 0) {
                { let r_ = 32i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                F::Flash_GotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_w_c__str28, ((((1i32) as i64 as u64)) as u8));
                F::EvoTop_PlayVoice(c, p_m_1, 3i32);
            } else {
                { let a_ = (p_m_1).wrapping_add(188); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a1_ = (c.m.r32((p_m_1).wrapping_add(188)) as i32); F::EvoTop_PlayVoice(c, p_m_1, a1_) };
            }
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                { let r_ = 32i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                F::EvoTop_PlayVoice(c, p_m_1, 3i32);
                F::Flash_GotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_w_c__str29, ((((1i32) as i64 as u64)) as u8));
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
            }
        }
    }
    if ((((((c.m.r32((p_m_1).wrapping_add(196)) as i32)) == (3600i32)) as i32)) != 0) {
        if (((((c.m.r32((p_m_1).wrapping_add(200)) as i32)) & (32i32))) != 0) {
            { let v_ = 0i32; let a_ = (p_m_1).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
        } else {
            F::EvoTop_PlayVoice(c, p_m_1, 17i32);
        }
    }
    if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_m_1).wrapping_add(188)) as i32)) == (17i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_m_1).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Input", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_Leave(c: &mut Ctx, mut p_m_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Leave", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_m_1).wrapping_add(200)) as i32)) & (1i32))) != 0) {
        if ((((((((!(((((c.m.r32((p_m_1).wrapping_add(200)) as i32)) & (2i32))) != 0)) as i32)) != 0) && ((F::DcSave_IsDone(c)) != 0)) as i32)) != 0) {
            if ((((((c.m.r32((p_m_1).wrapping_add(192)) as i32)) == (0i32)) as i32)) != 0) {
                F::Flash_GotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_w_c__str30, ((((1i32) as i64 as u64)) as u8));
            } else {
                F::Flash_GotoLabel(c, ((p_m_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_w_c__str31, ((((1i32) as i64 as u64)) as u8));
            }
            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
            F::MsgWin_Close(c);
            F::IconWin_Close(c);
            { let v_ = 30i32; let a_ = (p_m_1).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
            { let r_ = 2i32; let a_ = (p_m_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((F::DcSave_IsStarted(c)) != 0) {
            F::Voice_FadeOutStep(c, 0i32);
        }
        if (((((c.m.r32((p_m_1).wrapping_add(200)) as i32)) & (2i32))) != 0) {
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::StreamSe_FadeOutStep(c, 0i32);
                if ((((((c.m.r32((p_m_1).wrapping_add(192)) as i32)) == (0i32)) as i32)) != 0) {
                    F::Bgm_FadeOutStep(c);
                }
            }
            if ((((({ let a_ = (p_m_1).wrapping_add(196); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == (0i32)) as i32)) != 0) {
                { let v_ = (c.m.r32((p_m_1).wrapping_add(184)) as i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(((2000i32) as u32)); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
        } else {
            break 'f 0i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Leave", &PROBE_N, &[], true); }
    r_
}

pub fn EvoTop_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(208) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 208);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 208u32); };
        F::EvoTop_Init(c, fp, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::ColorFade_Update(c);
            F::Snd_Update(c);
            F::EvoTop_Advance(c, fp);
            F::EvoTop_Draw(c, fp);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::EvoTop_Leave(c, fp)) != 0) {
                break 'l1;
            }
            F::EvoTop_Input(c, fp);
        }
        F::EvoTop_Term(c, fp);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32((fp).wrapping_add(192)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoTop_Run", &PROBE_N, &[], true); }
    r_
}
