#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlObj_InitNop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitNop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitNop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_TermNop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_TermNop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_TermNop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_RebuildTable(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_RebuildTable", &PROBE_N, &[], false); }
    let mut l_node_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlObjWork)) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = F::memset(c, sym::gBtlObjTbl, 0i32, 48u32); };
        {
            l_node_1 = { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetHead(c, a0_) };
            'l1: loop {
                if !(((((l_node_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (l_node_1).wrapping_add(16); let a_ = (sym::gBtlObjTbl).wrapping_add(((((c.m.r32((l_node_1).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                l_node_1 = F::List_GetNext(c, l_node_1);
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_RebuildTable", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_Get(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_Get", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlObjWork)).wrapping_add(439152);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_Clear(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = F::BtlObjVis_Get(c); F::memset(c, a0_, 0i32, 124u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_AllocBit(c: &mut Ctx, mut p_bit_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_AllocBit", &PROBE_N, &[(p_bit_1) as u32], false); }
    let mut l_vis_2: u32 = 0;
    let mut l_mask_3: u32 = 0;
    let r_ = 'f: {
    l_vis_2 = F::BtlObjVis_Get(c);
    l_mask_3 = (((1i32).wrapping_shl((p_bit_1) as u32)) as u32);
    if ((((!((((c.m.r32((l_vis_2).wrapping_add(4))) & (l_mask_3))) != 0)) as i32)) != 0) {
        { let r_ = l_mask_3; let a_ = (l_vis_2).wrapping_add(4); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_AllocBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_FreeBit(c: &mut Ctx, mut p_bit_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_FreeBit", &PROBE_N, &[(p_bit_1) as u32], false); }
    let mut l_vis_2: u32 = 0;
    let r_ = 'f: {
    l_vis_2 = F::BtlObjVis_Get(c);
    { let r_ = (((!((1i32).wrapping_shl((p_bit_1) as u32)))) as u32); let a_ = (l_vis_2).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_FreeBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_AllocFreeBit(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_AllocFreeBit", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (15i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((F::BtlObjVis_AllocBit(c, l_i_1)) != 0) {
                    break 'f l_i_1;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_AllocFreeBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_GetViewCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_GetViewCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((c.m.r32(sym::gBtlObjWork)).wrapping_add(439152)).wrapping_add(8); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_GetViewCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_TestFlags(c: &mut Ctx, mut p_mask_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_TestFlags", &PROBE_N, &[(p_mask_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((({ let a_ = (c.m.r32(sym::gBtlObjWork)).wrapping_add(439152); c.m.r32(a_) }) & (p_mask_1))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_TestFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_GetList(c: &mut Ctx, mut p_view_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_GetList", &PROBE_N, &[(p_view_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r32(sym::gBtlObjWork)).wrapping_add(439152)).wrapping_add(20)).wrapping_add((((p_view_1) as u32)).wrapping_mul(52));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_GetList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjVis_GetView(c: &mut Ctx, mut p_view_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_GetView", &PROBE_N, &[(p_view_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (((c.m.r32(sym::gBtlObjWork)).wrapping_add(439152)).wrapping_add(12)).wrapping_add((((p_view_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjVis_GetView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_Get(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_Get", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlObjWork)).wrapping_add(439088);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_Init", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_light_1: u32 = 0;
    let mut l_res_2: u32 = 0;
    let r_ = 'f: {
        l_light_1 = F::BtlObjLight_Get(c);
        l_res_2 = c.m.r32(sym::gCommonRes);
        { let _ = F::memset(c, l_light_1, 0i32, 64u32); };
        { let v_ = (c.m.r32(((l_res_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add((({ let a_ = (c.m.r32(((l_res_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (l_light_1).wrapping_add(48); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32((l_light_1).wrapping_add(48)); let a2_ = c.m.r32((l_light_1).wrapping_add(48)); F::Res_RelocateOffsets(c, (l_light_1).wrapping_add(48), a1_, a2_) };
        F::Vec4_Set(c, fp, F32(0x3eae147a), F32(0xbbc49ba5), F32(0x3f70e560), F32(0x3f800000));
        F::BtlObjLight_SetDir(c, fp);
        F::Vec4_Set(c, fp.wrapping_add(16), F32(0x41200000), F32(0x41200000), F32(0x41200000), F32(0x42000000));
        F::BtlObjLight_SetColor(c, fp.wrapping_add(16));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_SetDir(c: &mut Ctx, mut p_dir_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_SetDir", &PROBE_N, &[(p_dir_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_light_2: u32 = 0;
    let r_ = 'f: {
        l_light_2 = F::BtlObjLight_Get(c);
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbfcccccc); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        F::Vec4_Copy(c, l_light_2, p_dir_1);
        F::Vec3_Normalize(c, l_light_2, l_light_2);
        F::Vec4_Copy(c, (l_light_2).wrapping_add(16), l_light_2);
        F::Vec4_Add(c, (l_light_2).wrapping_add(16), (l_light_2).wrapping_add(16), fp);
        F::Vec3_Scale(c, (l_light_2).wrapping_add(16), (l_light_2).wrapping_add(16), F32(0x3f000000));
        F::Vec3_Normalize(c, (l_light_2).wrapping_add(16), (l_light_2).wrapping_add(16));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_GetDir(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_GetDir", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = F::BtlObjLight_Get(c); F::Vec4_Copy(c, p_out_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_GetDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_GetHalf(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_GetHalf", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = (F::BtlObjLight_Get(c)).wrapping_add(16); F::Vec4_Copy(c, p_out_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_GetHalf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_SetColor(c: &mut Ctx, mut p_color_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_SetColor", &PROBE_N, &[(p_color_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (F::BtlObjLight_Get(c)).wrapping_add(32); F::Vec4_Copy(c, a0_, p_color_1) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_GetColor(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_GetColor", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = (F::BtlObjLight_Get(c)).wrapping_add(32); F::Vec4_Copy(c, p_out_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_GetColor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_FindRes0(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_FindRes0", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = (F::BtlObjLight_Get(c)).wrapping_add(48); c.m.r32(a_) }; F::TexFile_UploadOne(c, a0_, 0i32, 15680i32, (1i32).wrapping_neg()) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_FindRes0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjLight_FindRes1(c: &mut Ctx, mut p_key_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_FindRes1", &PROBE_N, &[(p_key_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = (F::BtlObjLight_Get(c)).wrapping_add(48); c.m.r32(a_) }; F::TexFile_UploadOne(c, a0_, 1i32, p_key_1, (1i32).wrapping_neg()) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjLight_FindRes1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPoolE0_Get(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_Get", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlObjWork)).wrapping_add(282624);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPoolE0_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_Init", &PROBE_N, &[], false); }
    let mut l_pool_1: u32 = 0;
    let mut l_node_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_pool_1 = F::BtlObjPoolE0_Get(c);
    { let _ = F::memset(c, l_pool_1, 0i32, 114704u32); };
    l_node_2 = l_pool_1;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (512i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::SList_PushFront(c, (l_pool_1).wrapping_add(114688), l_node_2);
                { let o_ = l_node_2; let n_ = (o_).wrapping_add(224); l_node_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPoolE0_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_Alloc", &PROBE_N, &[], false); }
    let mut l_node_1: u32 = 0;
    let r_ = 'f: {
    l_node_1 = { let a0_ = (F::BtlObjPoolE0_Get(c)).wrapping_add(114688); F::SList_PopFront(c, a0_) };
    if (((((l_node_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    { let _ = F::memset(c, l_node_1, 0i32, 224u32); };
    F::Vec4_Set(c, (l_node_1).wrapping_add(144), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
    F::Vec4_Set(c, (l_node_1).wrapping_add(160), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
    break 'f l_node_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPoolE0_FreeList(c: &mut Ctx, mut p_list_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_FreeList", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (F::BtlObjPoolE0_Get(c)).wrapping_add(114688); F::SList_AppendList(c, a0_, p_list_1) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_FreeList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPoolE0_GetFreeCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_GetFreeCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (F::BtlObjPoolE0_Get(c)).wrapping_add(114688); F::SList_GetCount(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPoolE0_GetFreeCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool70_Get(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_Get", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlObjWork)).wrapping_add(397328);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool70_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_Init", &PROBE_N, &[], false); }
    let mut l_pool_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_pool_1 = F::BtlObjPool70_Get(c);
    { let _ = F::memset(c, l_pool_1, 0i32, 464u32); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::SList_PushFront(c, (l_pool_1).wrapping_add(448), (l_pool_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(112)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool70_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_Alloc", &PROBE_N, &[], false); }
    let mut l_link_1: u32 = 0;
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    l_link_1 = { let a0_ = (F::BtlObjPool70_Get(c)).wrapping_add(448); F::SList_PopFront(c, a0_) };
    l_node_2 = l_link_1;
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool70_Free(c: &mut Ctx, mut p_node_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_Free", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (F::BtlObjPool70_Get(c)).wrapping_add(448); F::SList_PushBack(c, a0_, p_node_1) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_Free", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool70_GetFreeCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_GetFreeCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (F::BtlObjPool70_Get(c)).wrapping_add(448); F::SList_GetCount(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool70_GetFreeCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool3_Get(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool3_Get", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlObjWork)).wrapping_add(397792);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool3_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool3_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool3_Alloc", &PROBE_N, &[], false); }
    let mut l_link_1: u32 = 0;
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    l_link_1 = { let a0_ = (F::BtlObjPool3_Get(c)).wrapping_add(41280); F::SList_PopFront(c, a0_) };
    l_node_2 = l_link_1;
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool3_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool3_Free(c: &mut Ctx, mut p_node_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool3_Free", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (F::BtlObjPool3_Get(c)).wrapping_add(41280); F::SList_PushBack(c, a0_, p_node_1) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool3_Free", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPool3_GetFreeCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool3_GetFreeCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (F::BtlObjPool3_Get(c)).wrapping_add(41280); F::SList_GetCount(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPool3_GetFreeCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetTable(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetTable", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gBtlObjWork);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetTable", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetUsedList(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetUsedList", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (F::BtlObj_GetTable(c)).wrapping_add(69132);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetUsedList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_InitTable(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitTable", &PROBE_N, &[], false); }
    let mut l_tbl_1: u32 = 0;
    let mut l_node_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_tbl_1 = F::BtlObj_GetTable(c);
    { let _ = F::memset(c, l_tbl_1, 0i32, 282624u32); };
    { let v_ = F32(0x40000000); let a_ = (l_tbl_1).wrapping_add(282564); c.m.w32(a_, (v_).0); };
    F::List_Init(c, (l_tbl_1).wrapping_add(69120));
    F::List_Init(c, (l_tbl_1).wrapping_add(69132));
    l_node_2 = l_tbl_1;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (12i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = l_i_3; let a_ = (l_node_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                F::List_PushBack(c, (l_tbl_1).wrapping_add(69120), l_node_2);
                { let o_ = l_node_2; let n_ = (o_).wrapping_add(5760); l_node_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitTable", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetUnk44FC4(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetUnk44FC4", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlObj_GetTable(c)).wrapping_add(282564); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetUnk44FC4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SetUnk44FC4(c: &mut Ctx, mut p_value_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetUnk44FC4", &PROBE_N, &[(p_value_1).0], false); }
    let r_ = 'f: {
    { let v_ = p_value_1; let a_ = (F::BtlObj_GetTable(c)).wrapping_add(282564); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetUnk44FC4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetCount(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_AllocId(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_AllocId", &PROBE_N, &[], false); }
    let mut l_tbl_1: u32 = 0;
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    l_tbl_1 = F::BtlObj_GetTable(c);
    l_node_2 = F::List_PopFront(c, (l_tbl_1).wrapping_add(69120));
    if (((((l_node_2) != (((0i32) as u32))) as i32)) != 0) {
        F::List_PushBack(c, (l_tbl_1).wrapping_add(69132), l_node_2);
        F::BtlObj_RebuildTable(c);
        break 'f (c.m.r32((l_node_2).wrapping_add(8)) as i32);
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_AllocId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_FreeId(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FreeId", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_tbl_2: u32 = 0;
    let mut l_node_3: u32 = 0;
    let r_ = 'f: {
    l_tbl_2 = F::BtlObj_GetTable(c);
    {
        l_node_3 = { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetHead(c, a0_) };
        'l1: loop {
            if !(((((l_node_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_node_3).wrapping_add(8)) as i32)) == (p_id_1)) as i32)) != 0) {
                    { let a1_ = F::List_Remove(c, (l_tbl_2).wrapping_add(69132), l_node_3); F::List_PushBack(c, (l_tbl_2).wrapping_add(69120), a1_) };
                    break 'f 1i32;
                }
            }
            l_node_3 = F::List_GetNext(c, l_node_3);
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FreeId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_Get(c: &mut Ctx, mut p_id_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Get", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    break 'f c.m.r32((sym::gBtlObjTbl).wrapping_add((((p_id_1) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetCharaWork(c: &mut Ctx, mut p_id_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetCharaWork", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_tbl_2: u32 = 0;
    let r_ = 'f: {
    l_tbl_2 = F::BtlObj_GetTable(c);
    if (((((p_id_1) < (((3i32) as u32))) as i32)) != 0) {
        break 'f ((l_tbl_2).wrapping_add(69184)).wrapping_add((p_id_1).wrapping_mul(106688));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetCharaWork", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_Setup(c: &mut Ctx, mut p_obj_1: u32, mut p_type_2: i32, mut p_res_3: u32, mut p_id_4: i32, mut p_active_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Setup", &PROBE_N, &[(p_obj_1) as u32, (p_type_2) as u32, (p_res_3) as u32, (p_id_4) as u32, (p_active_5) as u32], false); }
    let r_ = 'f: {
    F::BtlObjMdl_Destroy(c, p_obj_1);
    F::BtlObjMdl_Create(c, p_type_2, p_obj_1, p_res_3);
    { let v_ = p_active_5; let a_ = (p_obj_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_id_4; let a_ = (p_obj_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::div_i32(((c.m.r32(((p_res_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_sub(1424i32), 10i32); let a_ = (p_obj_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::BtlObj_InitChains(c, p_obj_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Setup", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_Create(c: &mut Ctx, mut p_type_1: i32, mut p_res_2: u32, mut p_active_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Create", &PROBE_N, &[(p_type_1) as u32, (p_res_2) as u32, (p_active_3) as u32], false); }
    let mut l_id_4: i32 = 0;
    let mut l_obj_5: u32 = 0;
    let r_ = 'f: {
    l_id_4 = F::BtlObj_AllocId(c);
    if (((((l_id_4) < (0i32)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    l_obj_5 = F::BtlObj_Get(c, l_id_4);
    F::BtlObj_Setup(c, l_obj_5, p_type_1, p_res_2, l_id_4, p_active_3);
    if (((((p_type_1) == (0i32)) as i32)) != 0) {
        { let v_ = F::BtlObj_GetCharaWork(c, ((l_id_4) as u32)); let a_ = (l_obj_5).wrapping_add(5728); c.m.w32(a_, v_); };
        F::BtlObjFlash_Reset(c, l_obj_5);
    }
    break 'f l_id_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Create", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_Destroy(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Destroy", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_id_1);
    if (((((l_obj_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlObj_FreeId(c, p_id_1)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::BtlObjMdl_Destroy(c, l_obj_2);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Destroy", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_CreateChara(c: &mut Ctx, mut p_slot_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CreateChara", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = F::BtlRes_GetSlot(c, p_slot_1); F::BtlObj_Create(c, 0i32, a1_, 1i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CreateChara", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_Rebind(c: &mut Ctx, mut p_id_1: i32, mut p_slot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Rebind", &PROBE_N, &[(p_id_1) as u32, (p_slot_2) as u32], false); }
    let mut l_res_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_res_3 = F::BtlRes_GetSlot(c, p_slot_2);
    l_obj_4 = F::BtlObj_Get(c, p_id_1);
    F::BtlObj_Setup(c, l_obj_4, 0i32, l_res_3, p_id_1, 1i32);
    { let v_ = F::BtlObj_GetCharaWork(c, ((p_id_1) as u32)); let a_ = (l_obj_4).wrapping_add(5728); c.m.w32(a_, v_); };
    F::BtlObjFlash_Reset(c, l_obj_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Rebind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_GetWork(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_GetWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlObjWork)).wrapping_add(439276);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_RequestCharaModel(c: &mut Ctx, mut p_side_1: i32, mut p_chara_2: i32, mut p_costume_3: i32, mut p_variant_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_RequestCharaModel", &PROBE_N, &[(p_side_1) as u32, (p_chara_2) as u32, (p_costume_3) as u32, (p_variant_4) as u32], false); }
    let mut l_model_5: i32 = 0;
    let r_ = 'f: {
    if (((((p_variant_4) != (0i32)) as i32)) != 0) {
        l_model_5 = (((p_chara_2).wrapping_mul(10i32)).wrapping_add(p_costume_3)).wrapping_add(1428i32);
    } else {
        l_model_5 = (((p_chara_2).wrapping_mul(10i32)).wrapping_add(p_costume_3)).wrapping_add(1424i32);
    }
    break 'f F::BtlRes_Request(c, 1i32, l_model_5, ((p_chara_2).wrapping_mul(10i32)).wrapping_add(1432i32), ((p_chara_2).wrapping_mul(10i32)).wrapping_add(1433i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_RequestCharaModel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetWork(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gBtlObjWork);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_IsInit(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsInit", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((c.m.r32(sym::gBtlObjWork)) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsInit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_CommitReloadEx(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_CommitReloadEx", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlRes_CommitReload(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_CommitReloadEx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_Init(c: &mut Ctx, mut p_prealloc_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Init", &PROBE_N, &[(p_prealloc_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, 440128i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBtlObjWork; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gBtlObjWork); F::memset(c, a0_, 0i32, ((440128i32) as u32)) }; };
    F::BtlRes_Init(c, p_prealloc_1);
    F::BtlObj_InitTable(c);
    F::BtlObjPoolE0_Init(c);
    F::BtlObjPool70_Init(c);
    F::ObjShadow_InitPool(c);
    F::BtlObj_InitNop(c);
    F::BtlObjLight_Init(c);
    F::BtlObjVis_Clear(c);
    F::BtlObj_InitDraw(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlObj_TermDraw(c);
    F::BtlObj_TermNop(c);
    F::BtlRes_Term(c);
    F::ObjShadow_TermPool(c);
    { let a0_ = c.m.r32(sym::gBtlObjWork); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gBtlObjWork; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_UpdateBounds(c: &mut Ctx, mut p_obj_1: u32, mut p_state_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateBounds", &PROBE_N, &[(p_obj_1) as u32, (p_state_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(496) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 496);
    let mut l_bound_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let r_ = 'f: {
        l_bound_3 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
        F::StgAabb_SetEmpty(c, (p_state_2).wrapping_add(180));
        'l1: while ((1i32) != 0) {
            if (((((((c.m.r16((l_bound_3).wrapping_add(8))) as i32)) != (0i32)) as i32)) != 0) {
                l_part_4 = { let a1_ = ((c.m.r16((l_bound_3).wrapping_add(10))) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) };
                if ((((({ let a1_ = (c.m.r32((l_part_4).wrapping_add(8)) as i32); F::BtlObj_IsNodeShown(c, p_obj_1, a1_) }) == (1i32)) as i32)) != 0) {
                    { let v_ = ((1i32) as u8); let a_ = (l_part_4).wrapping_add(12); c.m.w8(a_, v_); };
                    F::Vec3_Sub(c, fp, (l_bound_3).wrapping_add(64), (l_bound_3).wrapping_add(16));
                    F::Mtx_MulVec4(c, fp.wrapping_add(16), (l_part_4).wrapping_add(16), fp);
                    F::Mtx_Copy(c, fp.wrapping_add(32), (l_part_4).wrapping_add(16));
                    F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(16));
                    { let a1_ = F32(c.m.r32((l_bound_3).wrapping_add(80))); let a2_ = F32(c.m.r32(((l_bound_3).wrapping_add(80)).wrapping_add(4))); let a3_ = F32(c.m.r32(((l_bound_3).wrapping_add(80)).wrapping_add(8))); F::Vec4_Set(c, fp.wrapping_add(112), a1_, a2_, a3_, F32(0x00000000)) };
                    { let a1_ = F32(c.m.r32(fp.wrapping_add(112))); let a2_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))); F::Vec4_Set(c, (fp.wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
                    { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(112)))); let a2_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))); F::Vec4_Set(c, (fp.wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
                    { let a1_ = F32(c.m.r32(fp.wrapping_add(112))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4)))); let a3_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))); F::Vec4_Set(c, (fp.wrapping_add(240)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
                    { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(112)))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4)))); let a3_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))); F::Vec4_Set(c, (fp.wrapping_add(240)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
                    { let a1_ = F32(c.m.r32(fp.wrapping_add(112))); let a2_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8)))); F::Vec4_Set(c, (fp.wrapping_add(240)).wrapping_add((((4i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
                    { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(112)))); let a2_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8)))); F::Vec4_Set(c, (fp.wrapping_add(240)).wrapping_add((((5i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
                    { let a1_ = F32(c.m.r32(fp.wrapping_add(112))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4)))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8)))); F::Vec4_Set(c, (fp.wrapping_add(240)).wrapping_add((((6i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
                    { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(112)))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4)))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8)))); F::Vec4_Set(c, (fp.wrapping_add(240)).wrapping_add((((7i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
                    F::Mtx_MulVec4(c, (fp.wrapping_add(368)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32), (fp.wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    F::Mtx_MulVec4(c, (fp.wrapping_add(368)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32), (fp.wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    F::Mtx_MulVec4(c, (fp.wrapping_add(368)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32), (fp.wrapping_add(240)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    F::Mtx_MulVec4(c, (fp.wrapping_add(368)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32), (fp.wrapping_add(240)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    F::Mtx_MulVec4(c, (fp.wrapping_add(368)).wrapping_add((((4i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32), (fp.wrapping_add(240)).wrapping_add((((4i32) as u32)).wrapping_mul(16)));
                    F::Mtx_MulVec4(c, (fp.wrapping_add(368)).wrapping_add((((5i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32), (fp.wrapping_add(240)).wrapping_add((((5i32) as u32)).wrapping_mul(16)));
                    F::Mtx_MulVec4(c, (fp.wrapping_add(368)).wrapping_add((((6i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32), (fp.wrapping_add(240)).wrapping_add((((6i32) as u32)).wrapping_mul(16)));
                    F::Mtx_MulVec4(c, (fp.wrapping_add(368)).wrapping_add((((7i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32), (fp.wrapping_add(240)).wrapping_add((((7i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_state_2).wrapping_add(180), (fp.wrapping_add(368)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_state_2).wrapping_add(180), (fp.wrapping_add(368)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_state_2).wrapping_add(180), (fp.wrapping_add(368)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_state_2).wrapping_add(180), (fp.wrapping_add(368)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_state_2).wrapping_add(180), (fp.wrapping_add(368)).wrapping_add((((4i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_state_2).wrapping_add(180), (fp.wrapping_add(368)).wrapping_add((((5i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_state_2).wrapping_add(180), (fp.wrapping_add(368)).wrapping_add((((6i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_state_2).wrapping_add(180), (fp.wrapping_add(368)).wrapping_add((((7i32) as u32)).wrapping_mul(16)));
                } else {
                    { let v_ = ((0i32) as u8); let a_ = (l_part_4).wrapping_add(12); c.m.w8(a_, v_); };
                }
            }
            if (((((((c.m.r16((l_bound_3).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                break 'f;
            }
            l_bound_3 = (l_bound_3).wrapping_add((((c.m.r32(l_bound_3) as i32)) as u32));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateBounds", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_CalcViewDist(c: &mut Ctx, mut p_state_1: u32, mut p_view_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CalcViewDist", &PROBE_N, &[(p_state_1) as u32, (p_view_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_i_3: i32 = 0;
    let mut l_dist_4: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = F32(c.m.r32((sym::gBtlObjFarDist).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_view_2).wrapping_add(68); c.m.w32(a_, (v_).0); };
        F::ColBox_GetCenter(c, (p_state_1).wrapping_add(180), fp);
        F::ColBox_GetHalf(c, (p_state_1).wrapping_add(180), fp.wrapping_add(16));
        { let a1_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp))); let a3_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8)))); F::Vec4_Set(c, (fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
        { let a1_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp))); let a3_ = rt::fsub(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))); F::Vec4_Set(c, (fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
        { let a1_ = rt::fsub(F32(c.m.r32(fp)), F32(c.m.r32(fp.wrapping_add(16)))); let a3_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8)))); F::Vec4_Set(c, (fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
        { let a1_ = rt::fsub(F32(c.m.r32(fp)), F32(c.m.r32(fp.wrapping_add(16)))); let a3_ = rt::fsub(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))); F::Vec4_Set(c, (fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_dist_4 = F::View_GetDistXZ(c, (fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    if (((rt::flt(l_dist_4, F32(c.m.r32((p_view_2).wrapping_add(68)))) as i32)) != 0) {
                        { let v_ = l_dist_4; let a_ = (p_view_2).wrapping_add(68); c.m.w32(a_, (v_).0); };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((rt::fgt(F32(c.m.r32((p_view_2).wrapping_add(68))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = { let a0_ = F32(c.m.r32((p_view_2).wrapping_add(68))); F::Port_sqrtf(c, a0_) }; let a_ = (p_view_2).wrapping_add(68); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CalcViewDist", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjView_SetFade(c: &mut Ctx, mut p_view_1: u32, mut p_nearDist_2: F32, mut p_farDist_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjView_SetFade", &PROBE_N, &[(p_view_1) as u32, (p_nearDist_2).0, (p_farDist_3).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_farDist_3, F32(c.m.r32((p_view_1).wrapping_add(68)))) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (p_view_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let r_ = (((!(8i32))) as u32); let a_ = p_view_1; let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    } else {
        if (((rt::flt(p_nearDist_2, F32(c.m.r32((p_view_1).wrapping_add(68)))) as i32)) != 0) {
            { let v_ = rt::fsub(F32(0x43000000), rt::fmul(rt::fdiv(rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(68))), p_nearDist_2), rt::fsub(p_farDist_3, p_nearDist_2)), F32(0x43000000))); let a_ = (p_view_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
            if (((((rt::f2i(F32(c.m.r32((p_view_1).wrapping_add(64))))) == (0i32)) as i32)) != 0) {
                { let r_ = (((!(8i32))) as u32); let a_ = p_view_1; let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            } else {
                { let r_ = ((8i32) as u32); let a_ = p_view_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        } else {
            { let v_ = F32(0x43000000); let a_ = (p_view_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
            { let r_ = ((8i32) as u32); let a_ = p_view_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjView_SetFade", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_UpdateAlpha(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateAlpha", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_state_2: u32 = 0;
    let mut l_mdl_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_alpha_5: i32 = 0;
    let r_ = 'f: {
    l_state_2 = (p_obj_1).wrapping_add(2624);
    l_mdl_3 = (p_obj_1).wrapping_add(24);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_alpha_5 = (((c.m.r8((l_state_2).wrapping_add(240))) as i32)).wrapping_add((({ let a_ = ((c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(44)).wrapping_add(((l_i_4) as u32)); c.m.r8(a_) }) as i32));
                if (((((l_alpha_5) >= (128i32)) as i32)) != 0) {
                    l_alpha_5 = 127i32;
                }
                { let v_ = ((l_alpha_5) as u8); let a_ = ((l_state_2).wrapping_add(224)).wrapping_add(((l_i_4) as u32)); c.m.w8(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_CalcViewColor(c: &mut Ctx, mut p_obj_1: u32, mut p_view_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CalcViewColor", &PROBE_N, &[(p_obj_1) as u32, (p_view_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        { let _ = F::BtlStage_GetAmbient(c, fp.wrapping_add(16)); };
        if ((((c.m.r32(p_view_2)) & (((65536i32) as u32)))) != 0) {
            { let v_ = rt::i2f((c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            F::Vec4_Copy(c, (p_view_2).wrapping_add(32), fp);
        } else {
            if ((((c.m.r32(p_view_2)) & (((131072i32) as u32)))) != 0) {
                F::Vec4_Copy(c, (p_view_2).wrapping_add(32), fp);
            } else {
                if ((((c.m.r32(p_view_2)) & (((262144i32) as u32)))) != 0) {
                    F::Vec4_Copy(c, (p_view_2).wrapping_add(32), fp);
                } else {
                    if ((((c.m.r32(p_view_2)) & (((524288i32) as u32)))) != 0) {
                        F::Vec4_Set(c, (p_view_2).wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x43000000));
                    } else {
                        if ((((c.m.r32(p_view_2)) & (((1048576i32) as u32)))) != 0) {
                            F::Vec4_Copy(c, (p_view_2).wrapping_add(32), fp);
                        } else {
                            if ((((c.m.r32(p_view_2)) & (((4194304i32) as u32)))) != 0) {
                                F::Vec4_Copy(c, (p_view_2).wrapping_add(32), fp);
                            } else {
                                F::Vec4_Copy(c, (p_view_2).wrapping_add(32), fp);
                            }
                        }
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CalcViewColor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_IsOffscreen(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsOffscreen", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_box_2: u32 = 0;
    let mut l_out_3: i32 = 0;
    let mut l_total_4: i32 = 0;
    let mut l_prev_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_out_3 = 0i32;
        { let _ = F::memset(c, fp.wrapping_add(224), 0i32, 20u32); };
        l_box_2 = ((p_obj_1).wrapping_add(2624)).wrapping_add(180);
        F::ColBox_GetCenter(c, l_box_2, fp.wrapping_add(128));
        F::ColBox_GetHalf(c, l_box_2, fp.wrapping_add(144));
        l_total_4 = 0i32;
        { let a1_ = F32(c.m.r32(fp.wrapping_add(144))); let a2_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(144)))); let a2_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(144))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4)))); let a3_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(144)))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4)))); let a3_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(144))); let a2_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8)))); F::Vec4_Set(c, (fp).wrapping_add((((4i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(144)))); let a2_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8)))); F::Vec4_Set(c, (fp).wrapping_add((((5i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(144))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4)))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8)))); F::Vec4_Set(c, (fp).wrapping_add((((6i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(144)))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4)))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8)))); F::Vec4_Set(c, (fp).wrapping_add((((7i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        F::Vu0Screen_StoreMtx(c, fp.wrapping_add(160));
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec3_Add(c, (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), fp.wrapping_add(128));
                    F::Mtx_ProjectInt(c, fp.wrapping_add(244), fp.wrapping_add(160), (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)));
                    { let r_ = 1792i32; let a_ = (fp.wrapping_add(244)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 1824i32; let a_ = (fp.wrapping_add(244)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32((fp.wrapping_add(244)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                        { let v_ = ((c.m.r32((fp.wrapping_add(244)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_neg(); let a_ = (fp.wrapping_add(244)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(244)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_neg(); let a_ = (fp.wrapping_add(244)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    l_prev_5 = l_total_4;
                    if ((((((c.m.r32((fp.wrapping_add(244)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                        { let a_ = (fp.wrapping_add(224)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_total_4; let n_ = (o_).wrapping_add(1); l_total_4 = n_; };
                    }
                    if ((((((c.m.r32((fp.wrapping_add(244)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < ({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let a_ = (fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_total_4; let n_ = (o_).wrapping_add(1); l_total_4 = n_; };
                    }
                    if (((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) < ((c.m.r32((fp.wrapping_add(244)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let a_ = (fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_total_4; let n_ = (o_).wrapping_add(1); l_total_4 = n_; };
                    }
                    if ((((((c.m.r32((fp.wrapping_add(244)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) < ({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let a_ = (fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_total_4; let n_ = (o_).wrapping_add(1); l_total_4 = n_; };
                    }
                    if (((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) < ((c.m.r32((fp.wrapping_add(244)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let a_ = (fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_total_4; let n_ = (o_).wrapping_add(1); l_total_4 = n_; };
                    }
                    if (((((l_prev_5) != (l_total_4)) as i32)) != 0) {
                        { let o_ = l_out_3; let n_ = (o_).wrapping_add(1); l_out_3 = n_; };
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if (((((l_out_3) == (8i32)) as i32)) != 0) {
            if ((((((c.m.r32((fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == (8i32)) as i32)) != 0) {
                break 'f 1i32;
            }
            if ((((((c.m.r32((fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) == (8i32)) as i32)) != 0) {
                break 'f 1i32;
            }
            if ((((((c.m.r32((fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) == (8i32)) as i32)) != 0) {
                break 'f 1i32;
            }
            if ((((((c.m.r32((fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) == (8i32)) as i32)) != 0) {
                break 'f 1i32;
            }
            if ((((((c.m.r32((fp.wrapping_add(224)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32)) == (8i32)) as i32)) != 0) {
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsOffscreen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_UpdateView(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateView", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_state_2: u32 = 0;
    let mut l_mdl_3: u32 = 0;
    let mut l_view_4: u32 = 0;
    let mut l_body_5: u32 = 0;
    let r_ = 'f: {
        l_state_2 = (p_obj_1).wrapping_add(2624);
        l_mdl_3 = (p_obj_1).wrapping_add(24);
        l_view_4 = c.m.r32(((p_obj_1).wrapping_add(2624)).wrapping_add(176));
        { let _ = F::memset(c, l_view_4, 0i32, 80u32); };
        { let v_ = c.m.r32(l_state_2); let a_ = l_view_4; c.m.w32(a_, v_); };
        F::BtlObj_CalcViewDist(c, l_state_2, l_view_4);
        if ((((c.m.r32(l_view_4)) & (((2i32) as u32)))) != 0) {
            if ((F::BtlObj_IsOffscreen(c, p_obj_1)) != 0) {
                { let r_ = (((!(2i32))) as u32); let a_ = l_view_4; let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            } else {
                if ((((((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((2305843009213693952i64) as u64)))) != 0)) as i32)) != 0) && ((((((c.m.r32(p_obj_1) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) && ((((c.m.r32(l_state_2)) & (((33554432i32) as u32)))) != 0)) as i32)) != 0) {
                    l_body_5 = (p_obj_1).wrapping_add(3920);
                    if (((((l_body_5) != (((0i32) as u32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp, a1_) };
                        { let s_ = (l_body_5).wrapping_add(112); let d_ = fp.wrapping_add(96); rt::memcpy(c, d_, s_, 24); };
                        F::ColBox_GetCenter(c, fp.wrapping_add(96), fp.wrapping_add(64));
                        F::ColBox_GetHalf(c, fp.wrapping_add(96), fp.wrapping_add(80));
                        F::Vec3_Scale(c, fp.wrapping_add(80), fp.wrapping_add(80), F32(0x400ccccc));
                        F::StgAabb_SetEmpty(c, fp.wrapping_add(96));
                        F::ColBox_SetCenterHalf(c, fp.wrapping_add(96), fp.wrapping_add(64), fp.wrapping_add(80));
                        if ((F::ColBox_ContainsPoint(c, fp.wrapping_add(96), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)))) != 0) {
                            { let v_ = ((((c.m.r32(l_view_4)) & (4278255615u32))) | (((2097152i32) as u32))); let a_ = l_view_4; c.m.w32(a_, v_); };
                        }
                    }
                }
                F::BtlObj_CalcViewColor(c, p_obj_1, l_view_4);
                if (((((((({ let a_ = (c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) && ((((!(((((((((((({ let a_ = (c.m.r32((l_state_2).wrapping_add(176))).wrapping_add(2); (rt::bf_get_u(c, a_, 3, 1) as u32) }) as i32)) != 0) || (((({ let a_ = (c.m.r32((l_state_2).wrapping_add(176))).wrapping_add(2); (rt::bf_get_u(c, a_, 4, 1) as u32) }) as i32)) != 0)) as i32)) != 0) || (((({ let a_ = (c.m.r32((l_state_2).wrapping_add(176))).wrapping_add(2); (rt::bf_get_u(c, a_, 5, 1) as u32) }) as i32)) != 0)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let a1_ = (if (((rt::feq({ let a_ = (c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) { F32(0x42200000) } else { { let a_ = (c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(16); F32(c.m.r32(a_)) } }); let a2_ = (if (((rt::feq({ let a_ = (c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) { F32(0x42f00000) } else { { let a_ = (c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(20); F32(c.m.r32(a_)) } }); F::BtlObjView_SetFade(c, l_view_4, a1_, a2_) };
                } else {
                    { let v_ = F32(0x00000000); let a_ = (l_view_4).wrapping_add(64); c.m.w32(a_, (v_).0); };
                    { let r_ = (((!(8i32))) as u32); let a_ = l_view_4; let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_BuildDrawList(c: &mut Ctx, mut p_list_1: u32, mut p_view_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_BuildDrawList", &PROBE_N, &[(p_list_1) as u32, (p_view_2) as u32], false); }
    let mut l_own_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_state_6: u32 = 0;
    let mut l_rec_7: u32 = 0;
    let mut l_pass_8: i32 = 0;
    let r_ = 'f: {
    l_own_3 = ((0i32) as u32);
    { let _ = F::memset(c, p_list_1, 0i32, 52u32); };
    {
        l_pass_8 = 0i32;
        'l1: loop {
            if !(((((l_pass_8) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_node_4 = { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetHead(c, a0_) };
                    'l3: loop {
                        if !(((((l_node_4) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_obj_5 = (l_node_4).wrapping_add(16);
                            l_state_6 = ((l_node_4).wrapping_add(16)).wrapping_add(2624);
                            if ((((((((((c.m.r32((l_obj_5).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) && ((((((c.m.r32((l_obj_5).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                                l_rec_7 = c.m.r32((l_state_6).wrapping_add(176));
                                if ((((c.m.r32(l_rec_7)) & (((2i32) as u32)))) != 0) {
                                    if (((((l_pass_8) == (0i32)) as i32)) != 0) {
                                        if ((((!(((((((((rt::bf_get_u(c, (l_rec_7).wrapping_add(2), 4, 1) as u32)) as i32)) != 0) || (((((rt::bf_get_u(c, (l_rec_7).wrapping_add(2), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                            { let v_ = l_obj_5; let a_ = (p_list_1).wrapping_add(((({ let a_ = (p_list_1).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                        }
                                    } else {
                                        if (((((l_pass_8) == (1i32)) as i32)) != 0) {
                                            if ((((c.m.r32(l_rec_7)) & (((1048576i32) as u32)))) != 0) {
                                                { let v_ = l_obj_5; let a_ = (p_list_1).wrapping_add(((({ let a_ = (p_list_1).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                            }
                                        } else {
                                            if ((((c.m.r32(l_rec_7)) & (((2097152i32) as u32)))) != 0) {
                                                if (((((((((p_view_2) == ((c.m.r32((l_obj_5).wrapping_add(16)) as i32))) as i32)) != 0) && ((((((c.m.r32(l_obj_5) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                                    l_own_3 = l_obj_5;
                                                } else {
                                                    { let v_ = l_obj_5; let a_ = (p_list_1).wrapping_add(((({ let a_ = (p_list_1).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l_node_4 = F::List_GetNext(c, l_node_4);
                    }
                }
            }
            { let o_ = l_pass_8; let n_ = (o_).wrapping_add(1); l_pass_8 = n_; };
        }
    }
    if (((((l_own_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_own_3; let a_ = (p_list_1).wrapping_add(((({ let a_ = (p_list_1).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_BuildDrawList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_UpdateAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateAll", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_node_1: u32 = 0;
    let mut l_obj_2: u32 = 0;
    let mut l_state_3: u32 = 0;
    let mut l_light_4: u32 = 0;
    let r_ = 'f: {
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            break 'f;
        }
        if ((F::BtlStage_IsReady(c)) != 0) {
            rt::memset(c, fp.wrapping_add(16), 0, 16);
            { let v_ = F32(0x41200000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x41200000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42800000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
            F::BtlStage_GetLightVecB(c, fp);
            l_light_4 = F::BtlStage_GetLightColors(c);
            if (((((l_light_4) != (((0i32) as u32))) as i32)) != 0) {
                { let a1_ = rt::i2f((c.m.r8((l_light_4).wrapping_add(((8i32) as u32)))) as i32); let a2_ = rt::i2f((c.m.r8((l_light_4).wrapping_add(((9i32) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((l_light_4).wrapping_add(((10i32) as u32)))) as i32); let a4_ = rt::i2f((c.m.r8((l_light_4).wrapping_add(((11i32) as u32)))) as i32); F::Vec4_Set(c, fp.wrapping_add(16), a1_, a2_, a3_, a4_) };
            }
            F::BtlObjLight_SetDir(c, fp);
            F::BtlObjLight_SetColor(c, fp.wrapping_add(16));
        }
        {
            l_node_1 = { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetHead(c, a0_) };
            'l1: loop {
                if !(((((l_node_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_obj_2 = (l_node_1).wrapping_add(16);
                    if ((((((((((c.m.r32((l_obj_2).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) && ((((((c.m.r32((l_obj_2).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        l_state_3 = ((l_node_1).wrapping_add(16)).wrapping_add(2624);
                        { let v_ = ((0i32) as u32); let a_ = (l_state_3).wrapping_add(4); c.m.w32(a_, v_); };
                        if ((((((c.m.r32((l_obj_2).wrapping_add(3184)) as i32)) == (0i32)) as i32)) != 0) {
                            F::BtlObj_SaveNodePositions(c, l_obj_2, 0i32);
                            F::BtlObjAnim_Step(c, l_obj_2);
                            F::BtlObjAnim_SamplePose(c, l_obj_2);
                            F::BtlObjAnim_UpdateEvents(c, l_obj_2);
                            F::BtlObjPose_CalcMatrices(c, l_obj_2);
                            F::BtlObj_UpdateChains(c, l_obj_2);
                            F::BtlObjPose_CalcMatrices(c, l_obj_2);
                        }
                        F::BtlObj_UpdateBounds(c, l_obj_2, l_state_3);
                        { let _ = F::BtlObj_UpdateFace(c, l_obj_2); };
                        { let _ = F::BtlObjFlash_Step(c, l_obj_2); };
                        { let _ = F::BtlObjFade_Step(c, l_obj_2); };
                        F::ObjShadow_Update(c, l_obj_2);
                        F::BtlObj_UpdateAlpha(c, l_obj_2);
                    }
                }
                l_node_1 = F::List_GetNext(c, l_node_1);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_UpdateVisibility(c: &mut Ctx, mut p_view_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateVisibility", &PROBE_N, &[(p_view_1) as u32], false); }
    let mut l_vis_2: u32 = 0;
    let mut l_node_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let mut l_state_5: u32 = 0;
    let mut l_idx_6: i32 = 0;
    let r_ = 'f: {
    l_vis_2 = F::BtlObjVis_Get(c);
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    l_idx_6 = (((p_view_1) == (1i32)) as i32);
    { let v_ = c.m.r32(sym::gBtlCamView); let a_ = ((l_vis_2).wrapping_add(12)).wrapping_add((((l_idx_6) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    {
        l_node_3 = { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetHead(c, a0_) };
        'l1: loop {
            if !(((((l_node_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_obj_4 = (l_node_3).wrapping_add(16);
                if ((((((((((c.m.r32((l_obj_4).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) && ((((((c.m.r32((l_obj_4).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                    l_state_5 = ((l_node_3).wrapping_add(16)).wrapping_add(2624);
                    { let v_ = ((l_state_5).wrapping_add(16)).wrapping_add((((l_idx_6) as u32)).wrapping_mul(80)); let a_ = (l_state_5).wrapping_add(176); c.m.w32(a_, v_); };
                    F::BtlObj_UpdateView(c, l_obj_4);
                }
            }
            l_node_3 = F::List_GetNext(c, l_node_3);
        }
    }
    { let a0_ = F::BtlObjVis_GetList(c, l_idx_6); F::BtlObj_BuildDrawList(c, a0_, l_idx_6) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateVisibility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_FinishVisibility(c: &mut Ctx, mut p_split_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FinishVisibility", &PROBE_N, &[(p_split_1) as u32], false); }
    let mut l_vis_2: u32 = 0;
    let mut l_node_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let mut l_state_5: u32 = 0;
    let mut l_rec_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
    l_vis_2 = F::BtlObjVis_Get(c);
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    { let v_ = ((0i32) as u32); let a_ = l_vis_2; c.m.w32(a_, v_); };
    { let v_ = (if (((((p_split_1) == (0i32)) as i32)) != 0) { 1i32 } else { 2i32 }); let a_ = (l_vis_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    {
        l_i_7 = 0i32;
        'l1: loop {
            if !(((((l_i_7) < ((c.m.r32((l_vis_2).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_node_3 = { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetHead(c, a0_) };
                    'l3: loop {
                        if !(((((l_node_3) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_obj_4 = (l_node_3).wrapping_add(16);
                            if ((((((((((c.m.r32((l_obj_4).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) && ((((((c.m.r32((l_obj_4).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                                l_state_5 = ((l_node_3).wrapping_add(16)).wrapping_add(2624);
                                l_rec_6 = ((l_state_5).wrapping_add(16)).wrapping_add((((l_i_7) as u32)).wrapping_mul(80));
                                { let r_ = c.m.r32(l_rec_6); let a_ = (l_state_5).wrapping_add(4); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                { let r_ = c.m.r32(l_rec_6); let a_ = l_vis_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                            }
                        }
                        l_node_3 = F::List_GetNext(c, l_node_3);
                    }
                }
            }
            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
        }
    }
    F::BtlObj_UploadAlphaTable(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FinishVisibility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_FillAlphaRow(c: &mut Ctx, mut p_table_1: u32, mut p_obj_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FillAlphaRow", &PROBE_N, &[(p_table_1) as u32, (p_obj_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_state_3: u32 = 0;
    let mut l_row_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_alpha_6: u8 = 0;
    let mut l_alpha_7: u8 = 0;
    let r_ = 'f: {
        l_state_3 = (p_obj_2).wrapping_add(2624);
        l_row_4 = ({ let a_ = (c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(40))).wrapping_add(40); (c.m.r32(a_) as i32) }).wrapping_mul(15i32);
        if ((((c.m.r32(l_state_3)) & (((524288i32) as u32)))) != 0) {
            {
                l_i_5 = 0i32;
                'l1: loop {
                    if !(((((l_i_5) < (15i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let v_ = ((255i32) as u8); let a_ = (p_table_1).wrapping_add((((l_row_4).wrapping_add(l_i_5)) as u32)); c.m.w8(a_, v_); };
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
        } else {
            if ((((c.m.r32((p_obj_2).wrapping_add(2624))) & (((262144i32) as u32)))) != 0) {
                { let _ = F::BtlObjFade_Get(c, p_obj_2, fp); };
                l_alpha_6 = ((rt::f2u(rt::fmul(F32(c.m.r32(fp)), F32(0x43000000)))) as u8);
                {
                    l_i_5 = 0i32;
                    'l3: loop {
                        if !(((((l_i_5) < (15i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let v_ = l_alpha_6; let a_ = (p_table_1).wrapping_add((((l_row_4).wrapping_add(l_i_5)) as u32)); c.m.w8(a_, v_); };
                        }
                        { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                    }
                }
            } else {
                if ((((c.m.r32((p_obj_2).wrapping_add(2624))) & (((4194304i32) as u32)))) != 0) {
                    l_alpha_7 = ((rt::f2u(F32(c.m.r32(((p_obj_2).wrapping_add(2624)).wrapping_add(220))))) as u8);
                    {
                        l_i_5 = 0i32;
                        'l5: loop {
                            if !(((((l_i_5) < (15i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                { let v_ = l_alpha_7; let a_ = (p_table_1).wrapping_add((((l_row_4).wrapping_add(l_i_5)) as u32)); c.m.w8(a_, v_); };
                            }
                            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                        }
                    }
                } else {
                    {
                        l_i_5 = 0i32;
                        'l7: loop {
                            if !(((((l_i_5) < (15i32)) as i32)) != 0) { break 'l7; }
                            'c8: {
                                { let v_ = c.m.r8(((l_state_3).wrapping_add(224)).wrapping_add(((l_i_5) as u32))); let a_ = (p_table_1).wrapping_add((((l_row_4).wrapping_add(l_i_5)) as u32)); c.m.w8(a_, v_); };
                            }
                            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                        }
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FillAlphaRow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_InitDraw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitDraw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::ObjOutline_Init(c);
    F::ObjGlow_Init(c);
    F::GfxAlphaKey_Init(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitDraw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_TermDraw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_TermDraw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::ObjOutline_Term(c);
    F::ObjGlow_Term(c);
    F::GfxAlphaKey_Term(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_TermDraw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_BeginDraw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_BeginDraw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Dma_AddScissor(c, 0i32, 511i32, 0i32, 447i32);
    F::GfxPost_CopyAlphaToDepth(c);
    F::GfxAlphaKey_Draw(c);
    F::ObjOutline_Draw(c);
    F::ObjGlow_Draw(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_BeginDraw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_UploadAlphaTable(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UploadAlphaTable", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_node_1: u32 = 0;
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 256u32); };
        {
            l_node_1 = { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetHead(c, a0_) };
            'l1: loop {
                if !(((((l_node_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_obj_2 = (l_node_1).wrapping_add(16);
                    if ((((((((((c.m.r32((l_obj_2).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) && ((((((c.m.r32((l_obj_2).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        F::BtlObj_FillAlphaRow(c, fp, l_obj_2);
                    }
                }
                l_node_1 = F::List_GetNext(c, l_node_1);
            }
        }
        F::ObjGlow_SetAlphaTable(c, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UploadAlphaTable", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_AllocFixed(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_AllocFixed", &PROBE_N, &[], false); }
    let mut l_slot_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_slot_1 = (F::BtlRes_GetWork(c)).wrapping_add(672);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!((((c.m.r32((l_slot_1).wrapping_add(48))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = ((1i32) as u32); let a_ = (l_slot_1).wrapping_add(48); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    break 'f l_slot_1;
                }
                { let o_ = l_slot_1; let n_ = (o_).wrapping_add(56); l_slot_1 = n_; };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_AllocFixed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_AllocSlot(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_AllocSlot", &PROBE_N, &[], false); }
    let mut l_slot_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_slot_1 = F::BtlRes_GetWork(c);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (12i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!((((c.m.r32((l_slot_1).wrapping_add(48))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = ((1i32) as u32); let a_ = (l_slot_1).wrapping_add(48); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    break 'f l_slot_1;
                }
                { let o_ = l_slot_1; let n_ = (o_).wrapping_add(56); l_slot_1 = n_; };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_AllocSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_FindSlot(c: &mut Ctx, mut p_number_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_FindSlot", &PROBE_N, &[(p_number_1) as u32], false); }
    let mut l_slot_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_slot_2 = F::BtlRes_GetWork(c);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (12i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_slot_2).wrapping_add(52)) as i32)) == (p_number_1)) as i32)) != 0) {
                    break 'f l_slot_2;
                }
                { let o_ = l_slot_2; let n_ = (o_).wrapping_add(56); l_slot_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_FindSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_CopyFile(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_CopyFile", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = c.m.r32(p_src_2); let a_ = p_dst_1; c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_src_2).wrapping_add(4)) as i32); let a_ = (p_dst_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_src_2).wrapping_add(8)) as i32); let a_ = (p_dst_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_CopyFile", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_SwapFile(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_SwapFile", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = c.m.r32(p_a_1); let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_a_1).wrapping_add(4)) as i32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((p_a_1).wrapping_add(8)) as i32); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::BtlRes_CopyFile(c, p_a_1, p_b_2);
        { let v_ = c.m.r32(fp); let a_ = p_b_2; c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((fp).wrapping_add(4)) as i32); let a_ = (p_b_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp).wrapping_add(8)) as i32); let a_ = (p_b_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_SwapFile", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_SetFileId(c: &mut Ctx, mut p_file_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_SetFileId", &PROBE_N, &[(p_file_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_id_2) >= (0i32)) as i32)) != 0) {
        { let v_ = p_id_2; let a_ = (p_file_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::File_GetSize(c, p_id_2); let a_ = (p_file_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((p_id_2) != ((99i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_file_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = (1i32).wrapping_neg(); let a_ = (p_file_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_SetFileId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_SetIds(c: &mut Ctx, mut p_slot_1: u32, mut p_model_2: i32, mut p_anim0_3: i32, mut p_anim1_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_SetIds", &PROBE_N, &[(p_slot_1) as u32, (p_model_2) as u32, (p_anim0_3) as u32, (p_anim1_4) as u32], false); }
    let r_ = 'f: {
    F::BtlRes_SetFileId(c, (p_slot_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_model_2);
    F::BtlRes_SetFileId(c, (p_slot_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_anim0_3);
    F::BtlRes_SetFileId(c, (p_slot_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_anim1_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_SetIds", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_FreeFile(c: &mut Ctx, mut p_file_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_FreeFile", &PROBE_N, &[(p_file_1) as u32], false); }
    let mut l_freed_2: i32 = 0;
    let r_ = 'f: {
    l_freed_2 = 0i32;
    if (((((c.m.r32(p_file_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(p_file_1); F::Heap_Free(c, a0_) };
        l_freed_2 = 1i32;
        { let v_ = ((0i32) as u32); let a_ = p_file_1; c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (p_file_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_file_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    break 'f l_freed_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_FreeFile", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_LoadFile(c: &mut Ctx, mut p_file_1: u32, mut p_fromHeap_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_LoadFile", &PROBE_N, &[(p_file_1) as u32, (p_fromHeap_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_fromHeap_2) == (1i32)) as i32)) != 0) {
        { let _ = F::BtlRes_FreeFile(c, p_file_1); };
        if ((((((c.m.r32((p_file_1).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
            { let v_ = { let a0_ = (c.m.r32((p_file_1).wrapping_add(8)) as i32); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32) }; let a_ = p_file_1; c.m.w32(a_, v_); };
        }
    } else {
        if ((((((c.m.r32((p_file_1).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
            { let v_ = { let a0_ = (c.m.r32((p_file_1).wrapping_add(8)) as i32); let a1_ = c.m.r32(p_file_1); let a2_ = (c.m.r32((p_file_1).wrapping_add(4)) as i32); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = p_file_1; c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_LoadFile", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_CommitReload(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_CommitReload", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    l_res_1 = F::BtlRes_GetWork(c);
    if ((((((c.m.r32((l_res_1).wrapping_add(836)) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = ({ let a0_ = (c.m.r32((l_res_1).wrapping_add(836)) as i32); F::BtlRes_GetSlot(c, a0_) }).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let a1_ = c.m.r32((l_res_1).wrapping_add(832)); F::BtlRes_SwapFile(c, a0_, a1_) };
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_res_1).wrapping_add(836); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_CommitReload", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_Init(c: &mut Ctx, mut p_prealloc_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Init", &PROBE_N, &[(p_prealloc_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let mut l_number_3: i32 = 0;
    let mut l_slot_4: u32 = 0;
    let mut l_spare_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_res_2 = F::BtlRes_GetWork(c);
    l_number_3 = 0i32;
    { let _ = F::memset(c, l_res_2, 0i32, 844u32); };
    { let v_ = p_prealloc_1; let a_ = (l_res_2).wrapping_add(840); c.m.w32(a_, (v_) as u32); };
    if (((((p_prealloc_1) != (0i32)) as i32)) != 0) {
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F::Heap_Alloc(c, 843776i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((l_res_2).wrapping_add(784)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)); c.m.w32(a_, v_); };
                    { let v_ = 0i32; let a_ = (((l_res_2).wrapping_add(784)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (1i32).wrapping_neg(); let a_ = (((l_res_2).wrapping_add(784)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = c.m.r32(((l_res_2).wrapping_add(784)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))); let a_ = (((l_res_2).wrapping_add(784)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, v_); };
                    { let _ = { let a0_ = c.m.r32(((l_res_2).wrapping_add(784)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))); F::memset(c, a0_, 0i32, ((843776i32) as u32)) }; };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        {
            l_i_6 = 0i32;
            'l3: loop {
                if !(((((l_i_6) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_slot_4 = ((l_res_2).wrapping_add(672)).wrapping_add((((l_i_6) as u32)).wrapping_mul(56));
                    l_spare_5 = ((l_res_2).wrapping_add(784)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16));
                    { let v_ = F::Heap_Alloc(c, 1443840i32, ((64i32) as u32), 0i32, 2i32); let a_ = (l_slot_4).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, v_); };
                    { let v_ = 0i32; let a_ = ((l_slot_4).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (1i32).wrapping_neg(); let a_ = ((l_slot_4).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = F::Heap_Alloc(c, 845824i32, ((64i32) as u32), 0i32, 2i32); let a_ = (l_slot_4).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, v_); };
                    { let v_ = 0i32; let a_ = ((l_slot_4).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (1i32).wrapping_neg(); let a_ = ((l_slot_4).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = { let a0_ = c.m.r32((l_slot_4).wrapping_add((((1i32) as u32)).wrapping_mul(16))); F::memset(c, a0_, 0i32, ((1443840i32) as u32)) }; };
                    { let _ = { let a0_ = c.m.r32((l_slot_4).wrapping_add((((2i32) as u32)).wrapping_mul(16))); F::memset(c, a0_, 0i32, ((845824i32) as u32)) }; };
                    F::BtlRes_CopyFile(c, (l_slot_4).wrapping_add((((0i32) as u32)).wrapping_mul(16)), l_spare_5);
                    { let v_ = { let o_ = l_number_3; let n_ = (o_).wrapping_add(1); l_number_3 = n_; o_ }; let a_ = (l_slot_4).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        { let v_ = ((l_res_2).wrapping_add(784)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); let a_ = (l_res_2).wrapping_add(832); c.m.w32(a_, v_); };
    }
    {
        l_i_6 = 0i32;
        'l5: loop {
            if !(((((l_i_6) < (12i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                l_slot_4 = (l_res_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(56));
                { let v_ = { let o_ = l_number_3; let n_ = (o_).wrapping_add(1); l_number_3 = n_; o_ }; let a_ = (l_slot_4).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    { let v_ = (1i32).wrapping_neg(); let a_ = (l_res_2).wrapping_add(836); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Term", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let mut l_slot_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_res_1 = F::BtlRes_GetWork(c);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (12i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(((l_res_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(56))).wrapping_add(52)) as i32); F::BtlRes_Free(c, a0_) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    l_slot_2 = (l_res_1).wrapping_add(672);
    {
        l_i_3 = 0i32;
        'l3: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                if (((((c.m.r32((l_slot_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = c.m.r32((l_slot_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))); F::Heap_Free(c, a0_) };
                }
                if (((((c.m.r32((l_slot_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = c.m.r32((l_slot_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))); F::Heap_Free(c, a0_) };
                }
                { let o_ = l_slot_2; let n_ = (o_).wrapping_add(56); l_slot_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    {
        l_i_3 = 0i32;
        'l5: loop {
            if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                if (((((c.m.r32((((l_res_1).wrapping_add(784)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = c.m.r32((((l_res_1).wrapping_add(784)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(12)); F::Heap_Free(c, a0_) };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_Request(c: &mut Ctx, mut p_fixed_1: i32, mut p_model_2: i32, mut p_anim0_3: i32, mut p_anim1_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Request", &PROBE_N, &[(p_fixed_1) as u32, (p_model_2) as u32, (p_anim0_3) as u32, (p_anim1_4) as u32], false); }
    let mut l_slot_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_fixed_1) == (1i32)) as i32)) != 0) {
        l_slot_5 = F::BtlRes_AllocFixed(c);
        if (((((l_slot_5) == (((0i32) as u32))) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        F::BtlRes_SetIds(c, l_slot_5, p_model_2, p_anim0_3, p_anim1_4);
        F::BtlRes_LoadFile(c, (l_slot_5).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 0i32);
        F::BtlRes_LoadFile(c, (l_slot_5).wrapping_add((((1i32) as u32)).wrapping_mul(16)), 0i32);
        F::BtlRes_LoadFile(c, (l_slot_5).wrapping_add((((2i32) as u32)).wrapping_mul(16)), 0i32);
    } else {
        l_slot_5 = F::BtlRes_AllocSlot(c);
        if (((((l_slot_5) == (((0i32) as u32))) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        F::BtlRes_SetIds(c, l_slot_5, p_model_2, p_anim0_3, p_anim1_4);
        F::BtlRes_LoadFile(c, (l_slot_5).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 1i32);
        F::BtlRes_LoadFile(c, (l_slot_5).wrapping_add((((1i32) as u32)).wrapping_mul(16)), 1i32);
        F::BtlRes_LoadFile(c, (l_slot_5).wrapping_add((((2i32) as u32)).wrapping_mul(16)), 1i32);
        { let r_ = ((2i32) as u32); let a_ = (l_slot_5).wrapping_add(48); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    break 'f (c.m.r32((l_slot_5).wrapping_add(52)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Request", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_Free(c: &mut Ctx, mut p_number_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Free", &PROBE_N, &[(p_number_1) as u32], false); }
    let mut l_slot_2: u32 = 0;
    let r_ = 'f: {
    if ((((((((({ let a_ = (F::BtlRes_GetWork(c)).wrapping_add(840); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && (((((p_number_1) < (2i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    l_slot_2 = F::BtlRes_FindSlot(c, p_number_1);
    if (((((l_slot_2) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = F::BtlRes_FreeFile(c, (l_slot_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))); };
        { let _ = F::BtlRes_FreeFile(c, (l_slot_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))); };
        { let _ = F::BtlRes_FreeFile(c, (l_slot_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))); };
        { let v_ = ((0i32) as u32); let a_ = (l_slot_2).wrapping_add(48); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Free", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_GetSlot(c: &mut Ctx, mut p_number_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_GetSlot", &PROBE_N, &[(p_number_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let mut l_slot_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_res_2 = F::BtlRes_GetWork(c);
    if ((((((c.m.r32((l_res_2).wrapping_add(840)) as i32)) != (0i32)) as i32)) != 0) {
        l_slot_3 = (l_res_2).wrapping_add(672);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32((l_slot_3).wrapping_add(52)) as i32)) == (p_number_1)) as i32)) != 0) {
                        break 'f l_slot_3;
                    }
                    { let o_ = l_slot_3; let n_ = (o_).wrapping_add(56); l_slot_3 = n_; };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
    l_slot_3 = l_res_2;
    {
        l_i_4 = 0i32;
        'l3: loop {
            if !(((((l_i_4) < (12i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                if ((((((c.m.r32((l_slot_3).wrapping_add(52)) as i32)) == (p_number_1)) as i32)) != 0) {
                    break 'f l_slot_3;
                }
                { let o_ = l_slot_3; let n_ = (o_).wrapping_add(56); l_slot_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_GetSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_Reload(c: &mut Ctx, mut p_number_1: i32, mut p_model_2: i32, mut p_anim0_3: i32, mut p_anim1_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Reload", &PROBE_N, &[(p_number_1) as u32, (p_model_2) as u32, (p_anim0_3) as u32, (p_anim1_4) as u32], false); }
    let mut l_res_5: u32 = 0;
    let mut l_slot_6: u32 = 0;
    let mut l_modelOnly_7: i32 = 0;
    let r_ = 'f: {
    l_res_5 = F::BtlRes_GetWork(c);
    l_modelOnly_7 = 0i32;
    if ((((((((((c.m.r32((l_res_5).wrapping_add(836)) as i32)) < (0i32)) as i32)) != 0) && (((((p_number_1) < (2i32)) as i32)) != 0)) as i32)) != 0) {
        l_slot_6 = F::BtlRes_GetSlot(c, p_number_1);
        if (((((l_slot_6) != (((0i32) as u32))) as i32)) != 0) {
            if (((((((((p_model_2) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((p_anim0_3) == ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                l_modelOnly_7 = (((p_anim1_4) == (p_anim0_3)) as i32);
            }
            if ((((!((l_modelOnly_7) != 0)) as i32)) != 0) {
                F::BtlRes_SetIds(c, l_slot_6, (99i32).wrapping_neg(), p_anim0_3, p_anim1_4);
                { let a0_ = c.m.r32((l_res_5).wrapping_add(832)); F::BtlRes_SetFileId(c, a0_, p_model_2) };
                { let a0_ = c.m.r32((l_res_5).wrapping_add(832)); F::BtlRes_LoadFile(c, a0_, 0i32) };
                F::BtlRes_LoadFile(c, (l_slot_6).wrapping_add((((1i32) as u32)).wrapping_mul(16)), 0i32);
                F::BtlRes_LoadFile(c, (l_slot_6).wrapping_add((((2i32) as u32)).wrapping_mul(16)), 0i32);
            } else {
                { let a0_ = c.m.r32((l_res_5).wrapping_add(832)); F::BtlRes_SetFileId(c, a0_, p_model_2) };
                { let a0_ = c.m.r32((l_res_5).wrapping_add(832)); F::BtlRes_LoadFile(c, a0_, 0i32) };
            }
            { let v_ = p_number_1; let a_ = (l_res_5).wrapping_add(836); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_Reload", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_CommitReload2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_CommitReload2", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    l_res_1 = F::BtlRes_GetWork(c);
    if ((((((c.m.r32((l_res_1).wrapping_add(836)) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = ({ let a0_ = (c.m.r32((l_res_1).wrapping_add(836)) as i32); F::BtlRes_GetSlot(c, a0_) }).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let a1_ = c.m.r32((l_res_1).wrapping_add(832)); F::BtlRes_SwapFile(c, a0_, a1_) };
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_res_1).wrapping_add(836); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_CommitReload2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_LoadSingle(c: &mut Ctx, mut p_fromHeap_1: i32, mut p_slot_2: u32, mut p_model_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_LoadSingle", &PROBE_N, &[(p_fromHeap_1) as u32, (p_slot_2) as u32, (p_model_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_model_3) > (0i32)) as i32)) != 0) {
        if (((((p_fromHeap_1) != (0i32)) as i32)) != 0) {
            { let _ = F::memset(c, p_slot_2, 0i32, 48u32); };
        } else {
            { let v_ = ((0i32) as u32); let a_ = (p_slot_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (p_slot_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, v_); };
        }
        F::BtlRes_SetIds(c, p_slot_2, p_model_3, (1i32).wrapping_neg(), (1i32).wrapping_neg());
        F::BtlRes_LoadFile(c, (p_slot_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_fromHeap_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_LoadSingle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlRes_FreeSingle(c: &mut Ctx, mut p_slot_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_FreeSingle", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlRes_FreeFile(c, (p_slot_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))); };
    { let _ = F::BtlRes_FreeFile(c, (p_slot_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))); };
    { let _ = F::BtlRes_FreeFile(c, (p_slot_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))); };
    { let _ = F::memset(c, p_slot_1, 0i32, 48u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlRes_FreeSingle", &PROBE_N, &[], true); }
    r_
}
