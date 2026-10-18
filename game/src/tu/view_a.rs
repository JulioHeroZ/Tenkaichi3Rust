#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn GetWin_Init(c: &mut Ctx, mut p_pack_1: u32, mut p_lang_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Init", &PROBE_N, &[(p_pack_1) as u32, (p_lang_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((848u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gGetWin; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gGetWin); F::memset(c, a0_, 0i32, 848u32) }; };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(2i32)) as u32)); let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(3i32)) as u32)); let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(2i32)) as u32)); let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(2i32)) as u32)); let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((4i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gGetWin)).wrapping_add(764)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((8i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((7i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((9i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((11i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((11i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); F::TextBox_SetColor(c, a0_, 4294901888u32) };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                    { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 1i32) };
                    { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)); F::TextBox_SetUnkC(c, a0_, 256i32, 0i32) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((10i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(816); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gGetWin)).wrapping_add(720); let a1_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add(((((if (((((p_lang_2) == (1i32)) as i32)) != 0) { 5i32 } else { 6i32 })) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gGetWin)).wrapping_add(764); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = (c.m.r32(sym::gGetWin)).wrapping_add(720); F::Flash_Play(c, a0_, 1i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Init", &PROBE_N, &[], true); }
    r_
}

pub fn GetWin_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gGetWin)).wrapping_add(720); F::Flash_Destroy(c, a0_) };
    if (((((c.m.r32(sym::gGetWin)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gGetWin); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gGetWin; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Term", &PROBE_N, &[], true); }
    r_
}

pub fn GetWin_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_kind_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        l_flash_1 = (c.m.r32(sym::gGetWin)).wrapping_add(720);
        l_kind_2 = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(820); (c.m.r32(a_) as i32) };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(720)).wrapping_add((((l_i_3) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        's3: {
            let sv4: i32 = l_kind_2;
            match sv4 {
                3i32 | 4i32 | 5i32 | 9i32 => {
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's3;
                }
                1i32 | 8i32 => {
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's3;
                }
                6i32 => {
                    { let v_ = 192i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's3;
                }
                _ => {
                }
            }
        }
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str0, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if (((((((((l_kind_2) >= (7i32)) as i32)) != 0) && (((((l_kind_2) <= (9i32)) as i32)) != 0)) as i32)) != 0) {
            F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
        } else {
            F::Flash_ClipSetTex(c, l_flash_1, fp, 0i32);
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        's4: {
            let sv5: i32 = l_kind_2;
            match sv5 {
                0i32 => {
                    { let v_ = 64i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                1i32 => {
                    { let v_ = 128i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 192i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                3i32 => {
                    { let v_ = 192i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                4i32 => {
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                5i32 => {
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                6i32 => {
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 192i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                8i32 => {
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(128i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                7i32 => {
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                9i32 => {
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 192i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                _ => {
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), (if (((((l_kind_2) != (8i32)) as i32)) != 0) { sym::view_a__str1 } else { sym::view_a__str2 }), fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str3, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, (((((((l_kind_2) == (8i32)) as i32)) as i64 as u64)) as u8)); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str4, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, (((((((l_kind_2) != (8i32)) as i32)) as i64 as u64)) as u8)); };
        {
            l_i_3 = 0i32;
            'l5: loop {
                if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::view_a__str5, &[((l_i_3) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, (((((((l_kind_2) == (6i32)) as i32)) as i64 as u64)) as u8)); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((((l_kind_2) == (6i32)) as i32)) != 0) {
            { let a4_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::view_a__str6, 0i32, 7i32, a4_, 32i32, 32i32, 0i32) };
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str7, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, (((((((l_kind_2) == (8i32)) as i32)) as i64 as u64)) as u8)); };
        if ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str8, fp);
            { let a4_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add(((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(824); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
        } else {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str9, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str10, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        { let a0_ = (c.m.r32(sym::gGetWin)).wrapping_add(720); F::Flash_Draw(c, a0_) };
        if ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
            if (((((l_kind_2) == (8i32)) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(840); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add(((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(140)); F::TextBox_SetLineOffsets(c, a0_, 12i32, 0i32, 0i32, 0i32, 0i32) };
                }
                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str11, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            }
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str12, fp);
            if (((((l_kind_2) != (6i32)) as i32)) != 0) {
                { let a4_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add(((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
            }
            { let a0_ = ((c.m.r32(sym::gGetWin)).wrapping_add(20)).wrapping_add(((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(140)); F::TextBox_SetLineOffsets(c, a0_, 0i32, 0i32, 0i32, 0i32, 0i32) };
        } else {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str13, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, (((((((l_kind_2) == (6i32)) as i32)) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::view_a__str14, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        F::Font_FlushAll(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn GetWin_Open(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Open", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gGetWin)).wrapping_add(720); F::Flash_GotoLabel(c, a0_, sym::view_a__str15, ((((1i32) as i64 as u64)) as u8)) };
    { let _ = { let a1_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); (c.m.r32(a_) as i32) }; F::Snd_PlaySe(c, ((2i32) as u32), a1_) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Open", &PROBE_N, &[], true); }
    r_
}

pub fn GetWin_Close(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Close", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gGetWin)).wrapping_add(720); F::Flash_GotoLabel(c, a0_, sym::view_a__str16, ((((1i32) as i64 as u64)) as u8)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Close", &PROBE_N, &[], true); }
    r_
}

pub fn GetWin_Next(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Next", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gGetWin)).wrapping_add(720); F::Flash_GotoLabel(c, a0_, sym::view_a__str17, ((((1i32) as i64 as u64)) as u8)) };
    { let _ = { let a1_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); (c.m.r32(a_) as i32) }; F::Snd_PlaySe(c, ((2i32) as u32), a1_) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Next", &PROBE_N, &[], true); }
    r_
}

pub fn GetWin_IsAnimating(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetWin_IsAnimating", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (((c.m.r32(sym::gGetWin)).wrapping_add(720)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "GetWin_IsAnimating", &PROBE_N, &[], true); }
    r_
}

pub fn GetWin_Setup(c: &mut Ctx, mut p_kind_1: i32, mut p_value_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Setup", &PROBE_N, &[(p_kind_1) as u32, (p_value_2) as u32], false); }
    let mut l_type_3: u8 = 0;
    let r_ = 'f: {
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_value_2).wrapping_abs(); let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(840); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let v_ = 0i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(824); c.m.w32(a_, (v_) as u32); };
    if (((((p_kind_1) == (2i32)) as i32)) != 0) {
        l_type_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(816); c.m.r32(a_) }).wrapping_add((((p_value_2) as u32)).wrapping_mul(40)); c.m.r8(a_) };
        p_kind_1 = 5i32;
        if (((((((l_type_3) as i32)) == (1i32)) as i32)) != 0) {
            p_kind_1 = 4i32;
        }
        if (((((((l_type_3) as i32)) == (0i32)) as i32)) != 0) {
            p_kind_1 = 4i32;
            { let o_ = p_kind_1; let n_ = (o_).wrapping_sub(1); p_kind_1 = n_; };
        }
        { let v_ = 2i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); c.m.w32(a_, (v_) as u32); };
        { let v_ = 6i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((p_kind_1) == (7i32)) as i32)) != 0) {
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); c.m.w32(a_, (v_) as u32); };
            { let v_ = 7i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((p_kind_1) == (8i32)) as i32)) != 0) {
                { let v_ = 8i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) < (3i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) < (7i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) >= (7i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) < (14i32)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 2i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((((((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) >= (14i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) < (19i32)) as i32)) != 0)) as i32)) != 0) {
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((((((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) >= (19i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) < (35i32)) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if ((((((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) >= (35i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) < (40i32)) as i32)) != 0)) as i32)) != 0) {
                                        { let v_ = 5i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        if ((((((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) >= (40i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) < (44i32)) as i32)) != 0)) as i32)) != 0) {
                                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            if ((((((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) >= (44i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); (c.m.r32(a_) as i32) }) < (48i32)) as i32)) != 0)) as i32)) != 0) {
                                                { let v_ = 7i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                { let v_ = 3i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(824); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); c.m.w32(a_, (v_) as u32); };
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a0_ = ({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(824); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(824); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Font_CountLines(c, a0_) }) > (0i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(840); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if (((((p_kind_1) == (0i32)) as i32)) != 0) {
                    { let v_ = { let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(832); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(824); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); c.m.w32(a_, (v_) as u32); };
                } else {
                    if (((((p_kind_1) == (6i32)) as i32)) != 0) {
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((((p_kind_1) == (1i32)) as i32)) != 0) {
                            { let r_ = 56i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 7i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if (((((p_kind_1) == (9i32)) as i32)) != 0) {
                                { let r_ = 91i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(836); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 4i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(828); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 8i32; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(844); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                }
            }
        }
    }
    { let v_ = p_kind_1; let a_ = (c.m.r32(sym::gGetWin)).wrapping_add(820); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GetWin_Setup", &PROBE_N, &[], true); }
    r_
}
