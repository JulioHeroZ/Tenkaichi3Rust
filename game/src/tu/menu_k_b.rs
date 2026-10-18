#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Bracket_OnSaveDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_OnSaveDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = 2i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 15i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6708); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(744); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_OnSaveDone", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Init", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((8336u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBracket; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gBracket); F::memset(c, a0_, 0i32, 8336u32) }; };
    { let a0_ = c.m.r32(sym::gBracket); F::Bracket_Load(c, a0_) };
    { let _ = 1i32; F::McFlow_Init(c) };
    F::McFlow_SetDoneCb(c, 0i32, fid::Bracket_OnSaveDone, ((((((0i32) as u32)) as u64)) as i32));
    F::McFlow_SetDoneCb(c, 1i32, fid::Bracket_OnSaveDone, ((((((0i32) as u32)) as u64)) as i32));
    { let a0_ = (c.m.r32(sym::gBracket)).wrapping_add(764); let a1_ = (c.m.r32(sym::gBracket)).wrapping_add(768); let a2_ = (c.m.r32(sym::gBracket)).wrapping_add(756); let a3_ = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(760); c.m.r32(a_) }; F::ChrGrid_Build(c, a0_, a1_, a2_, a3_, ((0i32) as u32), ((0i32) as u32)) };
    { let v_ = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(764); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(756); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(sym::gBracket)).wrapping_add(768); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(760); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let s_ = (c.m.r32(sym::gProgress)).wrapping_add(152); let d_ = (c.m.r32(sym::gBracket)).wrapping_add(6760); rt::memcpy(c, d_, s_, 680); };
    if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(832); F::memset(c, a0_, 0i32, 256u32) }; };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
        { let a0_ = c.m.r32(sym::gBracket); F::Bracket_FillEntrants(c, a0_) };
        { let a0_ = c.m.r32(sym::gBracket); F::Bracket_ShuffleEntrants(c, a0_) };
        { let a0_ = c.m.r32(sym::gBracket); F::Bracket_InitMatches(c, a0_) };
        F::StreamSe_PlayDefault(c, 0i32, 68554i32);
        { let r_ = 524288i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(144); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6724); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(148); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); c.m.w32(a_, (v_) as u32); };
        { let s_ = (c.m.r32(sym::gProgress)).wrapping_add(832); let d_ = (c.m.r32(sym::gBracket)).wrapping_add(7440); rt::memcpy(c, d_, s_, 256); };
        { let a0_ = c.m.r32(sym::gBracket); F::Bracket_LoadImages(c, a0_) };
        { let a0_ = c.m.r32(sym::gBracket); F::Bracket_ApplyResult(c, a0_) };
        { let a0_ = c.m.r32(sym::gBracket); F::Bracket_PlayCpuMatches(c, a0_) };
        's1: {
            let sv2: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) };
            match sv2 {
                0i32 => {
                    F::Bgm_Play(c, 68393i32);
                    break 's1;
                }
                2i32 => {
                    F::Bgm_Play(c, 68388i32);
                    break 's1;
                }
                1i32 | 3i32 => {
                    F::Bgm_Play(c, 68376i32);
                    break 's1;
                }
                4i32 => {
                    F::Bgm_Play(c, 68376i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    'l2: while ((1i32) != 0) {
        if (((((((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) || (((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'l2;
        }
        { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    { let a0_ = c.m.r32(sym::gBracket); F::Bracket_SetChipTex(c, a0_) };
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(736); c.m.w32(a_, (v_) as u32); };
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(7696)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(36); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, (((l_i_1).wrapping_add(1i32)) as u32)) };
                { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(7976)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(40); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, (((l_i_1).wrapping_add(3i32)) as u32)) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::GetWin_Term(c);
    F::Dialog_Term(c);
    F::MsgWin_Term(c);
    F::McFlow_Term(c);
    F::TourBg_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(44); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(44); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = c.m.r32(sym::gBracket); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = c.m.r32(sym::gBracket); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gBracket); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gBracket)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gBracket); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gBracket; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::TourBg_Draw(c);
    { let a0_ = c.m.r32(sym::gBracket); F::Bracket_SetupDraw(c, a0_) };
    { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
    { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
    { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
    if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (262144i32))) != 0) {
        { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a2_ = (if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (8i32))) != 0) { (1i32).wrapping_neg() } else { { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(736); (c.m.r32(a_) as i32) } }); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
    } else {
        { let a2_ = (if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (8i32))) != 0) { (1i32).wrapping_neg() } else { { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(736); (c.m.r32(a_) as i32) } }); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
    }
    F::GetWin_Draw(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_frames_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (524288i32))) != 0) {
        if (((((F::StreamSe_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            's3: {
                let sv3: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) };
                match sv3 {
                    0i32 => {
                        F::Bgm_Play(c, 68393i32);
                        break 's3;
                    }
                    2i32 => {
                        F::Bgm_Play(c, 68388i32);
                        break 's3;
                    }
                    1i32 | 3i32 => {
                        F::Bgm_Play(c, 68376i32);
                        break 's3;
                    }
                    4i32 => {
                        F::Bgm_Play(c, 68376i32);
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
            { let r_ = (!(524288i32)); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (1024i32))) != 0) {
        { let r_ = rt::i2f({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6736); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((((rt::f2i({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); F32(c.m.r32(a_)) })) >= (512i32)) as i32)) != 0) {
            { let v_ = F32(0x44000000); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); c.m.w32(a_, (v_).0); };
            { let r_ = 1024i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (((4i32) | (32i32))))) != 0)) as i32)) != 0) {
        if (((({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            if (((({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                    { let a0_ = c.m.r32(sym::gBracket); F::Bracket_StartWin(c, a0_) };
                } else {
                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
    if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
        if (((({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            if (((({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(16); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 58i32); };
            }
            if (((({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                { let r_ = 64i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6716); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_Reset(c, a0_, 1i32) };
                { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
                { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_k_b__str0, ((((1i32) as i64 as u64)) as u8)) };
                F::MsgWin_Close(c);
                { let r_ = 1024i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                l_frames_2 = 30i32;
                { let v_ = rt::div_i32((512i32).wrapping_sub(rt::f2i({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); F32(c.m.r32(a_)) })), l_frames_2); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6736); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
        if (((({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
            { let r_ = 32i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 4i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 2i32; let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(740); (c.m.r32(a_) as i32) }) ^ (1i32))) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
            { let r_ = (!(8i32)); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
            { let r_ = (!(8i32)); let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
            { let a0_ = ((c.m.r32(sym::gBracket)).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_SetFlag(c, a0_, ((4i32) as u32), ((1i32) as u8)) };
            { let v_ = (({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.r16(a_) }) as i32); let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(740); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (16384i32))) != 0) {
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 4i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                if ((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(128); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && (((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); c.m.r16(a_) }) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                    { let r_ = 4096i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 351i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6716); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 201i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6716); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if (((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); c.m.r16(a_) }) as i32)) & (16i32))) != 0) {
                    if (((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(740); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
                        { let r_ = 4i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(128); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let v_ = 251i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6716); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 301i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6716); c.m.w32(a_, (v_) as u32); };
                        }
                        { let r_ = 128i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        break 'f;
                    }
                } else {
                    if (((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); c.m.r16(a_) }) as i32)) & (4i32))) != 0) {
                        { let v_ = { let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(740); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(2)); c.m.r16(a_) }; let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.r16(a_) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    } else {
                        if (((((({ let a_ = ((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); c.m.r16(a_) }) as i32)) & (8i32))) != 0) {
                            { let v_ = { let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(740); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(2)); c.m.r16(a_) }; let a_ = ((((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gBracket)).wrapping_add(7440)).wrapping_add(((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.r16(a_) }) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                        }
                    }
                }
                { let a0_ = c.m.r32(sym::gBracket); F::Bracket_NextMatch(c, a0_) };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((1i32) as u32)))) != 0) {
            if (((rt::flt({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
                { let r_ = F32(0x40400000); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fgt({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); c.m.w32(a_, (v_).0); };
                }
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((2i32) as u32)))) != 0) {
                if (((rt::fgt({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); F32(c.m.r32(a_)) }, F32(0xc4000000)) as i32)) != 0) {
                    { let r_ = F32(0x40400000); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::flt({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); F32(c.m.r32(a_)) }, F32(0xc4000000)) as i32)) != 0) {
                        { let v_ = F32(0xc4000000); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6740); c.m.w32(a_, (v_).0); };
                    }
                }
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    { let a0_ = c.m.r32(sym::gBracket); F::Bracket_StartVs(c, a0_) };
                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Input", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_Run(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Run", &PROBE_N, &[], false); }
    let mut l_result_1: i32 = 0;
    let r_ = 'f: {
    l_result_1 = 1i32;
    F::Bracket_Init(c);
    { let v_ = l_result_1; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(744); c.m.w32(a_, (v_) as u32); };
    F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
    'l1: while ((1i32) != 0) {
        F::Gfx_BeginFrame(c);
        { let a0_ = c.m.r32(sym::gBracket); F::Bracket_UpdateImages(c, a0_) };
        F::Pad_Update(c);
        F::Snd_Update(c);
        F::ColorFade_Update(c);
        if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
            { let a0_ = c.m.r32(sym::gBracket); F::Bracket_UpdateSeq(c, a0_) };
            F::Bracket_Update(c);
        }
        F::Bracket_Draw(c);
        { let v_ = F::McFlow_Update(c); let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6732); c.m.w32(a_, (v_) as u32); };
        F::ColorFade_Draw(c);
        F::Gfx_EndFrame(c, 1i32);
        F::Dma_Flush(c);
        F::File_Stub264D90(c);
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            continue 'l1;
        }
        if ((F::ColorFade_IsInDone(c)) != 0) {
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6712); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6712); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 51i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6716); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6712); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6712); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 151i32; let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6716); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
        if ((F::ColorFade_IsFadingOut(c)) != 0) {
            F::Voice_FadeOutStep(c, 0i32);
            F::Bgm_FadeOutStep(c);
            continue 'l1;
        }
        if ((F::ColorFade_IsOutDone(c)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6720); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                continue 'l1;
            }
            break 'l1;
        }
        if (((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(732); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6708); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
            }
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6732); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                { let a0_ = (c.m.r32(sym::gBracket)).wrapping_add(744); F::Bracket_Input(c, a0_) };
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gBracket)).wrapping_add(744); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6724); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(6728); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
        { let s_ = (c.m.r32(sym::gBracket)).wrapping_add(6760); let d_ = (c.m.r32(sym::gProgress)).wrapping_add(152); rt::memcpy(c, d_, s_, 680); };
        { let s_ = (c.m.r32(sym::gBracket)).wrapping_add(7440); let d_ = (c.m.r32(sym::gProgress)).wrapping_add(832); rt::memcpy(c, d_, s_, 256); };
        { let a0_ = c.m.r32(sym::gBracket); F::Bracket_SetupBattle(c, a0_) };
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(152); F::memset(c, a0_, 0i32, 680u32) }; };
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(832); F::memset(c, a0_, 0i32, 256u32) }; };
        { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    l_result_1 = { let a_ = (c.m.r32(sym::gBracket)).wrapping_add(744); (c.m.r32(a_) as i32) };
    F::Bracket_Term(c);
    F::Dma_ResetBuffers(c);
    break 'f l_result_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Run", &PROBE_N, &[], true); }
    r_
}
