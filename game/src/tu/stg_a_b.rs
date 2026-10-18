#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgPart_Animate(c: &mut Ctx, mut p_part_1: u32, mut p_frame_2: F32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPart_Animate", &PROBE_N, &[(p_part_1) as u32, (p_frame_2).0, (p_out_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_node_4: u32 = 0;
    let mut l_keys_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_t_7: F32 = F32(0);
    let r_ = 'f: {
        l_node_4 = c.m.r32((p_part_1).wrapping_add(8));
        l_keys_5 = c.m.r32((l_node_4).wrapping_add(68));
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if (((rt::fle(rt::i2f((c.m.r32((l_node_4).wrapping_add(72)) as i32)), p_frame_2) as i32)) != 0) {
            { let v_ = { let a_ = (l_keys_5).wrapping_add((((((c.m.r32((l_node_4).wrapping_add(64)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(32)); F32(c.m.r32(a_)) }; let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = { let a_ = ((l_keys_5).wrapping_add((((((c.m.r32((l_node_4).wrapping_add(64)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a_ = ((l_keys_5).wrapping_add((((((c.m.r32((l_node_4).wrapping_add(64)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(8); F32(c.m.r32(a_)) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = { let a2_ = rt::fdiv(rt::fmul({ let a_ = ((l_keys_5).wrapping_add((((((c.m.r32((l_node_4).wrapping_add(64)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = { let a2_ = rt::fdiv(rt::fmul({ let a_ = ((l_keys_5).wrapping_add((((((c.m.r32((l_node_4).wrapping_add(64)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a2_ = rt::fdiv(rt::fmul({ let a_ = ((l_keys_5).wrapping_add((((((c.m.r32((l_node_4).wrapping_add(64)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(24); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vu0Cur_LoadIdentity(c);
            { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); F::StgVu_RotateX(c, a0_) };
            { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::StgVu_RotateY(c, a0_) };
            { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::StgVu_RotateZ(c, a0_) };
            F::Vu0Cur_Translate(c, fp);
            F::Vu0Cur_StoreMtx(c, l_node_4);
            F::Vec4_Copy(c, p_out_3, (l_node_4).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
            break 'f;
        }
        {
            l_i_6 = 1i32;
            'l1: loop {
                if !(((((l_i_6) < ((c.m.r32((l_node_4).wrapping_add(64)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((rt::feq(rt::i2f((c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(12)) as i32)), p_frame_2) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32)))); let a_ = fp; c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(16))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(20))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(24))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vu0Cur_LoadIdentity(c);
                        { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); F::StgVu_RotateX(c, a0_) };
                        { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::StgVu_RotateY(c, a0_) };
                        { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::StgVu_RotateZ(c, a0_) };
                        F::Vu0Cur_Translate(c, fp);
                        F::Vu0Cur_StoreMtx(c, l_node_4);
                        F::Vec4_Copy(c, p_out_3, (l_node_4).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        break 'f;
                    }
                    if (((((((rt::flt(rt::i2f((c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(12)) as i32)), p_frame_2) as i32)) != 0) && (((rt::flt(p_frame_2, rt::i2f((c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(12)) as i32))) as i32)) != 0)) as i32)) != 0) {
                        l_t_7 = rt::fdiv(rt::fsub(p_frame_2, rt::i2f((c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(12)) as i32))), rt::fsub(rt::i2f((c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(12)) as i32)), rt::i2f((c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(12)) as i32))));
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(rt::fadd(rt::fmul(rt::fsub(rt::fadd(F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(16))), F32(0x45610000)), rt::fadd(F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(16))), F32(0x45610000))), l_t_7), F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(16)))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(rt::fadd(rt::fmul(rt::fsub(rt::fadd(F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(20))), F32(0x45610000)), rt::fadd(F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(20))), F32(0x45610000))), l_t_7), F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(20)))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(rt::fadd(rt::fmul(rt::fsub(rt::fadd(F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(24))), F32(0x45610000)), rt::fadd(F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(24))), F32(0x45610000))), l_t_7), F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(24)))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32)))), F32(c.m.r32((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))))), l_t_7), F32(c.m.r32((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))))); let a_ = fp; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(4))), F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(4)))), l_t_7), F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(8))), F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(8)))), l_t_7), F32(c.m.r32(((l_keys_5).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(32))).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vu0Cur_LoadIdentity(c);
                        { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); F::StgVu_RotateX(c, a0_) };
                        { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::StgVu_RotateY(c, a0_) };
                        { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::StgVu_RotateZ(c, a0_) };
                        F::Vu0Cur_Translate(c, fp);
                        F::Vu0Cur_StoreMtx(c, l_node_4);
                        F::Vec4_Copy(c, p_out_3, (l_node_4).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        break 'f;
                    }
                    if (((rt::feq(p_frame_2, rt::i2f((c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(12)) as i32))) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32)))); let a_ = fp; c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(16))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(20))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a2_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_keys_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(32))).wrapping_add(24))), F32(0x40490fda)), F32(0x43340000)); F::Stg_WrapRange(c, F32(0xc0490fda), F32(0x40490fda), a2_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vu0Cur_LoadIdentity(c);
                        { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); F::StgVu_RotateX(c, a0_) };
                        { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::StgVu_RotateY(c, a0_) };
                        { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::StgVu_RotateZ(c, a0_) };
                        F::Vu0Cur_Translate(c, fp);
                        F::Vu0Cur_StoreMtx(c, l_node_4);
                        F::Vec4_Copy(c, p_out_3, (l_node_4).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        break 'f;
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgPart_Animate", &PROBE_N, &[], true); }
    r_
}

pub fn StgObj_GetDust(c: &mut Ctx, mut p_obj_1: u32, mut p_scale_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgObj_GetDust", &PROBE_N, &[(p_obj_1) as u32, (p_scale_2) as u32], false); }
    let mut l_type_3: i32 = 0;
    let mut l_small_4: i32 = 0;
    let r_ = 'f: {
    l_type_3 = (c.m.r32((p_obj_1).wrapping_add(52)) as i32);
    l_small_4 = ((l_type_3) & (512i32));
    { let r_ = ((((1i32) | (2i32))) | (4i32)); let n_ = ((l_type_3) & (r_)); l_type_3 = n_; };
    if (((((l_type_3) == (0i32)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if ((l_small_4) != 0) {
        { let v_ = F32(0x3f333333); let a_ = p_scale_2; c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = F32(0x3f800000); let a_ = p_scale_2; c.m.w32(a_, (v_).0); };
    }
    if (((((c.m.r32((p_obj_1).wrapping_add(52)) as i32)) & (1i32))) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_obj_1).wrapping_add(52)) as i32)) & (2i32))) != 0) {
        break 'f 1i32;
    }
    if (((((c.m.r32((p_obj_1).wrapping_add(52)) as i32)) & (4i32))) != 0) {
        break 'f 2i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgObj_GetDust", &PROBE_N, &[], true); }
    r_
}

pub fn StgObj_GetMaterial(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgObj_GetMaterial", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_obj_1).wrapping_add(52)) as i32)) & (1i32))) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_obj_1).wrapping_add(52)) as i32)) & (2i32))) != 0) {
        break 'f 1i32;
    }
    if (((((c.m.r32((p_obj_1).wrapping_add(52)) as i32)) & (4i32))) != 0) {
        break 'f 2i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgObj_GetMaterial", &PROBE_N, &[], true); }
    r_
}

pub fn StgObj_GetItemPos(c: &mut Ctx, mut p_out_1: u32, mut p_obj_2: u32, mut p_idx_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgObj_GetItemPos", &PROBE_N, &[(p_out_1) as u32, (p_obj_2) as u32, (p_idx_3) as u32], false); }
    let r_ = 'f: {
    if ((((!(((((c.m.r32((p_obj_2).wrapping_add(52)) as i32)) & (536870912i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let a1_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) }).wrapping_add(32); F::Vec4_Copy(c, p_out_1, a1_) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgObj_GetItemPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_BreakObj(c: &mut Ctx, mut p_idx_1: i32, mut p_hitPos_2: u32, mut p_info_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BreakObj", &PROBE_N, &[(p_idx_1) as u32, (p_hitPos_2) as u32, (p_info_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_objs_4: u32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_part_6: u32 = 0;
    let mut l_p_7: u32 = 0;
    let mut l_q_8: u32 = 0;
    let mut l_i_9: u32 = 0;
    let mut l_shake_10: i32 = 0;
    let mut l_time_11: F32 = F32(0);
    let mut l_power_12: F32 = F32(0);
    let mut l_rumbleTime_13: F32 = F32(0);
    let mut l_rumblePower_14: F32 = F32(0);
    let mut l_near_15: F32 = F32(0);
    let mut l_far_16: F32 = F32(0);
    let mut l_rumbleFar_17: F32 = F32(0);
    let mut l_r_18: F32 = F32(0);
    let r_ = 'f: {
        l_objs_4 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) };
        l_obj_5 = ((((p_idx_1).wrapping_mul(((80u32) as i32))).wrapping_add(((l_objs_4) as i32))) as u32);
        l_part_6 = ((0i32) as u32);
        if (((((c.m.r32(l_obj_5) as i32)) & (1i32))) != 0) {
            break 'f 0i32;
        }
        l_p_7 = c.m.r32(((l_obj_5).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)));
        if (((((l_p_7) != (((0i32) as u32))) as i32)) != 0) {
            l_q_8 = l_p_7;
            { let r_ = (!(1i32)); let a_ = (l_q_8).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
            l_part_6 = l_q_8;
        }
        l_p_7 = c.m.r32(((l_obj_5).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)));
        if (((((l_p_7) != (((0i32) as u32))) as i32)) != 0) {
            l_q_8 = l_p_7;
            { let r_ = (!(1i32)); let a_ = (l_q_8).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
        }
        l_p_7 = c.m.r32(((l_obj_5).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4)));
        if (((((l_p_7) != (((0i32) as u32))) as i32)) != 0) {
            l_q_8 = l_p_7;
            { let r_ = (!(1i32)); let a_ = (l_q_8).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
        }
        l_p_7 = c.m.r32(((l_obj_5).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)));
        if (((((l_p_7) != (((0i32) as u32))) as i32)) != 0) {
            l_q_8 = l_p_7;
            { let r_ = 1i32; let a_ = (l_q_8).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
        }
        l_p_7 = c.m.r32(((l_obj_5).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)));
        if (((((l_p_7) != (((0i32) as u32))) as i32)) != 0) {
            l_q_8 = l_p_7;
            { let r_ = 1i32; let a_ = (l_q_8).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
        }
        l_p_7 = c.m.r32(((l_obj_5).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4)));
        if (((((l_p_7) != (((0i32) as u32))) as i32)) != 0) {
            l_q_8 = l_p_7;
            { let r_ = 1i32; let a_ = (l_q_8).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
        }
        { let v_ = 0i32; let a_ = (l_obj_5).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((1i32) | (2i32)); let a_ = l_obj_5; c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_obj_5).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        F::Vec4_Copy(c, (l_obj_5).wrapping_add(64), p_hitPos_2);
        if (((((l_part_6) != (((0i32) as u32))) as i32)) != 0) {
            l_shake_10 = 0i32;
            l_time_11 = F32(0x00000000);
            l_power_12 = F32(0x00000000);
            l_rumbleTime_13 = F32(0x00000000);
            l_rumblePower_14 = F32(0x00000000);
            l_near_15 = F32(0x42c80000);
            l_far_16 = F32(0x44bb8000);
            l_rumbleFar_17 = F32(0x447a0000);
            { let r_ = (c.m.r32((l_obj_5).wrapping_add(52)) as i32); let a_ = (p_info_3).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = F32(c.m.r32((l_part_6).wrapping_add(12))); let a_ = (p_info_3).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let a1_ = F32(c.m.r32((l_part_6).wrapping_add(16))); let a2_ = F32(c.m.r32((l_part_6).wrapping_add(20))); let a3_ = F32(c.m.r32((l_part_6).wrapping_add(24))); F::Vec4_Set(c, p_info_3, a1_, a2_, a3_, F32(0x3f800000)) };
            { let a_ = (p_info_3).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if (((((c.m.r32((l_obj_5).wrapping_add(52)) as i32)) & (((((16i32) | (32i32))) | (64i32))))) != 0) {
                { let a1_ = F32(c.m.r32((l_part_6).wrapping_add(16))); let a2_ = F32(c.m.r32((l_part_6).wrapping_add(20))); let a3_ = F32(c.m.r32((l_part_6).wrapping_add(24))); F::Vec4_Set(c, fp, a1_, a2_, a3_, F32(0x3f800000)) };
                l_r_18 = F32(c.m.r32((l_part_6).wrapping_add(12)));
                if (((((c.m.r32((l_obj_5).wrapping_add(52)) as i32)) & (64i32))) != 0) {
                    l_power_12 = F32(0x40400000);
                    l_rumblePower_14 = F32(0x3f800000);
                    l_time_11 = F32(0x3f333333);
                    l_shake_10 = 1i32;
                    l_rumbleTime_13 = F32(0x3e999999);
                    l_near_15 = F32(0x43960000);
                    l_far_16 = F32(0x44bb8000);
                    l_rumbleFar_17 = F32(0x447a0000);
                }
                if (((((c.m.r32((l_obj_5).wrapping_add(52)) as i32)) & (32i32))) != 0) {
                    l_power_12 = F32(0x40000000);
                    l_shake_10 = 1i32;
                    l_time_11 = F32(0x3f000000);
                    l_rumblePower_14 = F32(0x3f4ccccc);
                    l_rumbleTime_13 = F32(0x3e4ccccc);
                    l_near_15 = F32(0x42c80000);
                    l_far_16 = F32(0x44bb8000);
                    l_rumbleFar_17 = F32(0x447a0000);
                }
                if (((((c.m.r32((l_obj_5).wrapping_add(52)) as i32)) & (16i32))) != 0) {
                    l_time_11 = F32(0x3e999999);
                    l_power_12 = F32(0x3f800000);
                    l_rumblePower_14 = F32(0x3f000000);
                    l_shake_10 = 1i32;
                    l_rumbleTime_13 = F32(0x3dcccccc);
                    l_near_15 = F32(0x42c80000);
                    l_far_16 = F32(0x44bb8000);
                    l_rumbleFar_17 = F32(0x447a0000);
                }
                if ((l_shake_10) != 0) {
                    { let r_ = l_r_18; let n_ = rt::fadd(l_near_15, r_); l_near_15 = n_; };
                    { let r_ = l_r_18; let n_ = rt::fadd(l_far_16, r_); l_far_16 = n_; };
                    { let r_ = l_r_18; let n_ = rt::fadd(l_rumbleFar_17, r_); l_rumbleFar_17 = n_; };
                    F::BtlCharApi_ShakeCamsNear(c, fp, l_near_15, l_far_16, l_power_12, l_time_11);
                    F::BtlCharApi_RumbleNear(c, fp, l_near_15, l_rumbleFar_17, l_rumblePower_14, l_rumbleTime_13);
                }
            }
        }
        {
            l_i_9 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_9) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(68); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32(((l_objs_4).wrapping_add((l_i_9).wrapping_mul(80))).wrapping_add(56)) as i32)) == (p_idx_1)) as i32)) != 0) {
                        { let _ = F::BtlStage_BreakObj(c, ((l_i_9) as i32), p_hitPos_2, p_info_3); };
                    }
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BreakObj", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_ResetObjs(c: &mut Ctx, mut p_broken_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_ResetObjs", &PROBE_N, &[(p_broken_1) as u32], false); }
    let mut l_objs_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_q_5: u32 = 0;
    let mut l_i_6: u32 = 0;
    let r_ = 'f: {
    l_objs_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) };
    if (((((p_broken_1) == (1i32)) as i32)) != 0) {
        l_obj_3 = l_objs_2;
        {
            l_i_6 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_6) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(68); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = (!(1i32)); let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = (!(1i32)); let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = (!(1i32)); let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = 1i32; let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = 1i32; let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = 1i32; let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    { let v_ = p_broken_1; let a_ = l_obj_3; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_obj_3).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_obj_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                }
                { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_obj_3; let n_ = (o_).wrapping_add(80); l_obj_3 = n_; }; };
            }
        }
    } else {
        {
            l_i_6 = ((0i32) as u32);
            'l3: loop {
                if !(((((l_i_6) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(68); c.m.r32(a_) })) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_obj_3 = (l_objs_2).wrapping_add((l_i_6).wrapping_mul(80));
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = 1i32; let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = 1i32; let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = 1i32; let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = (!(1i32)); let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = (!(1i32)); let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    l_p_4 = c.m.r32(((l_obj_3).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4)));
                    if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
                        l_q_5 = l_p_4;
                        { let r_ = (!(1i32)); let a_ = (l_q_5).wrapping_add(2); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
                    }
                    { let v_ = p_broken_1; let a_ = l_obj_3; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_obj_3).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                    F::StgObj_LoadDef(c, l_obj_3, ((l_i_6) as i32));
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_ResetObjs", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_DestroyObj(c: &mut Ctx, mut p_objId_1: i32, mut p_idx_2: i32, mut p_hitPos_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_DestroyObj", &PROBE_N, &[(p_objId_1) as u32, (p_idx_2) as u32, (p_hitPos_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_broke_4: i32 = 0;
    let mut l_objs_5: u32 = 0;
    let mut l_rule_6: u32 = 0;
    let mut l_result_7: u32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 32u32); };
        F::StgNav_UnblockObj(c, p_idx_2);
        l_broke_4 = F::BtlStage_BreakObj(c, p_idx_2, p_hitPos_3, fp);
        if ((((((c.m.r32((fp).wrapping_add(24)) as i32)) != (0i32)) as i32)) != 0) {
            { let a3_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(16))), F32(0x43480000)); let a4_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(16))), F32(0x44bb8000)); F::BtlCharApi_PlaySoundAt(c, fp, 1i32, 8i32, a3_, a4_) };
        }
        if (((((l_broke_4) != (0i32)) as i32)) != 0) {
            if (((((p_objId_1) >= (0i32)) as i32)) != 0) {
                l_objs_5 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) };
                l_rule_6 = (F::Battle_GetSetup(c)).wrapping_add(8);
                l_result_7 = F::BattleResult_GetPtr(c);
                if ((((!((F::BtlCharApi_IsInputInjected(c, p_objId_1)) != 0)) as i32)) != 0) {
                    if ((((((c.m.r32((l_rule_6).wrapping_add(24)) as i32)) != (0i32)) as i32)) != 0) {
                        if ((F::StgObj_GetItemPos(c, fp.wrapping_add(32), (l_objs_5).wrapping_add((((p_idx_2) as u32)).wrapping_mul(80)), p_idx_2)) != 0) {
                            if ((((((((F::Port_Rand(c)).wrapping_shr((8i32) as u32)) & (7i32))) == (0i32)) as i32)) != 0) {
                                { let v_ = (c.m.r32((l_rule_6).wrapping_add(24)) as i32); let a_ = (l_result_7).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (l_rule_6).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                F::StreamSe_PlayDefault(c, 0i32, 36154i32);
                                F::HudTimer_ShowMark(c);
                            }
                        }
                    }
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_DestroyObj", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_IsObjBroken(c: &mut Ctx, mut p_idx_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_IsObjBroken", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) }).wrapping_add((((p_idx_1) as u32)).wrapping_mul(80)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_IsObjBroken", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_DamageObj(c: &mut Ctx, mut p_idx_1: i32, mut p_damage_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_DamageObj", &PROBE_N, &[(p_idx_1) as u32, (p_damage_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_obj_3 = ((((p_idx_1).wrapping_mul(((80u32) as i32))).wrapping_add((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) }) as i32))) as u32);
    if (((((c.m.r32(l_obj_3) as i32)) & (1i32))) != 0) {
        break 'f 0i32;
    }
    { let r_ = p_damage_2; let a_ = (l_obj_3).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    break 'f ((((c.m.r32((l_obj_3).wrapping_add(8)) as i32)) <= (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_DamageObj", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetObjType(c: &mut Ctx, mut p_idx_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetObjType", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((((p_idx_1).wrapping_mul(((80u32) as i32))).wrapping_add((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) }) as i32))) as u32);
    if (((((c.m.r32(l_obj_2) as i32)) & (1i32))) != 0) {
        break 'f 0i32;
    }
    break 'f (c.m.r32((l_obj_2).wrapping_add(52)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetObjType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_UpdateObjs(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_UpdateObjs", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_objs_1: u32 = 0;
    let mut l_part_2: u32 = 0;
    let mut l_node_3: u32 = 0;
    let mut l_i_4: u32 = 0;
    let mut l_j_5: u32 = 0;
    let mut l_first_6: i32 = 0;
    let r_ = 'f: {
        l_objs_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) };
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            break 'f;
        }
        {
            l_i_4 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_4) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(68); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((!(((((c.m.r32((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((rt::feq(rt::i2f((c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(48)) as i32)), F32(0x00000000)) as i32)) != 0) {
                        l_first_6 = 1i32;
                    } else {
                        l_first_6 = 0i32;
                    }
                    { let v_ = rt::f2i(rt::fadd(rt::i2f((c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(48)) as i32)), F32(0x40000000))); let a_ = ((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(44)) as i32)) == (0i32)) as i32)) != 0) {
                        { let v_ = 148i32; let a_ = ((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                    }
                    if (((rt::fle(rt::i2f((c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(44)) as i32)), rt::i2f((c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(48)) as i32))) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = ((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        { let r_ = 2i32; let a_ = (l_objs_1).wrapping_add((l_i_4).wrapping_mul(80)); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                        {
                            l_j_5 = ((0i32) as u32);
                            'l3: loop {
                                if !(((((l_j_5) < (c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(36)))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    l_part_2 = { let a_ = (c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(40))).wrapping_add((l_j_5).wrapping_mul(4)); c.m.r32(a_) };
                                    l_node_3 = c.m.r32((l_part_2).wrapping_add(8));
                                    if ((F::StgNode_IsRigid(c, l_node_3)) != 0) {
                                        if ((((((c.m.r32((l_node_3).wrapping_add(76)) as i32)) >= (0i32)) as i32)) != 0) {
                                            { let a0_ = (c.m.r32((l_node_3).wrapping_add(76)) as i32); F::StgRigid_Release(c, a0_) };
                                        }
                                    }
                                }
                                { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                            }
                        }
                        break 'c2;
                    }
                    if ((l_first_6) != 0) {
                        F::Vec4_Copy(c, fp.wrapping_add(32), sym::D_002EC2C0);
                        {
                            l_j_5 = ((0i32) as u32);
                            'l5: loop {
                                if !(((((l_j_5) < (c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(36)))) as i32)) != 0) { break 'l5; }
                                'c6: {
                                    l_part_2 = { let a_ = (c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(40))).wrapping_add((l_j_5).wrapping_mul(4)); c.m.r32(a_) };
                                    if (((((c.m.r32((l_part_2).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
                                        F::StgPart_Animate(c, l_part_2, F32(0x00000000), fp);
                                        F::Vec4_Add(c, fp.wrapping_add(32), fp.wrapping_add(32), fp);
                                    }
                                }
                                { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                            }
                        }
                        { let a2_ = rt::fdiv(F32(0x3f800000), rt::u2f(c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(36)))); F::Vec3_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), a2_) };
                    }
                    {
                        l_j_5 = ((0i32) as u32);
                        'l7: loop {
                            if !(((((l_j_5) < (c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(36)))) as i32)) != 0) { break 'l7; }
                            'c8: {
                                l_part_2 = { let a_ = (c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(40))).wrapping_add((l_j_5).wrapping_mul(4)); c.m.r32(a_) };
                                l_node_3 = c.m.r32((l_part_2).wrapping_add(8));
                                if (((((l_node_3) == (((0i32) as u32))) as i32)) != 0) {
                                    break 'c8;
                                }
                                if ((((!((l_first_6) != 0)) as i32)) != 0) {
                                    F::StgPart_GetPos(c, l_part_2, fp.wrapping_add(16));
                                }
                                if ((F::StgNode_IsRigid(c, l_node_3)) != 0) {
                                    if ((l_first_6) != 0) {
                                        F::StgPart_Animate(c, l_part_2, F32(0x00000000), fp);
                                        { let v_ = { let a1_ = F32(c.m.r32((l_part_2).wrapping_add(12))); F::StgRigid_Create(c, fp, a1_, 0i32) }; let a_ = (l_node_3).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                                        if ((((((c.m.r32((l_node_3).wrapping_add(76)) as i32)) >= (0i32)) as i32)) != 0) {
                                            { let a0_ = (c.m.r32((l_node_3).wrapping_add(76)) as i32); let a2_ = F::StgObj_GetMaterial(c, (l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))); F::StgRigid_Launch(c, a0_, ((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(64), a2_) };
                                        }
                                    }
                                    { let a1_ = (c.m.r32((l_node_3).wrapping_add(76)) as i32); { let _ = F::StgRigid_GetMatrix(c, l_node_3, a1_); } };
                                } else {
                                    { let v_ = 0i32; let a_ = (l_node_3).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                                    { let a1_ = rt::i2f((c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(48)) as i32)); F::StgPart_Animate(c, l_part_2, a1_, fp) };
                                }
                                if ((l_first_6) != 0) {
                                    's9: {
                                        let sv7: i32 = F::StgObj_GetDust(c, (l_objs_1).wrapping_add((l_i_4).wrapping_mul(80)), fp.wrapping_add(48));
                                        match sv7 {
                                            0i32 => {
                                                if (((((((l_j_5) & (((3i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) {
                                                    { let a3_ = F32(c.m.r32(fp.wrapping_add(48))); F::EftGndDust_SpawnDebris(c, fp, F32(0x3f800000), F32(0x3f800000), a3_) };
                                                }
                                                break 's9;
                                            }
                                            1i32 => {
                                                if (((((((l_j_5) & (((3i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) {
                                                    { let a3_ = F32(c.m.r32(fp.wrapping_add(48))); F::EftGndDust_SpawnDebris(c, fp, F32(0x3f800000), F32(0x3f800000), a3_) };
                                                }
                                                break 's9;
                                            }
                                            2i32 => {
                                                if (((((((l_j_5) & (((3i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) {
                                                    { let a3_ = F32(c.m.r32(fp.wrapping_add(48))); F::EftGndDust_SpawnDebris(c, fp, F32(0x3f800000), F32(0x3f800000), a3_) };
                                                }
                                                break 's9;
                                            }
                                            _ => {
                                            }
                                        }
                                    }
                                    if ((l_first_6) != 0) {
                                        break 'c8;
                                    }
                                }
                                F::StgPart_GetPos(c, l_part_2, fp);
                                if (((((c.m.r32(((l_objs_1).wrapping_add((l_i_4).wrapping_mul(80))).wrapping_add(52)) as i32)) & (128i32))) != 0) {
                                    if (((((((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((fp).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0) {
                                        { let _ = F::EftWater_AddSplashAt(c, fp.wrapping_add(16), F32(0x40400000)); };
                                    }
                                }
                            }
                            { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                        }
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_UpdateObjs", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_RequestChange(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_RequestChange", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = F::BtlStage_GetChangeTarget(c); F::BtlLoad_RequestStageChange(c, a0_) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_RequestChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_BindPaths(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BindPaths", &PROBE_N, &[], false); }
    let mut l_file_1: u32 = 0;
    let r_ = 'f: {
    l_file_1 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(32)).wrapping_add(4); c.m.r32(a_) };
    { let v_ = (l_file_1).wrapping_add((rt::div_u32(c.m.r32((l_file_1).wrapping_add((((24i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = sym::gStgPaths; c.m.w32(a_, v_); };
    { let a0_ = c.m.r32(sym::gStgPaths); F::StgPathTable_Relocate(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BindPaths", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_PathStub(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_PathStub", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_PathStub", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_ClearPaths(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_ClearPaths", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = sym::gStgPaths; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_ClearPaths", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetPathCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPathCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gStgPaths)) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gStgPaths)).wrapping_add(16); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32(sym::gStgPaths)).wrapping_add(8); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPathCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetPath(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPath", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let mut l_paths_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgPaths)) != (((0i32) as u32))) as i32)) != 0) {
        l_count_2 = { let a_ = (c.m.r32(sym::gStgPaths)).wrapping_add(8); (c.m.r32(a_) as i32) };
        l_paths_3 = { let a_ = (c.m.r32(sym::gStgPaths)).wrapping_add(12); c.m.r32(a_) };
        if (((((p_n_1) < (l_count_2)) as i32)) != 0) {
            break 'f (l_paths_3).wrapping_add((((p_n_1) as u32)).wrapping_mul(16));
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPath", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetPath0(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPath0", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::BtlStage_GetPath(c, 0i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPath0", &PROBE_N, &[], true); }
    r_
}

pub fn stg_a_b__Stg_Normalize(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "stg_a_b__Stg_Normalize", &PROBE_N, &[(p_v_1) as u32], false); }
    let mut l_len_2: F32 = F32(0);
    let r_ = 'f: {
    l_len_2 = { let a0_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_v_1)), F32(c.m.r32(p_v_1))), rt::fmul(F32(c.m.r32((p_v_1).wrapping_add(4))), F32(c.m.r32((p_v_1).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_v_1).wrapping_add(8))), F32(c.m.r32((p_v_1).wrapping_add(8))))); F::Port_sqrtf(c, a0_) };
    if (((rt::fgt(l_len_2, F32(0x00000000)) as i32)) != 0) {
        { let r_ = l_len_2; let a_ = p_v_1; let n_ = rt::fdiv(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_len_2; let a_ = (p_v_1).wrapping_add(4); let n_ = rt::fdiv(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_len_2; let a_ = (p_v_1).wrapping_add(8); let n_ = rt::fdiv(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    } else {
        { let v_ = F32(0x00000000); let a_ = (p_v_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = p_v_1; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_v_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "stg_a_b__Stg_Normalize", &PROBE_N, &[], true); }
    r_
}

pub fn Stg_CalcYaw(c: &mut Ctx, mut p_x0_1: F32, mut p_z0_2: F32, mut p_x1_3: F32, mut p_z1_4: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Stg_CalcYaw", &PROBE_N, &[(p_x0_1).0, (p_z0_2).0, (p_x1_3).0, (p_z1_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_a_5: F32 = F32(0);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = rt::fsub(p_x0_1, p_x1_3); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(p_z0_2, p_z1_4); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::stg_a_b__Stg_Normalize(c, fp);
        l_a_5 = rt::fsub({ let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(0x40490fda));
        if (((rt::fgt(l_a_5, F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let n_ = rt::fsub(l_a_5, r_); l_a_5 = n_; };
        }
        if (((rt::flt(l_a_5, F32(0xc0490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let n_ = rt::fadd(l_a_5, r_); l_a_5 = n_; };
        }
        break 'f l_a_5;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Stg_CalcYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_BindMef(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BindMef", &PROBE_N, &[], false); }
    let mut l_m_1: u32 = 0;
    let mut l_p_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_m_1 = c.m.r32(sym::gBtlStageFxRes);
    if (((((((((l_m_1) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32(l_m_1)) == (((809911629i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = (l_m_1).wrapping_add(((((c.m.r32((l_m_1).wrapping_add(12))) as i32)) as u32)); let a_ = (l_m_1).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = (l_m_1).wrapping_add((((({ let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(20); c.m.r32(a_) }) as i32)) as u32)); let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = (l_m_1).wrapping_add((((({ let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(28); c.m.r32(a_) }) as i32)) as u32)); let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(28); c.m.w32(a_, v_); };
        {
            { l_i_3 = 0i32; l_p_2 = { let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(12); c.m.r32(a_) }; };
            'l1: loop {
                if !(((((l_i_3) < ({ let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(8); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fneg(F32(c.m.r32((l_p_2).wrapping_add(4)))); let a_ = (l_p_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(F32(c.m.r32((l_p_2).wrapping_add(8)))); let a_ = (l_p_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_p_2; let n_ = (o_).wrapping_add(48); l_p_2 = n_; }; };
            }
        }
        {
            { l_i_3 = 0i32; l_p_2 = { let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(20); c.m.r32(a_) }; };
            'l3: loop {
                if !(((((l_i_3) < ({ let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(16); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = rt::fneg(F32(c.m.r32((l_p_2).wrapping_add(4)))); let a_ = (l_p_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(F32(c.m.r32((l_p_2).wrapping_add(8)))); let a_ = (l_p_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_p_2; let n_ = (o_).wrapping_add(48); l_p_2 = n_; }; };
            }
        }
        l_p_2 = { let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(28); c.m.r32(a_) };
        {
            l_i_3 = 0i32;
            'l5: loop {
                if !(((((l_i_3) < ({ let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(24); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = rt::fneg(F32(c.m.r32(((l_p_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(4)))); let a_ = ((l_p_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(F32(c.m.r32(((l_p_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(8)))); let a_ = ((l_p_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BindMef", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_BindData(c: &mut Ctx, mut p_stage_1: u32, mut p_base_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BindData", &PROBE_N, &[(p_stage_1) as u32, (p_base_2) as u32], false); }
    let mut l_d_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_base_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_base_2; let a_ = (p_stage_1).wrapping_add(4); c.m.w32(a_, v_); };
        l_d_3 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(12))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(20))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(140))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(140); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(28))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(28); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(36))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(36); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(44))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(44); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(52))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(52); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(60))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(60); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(68))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(68); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(76))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(76); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(84))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(84); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(148))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(148); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(164))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(164); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(156))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(156); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(100))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(100); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(108))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(108); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(116))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(116); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(92))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(92); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(124))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(124); c.m.w32(a_, v_); };
        { let v_ = (p_base_2).wrapping_add(((((c.m.r32((l_d_3).wrapping_add(132))) as i32)) as u32)); let a_ = (l_d_3).wrapping_add(132); c.m.w32(a_, v_); };
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(12))).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(12))).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(12))).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(12))).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(16)) as i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(20))).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(20))).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l5: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(24)) as i32))) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(28))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(28))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(28))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(28))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(28))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(28))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(28))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(28))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l7: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(32)) as i32))) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(36))).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(36))).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(36))).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(36))).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l9: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(40)) as i32))) as i32)) != 0) { break 'l9; }
                'c10: {
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(44))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(44))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(44))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(44))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(44))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(44))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(44))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(44))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l11: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(48)) as i32))) as i32)) != 0) { break 'l11; }
                'c12: {
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(52))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(52))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(52))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(52))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(52))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(52))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(52))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(52))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l13: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(56)) as i32))) as i32)) != 0) { break 'l13; }
                'c14: {
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(60))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(60))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(60))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(60))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(60))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(60))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = (((c.m.r32((l_d_3).wrapping_add(60))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = (((c.m.r32((l_d_3).wrapping_add(60))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l15: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(96)) as i32))) as i32)) != 0) { break 'l15; }
                'c16: {
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(100))).wrapping_add((((l_i_4) as u32)).wrapping_mul(24))).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(100))).wrapping_add((((l_i_4) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(100))).wrapping_add((((l_i_4) as u32)).wrapping_mul(24))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(100))).wrapping_add((((l_i_4) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l17: loop {
                if !(((((l_i_4) < ((c.m.r32((l_d_3).wrapping_add(88)) as i32))) as i32)) != 0) { break 'l17; }
                'c18: {
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(92))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(92))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a_ = ((c.m.r32((l_d_3).wrapping_add(92))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((c.m.r32((l_d_3).wrapping_add(92))).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BindData", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_BindFile(c: &mut Ctx, mut p_stage_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BindFile", &PROBE_N, &[(p_stage_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let mut l_file_3: u32 = 0;
    let mut l_data_4: u32 = 0;
    let mut l_ofs_5: u32 = 0;
    let r_ = 'f: {
    l_res_2 = (c.m.r32(sym::gCommonRes)).wrapping_add(32);
    if (((((c.m.r32((p_stage_1).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
        l_file_3 = c.m.r32((l_res_2).wrapping_add(4));
        { let v_ = ((0i32) as u32); let a_ = sym::gBtlStageFxRes; c.m.w32(a_, v_); };
        if (((((l_file_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_data_4 = (l_file_3).wrapping_add((rt::div_u32(c.m.r32((l_file_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
        l_ofs_5 = (rt::div_u32(c.m.r32((l_file_3).wrapping_add((((19i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32));
        if (((((c.m.r32((l_file_3).wrapping_add((((20i32) as u32)).wrapping_mul(4)))) != (c.m.r32((l_file_3).wrapping_add((((19i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
            { let v_ = (l_file_3).wrapping_add(l_ofs_5); let a_ = sym::gBtlStageFxRes; c.m.w32(a_, v_); };
        }
        F::BtlStage_BindData(c, p_stage_1, l_data_4);
        F::BtlStage_BindMef(c);
        F::BtlStage_BindPaths(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_BindFile", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetWaterLevel(c: &mut Ctx, mut p_level_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetWaterLevel", &PROBE_N, &[(p_level_1) as u32], false); }
    let mut l_d_2: u32 = 0;
    let mut l_env_3: u32 = 0;
    let r_ = 'f: {
    if ((F::BtlStage_IsReady(c)) != 0) {
        l_d_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
        if (((((l_d_2) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = F32(c.m.r32(sym::gStgWaterY)); let a_ = p_level_1; c.m.w32(a_, (v_).0); };
            break 'f (c.m.r32(sym::gStgHasWater) as i32);
        }
        l_env_3 = c.m.r32((l_d_2).wrapping_add(20));
        if (((((c.m.r32((l_env_3).wrapping_add(4)) as i32)) & (2i32))) != 0) {
            { let v_ = 1i32; let a_ = sym::gStgHasWater; c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(c.m.r32((l_env_3).wrapping_add(8))); let a_ = sym::gStgWaterY; c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = 0i32; let a_ = sym::gStgHasWater; c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x00000000); let a_ = sym::gStgWaterY; c.m.w32(a_, (v_).0); };
        }
    }
    { let v_ = F32(c.m.r32(sym::gStgWaterY)); let a_ = p_level_1; c.m.w32(a_, (v_).0); };
    break 'f (c.m.r32(sym::gStgHasWater) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetWaterLevel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_ProbeGround(c: &mut Ctx, mut p_zone_1: i32, mut p_pos_2: u32, mut p_out_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_ProbeGround", &PROBE_N, &[(p_zone_1) as u32, (p_pos_2) as u32, (p_out_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_e_4: u32 = 0;
    let mut l_r_5: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(12), (v_).0); }
        l_e_4 = fp.wrapping_add(32);
        if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
            break 'f p_zone_1;
        }
        { let _ = F::memset(c, p_out_3, 0i32, 48u32); };
        l_r_5 = F::BtlStage_FindZoneNear(c, p_zone_1, p_pos_2);
        { let v_ = F32(c.m.r32(p_pos_2)); let a_ = p_out_3; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_pos_2).wrapping_add(4))); let a_ = (p_out_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_pos_2).wrapping_add(8))); let a_ = (p_out_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::ColBox_SetCenterHalf(c, fp, p_pos_2, l_e_4);
        { let a3_ = F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::StgGround_Probe(c, l_r_5, fp, p_out_3, a3_) };
        break 'f l_r_5;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_ProbeGround", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetStartPlace(c: &mut Ctx, mut p_side_1: i32, mut p_pos_2: u32, mut p_rot_3: u32, mut p_alt_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetStartPlace", &PROBE_N, &[(p_side_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32, (p_alt_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_zone_5: i32 = 0;
    let mut l_d_6: u32 = 0;
    let mut l_starts_7: u32 = 0;
    let mut l_p_8: u32 = 0;
    let mut l_yaw_9: F32 = F32(0);
    let r_ = 'f: {
        { let _ = F::BtlStage_IsReady(c); };
        l_d_6 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
        l_p_8 = c.m.r32((l_d_6).wrapping_add(52));
        l_starts_7 = c.m.r32((l_d_6).wrapping_add(44));
        if (((((p_alt_4) == (1i32)) as i32)) != 0) {
            l_yaw_9 = { let a0_ = F32(c.m.r32(l_p_8)); let a1_ = F32(c.m.r32((l_p_8).wrapping_add(8))); let a2_ = F32(c.m.r32((l_p_8).wrapping_add(16))); let a3_ = F32(c.m.r32(((l_p_8).wrapping_add(16)).wrapping_add(8))); F::Stg_CalcYaw(c, a0_, a1_, a2_, a3_) };
            { let v_ = F32(c.m.r32(l_p_8)); let a_ = p_pos_2; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0xc42f0000); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_p_8).wrapping_add(8))); let a_ = (p_pos_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (p_pos_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        } else {
            l_p_8 = ((((p_side_1).wrapping_mul(((32u32) as i32))).wrapping_add(((l_starts_7) as i32))) as u32);
            l_yaw_9 = { let a0_ = F32(c.m.r32(l_p_8)); let a1_ = F32(c.m.r32((l_p_8).wrapping_add(8))); let a2_ = F32(c.m.r32((l_p_8).wrapping_add(16))); let a3_ = F32(c.m.r32(((l_p_8).wrapping_add(16)).wrapping_add(8))); F::Stg_CalcYaw(c, a0_, a1_, a2_, a3_) };
            { let v_ = F32(c.m.r32(l_p_8)); let a_ = p_pos_2; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0xc42f0000); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_p_8).wrapping_add(8))); let a_ = (p_pos_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (p_pos_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        { let v_ = l_yaw_9; let a_ = (p_rot_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_rot_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = p_rot_3; c.m.w32(a_, (v_).0); };
        l_zone_5 = F::BtlStage_ProbeGround(c, (1i32).wrapping_neg(), p_pos_2, fp);
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
        break 'f l_zone_5;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetStartPlace", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetPlace(c: &mut Ctx, mut p_pos_1: u32, mut p_rot_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPlace", &PROBE_N, &[(p_pos_1) as u32, (p_rot_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_zone_3: i32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_yaw_5: F32 = F32(0);
    let r_ = 'f: {
        { let _ = F::BtlStage_IsReady(c); };
        l_p_4 = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(60); c.m.r32(a_) };
        l_yaw_5 = { let a0_ = F32(c.m.r32(l_p_4)); let a1_ = F32(c.m.r32((l_p_4).wrapping_add(8))); let a2_ = F32(c.m.r32((l_p_4).wrapping_add(16))); let a3_ = F32(c.m.r32(((l_p_4).wrapping_add(16)).wrapping_add(8))); F::Stg_CalcYaw(c, a0_, a1_, a2_, a3_) };
        { let v_ = F32(c.m.r32(l_p_4)); let a_ = p_pos_1; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xc42f0000); let a_ = (p_pos_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_p_4).wrapping_add(8))); let a_ = (p_pos_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_pos_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_rot_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = l_yaw_5; let a_ = (p_rot_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = p_rot_2; c.m.w32(a_, (v_).0); };
        l_zone_3 = F::BtlStage_ProbeGround(c, (1i32).wrapping_neg(), p_pos_1, fp);
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (p_pos_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        break 'f l_zone_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPlace", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetLightVecA(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetLightVecA", &PROBE_N, &[(p_out_1) as u32], false); }
    let mut l_l_2: u32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_1, sym::gStgLastLightVecA);
        break 'f;
    }
    l_l_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(28); c.m.r32(a_) };
    { let v_ = F32(c.m.r32(l_l_2)); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_l_2).wrapping_add(4))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_l_2).wrapping_add(8))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, sym::gStgLastLightVecA, p_out_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetLightVecA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetLightVecB(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetLightVecB", &PROBE_N, &[(p_out_1) as u32], false); }
    let mut l_l_2: u32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_1, sym::gStgLastLightVecB);
        break 'f;
    }
    l_l_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(28); c.m.r32(a_) };
    { let v_ = F32(c.m.r32((l_l_2).wrapping_add(16))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_l_2).wrapping_add(16)).wrapping_add(4))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_l_2).wrapping_add(16)).wrapping_add(8))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, sym::gStgLastLightVecB, p_out_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetLightVecB", &PROBE_N, &[], true); }
    r_
}

pub fn StgObj_LoadDef(c: &mut Ctx, mut p_obj_1: u32, mut p_idx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgObj_LoadDef", &PROBE_N, &[(p_obj_1) as u32, (p_idx_2) as u32], false); }
    let mut l_def_3: u32 = 0;
    let mut l_stg_4: u32 = 0;
    let r_ = 'f: {
    l_stg_4 = c.m.r32(sym::gBtlStage);
    if (((((((((l_stg_4) == (((0i32) as u32))) as i32)) != 0) || (((((c.m.r32((l_stg_4).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_def_3 = ((((p_idx_2).wrapping_mul(((64u32) as i32))).wrapping_add((({ let a_ = (c.m.r32((l_stg_4).wrapping_add(4))).wrapping_add(12); c.m.r32(a_) }) as i32))) as u32);
        { let v_ = (c.m.r32(l_def_3) as i32); let a_ = (p_obj_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((l_def_3).wrapping_add(4)) as i32); let a_ = (p_obj_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((l_def_3).wrapping_add(12)) as i32); let a_ = (p_obj_1).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgObj_LoadDef", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetObjDef(c: &mut Ctx, mut p_idx_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetObjDef", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_d_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((!(((((p_idx_1) < ((c.m.r32((l_d_2).wrapping_add(8)) as i32))) as i32)) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f (c.m.r32((l_d_2).wrapping_add(12))).wrapping_add((((p_idx_1) as u32)).wrapping_mul(64));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetObjDef", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList40(c: &mut Ctx, mut p_idx_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList40", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_d_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((!(((((p_idx_1) < ((c.m.r32((l_d_2).wrapping_add(64)) as i32))) as i32)) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f (c.m.r32((l_d_2).wrapping_add(68))).wrapping_add((((p_idx_1) as u32)).wrapping_mul(16));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList40", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList48(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList48", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(72)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(76));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList48", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList50(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList50", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(80)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(84));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList50", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList90(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList90", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(144)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(148));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList90", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetListA0(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetListA0", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(160)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(164));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetListA0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList98(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList98", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(152)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(156));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList98", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList60(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList60", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(96)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(100));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList60", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList68(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList68", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(104)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(108));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList68", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList58(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList58", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(88)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(92));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList58", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetLightColors(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetLightColors", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(120)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(124));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetLightColors", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetList80(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList80", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r32((l_d_1).wrapping_add(128)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f c.m.r32((l_d_1).wrapping_add(132));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetList80", &PROBE_N, &[], true); }
    r_
}
