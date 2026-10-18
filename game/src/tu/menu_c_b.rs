#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HistGuide_ClipGoto(c: &mut Ctx, mut p_menu_1: u32, mut p_guide_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistGuide_ClipGoto", &PROBE_N, &[(p_menu_1) as u32, (p_guide_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_c_b__str0, &[((p_guide_2) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistGuide_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn HistGuide_Update(c: &mut Ctx, mut p_menu_1: u32, mut p_result_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistGuide_Update", &PROBE_N, &[(p_menu_1) as u32, (p_result_2) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_menu_1).wrapping_add(496)) as i32)) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv3: i32 = (c.m.r32((p_menu_1).wrapping_add(496)) as i32);
        match sv3 {
            1i32 => {
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 8i32; let a_ = (p_menu_1).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            3i32 | 5i32 | 7i32 | 9i32 => {
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 | 4i32 | 6i32 | 8i32 | 10i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            11i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                F::Flash_GotoLabel(c, ((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_c_b__str1, ((((1i32) as i64 as u64)) as u8));
                { let v_ = 8i32; let a_ = (p_menu_1).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(5i32); let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 14i32); };
                break 's1;
            }
            1151i32 => {
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 8i32; let a_ = (p_menu_1).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                { let v_ = 44i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1152i32 | 1154i32 | 1156i32 | 1158i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            1153i32 | 1155i32 | 1157i32 => {
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1159i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                F::Flash_GotoLabel(c, ((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_c_b__str2, ((((1i32) as i64 as u64)) as u8));
                { let v_ = 8i32; let a_ = (p_menu_1).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(5i32); let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 14i32); };
                break 's1;
            }
            51i32 => {
                { let v_ = 8i32; let a_ = (p_menu_1).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                { let v_ = 43i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            52i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                }
                break 's1;
            }
            53i32 => {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            101i32 => {
                { let v_ = (((F::Rand_Range(c, ((4i32) as u32))).wrapping_add(((38i32) as u32))) as i32); let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            102i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            103i32 => {
                F::Flash_GotoLabel(c, ((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_c_b__str3, ((((1i32) as i64 as u64)) as u8));
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            104i32 => {
                if ((((c.m.r32((((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((1i32) as u32)))) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            105i32 => {
                F::Flash_StepFrames(c, ((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), 1i32);
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 12i32); };
                F::IconWin_Close(c);
                F::MsgWin_Close(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            106i32 => {
                if ((((c.m.r32((((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((2i32) as u32)))) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            107i32 => {
                { let v_ = 1i32; let a_ = p_result_2; c.m.w32(a_, (v_) as u32); };
                { let r_ = 1i32; let a_ = (p_menu_1).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (p_menu_1).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (p_menu_1).wrapping_add(476); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            151i32 => {
                F::Flash_GotoLabel(c, ((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_c_b__str4, ((((1i32) as i64 as u64)) as u8));
                { let v_ = { let a_ = ((p_menu_1).wrapping_add(500)).wrapping_add(((((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (p_menu_1).wrapping_add(556); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str5) };
                { let v_ = { let a_ = ((p_menu_1).wrapping_add(500)).wrapping_add(((((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (p_menu_1).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let r_ = 8i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = ((p_menu_1).wrapping_add(500)).wrapping_add(((((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            152i32 => {
                if ((((c.m.r32((((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((4i32) as u32)))) != 0) {
                    { let v_ = (({ let a_ = ((p_menu_1).wrapping_add(500)).wrapping_add(((((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(50i32)).wrapping_add(251i32); let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            201i32 => {
                F::Flash_StepFrames(c, ((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), 1i32);
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 12i32); };
                F::IconWin_Close(c);
                F::MsgWin_Close(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            202i32 => {
                if ((((c.m.r32((((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((2i32) as u32)))) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            203i32 => {
                { let v_ = 1i32; let a_ = p_result_2; c.m.w32(a_, (v_) as u32); };
                { let r_ = 1i32; let a_ = (p_menu_1).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (p_menu_1).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (p_menu_1).wrapping_add(476); c.m.w32(a_, (v_) as u32); };
                { let r_ = (!(2i32)); let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = ((p_menu_1).wrapping_add(500)).wrapping_add(((((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (!(64i32)); let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = ((p_menu_1).wrapping_add(500)).wrapping_add(((((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            251i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            253i32 | 261i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            255i32 | 257i32 | 259i32 | 263i32 | 265i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            252i32 | 254i32 | 256i32 | 258i32 | 260i32 | 262i32 | 264i32 | 266i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            301i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            303i32 | 307i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            305i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            302i32 | 304i32 | 306i32 | 308i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            351i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            353i32 | 357i32 | 361i32 | 365i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            355i32 | 359i32 | 363i32 | 367i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            352i32 | 354i32 | 356i32 | 358i32 | 360i32 | 362i32 | 364i32 | 366i32 | 368i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            401i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            403i32 | 409i32 | 411i32 | 413i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            405i32 | 407i32 | 415i32 | 417i32 | 419i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            402i32 | 404i32 | 406i32 | 408i32 | 410i32 | 412i32 | 414i32 | 416i32 | 418i32 | 420i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            451i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            453i32 | 463i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            455i32 | 457i32 | 459i32 | 461i32 | 465i32 | 467i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            452i32 | 454i32 | 456i32 | 458i32 | 460i32 | 462i32 | 464i32 | 466i32 | 468i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            501i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            503i32 | 511i32 | 519i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            505i32 | 507i32 | 509i32 | 513i32 | 515i32 | 517i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            502i32 | 504i32 | 506i32 | 508i32 | 510i32 | 512i32 | 514i32 | 516i32 | 520i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            518i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str6) };
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str7) };
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            551i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            553i32 | 557i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            555i32 | 559i32 | 561i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            552i32 | 554i32 | 556i32 | 558i32 | 560i32 | 562i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            601i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            603i32 | 609i32 | 615i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            605i32 | 607i32 | 611i32 | 613i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            602i32 | 604i32 | 606i32 | 608i32 | 610i32 | 612i32 | 614i32 | 616i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            267i32 | 309i32 | 369i32 | 421i32 | 469i32 | 521i32 | 563i32 | 617i32 => {
                { let v_ = 201i32; let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            651i32 => {
                { let v_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); let a_ = (p_menu_1).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                { let r_ = 32i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((((c.m.r32((p_menu_1).wrapping_add(556)) as i32)) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = (((c.m.r32((p_menu_1).wrapping_add(556)) as i32)).wrapping_mul(50i32)).wrapping_add(751i32); let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            701i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                F::Flash_GotoLabel(c, ((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_c_b__str8, ((((1i32) as i64 as u64)) as u8));
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_menu_1).wrapping_add(556); c.m.w32(a_, (v_) as u32); };
                { let v_ = 8i32; let a_ = (p_menu_1).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32(((p_menu_1).wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(5i32); let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 14i32); };
                break 's1;
            }
            751i32 => {
                { let v_ = 26i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            761i32 | 763i32 | 771i32 | 773i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            755i32 | 757i32 | 759i32 | 765i32 | 767i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            752i32 | 756i32 | 758i32 | 760i32 | 762i32 | 764i32 | 766i32 | 768i32 | 772i32 | 774i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            753i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str9) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            801i32 => {
                { let v_ = 20i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            809i32 | 811i32 | 819i32 | 821i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            805i32 | 807i32 | 813i32 | 815i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            802i32 | 806i32 | 808i32 | 810i32 | 812i32 | 814i32 | 816i32 | 820i32 | 822i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            803i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str10) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            851i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str11) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            853i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let v_ = 29i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            855i32 | 859i32 | 861i32 | 865i32 | 873i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            857i32 | 863i32 | 867i32 | 869i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            854i32 | 856i32 | 858i32 | 860i32 | 862i32 | 864i32 | 866i32 | 868i32 | 870i32 | 874i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            901i32 => {
                { let v_ = 30i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            909i32 | 913i32 | 917i32 | 919i32 | 921i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            905i32 | 907i32 | 911i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            902i32 | 906i32 | 908i32 | 910i32 | 912i32 | 914i32 | 918i32 | 920i32 | 922i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            903i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str12) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            951i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str13) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            953i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let v_ = 39i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            955i32 | 959i32 | 967i32 | 971i32 | 975i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            957i32 | 961i32 | 963i32 | 965i32 | 969i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            954i32 | 956i32 | 958i32 | 960i32 | 962i32 | 964i32 | 966i32 | 968i32 | 970i32 | 972i32 | 976i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            1001i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str14) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1003i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let v_ = 29i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1005i32 | 1007i32 | 1013i32 | 1021i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1009i32 | 1011i32 | 1015i32 | 1017i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1004i32 | 1006i32 | 1008i32 | 1010i32 | 1012i32 | 1014i32 | 1016i32 | 1018i32 | 1022i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            1051i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str15) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1053i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let v_ = 24i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1055i32 | 1059i32 | 1065i32 | 1067i32 | 1069i32 | 1071i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1057i32 | 1061i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1054i32 | 1056i32 | 1058i32 | 1060i32 | 1062i32 | 1066i32 | 1068i32 | 1070i32 | 1072i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            1101i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str16) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1103i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let v_ = 22i32; let a_ = (p_menu_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1105i32 | 1109i32 | 1111i32 | 1113i32 | 1119i32 => {
                { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1107i32 | 1115i32 => {
                { let v_ = 1i32; let a_ = (p_menu_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_menu_1).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::HistSel_PlayVoice(c);
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1104i32 | 1106i32 | 1108i32 | 1110i32 | 1112i32 | 1114i32 | 1116i32 | 1120i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            769i32 | 817i32 | 871i32 | 915i32 | 973i32 | 1019i32 | 1063i32 | 1117i32 => {
                { let a1_ = (c.m.r32((p_menu_1).wrapping_add(556)) as i32); F::HistGuide_ClipGoto(c, p_menu_1, a1_, sym::menu_c_b__str17) };
                { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            754i32 | 770i32 | 804i32 | 818i32 | 852i32 | 872i32 | 904i32 | 916i32 | 952i32 | 974i32 | 1002i32 | 1020i32 | 1052i32 | 1064i32 | 1102i32 | 1118i32 => {
                if ((((c.m.r32((((p_menu_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((4i32) as u32)))) != 0) {
                    { let a_ = (p_menu_1).wrapping_add(496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            775i32 | 823i32 | 875i32 | 923i32 | 977i32 | 1023i32 | 1073i32 | 1121i32 => {
                { let v_ = 701i32; let a_ = (p_menu_1).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
                F::Voice_FadeOutStep(c, 0i32);
                break 's1;
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistGuide_Update", &PROBE_N, &[], true); }
    r_
}
