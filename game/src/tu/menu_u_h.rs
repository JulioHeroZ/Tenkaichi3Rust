#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EvoZ_SetListScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_SetListScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 512i32, 198i32, 412i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_SetListScissor", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_ResetScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_ResetScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_ResetScissor", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_Refresh(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Refresh", &PROBE_N, &[(p_ez_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_flash_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_scale_5: F32 = F32(0);
    let mut l_y_6: i32 = 0;
    let mut l_arrow_7: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        if (((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(104); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
            { let r_ = F32(0x3d999999); let a_ = (p_ez_1).wrapping_add(6748); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge(F32(c.m.r32((p_ez_1).wrapping_add(6748))), F32(0x3f800000)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (p_ez_1).wrapping_add(6748); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x00000000); let a_ = (p_ez_1).wrapping_add(6748); c.m.w32(a_, (v_).0); };
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str0, fp);
        { let a2_ = F32(c.m.r32((p_ez_1).wrapping_add(6748))); F::Flash_ClipSetAlpha(c, l_flash_2, fp, a2_) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str1, fp);
        { let a4_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, ((p_ez_1).wrapping_add(12492)).wrapping_add((((0i32) as u32)).wrapping_mul(140))) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str2, fp);
        { let a4_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, ((p_ez_1).wrapping_add(12492)).wrapping_add((((1i32) as u32)).wrapping_mul(140))) };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_3).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str3, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    if (((((l_i_3) == (2i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let a2_ = (if ((((((c.m.r32((p_ez_1).wrapping_add(6768)) as i32)) == (0i32)) as i32)) != 0) { F32(0x3ecccccc) } else { F32(0x3f800000) }); F::Flash_ClipSetColor(c, l_flash_2, fp, a2_) };
                    }
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str4, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str5, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = 0i32;
            'l3: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_3).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str6, (if (((((l_i_3) != (0i32)) as i32)) != 0) { sym::menu_u_h__str7 } else { sym::menu_u_h__str8 }), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((p_ez_1).wrapping_add(568)) as i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r32((p_ez_1).wrapping_add(568)) as i32)).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str9, (if (((((l_i_3) != (0i32)) as i32)) != 0) { sym::menu_u_h__str10 } else { sym::menu_u_h__str11 }), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str12, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let _ = { let a2_ = (c.m.r32((p_ez_1).wrapping_add(580)) as i32); F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str13, &[((a2_) as i64 as u64)]) }; };
        {
            l_i_3 = 0i32;
            'l5: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_3).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), (if (((((l_i_3) != (0i32)) as i32)) != 0) { sym::menu_u_h__str14 } else { sym::menu_u_h__str15 }), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let a5_ = (c.m.r32((p_ez_1).wrapping_add(624)) as i32); F::Num_DrawChild(c, l_flash_2, sym::menu_u_h__str16, sym::menu_u_h__str17, 0i32, 5i32, a5_, 32i32, 32i32, 1i32, 1i32) };
        { let a5_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_2, sym::menu_u_h__str18, sym::menu_u_h__str19, 0i32, 7i32, a5_, 32i32, 32i32, 0i32, 0i32) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str20, fp);
        { let a2_ = (c.m.r32((p_ez_1).wrapping_add(6760)) as i32); F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
        {
            l_i_3 = 0i32;
            'l7: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str21, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_3).wrapping_mul(20i32)).wrapping_add(20i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_3).wrapping_mul(20i32)).wrapping_add(40i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str22, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    {
                        l_j_4 = 0i32;
                        'l9: loop {
                            if !(((((l_j_4) < (3i32)) as i32)) != 0) { break 'l9; }
                            'c10: {
                                { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_u_h__str23, &[(((l_j_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                if (((((l_j_4) < ((c.m.r32(((p_ez_1).wrapping_add(6776)).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                                { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_u_h__str24, &[(((l_j_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                if ((((((c.m.r32(((p_ez_1).wrapping_add(6776)).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32)) < ((l_j_4).wrapping_neg())) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                            }
                            { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 20i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str25, sym::menu_u_h__str26, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        {
            l_i_3 = 0i32;
            'l11: loop {
                if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l11; }
                'c12: {
                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_u_h__str27, &[(((l_i_3).wrapping_add(4i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str28, fp.wrapping_add(92), fp);
                    if (((((l_i_3) < ((c.m.r32(((p_ez_1).wrapping_add(6776)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = (rt::rem_i32((c.m.r32((p_ez_1).wrapping_add(6752)) as i32), 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::div_i32((c.m.r32((p_ez_1).wrapping_add(6752)) as i32), 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::rem_i32((c.m.r32((p_ez_1).wrapping_add(6752)) as i32), 4i32)).wrapping_mul(64i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::div_i32((c.m.r32((p_ez_1).wrapping_add(6752)) as i32), 4i32)).wrapping_mul(64i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str29, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str30, sym::menu_u_h__str31, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let v_ = (rt::rem_i32((c.m.r32((p_ez_1).wrapping_add(6756)) as i32), 4i32)).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::div_i32((c.m.r32((p_ez_1).wrapping_add(6756)) as i32), 4i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::rem_i32((c.m.r32((p_ez_1).wrapping_add(6756)) as i32), 4i32)).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::div_i32((c.m.r32((p_ez_1).wrapping_add(6756)) as i32), 4i32)).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str32, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str33, sym::menu_u_h__str34, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str35, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let a5_ = (c.m.r32((p_ez_1).wrapping_add(6764)) as i32); let a8_ = (if ((((((c.m.r32((p_ez_1).wrapping_add(6768)) as i32)) != (0i32)) as i32)) != 0) { 0i32 } else { 2i32 }); F::Num_DrawChild(c, l_flash_2, sym::menu_u_h__str36, sym::menu_u_h__str37, 0i32, 7i32, a5_, 32i32, 32i32, a8_, 0i32) };
        { let a5_ = (c.m.r32((p_ez_1).wrapping_add(6768)) as i32); let a8_ = (if ((((((c.m.r32((p_ez_1).wrapping_add(6768)) as i32)) != (0i32)) as i32)) != 0) { 0i32 } else { 2i32 }); F::Num_DrawChild(c, l_flash_2, sym::menu_u_h__str38, sym::menu_u_h__str39, 0i32, 7i32, a5_, 32i32, 32i32, a8_, 0i32) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str40, fp);
        { let a2_ = (if ((((((c.m.r32((p_ez_1).wrapping_add(6768)) as i32)) != (0i32)) as i32)) != 0) { F32(c.m.r32((p_ez_1).wrapping_add(6796))) } else { F32(0x40d51eb8) }); F::Flash_ClipSetScale(c, l_flash_2, fp, a2_, F32(0x3f800000)) };
        l_flash_2 = ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        {
            l_i_3 = 0i32;
            'l13: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l13; }
                'c14: {
                    { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_3).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str41, (if (((((l_i_3) != (0i32)) as i32)) != 0) { sym::menu_u_h__str42 } else { sym::menu_u_h__str43 }), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((p_ez_1).wrapping_add(592)) as i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((c.m.r32((p_ez_1).wrapping_add(592)) as i32)).wrapping_mul(64i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str44, sym::menu_u_h__str45, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let v_ = (rt::rem_i32((c.m.r32((p_ez_1).wrapping_add(6752)) as i32), 4i32)).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::div_i32((c.m.r32((p_ez_1).wrapping_add(6752)) as i32), 4i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::rem_i32((c.m.r32((p_ez_1).wrapping_add(6752)) as i32), 4i32)).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::div_i32((c.m.r32((p_ez_1).wrapping_add(6752)) as i32), 4i32)).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str46, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let v_ = (rt::rem_i32((c.m.r32((p_ez_1).wrapping_add(6756)) as i32), 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::div_i32((c.m.r32((p_ez_1).wrapping_add(6756)) as i32), 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::rem_i32((c.m.r32((p_ez_1).wrapping_add(6756)) as i32), 4i32)).wrapping_mul(64i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::div_i32((c.m.r32((p_ez_1).wrapping_add(6756)) as i32), 4i32)).wrapping_mul(64i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str47, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        if ((((((c.m.r32((p_ez_1).wrapping_add(616)) as i32)) == (0i32)) as i32)) != 0) {
            {
                l_i_3 = 0i32;
                'l15: loop {
                    if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l15; }
                    'c16: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str48, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str49, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str50, fp);
                        if (((((((((l_i_3) < ((c.m.r32((p_ez_1).wrapping_add(640)) as i32))) as i32)) != 0) || (((((l_i_3) == (7i32)) as i32)) != 0)) as i32)) != 0) {
                            if ((((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                F::Flash_ClipSetTex(c, l_flash_2, fp, 4i32);
                            } else {
                                F::Flash_ClipSetTex(c, l_flash_2, fp, 5i32);
                            }
                        } else {
                            F::Flash_ClipSetTex(c, l_flash_2, fp, 7i32);
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str51, fp);
                        if ((((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                            F::Flash_ClipSetTex(c, l_flash_2, fp, 0i32);
                        } else {
                            F::Flash_ClipSetTex(c, l_flash_2, fp, 1i32);
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str52, fp);
                        if ((((((((((c.m.r32((p_ez_1).wrapping_add(592)) as i32)) == (3i32)) as i32)) != 0) || ((((((c.m.r32((p_ez_1).wrapping_add(572)) as i32)) == (l_i_3)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = ((c.m.r32((p_ez_1).wrapping_add(592)) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (((c.m.r32((p_ez_1).wrapping_add(592)) as i32)).wrapping_mul(64i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str53, fp);
                        if ((((((((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) && ((((((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = (((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ({ let a0_ = ((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(1i32); let a1_ = c.m.r32((p_ez_1).wrapping_add(12488)); F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str54, fp);
                        if ((((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                            { let v_ = ((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str55, fp);
                        if ((((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                            { let a4_ = ((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(1i32); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, ((p_ez_1).wrapping_add(12492)).wrapping_add(((((10i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(140))) };
                        } else {
                            F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, (1i32).wrapping_neg(), ((p_ez_1).wrapping_add(12492)).wrapping_add(((((10i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(140)));
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        } else {
            { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str56, &[((1i32) as i64 as u64)]); };
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str57, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str58, fp);
            F::Flash_ClipSetTex(c, l_flash_2, fp, 4i32);
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str59, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str60, fp);
            if ((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)) != (0i32)) as i32)) != 0) {
                F::Flash_ClipSetTex(c, l_flash_2, fp, 4i32);
            } else {
                F::Flash_ClipSetTex(c, l_flash_2, fp, 5i32);
            }
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str61, fp);
            if ((((((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)) != (0i32)) as i32)) != 0) && ((((((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = (((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a0_ = ((c.m.r32((p_ez_1).wrapping_add(636)) as i32)).wrapping_sub(1i32); let a1_ = c.m.r32((p_ez_1).wrapping_add(12488)); F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
            } else {
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            }
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str62, fp);
            if ((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)) != (0i32)) as i32)) != 0) {
                { let v_ = ((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
            } else {
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            }
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str63, fp);
            if ((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)) != (0i32)) as i32)) != 0) {
                { let a4_ = ((c.m.r32((p_ez_1).wrapping_add(636)) as i32)).wrapping_sub(1i32); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, ((p_ez_1).wrapping_add(12492)).wrapping_add((((2i32) as u32)).wrapping_mul(140))) };
            } else {
                F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, (1i32).wrapping_neg(), ((p_ez_1).wrapping_add(12492)).wrapping_add((((2i32) as u32)).wrapping_mul(140)));
            }
            {
                l_i_3 = 0i32;
                'l17: loop {
                    if !(((((l_i_3) < (6i32)) as i32)) != 0) { break 'l17; }
                    'c18: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str64, &[(((l_i_3).wrapping_add(2i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 1024i32, ((((1i32) as i64 as u64)) as u8)); };
                        if (((((((((l_i_3) == (0i32)) as i32)) != 0) || (((((l_i_3) == (((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(20)) as i32)).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) {
                            F::Flash_ClipSetCallbackA(c, l_flash_2, fp, fid::EvoZ_SetListScissor, ((0i32) as u32));
                            F::Flash_ClipSetCallbackB(c, l_flash_2, fp, fid::EvoZ_ResetScissor, ((0i32) as u32));
                        }
                        if (((((({ let a1_ = { let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::EvoZ_CanEquip(c, p_ez_1, a1_) }) != 0) || ((((!(((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                            F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f800000));
                        } else {
                            F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3ecccccc));
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str65, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str66, fp);
                        if (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0) {
                            F::Flash_ClipSetTex(c, l_flash_2, fp, 2i32);
                        } else {
                            F::Flash_ClipSetTex(c, l_flash_2, fp, 6i32);
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str67, fp);
                        { let v_ = ((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add(((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str68, fp);
                        if ((((((((((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add(((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) && (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                            { let v_ = (((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add(((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ({ let a0_ = { let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = c.m.r32((p_ez_1).wrapping_add(12488)); F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str69, fp);
                        if (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (2i32))) != 0) {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str70, fp);
                        if (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0) {
                            { let a4_ = { let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, ((p_ez_1).wrapping_add(12492)).wrapping_add(((((3i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(140))) };
                        } else {
                            F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, 350i32, ((p_ez_1).wrapping_add(12492)).wrapping_add(((((3i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(140)));
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
            { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str71, &[((8i32) as i64 as u64)]); };
            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 1024i32, ((((1i32) as i64 as u64)) as u8)); };
            F::Flash_ClipSetCallbackA(c, l_flash_2, fp, fid::EvoZ_SetListScissor, ((0i32) as u32));
            F::Flash_ClipSetCallbackB(c, l_flash_2, fp, fid::EvoZ_ResetScissor, ((0i32) as u32));
            if (((((({ let a1_ = { let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::EvoZ_CanEquip(c, p_ez_1, a1_) }) != 0) || ((((!(((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f800000));
            } else {
                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3ecccccc));
            }
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str72, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str73, fp);
            if (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0) {
                F::Flash_ClipSetTex(c, l_flash_2, fp, 2i32);
            } else {
                F::Flash_ClipSetTex(c, l_flash_2, fp, 6i32);
            }
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str74, fp);
            { let v_ = ((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add(((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str75, fp);
            if ((((((((((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add(((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) && (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                { let v_ = (((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add(((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a0_ = { let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = c.m.r32((p_ez_1).wrapping_add(12488)); F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
            } else {
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            }
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str76, fp);
            if (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (2i32))) != 0) {
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
            } else {
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            }
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str77, fp);
            if (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0) {
                { let a4_ = { let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(28)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, ((p_ez_1).wrapping_add(12492)).wrapping_add((((9i32) as u32)).wrapping_mul(140))) };
            } else {
                F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, 350i32, ((p_ez_1).wrapping_add(12492)).wrapping_add((((9i32) as u32)).wrapping_mul(140)));
            }
            {
                l_i_3 = 0i32;
                'l19: loop {
                    if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l19; }
                    'c20: {
                        { let v_ = (l_i_3).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((l_i_3).wrapping_mul(64i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str78, &[((l_i_3) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        if ((((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) == (3i32)) as i32)) != 0) {
                            if (((((l_i_3) == (3i32)) as i32)) != 0) {
                                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f800000));
                            } else {
                                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3ecccccc));
                            }
                        } else {
                            if (((((l_i_3) != (3i32)) as i32)) != 0) {
                                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f800000));
                            } else {
                                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3ecccccc));
                            }
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str79, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str80, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_u_h__str81, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str82, fp);
            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
            {
                l_i_3 = 0i32;
                'l21: loop {
                    if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l21; }
                    'c22: {
                        { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((l_i_3).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, sym::menu_u_h__str83, (if (((((l_i_3) != (0i32)) as i32)) != 0) { sym::menu_u_h__str84 } else { sym::menu_u_h__str85 }), fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        if (((((l_i_3) == (0i32)) as i32)) != 0) {
                            { let a3_ = ((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
                        } else {
                            { let a3_ = ((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
            {
                l_scale_5 = rt::fmul(rt::fdiv(rt::i2f((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(20)) as i32)), rt::i2f({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })), F32(c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(24))));
                l_y_6 = rt::f2i(rt::fdiv(rt::fmul(rt::fmul(F32(c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(24))), F32(0x42000000)), rt::i2f({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })), rt::i2f({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })));
                F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str86, fp);
                F::Flash_ClipSetScale(c, l_flash_2, fp, F32(0x3f800000), l_scale_5);
                F::Flash_ClipSetOffset(c, l_flash_2, fp, 0i32, l_y_6);
            }
            if ((((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(20)) as i32)) >= ({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3ecccccc));
            } else {
                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f800000));
            }
        }
        if ((((!(((((c.m.r32((p_ez_1).wrapping_add(556)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
            l_flash_2 = ((p_ez_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str87, fp);
            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
            if ((((((c.m.r32((p_ez_1).wrapping_add(6744)) as i32)) >= (2i32)) as i32)) != 0) {
                {
                    l_i_3 = 0i32;
                    'l23: loop {
                        if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l23; }
                        'c24: {
                            { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((l_i_3).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            if (((((l_i_3) != (0i32)) as i32)) != 0) {
                                l_arrow_7 = sym::menu_u_h__str88;
                            } else {
                                l_arrow_7 = sym::menu_u_h__str89;
                            }
                            F::Flash_FindLabel(c, l_flash_2, l_arrow_7, (if (((((l_i_3) != (0i32)) as i32)) != 0) { sym::menu_u_h__str90 } else { sym::menu_u_h__str91 }), fp);
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), l_arrow_7, fp);
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                    }
                }
            } else {
                {
                    l_i_3 = 0i32;
                    'l25: loop {
                        if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l25; }
                        'c26: {
                            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if (((((l_i_3) != (0i32)) as i32)) != 0) { sym::menu_u_h__str92 } else { sym::menu_u_h__str93 }), fp);
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                    }
                }
            }
            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_u_h__str94, fp);
            if ((((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(112); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
            } else {
                { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
            }
            {
                l_i_3 = 0i32;
                'l27: loop {
                    if !(((((l_i_3) < (14i32)) as i32)) != 0) { break 'l27; }
                    'c28: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_h__str95, &[((l_i_3) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let a3_ = (((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(112); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Refresh", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_Load(c: &mut Ctx, mut p_ez_1: u32, mut p_section_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Load", &PROBE_N, &[(p_ez_1) as u32, (p_section_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = (c.m.r32(sym::gMenuArc7)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc7)).wrapping_add((((p_section_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = p_ez_1; c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(p_ez_1); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (p_ez_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::File_LoadSync(c, 960i32, ((0i32) as u32), 0i32); let a_ = (p_ez_1).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32((p_ez_1).wrapping_add(16)); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (p_ez_1).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((p_ez_1).wrapping_add(20)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (p_ez_1).wrapping_add(176); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), a1_, (p_ez_1).wrapping_add(180)) };
        F::Flash_Play(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), 1i32);
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), a1_, (p_ez_1).wrapping_add(352)) };
        F::Flash_Play(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), 1i32);
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(452)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), a1_, (p_ez_1).wrapping_add(452)) };
        F::Flash_Play(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), 1i32);
        { let a0_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemHelp_Init(c, a0_) };
        { let a0_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::PassWin_Init(c, a0_) };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_ez_1).wrapping_add(28); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32((p_ez_1).wrapping_add(28)); F::Dialog_Init(c, a0_, a1_, 0i32) };
        F::Dialog_SetLayout(c, 0i32);
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_ez_1).wrapping_add(12488); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((p_ez_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((p_ez_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((p_ez_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_ez_1).wrapping_add(24); c.m.w32(a_, v_); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (165i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (c.m.r32((p_ez_1).wrapping_add(24))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(24))).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = ((c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(16); let a_ = (p_ez_1).wrapping_add(796); c.m.w32(a_, v_); };
        { let v_ = (({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(4))).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a_ = (p_ez_1).wrapping_add(792); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Load", &PROBE_N, &[], true); }
    r_
}
