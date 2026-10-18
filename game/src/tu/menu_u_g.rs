#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EvoZ_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_box_3: u32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((15016u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gEvoZ; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEvoZ); F::memset(c, a0_, 0i32, 15016u32) }; };
        { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_Load(c, a0_, p_section_1) };
        { let a0_ = (c.m.r32(sym::gEvoZ)).wrapping_add(800); let a1_ = (c.m.r32(sym::gEvoZ)).wrapping_add(804); let a2_ = (c.m.r32(sym::gEvoZ)).wrapping_add(792); let a3_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(796); c.m.r32(a_) }; F::ChrGrid_Build(c, a0_, a1_, a2_, a3_, ((0i32) as u32), ((0i32) as u32)) };
        { let v_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(800); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(792); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(sym::gEvoZ)).wrapping_add(804); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(796); c.m.w32(a_, v_); };
        { let v_ = rt::div_i32({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(792); (c.m.r32(a_) as i32) }, 7i32); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6744); c.m.w32(a_, (v_) as u32); };
        if (((((rt::rem_i32({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(792); (c.m.r32(a_) as i32) }, 7i32)) != (0i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6744); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32(sym::gEvoZ)).wrapping_add(672); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.w32(a_, v_); };
        { let s_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); let d_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; rt::memcpy(c, d_, s_, 48); };
        if ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(796); c.m.r32(a_) }; let a1_ = ({ let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrGrid_IsSelectable(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; F::memset(c, a0_, 0i32, 48u32) }; };
            { let v_ = 0i32; let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
            { let s_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; let d_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); rt::memcpy(c, d_, s_, 48); };
        }
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
        { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_SetRowChips(c, a0_) };
        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
        { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_RefreshStatus(c, a0_) };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (350i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((((((((!(((({ let a_ = (({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12488); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(20); (c.m.r32(a_) as i32) }) & (5i32))) != 0)) as i32)) != 0) && ((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12488); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)) < (3i32)) as i32)) != 0)) as i32)) != 0) && (((({ let a_ = (({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12488); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(20); (c.m.r32(a_) as i32) }) & (64i32))) != 0)) as i32)) != 0) {
                        if ((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12488); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)) != (2i32)) as i32)) != 0) {
                            { let v_ = l_i_2; let a_ = ((((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                        { let v_ = l_i_2; let a_ = ((((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(48)).wrapping_add((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12488); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(32)).wrapping_add((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12488); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(32)).wrapping_add((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12488); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 6i32; let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x40c51eb8); let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(24); c.m.w32(a_, (v_).0); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(5648)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (({ let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(20); (c.m.r32(a_) as i32) })).wrapping_sub(1i32); let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(6808)).wrapping_add(5664)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(8); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(180)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        {
            l_i_2 = 0i32;
            'l5: loop {
                if !(((((l_i_2) < (18i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_box_3 = ((c.m.r32(sym::gEvoZ)).wrapping_add(12492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140));
                    if (((((l_i_2) <= (0i32)) as i32)) != 0) {
                        { let a1_ = { let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, l_box_3, a1_, ((1i32) as u32)) };
                    } else {
                        if (((((l_i_2) < (2i32)) as i32)) != 0) {
                            { let a1_ = { let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, l_box_3, a1_, ((3i32) as u32)) };
                        } else {
                            if (((((l_i_2) < (3i32)) as i32)) != 0) {
                                { let a1_ = { let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, l_box_3, a1_, ((6i32) as u32)) };
                                F::TextBox_SetUnk80(c, l_box_3, 1i32);
                            } else {
                                if (((((l_i_2) < (10i32)) as i32)) != 0) {
                                    { let a1_ = { let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, l_box_3, a1_, ((6i32) as u32)) };
                                    F::TextBox_SetUnk80(c, l_box_3, 1i32);
                                    F::TextBox_SetRect(c, l_box_3, 0i32, 512i32, 198i32, 408i32);
                                } else {
                                    { let a1_ = { let a_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, l_box_3, a1_, ((6i32) as u32)) };
                                    F::TextBox_SetUnk80(c, l_box_3, 1i32);
                                }
                            }
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::PassWin_Term(c);
    F::Dialog_Term(c);
    F::ItemHelp_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(20); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(20); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(20); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(16); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(16); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gEvoZ)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gEvoZ); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEvoZ; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Draw", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(176); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
    { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_Refresh(c, a0_) };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((l_i_1) != (1i32)) as i32)) != 0) || ((((!(((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::Font_FlushAll(c);
    F::PassWin_Draw(c);
    { let a0_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(620); (c.m.r32(a_) as i32) }; F::ItemHelp_Draw(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_1: i32 = 0;
    let mut l_base_2: i32 = 0;
    let r_ = 'f: {
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((((l_i_1) != (1i32)) as i32)) != 0) || ((((!(((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let a0_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        if ((((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(112); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
        }
        if (((((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (128i32))) != 0) && (((({ let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
            { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_ToggleList(c, a0_) };
        }
        if ((((((((!(((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            { let r_ = 256i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(624); (c.m.r32(a_) as i32) }) > (33i32)) as i32)) != 0) {
                { let r_ = 33i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(624); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                { let r_ = 33i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                { let r_ = 33i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(48); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let v_ = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(48); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6764); c.m.w32(a_, (v_) as u32); };
            } else {
                { let r_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(624); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                { let r_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(624); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(48); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let v_ = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(48); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6764); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; F::ChrTbl_GetMaxExp(c, a0_) }).wrapping_sub({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6764); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6772); c.m.w32(a_, (v_) as u32); };
                { let r_ = 32i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
            }
            if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6764); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6768); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                { let a0_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_u_g__str0, fp) };
                { let a0_ = ((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipGotoLabel(c, a0_, fp, sym::menu_u_g__str1) };
                { let r_ = 64i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(52); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
                { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_RefreshStatus(c, a0_) };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 47i32); };
            } else {
                l_base_2 = { let a0_ = c.m.r32(sym::gEvoZ); let a1_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = { let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(788); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::EvoZ_GetLevelExp(c, a0_, a1_, a2_) };
                { let v_ = rt::fmul(rt::fdiv(rt::i2f(({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6764); (c.m.r32(a_) as i32) }).wrapping_sub(l_base_2)), rt::i2f(({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6768); (c.m.r32(a_) as i32) }).wrapping_sub(l_base_2))), F32(0x40d51eb8)); let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(6796); c.m.w32(a_, (v_).0); };
            }
            if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(15012); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 5i32); };
                { let v_ = 10i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(15012); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (64i32))) != 0) && (((({ let a_ = (((c.m.r32(sym::gEvoZ)).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((8i32) as u32)))) != 0)) as i32)) != 0) {
            { let r_ = 64i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            if (((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                { let v_ = 6i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::EvoZ_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_UpdateFace(c, a0_) };
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::EvoZ_Update(c);
            }
            F::EvoZ_Draw(c);
            { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_UpdateDialog(c, a0_) };
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            { let _ = F::ColorFade_IsInDone(c); };
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(612); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                    continue 'l1;
                }
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(604); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((((((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(644); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((!(((({ let a_ = (c.m.r32(sym::gEvoZ)).wrapping_add(556); (c.m.r32(a_) as i32) }) & (128i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = c.m.r32(sym::gEvoZ); F::EvoZ_Input(c, a0_, fp) };
                }
            }
        }
        F::EvoZ_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Run", &PROBE_N, &[], true); }
    r_
}
