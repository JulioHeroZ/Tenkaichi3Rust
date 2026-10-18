#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftEmit_SpawnType9(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_arg3_4: i32, mut p_objId_5: i32, mut p_idx_6: i32, mut p_size_7: F32, mut p_scale_8: F32, mut p_rate_9: F32, mut p_pos_10: u32, mut p_dir_11: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType9", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_arg3_4) as u32, (p_objId_5) as u32, (p_idx_6) as u32, (p_size_7).0, (p_scale_8).0, (p_rate_9).0, (p_pos_10) as u32, (p_dir_11) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_grp_12: u32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_def_14: u32 = 0;
    let mut l_tex_15: u32 = 0;
    let mut l_res0_16: i32 = 0;
    let mut l_res1_17: i32 = 0;
    let r_ = 'f: {
        l_grp_12 = ((p_set_1).wrapping_add(12)).wrapping_add((((9i32) as u32)).wrapping_mul(16));
        l_n_13 = (((c.m.r8((l_grp_12).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
        l_def_14 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_13) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_def_14).wrapping_add(16))), p_scale_8); F::Vec3_ScaleAdd(c, fp, p_dir_11, p_pos_10, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_tex_15 = (c.m.r32((p_set_1).wrapping_add(772))).wrapping_add(((((((c.m.r8((l_grp_12).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_def_14).wrapping_add(1))) as i32))).wrapping_mul(136i32)) as u32));
                l_res0_16 = { let a_ = ((p_set_1).wrapping_add(12)).wrapping_add((((((((c.m.r8((l_grp_12).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)).wrapping_add(54i32)) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) };
                l_res1_17 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add((((((((c.m.r8((l_grp_12).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)).wrapping_add(54i32)) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) };
                rt::memset(c, fp.wrapping_add(16), 0, 64);
                rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 8);
                { let v_ = l_res0_16; c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_) as u32); }
                { let v_ = l_res1_17; c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_) as u32); }
                { let v_ = l_tex_15; c.m.w32(fp.wrapping_add(16).wrapping_add(8), v_); }
                rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
                rt::memset(c, fp.wrapping_add(16).wrapping_add(32), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(12), (v_).0); }
                { let v_ = p_size_7; c.m.w32(fp.wrapping_add(16).wrapping_add(48), (v_).0); }
                { let v_ = p_rate_9; c.m.w32(fp.wrapping_add(16).wrapping_add(52), (v_).0); }
                { let v_ = p_objId_5; c.m.w32(fp.wrapping_add(16).wrapping_add(56), (v_) as u32); }
                F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(32), p_dir_11);
                if ((((((c.m.r8(l_def_14)) as i32)) & (64i32))) != 0) {
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), p_pos_10);
                } else {
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), fp);
                }
                { let v_ = F::EftQuad_Create(c, fp.wrapping_add(16)); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_14).wrapping_add(5))) as i32); F::EftQuad_SetDelay(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_14).wrapping_add(6))) as i32); F::EftQuad_SetStopDelay(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_14).wrapping_add(7))) as i32); F::EftQuad_SetFade(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a2_ = ((c.m.r8((l_def_14).wrapping_add(2))) as i32); let a3_ = ((c.m.r8((l_def_14).wrapping_add(2))) as i32); F::EftQuad_SetTexPair(c, a0_, l_tex_15, a2_, a3_) };
                if ((((((c.m.r8(l_def_14)) as i32)) & (32i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); { let _ = F::EftQuad_SetViewOnly(c, a0_); } };
                }
                if ((((((c.m.r8((l_def_14).wrapping_add(52))) as i32)) & (8i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_SetOwnOrigin(c, a0_, 1i32) };
                }
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_SetCut(c, a0_, p_arg3_4) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_objId_5, p_arg3_4) };
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (16i32))) != 0) {
                if ((((p_flags_3) & (32i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_Warp(c, a0_, fp) };
                } else {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_SetPos(c, a0_, fp) };
                }
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_SetSize(c, a0_, p_size_7) };
            }
            if ((((p_flags_3) & (128i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_SetDir(c, a0_, p_dir_11) };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_Kill(c, a0_) };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    if ((((p_flags_3) & (4i32))) != 0) {
                        { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_SetStopDelay(c, a0_, 0i32) };
                    }
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftQuad_Stop(c, a0_) };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType9", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType10(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_arg3_4: i32, mut p_objId_5: i32, mut p_idx_6: i32, mut p_size_7: F32, mut p_scale_8: F32, mut p_rate_9: F32, mut p_pos_10: u32, mut p_unused_11: u32, mut p_dir_12: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType10", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_arg3_4) as u32, (p_objId_5) as u32, (p_idx_6) as u32, (p_size_7).0, (p_scale_8).0, (p_rate_9).0, (p_pos_10) as u32, (p_unused_11) as u32, (p_dir_12) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_grp_13: u32 = 0;
    let mut l_n_14: i32 = 0;
    let mut l_def_15: u32 = 0;
    let mut l_tex_16: u32 = 0;
    let mut l_res0_17: i32 = 0;
    let mut l_res1_18: i32 = 0;
    let r_ = 'f: {
        l_grp_13 = ((p_set_1).wrapping_add(12)).wrapping_add((((10i32) as u32)).wrapping_mul(16));
        l_n_14 = (((c.m.r8((l_grp_13).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
        l_def_15 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_14) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_def_15).wrapping_add(16))), p_scale_8); F::Vec3_ScaleAdd(c, fp, p_dir_12, p_pos_10, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_tex_16 = (c.m.r32((p_set_1).wrapping_add(768))).wrapping_add(((((((c.m.r8((l_grp_13).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_def_15).wrapping_add(1))) as i32))).wrapping_mul(264i32)) as u32));
                l_res0_17 = { let a_ = ((p_set_1).wrapping_add(12)).wrapping_add((((((((c.m.r8((l_grp_13).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)).wrapping_add(54i32)) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) };
                l_res1_18 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add((((((((c.m.r8((l_grp_13).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)).wrapping_add(54i32)) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) };
                rt::memset(c, fp.wrapping_add(16), 0, 80);
                rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 8);
                { let v_ = l_res0_17; c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_) as u32); }
                { let v_ = l_res1_18; c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_) as u32); }
                { let v_ = l_tex_16; c.m.w32(fp.wrapping_add(16).wrapping_add(8), v_); }
                rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
                rt::memset(c, fp.wrapping_add(16).wrapping_add(32), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(12), (v_).0); }
                { let v_ = p_size_7; c.m.w32(fp.wrapping_add(16).wrapping_add(48), (v_).0); }
                { let v_ = p_rate_9; c.m.w32(fp.wrapping_add(16).wrapping_add(52), (v_).0); }
                { let v_ = ((c.m.r8((l_def_15).wrapping_add(2))) as i32); c.m.w32(fp.wrapping_add(16).wrapping_add(56), (v_) as u32); }
                { let v_ = ((c.m.r8((l_def_15).wrapping_add(2))) as i32); c.m.w32(fp.wrapping_add(16).wrapping_add(60), (v_) as u32); }
                { let v_ = p_objId_5; c.m.w32(fp.wrapping_add(16).wrapping_add(64), (v_) as u32); }
                F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(32), p_dir_12);
                if ((((((c.m.r8(l_def_15)) as i32)) & (64i32))) != 0) {
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), p_pos_10);
                } else {
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), fp);
                }
                { let v_ = F::EftPart10_Create(c, fp.wrapping_add(16)); let a_ = (p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_15).wrapping_add(5))) as i32); F::EftPart10_SetDelay(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_15).wrapping_add(6))) as i32); F::EftPart10_SetHold(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_15).wrapping_add(7))) as i32); F::EftPart10_SetFade(c, a0_, a1_) };
                if ((((((c.m.r8(l_def_15)) as i32)) & (32i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); { let _ = F::EftPart10_SetNoDepth(c, a0_); } };
                }
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); { let _ = F::EftPart10_SetKind(c, a0_, p_arg3_4); } };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_objId_5, p_arg3_4) };
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (16i32))) != 0) {
                if ((((p_flags_3) & (32i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftPart10_Warp(c, a0_, fp) };
                } else {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftPart10_SetPos(c, a0_, fp) };
                }
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftPart10_SetSize(c, a0_, p_size_7) };
            }
            if ((((p_flags_3) & (128i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftPart10_SetDir(c, a0_, p_dir_12) };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftPart10_Kill(c, a0_) };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    if ((((p_flags_3) & (4i32))) != 0) {
                        { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftPart10_SetHold(c, a0_, 0i32) };
                    }
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftPart10_Stop(c, a0_) };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType10", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType15(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_arg3_4: i32, mut p_objId_5: i32, mut p_idx_6: i32, mut p_size_7: F32, mut p_scale_8: F32, mut p_rate_9: F32, mut p_pos_10: u32, mut p_pos2_11: u32, mut p_dir_12: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType15", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_arg3_4) as u32, (p_objId_5) as u32, (p_idx_6) as u32, (p_size_7).0, (p_scale_8).0, (p_rate_9).0, (p_pos_10) as u32, (p_pos2_11) as u32, (p_dir_12) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_grp_13: u32 = 0;
    let mut l_n_14: i32 = 0;
    let mut l_def_15: u32 = 0;
    let mut l_tex_16: u32 = 0;
    let mut l_res0_17: i32 = 0;
    let mut l_res1_18: i32 = 0;
    let r_ = 'f: {
        l_grp_13 = ((p_set_1).wrapping_add(12)).wrapping_add((((15i32) as u32)).wrapping_mul(16));
        l_n_14 = (((c.m.r8((l_grp_13).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
        l_def_15 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_14) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_def_15).wrapping_add(16))), p_scale_8); F::Vec3_ScaleAdd(c, fp, p_dir_12, p_pos_10, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let a3_ = rt::fmul(F32(c.m.r32((l_def_15).wrapping_add(20))), p_scale_8); F::Vec3_ScaleAdd(c, fp.wrapping_add(16), p_dir_12, p_pos2_11, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_tex_16 = (c.m.r32((p_set_1).wrapping_add(768))).wrapping_add(((((((c.m.r8((l_grp_13).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_def_15).wrapping_add(1))) as i32))).wrapping_mul(264i32)) as u32));
                l_res0_17 = { let a_ = ((p_set_1).wrapping_add(12)).wrapping_add((((((((c.m.r8((l_grp_13).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)).wrapping_add(54i32)) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) };
                l_res1_18 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add((((((((c.m.r8((l_grp_13).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)).wrapping_add(54i32)) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) };
                rt::memset(c, fp.wrapping_add(32), 0, 80);
                rt::memset(c, fp.wrapping_add(32).wrapping_add(0), 0, 8);
                { let v_ = l_res0_17; c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_) as u32); }
                { let v_ = l_res1_18; c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_) as u32); }
                { let v_ = l_tex_16; c.m.w32(fp.wrapping_add(32).wrapping_add(8), v_); }
                rt::memset(c, fp.wrapping_add(32).wrapping_add(16), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(12), (v_).0); }
                rt::memset(c, fp.wrapping_add(32).wrapping_add(32), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(32).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(32).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(32).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(32).wrapping_add(12), (v_).0); }
                { let v_ = p_size_7; c.m.w32(fp.wrapping_add(32).wrapping_add(48), (v_).0); }
                { let v_ = p_rate_9; c.m.w32(fp.wrapping_add(32).wrapping_add(52), (v_).0); }
                { let v_ = ((c.m.r8((l_def_15).wrapping_add(2))) as i32); c.m.w32(fp.wrapping_add(32).wrapping_add(56), (v_) as u32); }
                { let v_ = 0i32; c.m.w32(fp.wrapping_add(32).wrapping_add(60), (v_) as u32); }
                { let v_ = p_objId_5; c.m.w32(fp.wrapping_add(32).wrapping_add(64), (v_) as u32); }
                if ((((((c.m.r8(l_def_15)) as i32)) & (64i32))) != 0) {
                    F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(16), p_pos_10);
                    F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(32), p_pos2_11);
                } else {
                    F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(16), fp);
                    F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(32), fp.wrapping_add(16));
                }
                { let v_ = F::EftLink_Create(c, fp.wrapping_add(32)); let a_ = (p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_15).wrapping_add(5))) as i32); F::EftLink_SetDelay(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_15).wrapping_add(6))) as i32); F::EftLink_SetStopDelay(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_15).wrapping_add(7))) as i32); F::EftLink_SetFade(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); { let _ = F::EftLink_SetType(c, a0_, p_arg3_4); } };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_objId_5, p_arg3_4) };
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (16i32))) != 0) {
                if ((((p_flags_3) & (32i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_Warp(c, a0_, fp) };
                } else {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_SetPos(c, a0_, fp) };
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_SetPos2(c, a0_, fp.wrapping_add(16)) };
                }
            } else {
                if ((((p_flags_3) & (512i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_SetPos2(c, a0_, fp.wrapping_add(16)) };
                }
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_SetSize(c, a0_, p_size_7) };
            }
            if ((((p_flags_3) & (128i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_SetDir(c, a0_, p_dir_12) };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_Kill(c, a0_) };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    if ((((p_flags_3) & (4i32))) != 0) {
                        { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_SetStopDelay(c, a0_, 0i32) };
                    }
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftLink_Stop(c, a0_) };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType15", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType12(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_arg3_4: i32, mut p_objId_5: i32, mut p_idx_6: i32, mut p_size_7: F32, mut p_scale_8: F32, mut p_rate_9: F32, mut p_pos_10: u32, mut p_dir_11: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType12", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_arg3_4) as u32, (p_objId_5) as u32, (p_idx_6) as u32, (p_size_7).0, (p_scale_8).0, (p_rate_9).0, (p_pos_10) as u32, (p_dir_11) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_grp_12: u32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_def_14: u32 = 0;
    let mut l_tex_15: u32 = 0;
    let mut l_res0_16: i32 = 0;
    let mut l_res1_17: i32 = 0;
    let r_ = 'f: {
        l_grp_12 = ((p_set_1).wrapping_add(12)).wrapping_add((((12i32) as u32)).wrapping_mul(16));
        l_n_13 = (((c.m.r8((l_grp_12).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
        l_def_14 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_13) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_def_14).wrapping_add(16))), p_scale_8); F::Vec3_ScaleAdd(c, fp, p_dir_11, p_pos_10, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_tex_15 = (c.m.r32((p_set_1).wrapping_add(768))).wrapping_add(((((((c.m.r8((l_grp_12).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_def_14).wrapping_add(1))) as i32))).wrapping_mul(264i32)) as u32));
                l_res0_16 = { let a_ = ((p_set_1).wrapping_add(12)).wrapping_add((((((((c.m.r8((l_grp_12).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)).wrapping_add(54i32)) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) };
                l_res1_17 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add((((((((c.m.r8((l_grp_12).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)).wrapping_add(54i32)) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) };
                rt::memset(c, fp.wrapping_add(16), 0, 80);
                rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 8);
                { let v_ = l_res0_16; c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_) as u32); }
                { let v_ = l_res1_17; c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_) as u32); }
                { let v_ = l_tex_15; c.m.w32(fp.wrapping_add(16).wrapping_add(8), v_); }
                rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
                rt::memset(c, fp.wrapping_add(16).wrapping_add(32), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(12), (v_).0); }
                { let v_ = p_size_7; c.m.w32(fp.wrapping_add(16).wrapping_add(48), (v_).0); }
                { let v_ = p_rate_9; c.m.w32(fp.wrapping_add(16).wrapping_add(52), (v_).0); }
                { let v_ = ((c.m.r8((l_def_14).wrapping_add(2))) as i32); c.m.w32(fp.wrapping_add(16).wrapping_add(56), (v_) as u32); }
                { let v_ = p_objId_5; c.m.w32(fp.wrapping_add(16).wrapping_add(60), (v_) as u32); }
                rt::memset(c, fp.wrapping_add(16).wrapping_add(64), 0, 1);
                { let v_ = ((p_arg3_4) as u8); c.m.w8(fp.wrapping_add(16).wrapping_add(64).wrapping_add(0), v_); }
                if ((((((c.m.r8(l_def_14)) as i32)) & (64i32))) != 0) {
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(32), p_pos_10);
                } else {
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(32), fp);
                }
                F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), p_dir_11);
                { let v_ = F::EftZap_Create(c, fp.wrapping_add(16)); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_14).wrapping_add(5))) as i32); F::EftZap_SetDelay(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_14).wrapping_add(6))) as i32); F::EftZap_SetFadeDelay(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_def_14).wrapping_add(7))) as i32); F::EftZap_SetFadeTime(c, a0_, a1_) };
                if ((((((c.m.r8(l_def_14)) as i32)) & (32i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); { let _ = F::EftZap_SetFlag20000(c, a0_); } };
                }
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_objId_5, p_arg3_4) };
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (16i32))) != 0) {
                if ((((p_flags_3) & (32i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftZap_WarpPos(c, a0_, fp) };
                } else {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftZap_SetPos(c, a0_, fp) };
                }
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftZap_SetSize(c, a0_, p_size_7) };
            }
            if ((((p_flags_3) & (128i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftZap_SetDir(c, a0_, p_dir_11) };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftZap_Kill(c, a0_) };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    if ((((p_flags_3) & (4i32))) != 0) {
                        { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftZap_SetFadeDelay(c, a0_, 0i32) };
                    }
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftZap_Stop(c, a0_) };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType12", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnOwn(c: &mut Ctx, mut p_set_1: u32, mut p_state_2: u32, mut p_nodes_3: u32, mut p_pos_4: u32, mut p_dir_5: u32, mut p_type_6: i32, mut p_idx_7: i32, mut p_flags_8: i32, mut p_scale_9: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnOwn", &PROBE_N, &[(p_set_1) as u32, (p_state_2) as u32, (p_nodes_3) as u32, (p_pos_4) as u32, (p_dir_5) as u32, (p_type_6) as u32, (p_idx_7) as u32, (p_flags_8) as u32, (p_scale_9).0], false); }
    let r_ = 'f: {
    { let a5_ = { let a_ = c.m.r32((p_set_1).wrapping_add(796)); (c.m.r32(a_) as i32) }; let a6_ = (({ let a_ = ({ let a_ = (c.m.r32((p_set_1).wrapping_add(796))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(6); (c.m.r8(a_) as i8) }) as i32); F::EftEmit_Spawn(c, p_set_1, p_state_2, p_nodes_3, p_pos_4, p_dir_5, a5_, a6_, 1i32, p_type_6, p_idx_7, p_flags_8, p_scale_9) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnOwn", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_Spawn(c: &mut Ctx, mut p_set_1: u32, mut p_state_2: u32, mut p_nodes_3: u32, mut p_pos_4: u32, mut p_dir_5: u32, mut p_objId_6: i32, mut p_node_7: i32, mut p_arg7_8: i32, mut p_type_9: i32, mut p_idx_10: i32, mut p_flags_11: i32, mut p_scale_12: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_Spawn", &PROBE_N, &[(p_set_1) as u32, (p_state_2) as u32, (p_nodes_3) as u32, (p_pos_4) as u32, (p_dir_5) as u32, (p_objId_6) as u32, (p_node_7) as u32, (p_arg7_8) as u32, (p_type_9) as u32, (p_idx_10) as u32, (p_flags_11) as u32, (p_scale_12).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_half_13: i32 = 0;
    let mut l_count_14: i32 = 0;
    let mut l_slot_15: i32 = 0;
    let mut l_sel_16: i32 = 0;
    let mut l_n_17: i32 = 0;
    let mut l_def_18: u32 = 0;
    let mut l_rate_19: F32 = F32(0);
    let mut l_size_20: F32 = F32(0);
    let mut l_handles_21: u32 = 0;
    let mut l_spread_22: F32 = F32(0);
    let mut l_angle_23: F32 = F32(0);
    let mut l_pitch_24: F32 = F32(0);
    let mut l_yaw_25: F32 = F32(0);
    let mut l_total_26: F32 = F32(0);
    let mut l_first_27: F32 = F32(0);
    let mut l_t_28: F32 = F32(0);
    let mut l_dd_29: F32 = F32(0);
    let mut l_r_30: F32 = F32(0);
    let mut sv_31: i32 = 0;
    let mut sv_32: i32 = 0;
    let mut sv_33: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_count_14 = 1i32;
                    l_n_17 = (((c.m.r8((((p_set_1).wrapping_add(12)).wrapping_add((((p_type_9) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(p_idx_10);
                    l_def_18 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_17) as u32)).wrapping_mul(64));
                    if ((((((c.m.r8((l_def_18).wrapping_add(52))) as i32)) & (1i32))) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    if ((((((c.m.r8((l_def_18).wrapping_add(52))) as i32)) & (16i32))) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                2 => {
                    l_sel_16 = ((c.m.r8((l_def_18).wrapping_add(10))) as i32);
                    if ((((((c.m.r8(((p_state_2).wrapping_add(320)).wrapping_add(((l_n_17) as u32)))) as i32)) & (32i32))) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
                }
                3 => {
                    if ((((!(((({ let a0_ = F::BtlCharApi_GetOpponentObjId(c, p_objId_6); F::BtlCharApi_ObjGetParamFlags0(c, a0_) }) & (128i32))) != 0)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
                }
                4 => {
                    bb_ = 2; continue 'fl1;
                }
                5 => {
                    if (((({ let a0_ = F::BtlCharApi_GetOpponentObjId(c, p_objId_6); F::BtlCharApi_ObjGetParamFlags0(c, a0_) }) & (128i32))) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
                }
                6 => {
                    break 'f;
                }
                7 => {
                    bb_ = 4; continue 'fl1;
                }
                8 => {
                    bb_ = 7; continue 'fl1;
                }
                9 => {
                    break 'f;
                }
                10 => {
                    bb_ = 4; continue 'fl1;
                }
                11 => {
                    bb_ = 10; continue 'fl1;
                }
                12 => {
                    l_sel_16 = ((c.m.r8((l_def_18).wrapping_add(54))) as i32);
                    bb_ = 13; continue 'fl1;
                }
                13 => {
                    l_slot_15 = (c.m.r32((sym::gEftEmitNodeSlot).wrapping_add((((l_sel_16) as u32)).wrapping_mul(4))) as i32);
                    if ((((p_flags_11) & (1i32))) != 0) { bb_ = 14; continue 'fl1; } else { bb_ = 15; continue 'fl1; }
                }
                14 => {
                    if ((((!((((((c.m.r8(((p_state_2).wrapping_add(320)).wrapping_add(((l_n_17) as u32)))) as i32)) & (2i32))) != 0)) as i32)) != 0) { bb_ = 16; continue 'fl1; } else { bb_ = 17; continue 'fl1; }
                }
                15 => {
                    l_rate_19 = F32(c.m.r32((l_def_18).wrapping_add(24)));
                    bb_ = 31; continue 'fl1;
                }
                16 => {
                    if (((((((((((c.m.r8((l_def_18).wrapping_add(8))) as i32)) == (2i32)) as i32)) != 0) || (((((((c.m.r8((l_def_18).wrapping_add(8))) as i32)) == (6i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
                }
                17 => {
                    bb_ = 15; continue 'fl1;
                }
                18 => {
                    if (((((((c.m.r8((l_def_18).wrapping_add(9))) as i32)) != (0i32)) as i32)) != 0) { bb_ = 20; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
                }
                19 => {
                    if (((((((c.m.r8((l_def_18).wrapping_add(9))) as i32)) == (0i32)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
                }
                20 => {
                    break 'f;
                }
                21 => {
                    if (((rt::fle(F32(c.m.r32((l_def_18).wrapping_add(24))), F32(0x00000000)) as i32)) != 0) { bb_ = 23; continue 'fl1; } else { bb_ = 24; continue 'fl1; }
                }
                22 => {
                    bb_ = 21; continue 'fl1;
                }
                23 => {
                    break 'f;
                }
                24 => {
                    { let r_ = 1i32; let a_ = ((p_state_2).wrapping_add(320)).wrapping_add(((l_n_17) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                    bb_ = 19; continue 'fl1;
                }
                25 => {
                    bb_ = 24; continue 'fl1;
                }
                26 => {
                    l_rate_19 = F32(c.m.r32((l_def_18).wrapping_add(24)));
                    if (((rt::fle(l_rate_19, F32(0x00000000)) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
                }
                27 => {
                    bb_ = 17; continue 'fl1;
                }
                28 => {
                    break 'f;
                }
                29 => {
                    bb_ = 31; continue 'fl1;
                }
                30 => {
                    bb_ = 29; continue 'fl1;
                }
                31 => {
                    if ((((p_flags_11) & (2i32))) != 0) { bb_ = 33; continue 'fl1; } else { bb_ = 34; continue 'fl1; }
                }
                32 => {
                    bb_ = 27; continue 'fl1;
                }
                33 => {
                    if (((((((c.m.r8((l_def_18).wrapping_add(8))) as i32)) == (2i32)) as i32)) != 0) { bb_ = 35; continue 'fl1; } else { bb_ = 36; continue 'fl1; }
                }
                34 => {
                    sv_31 = ((c.m.r8((l_def_18).wrapping_add(55))) as i32);
                    bb_ = match sv_31 { 0i32 => 39, 1i32 => 41, 2i32 => 43, _ => 45, }; continue 'fl1;
                }
                35 => {
                    { let r_ = (!(2i32)); let n_ = ((p_flags_11) & (r_)); p_flags_11 = n_; };
                    bb_ = 36; continue 'fl1;
                }
                36 => {
                    bb_ = 34; continue 'fl1;
                }
                37 => {
                    sv_32 = ((c.m.r8((l_def_18).wrapping_add(11))) as i32);
                    bb_ = match sv_32 { 0i32 => 49, 1i32 => 51, 2i32 => 53, 3i32 => 57, 4i32 => 61, 5i32 => 63, _ => 47, }; continue 'fl1;
                }
                38 => {
                    bb_ = 39; continue 'fl1;
                }
                39 => {
                    l_size_20 = rt::fmul(F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), p_scale_12);
                    bb_ = 37; continue 'fl1;
                }
                40 => {
                    bb_ = 41; continue 'fl1;
                }
                41 => {
                    l_size_20 = rt::fmul(F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), F::BtlScene_GetCharScale(c, p_objId_6));
                    bb_ = 37; continue 'fl1;
                }
                42 => {
                    bb_ = 43; continue 'fl1;
                }
                43 => {
                    l_size_20 = rt::fmul(F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), { let a0_ = F::BtlCharApi_GetOpponentObjId(c, p_objId_6); F::BtlScene_GetCharScale(c, a0_) });
                    bb_ = 37; continue 'fl1;
                }
                44 => {
                    bb_ = 45; continue 'fl1;
                }
                45 => {
                    l_size_20 = rt::fmul(F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), p_scale_12);
                    bb_ = 37; continue 'fl1;
                }
                46 => {
                    bb_ = 37; continue 'fl1;
                }
                47 => {
                    if ((((c.m.r32(p_nodes_3)) & ((((1i32).wrapping_shl((((l_slot_15).wrapping_mul(2i32)).wrapping_add(1i32)) as u32)) as u32)))) != 0) { bb_ = 65; continue 'fl1; } else { bb_ = 66; continue 'fl1; }
                }
                48 => {
                    bb_ = 49; continue 'fl1;
                }
                49 => {
                    F::Vec4_Copy(c, fp.wrapping_add(32), p_dir_5);
                    bb_ = 47; continue 'fl1;
                }
                50 => {
                    bb_ = 51; continue 'fl1;
                }
                51 => {
                    F::Vec4_Copy(c, fp.wrapping_add(32), p_dir_5);
                    F::Vec3_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), F32(0xbf800000));
                    bb_ = 47; continue 'fl1;
                }
                52 => {
                    bb_ = 53; continue 'fl1;
                }
                53 => {
                    F::Vec4_Set(c, fp.wrapping_add(48), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x3f800000));
                    if (((rt::feq(F32(c.m.r32((p_dir_5).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) { bb_ = 54; continue 'fl1; } else { bb_ = 55; continue 'fl1; }
                }
                54 => {
                    F::Vec4_Set(c, fp.wrapping_add(48), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
                    bb_ = 55; continue 'fl1;
                }
                55 => {
                    F::Vec3_Cross(c, fp.wrapping_add(32), p_dir_5, fp.wrapping_add(48));
                    F::Vec3_Cross(c, fp.wrapping_add(32), p_dir_5, fp.wrapping_add(32));
                    bb_ = 47; continue 'fl1;
                }
                56 => {
                    bb_ = 57; continue 'fl1;
                }
                57 => {
                    F::Vec4_Set(c, fp.wrapping_add(48), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x3f800000));
                    if (((rt::feq(F32(c.m.r32((p_dir_5).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) { bb_ = 58; continue 'fl1; } else { bb_ = 59; continue 'fl1; }
                }
                58 => {
                    F::Vec4_Set(c, fp.wrapping_add(48), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
                    bb_ = 59; continue 'fl1;
                }
                59 => {
                    F::Vec3_Cross(c, fp.wrapping_add(32), p_dir_5, fp.wrapping_add(48));
                    F::Vec3_Cross(c, fp.wrapping_add(32), p_dir_5, fp.wrapping_add(32));
                    F::Vec3_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), F32(0xbf800000));
                    bb_ = 47; continue 'fl1;
                }
                60 => {
                    bb_ = 61; continue 'fl1;
                }
                61 => {
                    F::Vec4_Set(c, fp.wrapping_add(32), F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
                    bb_ = 47; continue 'fl1;
                }
                62 => {
                    bb_ = 63; continue 'fl1;
                }
                63 => {
                    F::Vec4_Set(c, fp.wrapping_add(32), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x3f800000));
                    bb_ = 47; continue 'fl1;
                }
                64 => {
                    bb_ = 47; continue 'fl1;
                }
                65 => {
                    l_count_14 = 2i32;
                    bb_ = 66; continue 'fl1;
                }
                66 => {
                    l_half_13 = 0i32;
                    bb_ = 67; continue 'fl1;
                }
                67 => {
                    if (((((l_half_13) < (l_count_14)) as i32)) != 0) { bb_ = 68; continue 'fl1; } else { bb_ = 70; continue 'fl1; }
                }
                68 => {
                    l_handles_21 = (p_state_2).wrapping_add((((l_half_13) as u32)).wrapping_mul(160));
                    if (((((l_slot_15) >= (0i32)) as i32)) != 0) { bb_ = 71; continue 'fl1; } else { bb_ = 73; continue 'fl1; }
                }
                69 => {
                    { let o_ = l_half_13; let n_ = (o_).wrapping_add(1); l_half_13 = n_; };
                    bb_ = 67; continue 'fl1;
                }
                70 => {
                    if ((((p_flags_11) & (1i32))) != 0) { bb_ = 110; continue 'fl1; } else { bb_ = 111; continue 'fl1; }
                }
                71 => {
                    if ((((((c.m.r8(l_def_18)) as i32)) & (8i32))) != 0) { bb_ = 74; continue 'fl1; } else { bb_ = 76; continue 'fl1; }
                }
                72 => {
                    if ((((p_flags_11) & (1i32))) != 0) { bb_ = 80; continue 'fl1; } else { bb_ = 81; continue 'fl1; }
                }
                73 => {
                    if (((((l_sel_16) == (6i32)) as i32)) != 0) { bb_ = 77; continue 'fl1; } else { bb_ = 79; continue 'fl1; }
                }
                74 => {
                    F::Vec4_Copy(c, fp, (((p_nodes_3).wrapping_add(400)).wrapping_add((((l_slot_15) as u32)).wrapping_mul(32))).wrapping_add(16));
                    bb_ = 75; continue 'fl1;
                }
                75 => {
                    bb_ = 72; continue 'fl1;
                }
                76 => {
                    F::Vec4_Copy(c, fp, ((((p_nodes_3).wrapping_add(16)).wrapping_add((((l_slot_15) as u32)).wrapping_mul(64))).wrapping_add((((l_half_13) as u32)).wrapping_mul(32))).wrapping_add(16));
                    bb_ = 75; continue 'fl1;
                }
                77 => {
                    { let a0_ = F::BtlCharApi_GetOpponentObjId(c, p_objId_6); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
                    bb_ = 78; continue 'fl1;
                }
                78 => {
                    bb_ = 72; continue 'fl1;
                }
                79 => {
                    F::Vec4_Copy(c, fp, p_pos_4);
                    bb_ = 78; continue 'fl1;
                }
                80 => {
                    F::Vec4_Set(c, fp.wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                    l_spread_22 = F32(c.m.r32((l_def_18).wrapping_add(12)));
                    if (((rt::flt(F32(0x00000000), l_spread_22) as i32)) != 0) { bb_ = 82; continue 'fl1; } else { bb_ = 83; continue 'fl1; }
                }
                81 => {
                    sv_33 = p_type_9;
                    bb_ = match sv_33 { 0i32 => 88, 2i32 => 90, 16i32 => 92, 17i32 => 94, 18i32 => 96, 14i32 => 98, 5i32 => 100, 9i32 => 102, 10i32 => 104, 15i32 => 106, 12i32 => 108, _ => 86, }; continue 'fl1;
                }
                82 => {
                    l_angle_23 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                    if (((rt::fle(F32(0x40490fda), l_angle_23) as i32)) != 0) { bb_ = 84; continue 'fl1; } else { bb_ = 85; continue 'fl1; }
                }
                83 => {
                    bb_ = 81; continue 'fl1;
                }
                84 => {
                    { let r_ = F32(0x40c90fda); let n_ = rt::fsub(l_angle_23, r_); l_angle_23 = n_; };
                    bb_ = 85; continue 'fl1;
                }
                85 => {
                    { let a2_ = rt::fmul(l_spread_22, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))); F::Vec3_Set(c, fp.wrapping_add(16), F32(0x00000000), a2_, F32(0x00000000)) };
                    F::Mtx_StoreIdentity(c, fp.wrapping_add(64));
                    F::Mtx_RotateZ(c, fp.wrapping_add(64), fp.wrapping_add(64), l_angle_23);
                    F::Mtx_MulVec4(c, fp.wrapping_add(16), fp.wrapping_add(64), fp.wrapping_add(16));
                    l_pitch_24 = { let a0_ = rt::fneg(F32(c.m.r32((p_dir_5).wrapping_add(4)))); F::Port_asinf(c, a0_) };
                    l_yaw_25 = { let a0_ = F32(c.m.r32(p_dir_5)); let a1_ = F32(c.m.r32((p_dir_5).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                    F::Mtx_StoreIdentity(c, fp.wrapping_add(64));
                    F::Mtx_RotateX(c, fp.wrapping_add(64), fp.wrapping_add(64), l_pitch_24);
                    F::Mtx_RotateY(c, fp.wrapping_add(64), fp.wrapping_add(64), l_yaw_25);
                    F::Mtx_MulVec4(c, fp.wrapping_add(16), fp.wrapping_add(64), fp.wrapping_add(16));
                    F::Vec3_Add(c, fp, fp, fp.wrapping_add(16));
                    bb_ = 83; continue 'fl1;
                }
                86 => {
                    bb_ = 69; continue 'fl1;
                }
                87 => {
                    bb_ = 88; continue 'fl1;
                }
                88 => {
                    { let _ = fp.wrapping_add(32); F::EftEmit_SpawnType0(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, fp, l_size_20, p_scale_12, l_rate_19) };
                    bb_ = 86; continue 'fl1;
                }
                89 => {
                    bb_ = 90; continue 'fl1;
                }
                90 => {
                    F::EftEmit_SpawnType2(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, fp, fp.wrapping_add(32), l_size_20, p_scale_12, l_rate_19);
                    bb_ = 86; continue 'fl1;
                }
                91 => {
                    bb_ = 92; continue 'fl1;
                }
                92 => {
                    F::EftEmit_SpawnType16(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, fp, fp.wrapping_add(32), l_size_20, p_scale_12, l_rate_19);
                    bb_ = 86; continue 'fl1;
                }
                93 => {
                    bb_ = 94; continue 'fl1;
                }
                94 => {
                    F::EftEmit_SpawnType17(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_node_7, p_idx_10, fp, p_pos_4, fp.wrapping_add(32), l_size_20, p_scale_12, l_rate_19);
                    bb_ = 86; continue 'fl1;
                }
                95 => {
                    bb_ = 96; continue 'fl1;
                }
                96 => {
                    F::EftEmit_SpawnType18(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_node_7, p_idx_10, fp, fp.wrapping_add(32), l_size_20, p_scale_12, l_rate_19);
                    bb_ = 86; continue 'fl1;
                }
                97 => {
                    bb_ = 98; continue 'fl1;
                }
                98 => {
                    F::EftEmit_SpawnType14(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, fp, fp.wrapping_add(32), l_size_20, p_scale_12, l_rate_19);
                    bb_ = 86; continue 'fl1;
                }
                99 => {
                    bb_ = 100; continue 'fl1;
                }
                100 => {
                    F::EftEmit_SpawnType5(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, fp, fp.wrapping_add(32), l_size_20, p_scale_12, l_rate_19);
                    bb_ = 86; continue 'fl1;
                }
                101 => {
                    bb_ = 102; continue 'fl1;
                }
                102 => {
                    F::EftEmit_SpawnType9(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, l_size_20, p_scale_12, l_rate_19, fp, fp.wrapping_add(32));
                    bb_ = 86; continue 'fl1;
                }
                103 => {
                    bb_ = 104; continue 'fl1;
                }
                104 => {
                    F::EftEmit_SpawnType10(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, l_size_20, p_scale_12, l_rate_19, fp, p_pos_4, fp.wrapping_add(32));
                    bb_ = 86; continue 'fl1;
                }
                105 => {
                    bb_ = 106; continue 'fl1;
                }
                106 => {
                    F::EftEmit_SpawnType15(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, l_size_20, p_scale_12, l_rate_19, fp, p_pos_4, fp.wrapping_add(32));
                    bb_ = 86; continue 'fl1;
                }
                107 => {
                    bb_ = 108; continue 'fl1;
                }
                108 => {
                    F::EftEmit_SpawnType12(c, p_set_1, l_handles_21, p_flags_11, p_arg7_8, p_objId_6, p_idx_10, l_size_20, p_scale_12, l_rate_19, fp, fp.wrapping_add(32));
                    bb_ = 86; continue 'fl1;
                }
                109 => {
                    bb_ = 86; continue 'fl1;
                }
                110 => {
                    { let v_ = ((((((((c.m.r8(((p_state_2).wrapping_add(320)).wrapping_add(((l_n_17) as u32)))) as i32)) | (2i32))) & ((!(4i32))))) as u8); let a_ = ((p_state_2).wrapping_add(320)).wrapping_add(((l_n_17) as u32)); c.m.w8(a_, v_); };
                    bb_ = 111; continue 'fl1;
                }
                111 => {
                    if ((((p_flags_11) & (2i32))) != 0) { bb_ = 112; continue 'fl1; } else { bb_ = 113; continue 'fl1; }
                }
                112 => {
                    { let r_ = 4i32; let a_ = ((p_state_2).wrapping_add(320)).wrapping_add(((l_n_17) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                    bb_ = 113; continue 'fl1;
                }
                113 => {
                    if ((((((c.m.r8(((p_state_2).wrapping_add(320)).wrapping_add(((l_n_17) as u32)))) as i32)) & (2i32))) != 0) { bb_ = 114; continue 'fl1; } else { bb_ = 115; continue 'fl1; }
                }
                114 => {
                    l_def_18 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_17) as u32)).wrapping_mul(64));
                    if ((((p_flags_11) & (64i32))) != 0) { bb_ = 116; continue 'fl1; } else { bb_ = 117; continue 'fl1; }
                }
                115 => {
                    break 'fl1;
                }
                116 => {
                    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_def_18).wrapping_add(40)))) as i32)) != 0) { bb_ = 118; continue 'fl1; } else { bb_ = 120; continue 'fl1; }
                }
                117 => {
                    bb_ = 115; continue 'fl1;
                }
                118 => {
                    l_total_26 = rt::fmul(F32(c.m.r32((l_def_18).wrapping_add(40))), F32(0x41f00000));
                    l_first_27 = rt::fmul(l_total_26, F32(c.m.r32((l_def_18).wrapping_add(44))));
                    l_t_28 = rt::fadd(F32(c.m.r32(((p_state_2).wrapping_add(520)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), F32(0x3f800000));
                    { let v_ = l_t_28; let a_ = ((p_state_2).wrapping_add(520)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    if (((rt::flt(l_t_28, l_first_27) as i32)) != 0) { bb_ = 121; continue 'fl1; } else { bb_ = 123; continue 'fl1; }
                }
                119 => {
                    bb_ = 117; continue 'fl1;
                }
                120 => {
                    { let r_ = F32(c.m.r32((l_def_18).wrapping_add(48))); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fne(F32(c.m.r32((l_def_18).wrapping_add(36))), F32(0x00000000)) as i32)) != 0) { bb_ = 131; continue 'fl1; } else { bb_ = 132; continue 'fl1; }
                }
                121 => {
                    l_r_30 = rt::fdiv(l_t_28, l_first_27);
                    l_dd_29 = rt::fsub(F32(c.m.r32((l_def_18).wrapping_add(32))), F32(c.m.r32((l_def_18).wrapping_add(28))));
                    { let v_ = rt::fadd(F32(c.m.r32((l_def_18).wrapping_add(28))), rt::fmul(l_dd_29, l_r_30)); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    if (((rt::flt(F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) { bb_ = 124; continue 'fl1; } else { bb_ = 125; continue 'fl1; }
                }
                122 => {
                    bb_ = 119; continue 'fl1;
                }
                123 => {
                    if (((rt::flt(l_t_28, l_total_26) as i32)) != 0) { bb_ = 126; continue 'fl1; } else { bb_ = 128; continue 'fl1; }
                }
                124 => {
                    { let v_ = F32(0x00000000); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 125; continue 'fl1;
                }
                125 => {
                    bb_ = 122; continue 'fl1;
                }
                126 => {
                    l_r_30 = rt::fdiv(rt::fsub(l_t_28, l_first_27), rt::fsub(l_total_26, l_first_27));
                    l_dd_29 = rt::fsub(F32(c.m.r32((l_def_18).wrapping_add(36))), F32(c.m.r32((l_def_18).wrapping_add(32))));
                    { let v_ = rt::fadd(F32(c.m.r32((l_def_18).wrapping_add(32))), rt::fmul(l_dd_29, l_r_30)); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    if (((rt::flt(F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) { bb_ = 129; continue 'fl1; } else { bb_ = 130; continue 'fl1; }
                }
                127 => {
                    bb_ = 122; continue 'fl1;
                }
                128 => {
                    { let v_ = F32(c.m.r32((l_def_18).wrapping_add(36))); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 127; continue 'fl1;
                }
                129 => {
                    { let v_ = F32(0x00000000); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 130; continue 'fl1;
                }
                130 => {
                    bb_ = 127; continue 'fl1;
                }
                131 => {
                    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_def_18).wrapping_add(48)))) as i32)) != 0) { bb_ = 133; continue 'fl1; } else { bb_ = 135; continue 'fl1; }
                }
                132 => {
                    bb_ = 119; continue 'fl1;
                }
                133 => {
                    if (((rt::flt(F32(c.m.r32((l_def_18).wrapping_add(36))), F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4))))) as i32)) != 0) { bb_ = 136; continue 'fl1; } else { bb_ = 137; continue 'fl1; }
                }
                134 => {
                    bb_ = 132; continue 'fl1;
                }
                135 => {
                    if (((rt::flt(F32(c.m.r32((l_def_18).wrapping_add(48))), F32(0x00000000)) as i32)) != 0) { bb_ = 138; continue 'fl1; } else { bb_ = 139; continue 'fl1; }
                }
                136 => {
                    { let v_ = F32(c.m.r32((l_def_18).wrapping_add(36))); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 137; continue 'fl1;
                }
                137 => {
                    bb_ = 134; continue 'fl1;
                }
                138 => {
                    if (((rt::flt(F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_def_18).wrapping_add(36)))) as i32)) != 0) { bb_ = 140; continue 'fl1; } else { bb_ = 141; continue 'fl1; }
                }
                139 => {
                    bb_ = 134; continue 'fl1;
                }
                140 => {
                    { let v_ = F32(c.m.r32((l_def_18).wrapping_add(36))); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 141; continue 'fl1;
                }
                141 => {
                    if (((rt::flt(F32(c.m.r32(((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) { bb_ = 142; continue 'fl1; } else { bb_ = 143; continue 'fl1; }
                }
                142 => {
                    { let v_ = F32(0x00000000); let a_ = ((p_state_2).wrapping_add(360)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 143; continue 'fl1;
                }
                143 => {
                    bb_ = 139; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_MarkCond(c: &mut Ctx, mut p_objId_1: i32, mut p_extra_2: i32, mut p_set_3: u32, mut p_state_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_MarkCond", &PROBE_N, &[(p_objId_1) as u32, (p_extra_2) as u32, (p_set_3) as u32, (p_state_4) as u32], false); }
    let mut l_mask_5: u32 = 0;
    let mut l_half_6: i32 = 0;
    let mut l_type_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_hdr_9: u32 = 0;
    let mut l_grp_10: u32 = 0;
    let mut l_gd_11: u32 = 0;
    let mut l_n_12: i32 = 0;
    let mut l_def_13: u32 = 0;
    let r_ = 'f: {
    l_mask_5 = (((((F::EftShot_TestBits(c, p_objId_1, 2i32)) != (0i32)) as i32)) as u32);
    if ((F::EftShot_TestBits(c, p_objId_1, 4i32)) != 0) {
        { let r_ = ((2i32) as u32); let n_ = ((l_mask_5) | (r_)); l_mask_5 = n_; };
    }
    if ((F::EftShot_TestBits(c, p_objId_1, 16i32)) != 0) {
        { let r_ = ((4i32) as u32); let n_ = ((l_mask_5) | (r_)); l_mask_5 = n_; };
    }
    if ((F::EftShot_TestBits(c, p_objId_1, 32i32)) != 0) {
        { let r_ = ((8i32) as u32); let n_ = ((l_mask_5) | (r_)); l_mask_5 = n_; };
    }
    if ((F::EftShot_TestBits(c, p_objId_1, 64i32)) != 0) {
        { let r_ = ((16i32) as u32); let n_ = ((l_mask_5) | (r_)); l_mask_5 = n_; };
    }
    if ((p_extra_2) != 0) {
        { let r_ = ((32i32) as u32); let n_ = ((l_mask_5) | (r_)); l_mask_5 = n_; };
    }
    l_hdr_9 = c.m.r32(p_set_3);
    {
        l_half_6 = 0i32;
        'l1: loop {
            if !(((((l_half_6) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_type_7 = 0i32;
                    'l3: loop {
                        if !(((((l_type_7) < (19i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((((c.m.r32(l_hdr_9)) & ((((1i32).wrapping_shl((l_type_7) as u32)) as u32)))) != 0) {
                                l_grp_10 = ((p_set_3).wrapping_add(12)).wrapping_add((((l_type_7) as u32)).wrapping_mul(16));
                                l_gd_11 = c.m.r32(l_grp_10);
                                {
                                    l_i_8 = 0i32;
                                    'l5: loop {
                                        if !(((((l_i_8) < (((c.m.r8((l_gd_11).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_n_12 = (((c.m.r8((l_grp_10).wrapping_add(8))) as i32)).wrapping_add(l_i_8);
                                            l_def_13 = (c.m.r32((p_set_3).wrapping_add(8))).wrapping_add((((l_n_12) as u32)).wrapping_mul(64));
                                            if ((((((((((c.m.r8((l_def_13).wrapping_add(52))) as i32)) & (4i32))) != 0) && ((((l_mask_5) & ((((1i32).wrapping_shl((((c.m.r8((l_def_13).wrapping_add(53))) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
                                                { let r_ = 32i32; let a_ = ((p_state_4).wrapping_add(320)).wrapping_add(((l_n_12) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                                            }
                                        }
                                        { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                                    }
                                }
                            }
                        }
                        { let o_ = l_type_7; let n_ = (o_).wrapping_add(1); l_type_7 = n_; };
                    }
                }
            }
            { let o_ = l_half_6; let n_ = (o_).wrapping_add(1); l_half_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_MarkCond", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_MarkKind6(c: &mut Ctx, mut p_set_1: u32, mut p_state_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_MarkKind6", &PROBE_N, &[(p_set_1) as u32, (p_state_2) as u32], false); }
    let mut l_type_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_hdr_5: u32 = 0;
    let mut l_grp_6: u32 = 0;
    let mut l_gd_7: u32 = 0;
    let mut l_n_8: i32 = 0;
    let mut l_def_9: u32 = 0;
    let mut l_stopped_10: i32 = 0;
    let r_ = 'f: {
    l_hdr_5 = c.m.r32(p_set_1);
    {
        l_type_3 = 0i32;
        'l1: loop {
            if !(((((l_type_3) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((c.m.r32(l_hdr_5)) & ((((1i32).wrapping_shl((l_type_3) as u32)) as u32)))) != 0) {
                    l_grp_6 = ((p_set_1).wrapping_add(12)).wrapping_add((((l_type_3) as u32)).wrapping_mul(16));
                    l_gd_7 = c.m.r32(l_grp_6);
                    {
                        l_i_4 = 0i32;
                        'l3: loop {
                            if !(((((l_i_4) < (((c.m.r8((l_gd_7).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_n_8 = (((c.m.r8((l_grp_6).wrapping_add(8))) as i32)).wrapping_add(l_i_4);
                                l_def_9 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_8) as u32)).wrapping_mul(64));
                                if ((((!((((((c.m.r8(((p_state_2).wrapping_add(320)).wrapping_add(((l_n_8) as u32)))) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                                    l_stopped_10 = ((((c.m.r8(((p_state_2).wrapping_add(320)).wrapping_add(((l_n_8) as u32)))) as i32)) & (4i32));
                                    if ((((((((!((l_stopped_10) != 0)) as i32)) != 0) && (((((((c.m.r8((l_def_9).wrapping_add(8))) as i32)) == (6i32)) as i32)) != 0)) as i32)) != 0) {
                                        { let r_ = 64i32; let a_ = ((p_state_2).wrapping_add(320)).wrapping_add(((l_n_8) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                                    }
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_type_3; let n_ = (o_).wrapping_add(1); l_type_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_MarkKind6", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_KillAll(c: &mut Ctx, mut p_set_1: u32, mut p_state_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_KillAll", &PROBE_N, &[(p_set_1) as u32, (p_state_2) as u32], false); }
    let mut l_half_3: i32 = 0;
    let mut l_type_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_hdr_6: u32 = 0;
    let mut l_handles_7: u32 = 0;
    let mut l_grp_8: u32 = 0;
    let mut l_gd_9: u32 = 0;
    let mut l_n_10: i32 = 0;
    let r_ = 'f: {
    l_hdr_6 = c.m.r32(p_set_1);
    {
        l_half_3 = 0i32;
        'l1: loop {
            if !(((((l_half_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_handles_7 = (p_state_2).wrapping_add((((l_half_3) as u32)).wrapping_mul(160));
                {
                    l_type_4 = 0i32;
                    'l3: loop {
                        if !(((((l_type_4) < (19i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((((c.m.r32(l_hdr_6)) & ((((1i32).wrapping_shl((l_type_4) as u32)) as u32)))) != 0) {
                                l_grp_8 = ((p_set_1).wrapping_add(12)).wrapping_add((((l_type_4) as u32)).wrapping_mul(16));
                                l_gd_9 = c.m.r32(l_grp_8);
                                {
                                    l_i_5 = 0i32;
                                    'l5: loop {
                                        if !(((((l_i_5) < (((c.m.r8((l_gd_9).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_n_10 = (((c.m.r8((l_grp_8).wrapping_add(8))) as i32)).wrapping_add(l_i_5);
                                            's7: {
                                                let sv11: i32 = l_type_4;
                                                match sv11 {
                                                    0i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); { let _ = F::EftRay_Kill(c, a0_); } };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    2i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); F::EftRays_Kill(c, a0_) };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    16i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); { let _ = F::EftBill_Kill(c, a0_); } };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    17i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); { let _ = F::EftRibbon_Kill(c, a0_); } };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    18i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); { let _ = F::EftChain_Kill(c, a0_); } };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    14i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); { let _ = F::EftAnimPart_Kill(c, a0_); } };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    5i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); { let _ = F::EftPtcl_Kill(c, a0_); } };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    9i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); F::EftQuad_Kill(c, a0_) };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    10i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); F::EftPart10_Kill(c, a0_) };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    15i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); F::EftLink_Kill(c, a0_) };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    12i32 => {
                                                        if (((((c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                                            { let a0_ = c.m.r32((l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4))); F::EftZap_Kill(c, a0_) };
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_7).wrapping_add((((l_n_10) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    _ => {
                                                    }
                                                }
                                            }
                                        }
                                        { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                                    }
                                }
                            }
                        }
                        { let o_ = l_type_4; let n_ = (o_).wrapping_add(1); l_type_4 = n_; };
                    }
                }
            }
            { let o_ = l_half_3; let n_ = (o_).wrapping_add(1); l_half_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_KillAll", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_UpdateAlive(c: &mut Ctx, mut p_set_1: u32, mut p_state_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateAlive", &PROBE_N, &[(p_set_1) as u32, (p_state_2) as u32], false); }
    let mut l_result_3: i32 = 0;
    let mut l_hdr_4: u32 = 0;
    let mut l_half_5: i32 = 0;
    let mut l_type_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_handles_8: u32 = 0;
    let mut l_grp_9: u32 = 0;
    let mut l_gd_10: u32 = 0;
    let mut l_n_11: i32 = 0;
    let mut l_def_12: u32 = 0;
    let r_ = 'f: {
    l_result_3 = 0i32;
    l_hdr_4 = c.m.r32(p_set_1);
    {
        l_half_5 = 0i32;
        'l1: loop {
            if !(((((l_half_5) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_handles_8 = (p_state_2).wrapping_add((((l_half_5) as u32)).wrapping_mul(160));
                {
                    l_type_6 = 0i32;
                    'l3: loop {
                        if !(((((l_type_6) < (19i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((((c.m.r32(l_hdr_4)) & ((((1i32).wrapping_shl((l_type_6) as u32)) as u32)))) != 0) {
                                l_grp_9 = ((p_set_1).wrapping_add(12)).wrapping_add((((l_type_6) as u32)).wrapping_mul(16));
                                l_gd_10 = c.m.r32(l_grp_9);
                                {
                                    l_i_7 = 0i32;
                                    'l5: loop {
                                        if !(((((l_i_7) < (((c.m.r8((l_gd_10).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_n_11 = (((c.m.r8((l_grp_9).wrapping_add(8))) as i32)).wrapping_add(l_i_7);
                                            l_def_12 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_11) as u32)).wrapping_mul(64));
                                            's7: {
                                                let sv13: i32 = l_type_6;
                                                match sv13 {
                                                    0i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftRay_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    2i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftRays_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    16i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftBill_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    17i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftRibbon_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    18i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftChain_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    14i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftAnimPart_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    5i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftPtcl_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    9i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftQuad_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    10i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftPart10_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    15i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftLink_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    12i32 => {
                                                        if (({ let a0_ = c.m.r32((l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); F::EftZap_IsAlive(c, a0_) }) != 0) {
                                                            if (((((((c.m.r8((l_def_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) {
                                                                { let r_ = (1i32).wrapping_shl((l_type_6) as u32); let n_ = ((l_result_3) | (r_)); l_result_3 = n_; };
                                                            }
                                                        } else {
                                                            { let v_ = ((0i32) as u32); let a_ = (l_handles_8).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                        }
                                                        break 's7;
                                                    }
                                                    _ => {
                                                    }
                                                }
                                            }
                                        }
                                        { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                                    }
                                }
                            }
                        }
                        { let o_ = l_type_6; let n_ = (o_).wrapping_add(1); l_type_6 = n_; };
                    }
                }
            }
            { let o_ = l_half_5; let n_ = (o_).wrapping_add(1); l_half_5 = n_; };
        }
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_UpdateNodesReq(c: &mut Ctx, mut p_owner_1: u32, mut p_nodes_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateNodesReq", &PROBE_N, &[(p_owner_1) as u32, (p_nodes_2) as u32], false); }
    let mut l_objId_3: i32 = 0;
    let mut l_req_4: i32 = 0;
    let r_ = 'f: {
    l_objId_3 = (c.m.r32(p_owner_1) as i32);
    l_req_4 = (if ((F::EftShot_TestBits(c, l_objId_3, 2i32)) != 0) { 2i32 } else { 0i32 });
    if ((F::EftShot_TestBits(c, l_objId_3, 4i32)) != 0) {
        { let r_ = 4i32; let n_ = ((l_req_4) | (r_)); l_req_4 = n_; };
    }
    if ((F::EftShot_TestBits(c, l_objId_3, 8i32)) != 0) {
        { let r_ = 8i32; let n_ = ((l_req_4) | (r_)); l_req_4 = n_; };
    }
    if ((F::EftShot_TestBits(c, l_objId_3, 16i32)) != 0) {
        { let r_ = 16i32; let n_ = ((l_req_4) | (r_)); l_req_4 = n_; };
    }
    if ((F::EftShot_TestBits(c, l_objId_3, 32i32)) != 0) {
        { let r_ = 32i32; let n_ = ((l_req_4) | (r_)); l_req_4 = n_; };
    }
    if ((F::EftShot_TestBits(c, l_objId_3, 64i32)) != 0) {
        { let r_ = 64i32; let n_ = ((l_req_4) | (r_)); l_req_4 = n_; };
    }
    if ((F::EftShot_TestBits(c, l_objId_3, 1024i32)) != 0) {
        { let r_ = 1024i32; let n_ = ((l_req_4) | (r_)); l_req_4 = n_; };
    }
    F::EftEmit_UpdateNodes(c, p_owner_1, p_nodes_2, l_req_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateNodesReq", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_UpdateNodes(c: &mut Ctx, mut p_owner_1: u32, mut p_nodes_2: u32, mut p_req_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateNodes", &PROBE_N, &[(p_owner_1) as u32, (p_nodes_2) as u32, (p_req_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_objId_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 24);
        { let v_ = 2i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 32i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 64i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        l_objId_4 = (c.m.r32(p_owner_1) as i32);
        if ((((p_req_3) & (1024i32))) != 0) {
            { let r_ = ((4096i32) as u32); let a_ = p_nodes_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32(p_nodes_2)) & (((4096i32) as u32)))) != 0) {
            break 'f;
        }
        if (((({ let a_ = (c.m.r32((p_owner_1).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
            {
                l_i_5 = 0i32;
                'l1: loop {
                    if !(((((l_i_5) < (6i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if ((((!((((c.m.r32(p_nodes_2)) & ((((1i32).wrapping_shl(((l_i_5).wrapping_mul(2i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
                            if ((((p_req_3) & ((c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)))) != 0) {
                                { let r_ = (((1i32).wrapping_shl(((l_i_5).wrapping_mul(2i32)) as u32)) as u32); let a_ = p_nodes_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                { let v_ = (({ let a_ = ((c.m.r32((p_owner_1).wrapping_add(36))).wrapping_add(14)).wrapping_add(((l_i_5) as u32)); (c.m.r8(a_) as i8) }) as i32); let a_ = (((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        if ((((c.m.r32(p_nodes_2)) & ((((1i32).wrapping_shl(((l_i_5).wrapping_mul(2i32)) as u32)) as u32)))) != 0) {
                            { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                        } else {
                            if ((((c.m.r32(p_nodes_2)) & (((1i32) as u32)))) != 0) {
                                { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                            }
                        }
                        if ((((c.m.r32(p_nodes_2)) & ((((1i32).wrapping_shl((((l_i_5).wrapping_mul(2i32)).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
                            { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((1i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                        } else {
                            if ((((c.m.r32(p_nodes_2)) & (((1i32) as u32)))) != 0) {
                                { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                            }
                        }
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
        } else {
            {
                l_i_5 = 0i32;
                'l3: loop {
                    if !(((((l_i_5) < (6i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if ((((!((((c.m.r32(p_nodes_2)) & ((((1i32).wrapping_shl(((l_i_5).wrapping_mul(2i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
                            if ((((p_req_3) & ((c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)))) != 0) {
                                { let r_ = (((1i32).wrapping_shl(((l_i_5).wrapping_mul(2i32)) as u32)) as u32); let a_ = p_nodes_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                if (({ let a1_ = (((c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u64); F::EftShot_HasTwoAttrs(c, l_objId_4, a1_) }) != 0) {
                                    { let r_ = (((1i32).wrapping_shl((((l_i_5).wrapping_mul(2i32)).wrapping_add(1i32)) as u32)) as u32); let a_ = p_nodes_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                    { let a1_ = (((c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u64); { let _ = F::EftShot_GetAttrPair(c, l_objId_4, a1_, (((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32)), (((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((1i32) as u32)).wrapping_mul(32))); } };
                                } else {
                                    { let v_ = { let a1_ = (((c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u64); F::EftShot_GetAttrKind(c, l_objId_4, a1_) }; let a_ = (((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32)); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                        if ((((c.m.r32(p_nodes_2)) & ((((1i32).wrapping_shl(((l_i_5).wrapping_mul(2i32)) as u32)) as u32)))) != 0) {
                            { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                        } else {
                            if ((((c.m.r32(p_nodes_2)) & (((1i32) as u32)))) != 0) {
                                { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                            }
                        }
                        if ((((c.m.r32(p_nodes_2)) & ((((1i32).wrapping_shl((((l_i_5).wrapping_mul(2i32)).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
                            { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((1i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                        } else {
                            if ((((c.m.r32(p_nodes_2)) & (((1i32) as u32)))) != 0) {
                                { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(64))).wrapping_add((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                            }
                        }
                        if (((({ let a_ = (c.m.r32((p_owner_1).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                            if ((((!((((c.m.r32((p_nodes_2).wrapping_add(4))) & ((((1i32).wrapping_shl((l_i_5) as u32)) as u32)))) != 0)) as i32)) != 0) {
                                if ((((p_req_3) & ((c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)))) != 0) {
                                    { let r_ = (((1i32).wrapping_shl((l_i_5) as u32)) as u32); let a_ = (p_nodes_2).wrapping_add(4); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                    { let r_ = (((1i32).wrapping_shl(((l_i_5).wrapping_mul(2i32)) as u32)) as u32); let a_ = p_nodes_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                    { let v_ = (({ let a_ = ((c.m.r32((p_owner_1).wrapping_add(36))).wrapping_add(14)).wrapping_add(((l_i_5) as u32)); (c.m.r8(a_) as i8) }) as i32); let a_ = ((p_nodes_2).wrapping_add(400)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32)); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            if ((((c.m.r32((p_nodes_2).wrapping_add(4))) & ((((1i32).wrapping_shl((l_i_5) as u32)) as u32)))) != 0) {
                                { let a1_ = (c.m.r32(((p_nodes_2).wrapping_add(400)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, (((p_nodes_2).wrapping_add(400)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                            } else {
                                if ((((c.m.r32((p_nodes_2).wrapping_add(4))) & (((1i32) as u32)))) != 0) {
                                    { let a1_ = (c.m.r32(((p_nodes_2).wrapping_add(400)).wrapping_add((((0i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, l_objId_4, a1_, (((p_nodes_2).wrapping_add(400)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                                }
                            }
                        }
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SetNode(c: &mut Ctx, mut p_nodes_1: u32, mut p_slot_2: i32, mut p_node_3: i32, mut p_pos_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SetNode", &PROBE_N, &[(p_nodes_1) as u32, (p_slot_2) as u32, (p_node_3) as u32, (p_pos_4) as u32], false); }
    let r_ = 'f: {
    { let r_ = (((1i32).wrapping_shl(((p_slot_2).wrapping_mul(2i32)) as u32)) as u32); let a_ = p_nodes_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    if (((((p_node_3) >= (0i32)) as i32)) != 0) {
        { let r_ = (((1i32).wrapping_shl(((p_slot_2).wrapping_mul(2i32)) as u32)) as u32); let a_ = (p_nodes_1).wrapping_add(8); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let v_ = p_node_3; let a_ = (((p_nodes_1).wrapping_add(16)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32)); c.m.w32(a_, (v_) as u32); };
    } else {
        F::Vec4_Copy(c, ((((p_nodes_1).wrapping_add(16)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16), p_pos_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SetNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SetNodePos(c: &mut Ctx, mut p_nodes_1: u32, mut p_slot_2: i32, mut p_pos_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SetNodePos", &PROBE_N, &[(p_nodes_1) as u32, (p_slot_2) as u32, (p_pos_3) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, ((((p_nodes_1).wrapping_add(16)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16), p_pos_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SetNodePos", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_RefreshFixedNodes(c: &mut Ctx, mut p_objId_1: i32, mut p_nodes_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_RefreshFixedNodes", &PROBE_N, &[(p_objId_1) as u32, (p_nodes_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_bit_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_bit_4 = (1i32).wrapping_shl(((l_i_3).wrapping_mul(2i32)) as u32);
                if ((((((((c.m.r32(p_nodes_2)) & (((l_bit_4) as u32)))) != 0) && ((((c.m.r32((p_nodes_2).wrapping_add(8))) & (((l_bit_4) as u32)))) != 0)) as i32)) != 0) {
                    { let a1_ = (c.m.r32((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))) as i32); F::BtlCharApi_GetNodePos(c, p_objId_1, a1_, ((((p_nodes_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_RefreshFixedNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_UpdateTrailWidth(c: &mut Ctx, mut p_set_1: u32, mut p_state_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateTrailWidth", &PROBE_N, &[(p_set_1) as u32, (p_state_2) as u32], false); }
    let mut l_hdr_3: u32 = 0;
    let mut l_total_4: F32 = F32(0);
    let mut l_first_5: F32 = F32(0);
    let mut l_t_6: F32 = F32(0);
    let mut l_d_7: F32 = F32(0);
    let mut l_r_8: F32 = F32(0);
    let r_ = 'f: {
    l_hdr_3 = c.m.r32(p_set_1);
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_hdr_3).wrapping_add(20)))) as i32)) != 0) {
        l_total_4 = rt::fmul(F32(c.m.r32((l_hdr_3).wrapping_add(20))), F32(0x41f00000));
        l_first_5 = rt::fmul(l_total_4, F32(c.m.r32((l_hdr_3).wrapping_add(24))));
        l_t_6 = rt::fadd(F32(c.m.r32((p_state_2).wrapping_add(692))), F32(0x3f800000));
        { let v_ = l_t_6; let a_ = (p_state_2).wrapping_add(692); c.m.w32(a_, (v_).0); };
        if (((rt::flt(l_t_6, l_first_5) as i32)) != 0) {
            l_r_8 = rt::fdiv(l_t_6, l_first_5);
            l_d_7 = rt::fsub(F32(c.m.r32((l_hdr_3).wrapping_add(12))), F32(c.m.r32((l_hdr_3).wrapping_add(8))));
            { let v_ = rt::fadd(F32(c.m.r32((l_hdr_3).wrapping_add(8))), rt::fmul(l_d_7, l_r_8)); let a_ = (p_state_2).wrapping_add(680); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(c.m.r32((p_state_2).wrapping_add(680))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_state_2).wrapping_add(680); c.m.w32(a_, (v_).0); };
            }
        } else {
            if (((rt::flt(l_t_6, l_total_4) as i32)) != 0) {
                l_r_8 = rt::fdiv(rt::fsub(l_t_6, l_first_5), rt::fsub(l_total_4, l_first_5));
                l_d_7 = rt::fsub(F32(c.m.r32((l_hdr_3).wrapping_add(16))), F32(c.m.r32((l_hdr_3).wrapping_add(12))));
                { let v_ = rt::fadd(F32(c.m.r32((l_hdr_3).wrapping_add(12))), rt::fmul(l_d_7, l_r_8)); let a_ = (p_state_2).wrapping_add(680); c.m.w32(a_, (v_).0); };
                if (((rt::flt(F32(c.m.r32((p_state_2).wrapping_add(680))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (p_state_2).wrapping_add(680); c.m.w32(a_, (v_).0); };
                }
            } else {
                { let v_ = F32(c.m.r32((l_hdr_3).wrapping_add(16))); let a_ = (p_state_2).wrapping_add(680); c.m.w32(a_, (v_).0); };
            }
        }
    } else {
        { let r_ = F32(c.m.r32((l_hdr_3).wrapping_add(28))); let a_ = (p_state_2).wrapping_add(680); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((((((rt::flt(F32(0x00000000), F32(c.m.r32((l_hdr_3).wrapping_add(16)))) as i32)) != 0) && (((rt::flt(F32(c.m.r32((l_hdr_3).wrapping_add(16))), F32(c.m.r32((p_state_2).wrapping_add(680)))) as i32)) != 0)) as i32)) != 0) {
            { let v_ = F32(c.m.r32((l_hdr_3).wrapping_add(16))); let a_ = (p_state_2).wrapping_add(680); c.m.w32(a_, (v_).0); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateTrailWidth", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetTrailWidth(c: &mut Ctx, mut p_state_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetTrailWidth", &PROBE_N, &[(p_state_1) as u32], false); }
    let r_ = 'f: {
    break 'f F32(c.m.r32((p_state_1).wrapping_add(680)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetTrailWidth", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_HasWidth2(c: &mut Ctx, mut p_set_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_HasWidth2", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    if (((((({ let a_ = (c.m.r32(p_set_1)).wrapping_add(33); c.m.r8(a_) }) as i32)) & (4i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_HasWidth2", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_UpdateWidth2(c: &mut Ctx, mut p_set_1: u32, mut p_state_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateWidth2", &PROBE_N, &[(p_set_1) as u32, (p_state_2) as u32], false); }
    let mut l_hdr_3: u32 = 0;
    let mut l_total_4: F32 = F32(0);
    let mut l_first_5: F32 = F32(0);
    let mut l_t_6: F32 = F32(0);
    let mut l_d_7: F32 = F32(0);
    let mut l_r_8: F32 = F32(0);
    let r_ = 'f: {
    l_hdr_3 = c.m.r32(p_set_1);
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_hdr_3).wrapping_add(56)))) as i32)) != 0) {
        l_total_4 = rt::fmul(F32(c.m.r32((l_hdr_3).wrapping_add(56))), F32(0x41f00000));
        l_t_6 = rt::fadd(F32(c.m.r32((p_state_2).wrapping_add(708))), F32(0x3f800000));
        l_first_5 = rt::fmul(l_total_4, F32(c.m.r32((l_hdr_3).wrapping_add(60))));
        { let v_ = l_t_6; let a_ = (p_state_2).wrapping_add(708); c.m.w32(a_, (v_).0); };
        if (((rt::flt(l_t_6, l_first_5) as i32)) != 0) {
            l_r_8 = rt::fdiv(l_t_6, l_first_5);
            l_d_7 = rt::fsub(F32(c.m.r32((l_hdr_3).wrapping_add(48))), F32(c.m.r32((l_hdr_3).wrapping_add(44))));
            { let v_ = rt::fadd(F32(c.m.r32((l_hdr_3).wrapping_add(44))), rt::fmul(l_d_7, l_r_8)); let a_ = (p_state_2).wrapping_add(696); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(c.m.r32((p_state_2).wrapping_add(696))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_state_2).wrapping_add(696); c.m.w32(a_, (v_).0); };
            }
        } else {
            if (((rt::flt(l_t_6, l_total_4) as i32)) != 0) {
                l_r_8 = rt::fdiv(rt::fsub(l_t_6, l_first_5), rt::fsub(l_total_4, l_first_5));
                l_d_7 = rt::fsub(F32(c.m.r32((l_hdr_3).wrapping_add(52))), F32(c.m.r32((l_hdr_3).wrapping_add(48))));
                { let v_ = rt::fadd(F32(c.m.r32((l_hdr_3).wrapping_add(48))), rt::fmul(l_d_7, l_r_8)); let a_ = (p_state_2).wrapping_add(696); c.m.w32(a_, (v_).0); };
                if (((rt::flt(F32(c.m.r32((p_state_2).wrapping_add(696))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (p_state_2).wrapping_add(696); c.m.w32(a_, (v_).0); };
                }
            } else {
                { let v_ = F32(c.m.r32((l_hdr_3).wrapping_add(52))); let a_ = (p_state_2).wrapping_add(696); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_UpdateWidth2", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetWidth2(c: &mut Ctx, mut p_state_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetWidth2", &PROBE_N, &[(p_state_1) as u32], false); }
    let r_ = 'f: {
    break 'f F32(c.m.r32((p_state_1).wrapping_add(696)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetWidth2", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_AddMark(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_AddMark", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_hit_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_active_5: i32 = 0;
    let mut l_width_6: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_hit_3 = F::StgCol_GetHit(c);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (15i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_active_5 = (c.m.r32((((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)) as i32);
                    if (((((l_active_5) == (0i32)) as i32)) != 0) {
                        { let v_ = F32(0x41200000); let a_ = (((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(20); c.m.w32(a_, (v_).0); };
                        l_active_5 = 1i32;
                        { let v_ = l_active_5; let a_ = (((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        F::Vec4_Copy(c, ((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32)), (l_hit_3).wrapping_add(16));
                        l_width_6 = rt::fmul(rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(1932))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(100))), F32(0x3f000000));
                        F::Vec3_Sub(c, fp.wrapping_add(16), l_w_2, ((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32)));
                        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                        F::Vec3_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), l_width_6);
                        F::Vec3_Add(c, ((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32)), ((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32)), fp.wrapping_add(16));
                        { let v_ = F32(0x3f800000); let a_ = (((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec4_Copy(c, ((l_w_2).wrapping_add(32)).wrapping_add(16), ((l_w_2).wrapping_add(1424)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32)));
                        if ((((((c.m.r32((l_hit_3).wrapping_add(64)) as i32)) >= (0i32)) as i32)) != 0) {
                            F::Vec3_Sub(c, fp, ((l_w_2).wrapping_add(32)).wrapping_add(16), l_w_2);
                            F::Vec3_Normalize(c, fp, fp);
                            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let a1_ = (c.m.r32((l_hit_3).wrapping_add(64)) as i32); { let _ = F::BtlStage_DestroyObj(c, (1i32).wrapping_neg(), a1_, fp); } };
                        }
                        break 'f;
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_AddMark", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_UpdateMarks(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_UpdateMarks", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_any_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_owner_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_pos_6: u32 = 0;
    let mut l_effect_7: i32 = 0;
    let mut l_size_8: F32 = F32(0);
    let r_ = 'f: {
        l_any_2 = 0i32;
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_owner_4 = c.m.r32((l_w_3).wrapping_add(96));
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (15i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_pos_6 = ((l_w_3).wrapping_add(1424)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32));
                    if (((c.m.r32((((l_w_3).wrapping_add(1424)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(16)) as i32)) != 0) {
                        if (((rt::fle(F32(c.m.r32((((l_w_3).wrapping_add(1424)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
                            l_effect_7 = (({ let a_ = ({ let a_ = (c.m.r32((l_w_3).wrapping_add(96))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(66); (c.m.r16(a_) as i16) }) as i32);
                            if (((((l_effect_7) >= (0i32)) as i32)) != 0) {
                                { let s_ = { rt::memset(c, fp.wrapping_add(48), 0, 48);; { let s_ = l_pos_6; rt::memcpy(c, fp.wrapping_add(48).wrapping_add(0), s_, 16); }; rt::memset(c, fp.wrapping_add(48).wrapping_add(16), 0, 16);; rt::memset(c, fp.wrapping_add(48).wrapping_add(16).wrapping_add(0), 0, 16);; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }; { let v_ = l_effect_7; c.m.w32(fp.wrapping_add(48).wrapping_add(32), (v_) as u32); }; { let v_ = (c.m.r32(l_owner_4) as i32); c.m.w32(fp.wrapping_add(48).wrapping_add(36), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(40), (v_) as u32); }; fp.wrapping_add(48) }; let d_ = fp; rt::memcpy(c, d_, s_, 48); };
                                l_size_8 = { let a_ = (c.m.r32((l_owner_4).wrapping_add(36))).wrapping_add(84); F32(c.m.r32(a_)) };
                                F::Vec4_Sub(c, (fp).wrapping_add(16), ((l_w_3).wrapping_add(32)).wrapping_add(16), l_w_3);
                                F::Vec3_Normalize(c, (fp).wrapping_add(16), (fp).wrapping_add(16));
                                { let _ = F::EftImpact_SpawnBlast(c, fp, l_size_8, F32(0x3f800000)); };
                            }
                            { let v_ = 0i32; let a_ = (((l_w_3).wrapping_add(1424)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        } else {
                            l_any_2 = 1i32;
                            { let r_ = F32(0x3f800000); let a_ = (((l_w_3).wrapping_add(1424)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32))).wrapping_add(20); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        break 'f l_any_2;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_UpdateMarks", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_InitPath(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_InitPath", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_owner_3 = c.m.r32((l_w_2).wrapping_add(96));
        l_opp_4 = { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(16)) };
        F::BtlCharApi_GetNodePos(c, l_opp_4, 3i32, fp);
        F::Vec4_Sub(c, fp.wrapping_add(16), fp, fp.wrapping_add(16));
        { let v_ = rt::fadd({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16)); F::Port_sqrtf(c, a0_) }, F32(0x42480000)); let a_ = (l_w_2).wrapping_add(1904); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(0x44480000), F32(c.m.r32((l_w_2).wrapping_add(1904)))) as i32)) != 0) {
            { let v_ = F32(0x44480000); let a_ = (l_w_2).wrapping_add(1904); c.m.w32(a_, (v_).0); };
        }
        { let v_ = F32(0x3c9a6a61); let a_ = (l_w_2).wrapping_add(1912); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xbe545245); let a_ = (l_w_2).wrapping_add(1908); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_InitPath", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_Move(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Move", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let mut l_cur_4: u32 = 0;
    let mut l_opp_5: i32 = 0;
    let mut l_pitch_6: F32 = F32(0);
    let mut l_yaw_7: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_owner_3 = c.m.r32((l_w_2).wrapping_add(96));
        l_cur_4 = ((l_w_2).wrapping_add(32)).wrapping_add(16);
        l_opp_5 = { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(128)) };
        F::BtlCharApi_GetNodePos(c, l_opp_5, 3i32, fp.wrapping_add(144));
        F::Vec4_Sub(c, fp.wrapping_add(128), fp.wrapping_add(144), fp.wrapping_add(128));
        { let v_ = rt::fadd({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(128), fp.wrapping_add(128)); F::Port_sqrtf(c, a0_) }, F32(0x42480000)); let a_ = (l_w_2).wrapping_add(1904); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(0x44480000), F32(c.m.r32((l_w_2).wrapping_add(1904)))) as i32)) != 0) {
            { let v_ = F32(0x44480000); let a_ = (l_w_2).wrapping_add(1904); c.m.w32(a_, (v_).0); };
        }
        { let a3_ = F32(c.m.r32((l_w_2).wrapping_add(1904))); F::Vec4_Set(c, l_cur_4, F32(0x00000000), F32(0x00000000), a3_, F32(0x3f800000)) };
        F::Mtx_StoreIdentity(c, fp);
        { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(1908))); F::Mtx_RotateY(c, fp, fp, a2_) };
        F::Mtx_MulVec4(c, l_cur_4, fp, l_cur_4);
        l_pitch_6 = { let a0_ = rt::fneg(F32(c.m.r32(((l_w_2).wrapping_add(16)).wrapping_add(4)))); F::Port_asinf(c, a0_) };
        l_yaw_7 = { let a0_ = F32(c.m.r32((l_w_2).wrapping_add(16))); let a1_ = F32(c.m.r32(((l_w_2).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
        F::Mtx_RotateX(c, fp.wrapping_add(64), fp, l_pitch_6);
        F::Mtx_RotateY(c, fp.wrapping_add(64), fp.wrapping_add(64), l_yaw_7);
        F::Mtx_MulVec4(c, l_cur_4, fp.wrapping_add(64), l_cur_4);
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            F::Vec3_Add(c, ((l_w_2).wrapping_add(32)).wrapping_add(32), l_cur_4, l_w_2);
            { let r_ = 64i32; let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        F::Vec3_Add(c, l_cur_4, l_cur_4, l_w_2);
        { let r_ = F32(c.m.r32((l_w_2).wrapping_add(1912))); let a_ = (l_w_2).wrapping_add(1908); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(F32(0x40490fda), F32(c.m.r32((l_w_2).wrapping_add(1908)))) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let a_ = (l_w_2).wrapping_add(1908); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Move", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_AddHit(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_AddHit", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_width_4: F32 = F32(0);
    let mut l_a_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let mut l_a_7: u32 = 0;
    let mut l_b_8: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    l_width_4 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(1932))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(100)));
    { let s_ = (l_w_2).wrapping_add(32); let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_w_2).wrapping_add(96)); let a_ = (l_rec_3).wrapping_add(100); c.m.w32(a_, v_); };
    { let r_ = 64i32; let a_ = (l_rec_3).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    's1: {
        let sv9: i32 = (({ let a_ = ({ let a_ = (c.m.r32((l_w_2).wrapping_add(96))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(8); (c.m.r8(a_) as i8) }) as i32);
        match sv9 {
            1i32 => {
                {
                    l_a_5 = F::EftHitArena_AllocBox(c);
                    l_b_6 = F::EftHitArena_AllocBox(c);
                    F::ColCapsule_Set(c, l_a_5, (((((l_w_2).wrapping_add(816)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16), ((l_w_2).wrapping_add(32)).wrapping_add(16), l_width_4);
                    F::ColCapsule_Set(c, l_b_6, (((((l_w_2).wrapping_add(816)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16), ((l_w_2).wrapping_add(32)).wrapping_add(32), l_width_4);
                    F::EftHit_SetShapeBoxes(c, l_rec_3, l_a_5, l_b_6);
                    break 's1;
                }
            }
            0i32 => {
                {
                    l_a_7 = F::EftHitArena_AllocSphere(c);
                    l_b_8 = F::EftHitArena_AllocSphere(c);
                    F::ColSphere_Set(c, l_a_7, ((l_w_2).wrapping_add(32)).wrapping_add(16), l_width_4);
                    F::ColSphere_Set(c, l_b_8, ((l_w_2).wrapping_add(32)).wrapping_add(32), l_width_4);
                    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_7, l_b_8);
                    break 's1;
                }
            }
            _ => {
                break 'f;
            }
        }
    }
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_AddHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_Emit(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_reset_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Emit", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_reset_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_type_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_gd_8: u32 = 0;
    let mut l_flags_9: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_type_6 = 0i32;
        'l1: loop {
            if !(((((l_type_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_type_6) as u32)) as u32)))) != 0) {
                    l_gd_8 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_type_6) as u32)).wrapping_mul(16)));
                    {
                        l_i_7 = 0i32;
                        'l3: loop {
                            if !(((((l_i_7) < (((c.m.r8((l_gd_8).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((p_reset_4) == (0i32)) as i32)) != 0) {
                                    l_flags_9 = { let a5_ = (((c.m.r32((l_w_5).wrapping_add(1916)) as i32)) & (1i32)); let a6_ = (((c.m.r32((l_w_5).wrapping_add(1916)) as i32)) & (4i32)); F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_5).wrapping_add(100), p_objId_1, l_type_6, l_i_7, a5_, a6_) };
                                } else {
                                    l_flags_9 = F::EftEmit_GetResetFlags(c, p_set_3, (l_w_5).wrapping_add(100), l_type_6, l_i_7);
                                }
                                if (((((c.m.r32((l_w_5).wrapping_add(1916)) as i32)) & (128i32))) != 0) {
                                    l_flags_9 = 2i32;
                                }
                                if ((l_flags_9) != 0) {
                                    { let a8_ = F32(c.m.r32((l_w_5).wrapping_add(1928))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_5).wrapping_add(100), (l_w_5).wrapping_add(816), ((l_w_5).wrapping_add(32)).wrapping_add(16), (l_w_5).wrapping_add(16), l_type_6, l_i_7, l_flags_9, a8_) };
                                }
                            }
                            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_type_6; let n_ = (o_).wrapping_add(1); l_type_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_Init(c: &mut Ctx, mut p_task_1: u32, mut p_owner_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Init", &PROBE_N, &[(p_task_1) as u32, (p_owner_2) as u32], false); }
    let mut l_group_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_set_5: u32 = 0;
    let mut l_p_6: u32 = 0;
    let r_ = 'f: {
    l_group_3 = F::BtlTask_GetParent(c, p_task_1);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_set_5 = (c.m.r32((l_group_3).wrapping_add(56))).wrapping_add(4);
    { let _ = F::memset(c, l_w_4, 0i32, 1952u32); };
    { let v_ = p_owner_2; let a_ = (l_w_4).wrapping_add(96); c.m.w32(a_, v_); };
    l_p_6 = c.m.r32((p_owner_2).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(52))); let a_ = (l_w_4).wrapping_add(1920); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1924); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1928); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(1924))); let a_ = (l_w_4).wrapping_add(1932); c.m.w32(a_, (v_).0); };
    { let v_ = l_set_5; let a_ = (l_w_4).wrapping_add(1408); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, l_set_5, (l_w_4).wrapping_add(100));
    if (({ let a0_ = c.m.r32((l_w_4).wrapping_add(1408)); F::EftEmit_HasWidth2(c, a0_) }) != 0) {
        { let v_ = F::EftEmit_GetWidth2(c, (l_w_4).wrapping_add(100)); let a_ = (l_w_4).wrapping_add(1920); c.m.w32(a_, (v_).0); };
        { let r_ = 256i32; let a_ = (l_w_4).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(1408)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(1944); c.m.w32(a_, (v_).0); };
    F::EftSweep_InitPath(c, p_task_1);
    { let a1_ = (if ((((((c.m.r32(p_owner_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_owner_3 = c.m.r32((l_w_2).wrapping_add(96));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(1408)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(100)) };
    if ((((((({ let a_ = (c.m.r32((l_owner_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let mut l_next_4: i32 = 0;
    let mut l_any_5: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_owner_3 = c.m.r32((l_w_2).wrapping_add(96));
        l_next_4 = 0i32;
        if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        F::EftEmit_UpdateNodesReq(c, l_owner_3, (l_w_2).wrapping_add(816));
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
                { let a2_ = (c.m.r32(l_owner_3) as i32); { let _ = F::EftAim_GetDir(c, (l_w_2).wrapping_add(16), (((((((((l_w_2).wrapping_add(816)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16)) as u64)) as i32), a2_); } };
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                    { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (32i32))) != 0) {
            's1: {
                let sv6: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
                match sv6 {
                    0i32 => {
                        if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                            { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            F::Vec4_Copy(c, l_w_2, (((((l_w_2).wrapping_add(816)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16));
                            F::Vec4_Copy(c, (l_w_2).wrapping_add(32), (((((l_w_2).wrapping_add(816)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16));
                            F::Vec4_Copy(c, ((l_w_2).wrapping_add(32)).wrapping_add(16), (l_w_2).wrapping_add(32));
                            { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, ((l_w_2).wrapping_add(32)).wrapping_add(32)) };
                            { let a3_ = (c.m.r32(l_owner_3) as i32); { let _ = F::EftAim_GetDirKeep(c, l_owner_3, (l_w_2).wrapping_add(16), ((((((l_w_2).wrapping_add(32)).wrapping_add(16)) as u64)) as i32), a3_); } };
                            { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(1920))); F::Vec3_Scale(c, ((l_w_2).wrapping_add(32)).wrapping_add(48), (l_w_2).wrapping_add(16), a2_) };
                            { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        }
                        break 's1;
                    }
                    2i32 => {
                        F::Vec4_Copy(c, ((l_w_2).wrapping_add(32)).wrapping_add(32), ((l_w_2).wrapping_add(32)).wrapping_add(16));
                        F::Vec4_Copy(c, l_w_2, (((((l_w_2).wrapping_add(816)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16));
                        F::EftSweep_Move(c, p_task_1);
                        F::ColSeg_Set(c, fp, l_w_2, ((l_w_2).wrapping_add(32)).wrapping_add(16));
                        if ((F::StgCol_TraceSegment(c, fp)) != 0) {
                            F::EftSweep_AddMark(c, p_task_1);
                        }
                        if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                            { let r_ = ((8i32) | (1i32)); let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        }
                        break 's1;
                    }
                    1i32 | 3i32 => {
                        l_next_4 = 1i32;
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let r_ = 1i32; let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a0_ = (c.m.r32(l_owner_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(1408)); F::EftSweep_Emit(c, a0_, p_task_1, a2_, 0i32) };
        if ((l_next_4) != 0) {
            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        l_any_5 = F::EftSweep_UpdateMarks(c, p_task_1);
        if (((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (1i32))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1940); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((((((!((l_any_5) != 0)) as i32)) != 0) && (((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
        if (((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (1i32))) != 0) {
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                if ((((!(((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(1940))), F32(c.m.r32((l_w_2).wrapping_add(1944)))) as i32)) != 0)) as i32)) != 0) {
                    break 'f;
                }
            }
            { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if (((({ let a_ = (c.m.r32((l_owner_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (16i32))) != 0) {
                    { let a0_ = c.m.r32((l_w_2).wrapping_add(1408)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_2).wrapping_add(100)) };
                    F::EftSweep_AddHit(c, p_task_1);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(({ let a0_ = { let a_ = c.m.r32((l_w_2).wrapping_add(96)); (c.m.r32(a_) as i32) }; F::BtlScene_IsCharStopped(c, a0_) }) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(1408)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(100)) }; };
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (1i32))) != 0) {
            { let r_ = (!(16i32)); let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (2i32))) != 0) {
                { let r_ = (!(16i32)); let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1916)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        { let r_ = 128i32; let a_ = (l_w_2).wrapping_add(1916); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1408)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(100)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweep_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweep_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweepGroup_Init(c: &mut Ctx, mut p_task_1: u32, mut p_owner_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweepGroup_Init", &PROBE_N, &[(p_task_1) as u32, (p_owner_2) as u32], false); }
    let mut l_gw_3: u32 = 0;
    let r_ = 'f: {
    l_gw_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = ((804u32) as i32); F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_gw_3, 0i32, 804u32); };
    { let a3_ = c.m.r32((p_owner_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_owner_2, (l_gw_3).wrapping_add(4), ((((0i32) as i64 as u64)) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((1952u32) as i32)) as i64 as u64)) as u32)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweepGroup_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweepGroup_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweepGroup_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweepGroup_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweepGroup_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweepGroup_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); F::EftEmit_BeginFrame(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweepGroup_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftSweepGroup_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSweepGroup_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSweepGroup_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollow_AddHit(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_AddHit", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_width_4: F32 = F32(0);
    let mut l_a_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    l_width_4 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(16))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(132)));
    { let s_ = (l_w_2).wrapping_add(64); let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_w_2).wrapping_add(128)); let a_ = (l_rec_3).wrapping_add(100); c.m.w32(a_, v_); };
    { let r_ = 64i32; let a_ = (l_rec_3).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    l_a_5 = F::EftHitArena_AllocSphere(c);
    l_b_6 = F::EftHitArena_AllocSphere(c);
    F::ColSphere_Set(c, l_a_5, ((l_w_2).wrapping_add(64)).wrapping_add(16), l_width_4);
    F::ColSphere_Set(c, l_b_6, ((l_w_2).wrapping_add(64)).wrapping_add(32), l_width_4);
    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_5, l_b_6);
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_AddHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollow_UpdateLight(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_UpdateLight", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let mut l_off_4: i32 = 0;
    let mut l_a0_5: i32 = 0;
    let mut l_a1_6: i32 = 0;
    let mut l_a2_7: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_owner_3 = c.m.r32((l_w_2).wrapping_add(128));
        if ((((((({ let a_ = (c.m.r32((l_owner_3).wrapping_add(36))).wrapping_add(72); (c.m.r16(a_) as i16) }) as i32)) != (2i32)) as i32)) != 0) {
            rt::memset(c, fp.wrapping_add(16), 0, 24);
            { let v_ = 2i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
            { let v_ = 8i32; c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_) as u32); }
            { let v_ = 16i32; c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_) as u32); }
            { let v_ = 32i32; c.m.w32(fp.wrapping_add(16).wrapping_add(16), (v_) as u32); }
            { let v_ = 64i32; c.m.w32(fp.wrapping_add(16).wrapping_add(20), (v_) as u32); }
            l_off_4 = { let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32((l_owner_3).wrapping_add(36))).wrapping_add(74); (c.m.r16(a_) as i16) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            if (({ let a0_ = (c.m.r32(l_owner_3) as i32); let a1_ = { let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32((l_owner_3).wrapping_add(36))).wrapping_add(72); (c.m.r16(a_) as i16) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::EftShot_TestBits(c, a0_, a1_) }) != 0) {
                { let r_ = 1024i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let a2_ = (c.m.r32(l_owner_3) as i32); { let _ = F::EftAim_GetDir(c, (l_w_2).wrapping_add(48), ((((fp) as u64)) as i32), a2_); } };
            }
            if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, l_off_4) }) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let r_ = (!(1024i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((c.m.r32(l_w_2) as i32)) & (1024i32))) != 0) {
                { let r_ = F32(0x3dcccccc); let a_ = (l_w_2).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(0x3f800000), F32(c.m.r32((l_w_2).wrapping_add(32)))) as i32)) != 0) {
                    { let v_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                }
            }
            l_a0_5 = rt::f2i(rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(32))), F32(0x42000000)));
            l_a1_6 = rt::f2i(rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(32))), F32(0x42800000)));
            l_a2_7 = rt::f2i(rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(32))), F32(0x43000000)));
            F::StgBlur_SetColor0Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((l_a0_5) & (255i32))) as i64 as u64)) as u8));
            F::StgBlur_SetColor1Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((l_a1_6) & (255i32))) as i64 as u64)) as u8));
            F::StgBlur_SetColor2Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((l_a2_7) & (255i32))) as i64 as u64)) as u8));
            F::StgBlur_SetColor3Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((64i32) as i64 as u64)) as u8));
            F::StgBlur_SetCenter(c, 0i32, (l_w_2).wrapping_add(48), 0i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_UpdateLight", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollow_Emit(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_reset_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Emit", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_reset_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_type_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_gd_8: u32 = 0;
    let mut l_flags_9: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_type_6 = 0i32;
        'l1: loop {
            if !(((((l_type_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_type_6) as u32)) as u32)))) != 0) {
                    l_gd_8 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_type_6) as u32)).wrapping_mul(16)));
                    {
                        l_i_7 = 0i32;
                        'l3: loop {
                            if !(((((l_i_7) < (((c.m.r8((l_gd_8).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((p_reset_4) == (0i32)) as i32)) != 0) {
                                    l_flags_9 = { let a5_ = (((c.m.r32(l_w_5) as i32)) & (1i32)); let a6_ = (((c.m.r32(l_w_5) as i32)) & (4i32)); F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_5).wrapping_add(132), p_objId_1, l_type_6, l_i_7, a5_, a6_) };
                                } else {
                                    l_flags_9 = F::EftEmit_GetResetFlags(c, p_set_3, (l_w_5).wrapping_add(132), l_type_6, l_i_7);
                                }
                                if (((((c.m.r32(l_w_5) as i32)) & (128i32))) != 0) {
                                    l_flags_9 = 2i32;
                                }
                                if ((l_flags_9) != 0) {
                                    { let a8_ = F32(c.m.r32((l_w_5).wrapping_add(12))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_5).wrapping_add(132), (l_w_5).wrapping_add(848), ((l_w_5).wrapping_add(64)).wrapping_add(16), (l_w_5).wrapping_add(48), l_type_6, l_i_7, l_flags_9, a8_) };
                                }
                            }
                            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_type_6; let n_ = (o_).wrapping_add(1); l_type_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollow_Init(c: &mut Ctx, mut p_task_1: u32, mut p_owner_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Init", &PROBE_N, &[(p_task_1) as u32, (p_owner_2) as u32], false); }
    let mut l_group_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_set_5: u32 = 0;
    let mut l_p_6: u32 = 0;
    let r_ = 'f: {
    l_group_3 = F::BtlTask_GetParent(c, p_task_1);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_set_5 = (c.m.r32((l_group_3).wrapping_add(56))).wrapping_add(4);
    { let _ = F::memset(c, l_w_4, 0i32, 1456u32); };
    { let v_ = p_owner_2; let a_ = (l_w_4).wrapping_add(128); c.m.w32(a_, v_); };
    l_p_6 = c.m.r32((p_owner_2).wrapping_add(36));
    { let v_ = F32(0x00000000); let a_ = (l_w_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(8))); let a_ = (l_w_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = l_set_5; let a_ = (l_w_4).wrapping_add(1440); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, l_set_5, (l_w_4).wrapping_add(132));
    if (({ let a0_ = c.m.r32((l_w_4).wrapping_add(1440)); F::EftEmit_HasWidth2(c, a0_) }) != 0) {
        { let v_ = F::EftEmit_GetWidth2(c, (l_w_4).wrapping_add(132)); let a_ = (l_w_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let r_ = 512i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(1440)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let a1_ = (if ((((((c.m.r32(p_owner_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollow_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_owner_3 = c.m.r32((l_w_2).wrapping_add(128));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(132)) };
    { let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollow_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let mut l_next_4: i32 = 0;
    let mut l_flags_5: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_owner_3 = c.m.r32((l_w_2).wrapping_add(128));
        l_next_4 = 0i32;
        if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        F::EftEmit_UpdateNodesReq(c, l_owner_3, (l_w_2).wrapping_add(848));
        if ((((!(((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            if (((((({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) || (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0)) as i32)) != 0) {
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if (((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0) {
            's1: {
                let sv6: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
                match sv6 {
                    0i32 => {
                        if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                            { let r_ = 16i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, (l_w_2).wrapping_add(64)) };
                            F::Vec4_Copy(c, ((l_w_2).wrapping_add(64)).wrapping_add(16), (l_w_2).wrapping_add(64));
                            { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, ((l_w_2).wrapping_add(64)).wrapping_add(32)) };
                            { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        }
                        break 's1;
                    }
                    1i32 | 3i32 => {
                        l_next_4 = 1i32;
                        break 's1;
                    }
                    2i32 => {
                        if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                            { let r_ = ((8i32) | (1i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
            if ((((!(((((c.m.r32(l_w_2) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                { let r_ = 4i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 46i32, (l_w_2).wrapping_add(48)) };
        { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
        F::Vec3_Sub(c, (l_w_2).wrapping_add(48), fp, (l_w_2).wrapping_add(48));
        { let v_ = F32(0x3f800000); let a_ = ((l_w_2).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, (l_w_2).wrapping_add(48), (l_w_2).wrapping_add(48));
        F::Vec4_Copy(c, ((l_w_2).wrapping_add(64)).wrapping_add(32), ((l_w_2).wrapping_add(64)).wrapping_add(16));
        { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, ((l_w_2).wrapping_add(64)).wrapping_add(16)) };
        { let a0_ = (c.m.r32(l_owner_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftFollow_Emit(c, a0_, p_task_1, a2_, 0i32) };
        if (((({ let a_ = (c.m.r32((l_owner_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (512i32))) != 0) {
            F::EftFollow_UpdateLight(c, p_task_1);
        }
        if ((l_next_4) != 0) {
            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        l_flags_5 = (c.m.r32(l_w_2) as i32);
        if ((((l_flags_5) & (1i32))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((l_flags_5) & (2i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
        if ((((l_flags_5) & (1i32))) != 0) {
            if ((((!((((l_flags_5) & (4i32))) != 0)) as i32)) != 0) {
                if ((((!(((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(24))), F32(c.m.r32((l_w_2).wrapping_add(28)))) as i32)) != 0)) as i32)) != 0) {
                    break 'f;
                }
            }
            { let v_ = ((l_flags_5) | (2i32)); let a_ = l_w_2; c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((l_flags_5) & (16i32))) != 0) {
                { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_2).wrapping_add(132)) };
                F::EftFollow_AddHit(c, p_task_1);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftFollow_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_state_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_state_3 = (l_w_2).wrapping_add(132);
    if ((((!(({ let a0_ = { let a_ = c.m.r32((l_w_2).wrapping_add(128)); (c.m.r32(a_) as i32) }; F::BtlScene_IsCharStopped(c, a0_) }) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_UpdateAlive(c, a0_, l_state_3) }; };
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (1i32))) != 0) {
            { let a0_ = c.m.r32((l_w_2).wrapping_add(1440)); F::EftEmit_MarkKind6(c, a0_, l_state_3) };
            { let r_ = (!(16i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (2i32))) != 0) {
                { let r_ = (!(16i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFollow_PostUpdate", &PROBE_N, &[], true); }
    r_
}
