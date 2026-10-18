#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn late_a_b__ShenCfm_DebugFile(c: &mut Ctx, mut p_name_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_DebugFile", &PROBE_N, &[(p_name_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_DebugFile", &PROBE_N, &[], true); }
    r_
}

pub fn late_a_b__ShenCfm_PlatePlay(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_PlatePlay", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_on_3) as u32], false); }
    let r_ = 'f: {
    if ((p_on_3) != 0) {
        F::Flash_ClipGotoLabel(c, p_flash_1, p_ref_2, sym::late_a_b__str0);
    } else {
        F::Flash_ClipGotoLabel(c, p_flash_1, p_ref_2, sym::late_a_b__str1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_PlatePlay", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_Init(c: &mut Ctx, mut p_pack_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Init", &PROBE_N, &[(p_pack_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((88u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gShenCfm; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gShenCfm); F::memset(c, a0_, 0i32, 88u32) }; };
        F::late_a_b__ShenCfm_DebugFile(c, sym::late_a_b__str2);
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gShenCfm)).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gShenCfm)).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(2i32)) as u32)); let a_ = ((c.m.r32(sym::gShenCfm)).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(3i32)) as u32)); let a_ = ((c.m.r32(sym::gShenCfm)).wrapping_add(44)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::late_a_b__ShenCfm_DebugFile(c, sym::late_a_b__str3);
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gShenCfm)).wrapping_add(44)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gShenCfm)).wrapping_add(44)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::late_a_b__ShenCfm_DebugFile(c, sym::late_a_b__str4);
        { let a0_ = c.m.r32(sym::gShenCfm); let a1_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gShenCfm)).wrapping_add(44); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = c.m.r32(sym::gShenCfm); F::Flash_Play(c, a0_, 1i32) };
        F::late_a_b__ShenCfm_DebugFile(c, sym::late_a_b__str5);
        { let a0_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((4i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemHelp_Init(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(sym::gShenCfm)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::ItemHelp_Term(c);
    if (((((c.m.r32(sym::gShenCfm)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gShenCfm); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gShenCfm; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_IsOwned(c: &mut Ctx, mut p_cfm_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_IsOwned", &PROBE_N, &[(p_cfm_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_cfm_1).wrapping_add(76)) as i32);
        match sv2 {
            0i32 => {
                break 'f (((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((((c.m.r32((p_cfm_1).wrapping_add(80)) as i32)) as u32)); c.m.r8(a_) }) as i32)) & (1i32));
            }
            1i32 => {
                break 'f ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3112); c.m.r64(a_) }).wrapping_shr(((c.m.r32((p_cfm_1).wrapping_add(80)) as i32)) as u32)) & (((1i32) as u64)))) as i32);
            }
            2i32 => {
                break 'f ((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((rt::div_i32((c.m.r32((p_cfm_1).wrapping_add(80)) as i32), 64i32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }).wrapping_shr((rt::rem_i32((c.m.r32((p_cfm_1).wrapping_add(80)) as i32), 64i32)) as u32)) & (((1i32) as u64)))) as i32);
            }
            3i32 => {
                break 'f 0i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_IsOwned", &PROBE_N, &[], true); }
    r_
}

pub fn late_a_b__ShenCfm_SetClipUv(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_SetClipUv", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_SetClipUv", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(816) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 816);
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = (c.m.r32(sym::gShenCfm)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_shl((5i32) as u32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_1).wrapping_shl((5i32) as u32)).wrapping_add(32i32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(20), sym::late_a_b__str6, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    { let a0_ = c.m.r32(sym::gShenCfm); F::late_a_b__ShenCfm_SetClipUv(c, a0_, fp.wrapping_add(20), sym::late_a_b__str7, fp) };
                    { let a0_ = c.m.r32(sym::gShenCfm); F::late_a_b__ShenCfm_SetClipUv(c, a0_, fp.wrapping_add(20), sym::late_a_b__str8, fp) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        if ((((!(({ let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(84); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
            { let _ = F::sprintf(c, fp.wrapping_add(276), sym::late_a_b__str9, &[((3i32) as i64 as u64)]); };
            { let a0_ = c.m.r32(sym::gShenCfm); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(276), fp.wrapping_add(532)) };
            { let a0_ = c.m.r32(sym::gShenCfm); F::Flash_ClipSetColor(c, a0_, fp.wrapping_add(532), F32(0x3f000000)) };
        }
        if (({ let a0_ = c.m.r32(sym::gShenCfm); F::ShenCfm_IsOwned(c, a0_) }) != 0) {
            { let _ = F::sprintf(c, fp.wrapping_add(540), sym::late_a_b__str10, &[((1i32) as i64 as u64)]); };
            { let a0_ = c.m.r32(sym::gShenCfm); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(540), fp.wrapping_add(796)) };
            { let a0_ = c.m.r32(sym::gShenCfm); F::Flash_ClipSetColor(c, a0_, fp.wrapping_add(796), F32(0x3f000000)) };
        }
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let a0_ = (c.m.r32(sym::gShenCfm)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(76); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a0_ = { let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(80); (c.m.r32(a_) as i32) }; F::ItemHelp_Draw(c, a0_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn late_a_b__ShenCfm_SetPlate(c: &mut Ctx, mut p_cfm_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_SetPlate", &PROBE_N, &[(p_cfm_1) as u32, (p_on_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let r_ = 'f: {
        { let _ = { let a2_ = ((c.m.r32((p_cfm_1).wrapping_add(68)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::late_a_b__str11, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, p_cfm_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::late_a_b__ShenCfm_PlatePlay(c, p_cfm_1, fp, p_on_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_SetPlate", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_Open(c: &mut Ctx, mut p_kind_1: i32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Open", &PROBE_N, &[(p_kind_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_id_2; let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_kind_1; let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
    { let a0_ = c.m.r32(sym::gShenCfm); F::late_a_b__ShenCfm_SetPlate(c, a0_, 0i32) };
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    { let a0_ = c.m.r32(sym::gShenCfm); F::late_a_b__ShenCfm_SetPlate(c, a0_, 1i32) };
    { let v_ = (((p_kind_1) == (0i32)) as i32); let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let a0_ = c.m.r32(sym::gShenCfm); F::Flash_GotoLabel(c, a0_, sym::late_a_b__str12, ((((1i32) as i64 as u64)) as u8)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Open", &PROBE_N, &[], true); }
    r_
}

pub fn late_a_b__ShenCfm_WrapCursor(c: &mut Ctx, mut p_cursor_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_WrapCursor", &PROBE_N, &[(p_cursor_1) as u32], false); }
    let mut l_max_2: i32 = 0;
    let r_ = 'f: {
    l_max_2 = 2i32;
    if ((((((c.m.r32(p_cursor_1) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = l_max_2; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(p_cursor_1) as i32)) > (l_max_2)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "late_a_b__ShenCfm_WrapCursor", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_MoveCursor(c: &mut Ctx, mut p_cfm_1: u32, mut p_dir_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_MoveCursor", &PROBE_N, &[(p_cfm_1) as u32, (p_dir_2) as u32], false); }
    let r_ = 'f: {
    F::late_a_b__ShenCfm_SetPlate(c, p_cfm_1, 0i32);
    { let v_ = 0i32; let a_ = (p_cfm_1).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
    { let r_ = p_dir_2; let a_ = (p_cfm_1).wrapping_add(68); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    F::late_a_b__ShenCfm_WrapCursor(c, (p_cfm_1).wrapping_add(68));
    F::late_a_b__ShenCfm_SetPlate(c, p_cfm_1, 1i32);
    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_MoveCursor", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_PlayPlateOk(c: &mut Ctx, mut p_cfm_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_PlayPlateOk", &PROBE_N, &[(p_cfm_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let r_ = 'f: {
        { let _ = { let a2_ = ((c.m.r32((p_cfm_1).wrapping_add(68)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::late_a_b__str13, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, p_cfm_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, p_cfm_1, fp, sym::late_a_b__str14);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_PlayPlateOk", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_Update(c: &mut Ctx, mut p_answer_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Update", &PROBE_N, &[(p_answer_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(72); (c.m.r32(a_) as i32) };
        match sv2 {
            0i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    { let a0_ = c.m.r32(sym::gShenCfm); F::ShenCfm_MoveCursor(c, a0_, (1i32).wrapping_neg()) };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        { let a0_ = c.m.r32(sym::gShenCfm); F::ShenCfm_MoveCursor(c, a0_, 1i32) };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let a0_ = c.m.r32(sym::gShenCfm); F::ShenCfm_PlayPlateOk(c, a0_) };
                            if ((((((((((((!(({ let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(84); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(68); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0)) as i32)) != 0) || (((((({ let a0_ = c.m.r32(sym::gShenCfm); F::ShenCfm_IsOwned(c, a0_) }) != 0) && ((((({ let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(68); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                            } else {
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                if ((((({ let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(68); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                                    F::ItemHelp_Open(c);
                                } else {
                                    { let v_ = { let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(68); (c.m.r32(a_) as i32) }; let a_ = p_answer_1; c.m.w32(a_, (v_) as u32); };
                                    break 'f 1i32;
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let v_ = 1i32; let a_ = p_answer_1; c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                break 'f 1i32;
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((c.m.r64(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & ((((((512i32) | (1024i32))) as u64)).wrapping_shl((32i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gShenCfm)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                    F::ItemHelp_Close(c);
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ShenCfm_Close(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Close", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gShenCfm); F::Flash_GotoLabel(c, a0_, sym::late_a_b__str15, ((((1i32) as i64 as u64)) as u8)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenCfm_Close", &PROBE_N, &[], true); }
    r_
}
