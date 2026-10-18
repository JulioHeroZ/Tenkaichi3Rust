#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn TourBg_Init(c: &mut Ctx, mut p_file_1: u32, mut p_kind_2: u32, mut p_tex_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourBg_Init", &PROBE_N, &[(p_file_1) as u32, (p_kind_2) as u32, (p_tex_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((104u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gTourBg; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gTourBg); F::memset(c, a0_, 0i32, 104u32) }; };
        { let v_ = F::Sprite_Unpack(c, p_file_1, ((0i32) as u32), ((0i32) as u32)); let a_ = c.m.r32(sym::gTourBg); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = c.m.r32(sym::gTourBg); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = c.m.r32(sym::gTourBg); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gTourBg)).wrapping_add(48); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = c.m.r32(sym::gTourBg); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = c.m.r32(sym::gTourBg); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        's1: {
            let sv4: u32 = p_kind_2;
            match sv4 {
                0u32 | 2u32 | 4u32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                1u32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                3u32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((p_tex_3) != (((0i32) as u32))) as i32)) != 0) {
            's2: {
                let sv5: u32 = p_kind_2;
                match sv5 {
                    1u32 => {
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        break 's2;
                    }
                    0u32 => {
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        break 's2;
                    }
                    2u32 => {
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        break 's2;
                    }
                    3u32 => {
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        break 's2;
                    }
                    4u32 => {
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = c.m.r32((p_tex_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = ((c.m.r32(sym::gTourBg)).wrapping_add(52)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        }
        { let a0_ = ((c.m.r32(sym::gTourBg)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = c.m.r32(sym::gTourBg); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = c.m.r32(sym::gTourBg); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gTourBg)).wrapping_add(52); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTourBg)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ((p_kind_2) as i32); let a_ = (c.m.r32(sym::gTourBg)).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TourBg_Init", &PROBE_N, &[], true); }
    r_
}

pub fn TourBg_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourBg_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTourBg)).wrapping_add(4)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = c.m.r32(sym::gTourBg); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = c.m.r32(sym::gTourBg); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gTourBg); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gTourBg)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gTourBg); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gTourBg; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TourBg_Term", &PROBE_N, &[], true); }
    r_
}

pub fn TourBg_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourBg_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gTourBg)).wrapping_add(4)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = { let a_ = (c.m.r32(sym::gTourBg)).wrapping_add(48); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gTourBg)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_k_d__str0, fp);
        's3: {
            let sv3: i32 = { let a_ = (c.m.r32(sym::gTourBg)).wrapping_add(96); (c.m.r32(a_) as i32) };
            match sv3 {
                0i32 => {
                    { let a3_ = (c.m.r32(sym::gTourBg)).wrapping_add(100); F::FlashAnim_Scroll(c, l_flash_1, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), F32(0xbe11a2b3), F32(0x00000000)) };
                    break 's3;
                }
                2i32 => {
                    { let a3_ = (c.m.r32(sym::gTourBg)).wrapping_add(100); F::FlashAnim_Scroll(c, l_flash_1, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), F32(0xbe2ec33e), F32(0x00000000)) };
                    break 's3;
                }
                4i32 => {
                    { let a3_ = (c.m.r32(sym::gTourBg)).wrapping_add(100); F::FlashAnim_Scroll(c, l_flash_1, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), F32(0x3e1ee009), F32(0x00000000)) };
                    break 's3;
                }
                _ => {
                }
            }
        }
        { let a0_ = ((c.m.r32(sym::gTourBg)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TourBg_Draw", &PROBE_N, &[], true); }
    r_
}
