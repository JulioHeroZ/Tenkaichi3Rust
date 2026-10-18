#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Snd_RpcCall(c: &mut Ctx, mut p_cmd_1: i32, mut p_data_2: u32, mut p_size_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCall", &PROBE_N, &[(p_cmd_1) as u32, (p_data_2) as u32, (p_size_3) as u32], false); }
    let r_ = 'f: {
    F::FlushCache(c, 0i32);
    { let _ = { let a0_ = (c.m.r32(sym::gSndRpc)).wrapping_add(64); let a5_ = c.m.r32(sym::gSndRpc); F::sceSifCallRpc(c, a0_, p_cmd_1, 0i32, p_data_2, p_size_3, a5_, 64i32, ((0i32) as u32), ((0i32) as u32)) }; };
    break 'f { let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCall", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcIsBusy(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcIsBusy", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (c.m.r32(sym::gSndRpc)).wrapping_add(64); F::func_002B4AF0(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcIsBusy", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcInit(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcInit", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((492u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSndRpc; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gSndRpc); F::memset(c, a0_, 0i32, 492u32) }; };
    F::sceSifInitRpc(c, ((((0i32) as i64 as u64)) as u32));
    'l1: while ((1i32) != 0) {
        { let _ = { let a0_ = (c.m.r32(sym::gSndRpc)).wrapping_add(64); F::sceSifBindRpc(c, a0_, 33554436i32, 0i32) }; };
        if ((((({ let a_ = ((c.m.r32(sym::gSndRpc)).wrapping_add(64)).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            break 'l1;
        }
        l_i_1 = 10000i32;
        'l2: while (({ let o_ = l_i_1; let n_ = (o_).wrapping_sub(1); l_i_1 = n_; o_ }) != 0) {
        }
    }
    { let _ = F::Snd_RpcCall(c, 0i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcInit", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcTerm(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcTerm", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::Snd_RpcCall(c, 1i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcTerm", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcReset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcReset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); F::memset(c, a0_, 0i32, 388u32) }; };
    { let _ = F::Snd_RpcCall(c, 2i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcReset", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcSetBank(c: &mut Ctx, mut p_mask_1: i32, mut p_spuAddr_2: i32, mut p_hdIop_3: u32, mut p_seqIop_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetBank", &PROBE_N, &[(p_mask_1) as u32, (p_spuAddr_2) as u32, (p_hdIop_3) as u32, (p_seqIop_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_mask_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_spuAddr_2; let a_ = (sym::gSndRpcBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_hdIop_3) as i32); let a_ = (sym::gSndRpcBuf).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_seqIop_4) as i32); let a_ = (sym::gSndRpcBuf).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 3i32, sym::gSndRpcBuf, 16i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcFreeBank(c: &mut Ctx, mut p_mask_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcFreeBank", &PROBE_N, &[(p_mask_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_mask_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 4i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcFreeBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_QueuePlay(c: &mut Ctx, mut p_mask_1: i32, mut p_id_2: u32, mut p_volume_3: u32, mut p_pan_4: u32, mut p_pitch_5: i32, mut p_loop_6: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_QueuePlay", &PROBE_N, &[(p_mask_1) as u32, (p_id_2) as u32, (p_volume_3) as u32, (p_pan_4) as u32, (p_pitch_5) as u32, (p_loop_6) as u32], false); }
    let mut l_cmd_7: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); c.m.r32(a_) }) >= (((32i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if (((((p_id_2) >= (((256i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if (((((p_volume_3) >= (((128i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if (((((p_pan_4) >= (((128i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if (((((((((p_pitch_5) < ((1200i32).wrapping_neg())) as i32)) != 0) || (((((p_pitch_5) > (1200i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    l_cmd_7 = (((c.m.r32(sym::gSndRpc)).wrapping_add(104)).wrapping_add(4)).wrapping_add(({ let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); c.m.r32(a_) }).wrapping_mul(12));
    { let v_ = ((p_loop_6) as u8); let a_ = (l_cmd_7).wrapping_add(1); c.m.w8(a_, v_); };
    { let v_ = ((p_mask_1) as u8); let a_ = (l_cmd_7).wrapping_add(4); c.m.w8(a_, v_); };
    { let v_ = ((p_id_2) as u8); let a_ = (l_cmd_7).wrapping_add(5); c.m.w8(a_, v_); };
    { let v_ = ((p_volume_3) as u8); let a_ = (l_cmd_7).wrapping_add(6); c.m.w8(a_, v_); };
    { let v_ = ((p_pan_4) as u8); let a_ = (l_cmd_7).wrapping_add(7); c.m.w8(a_, v_); };
    { let v_ = p_pitch_5; let a_ = (l_cmd_7).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u8); let a_ = l_cmd_7; c.m.w8(a_, v_); };
    { let v_ = c.m.r16(sym::gSndSerial); let a_ = (l_cmd_7).wrapping_add(2); c.m.w16(a_, v_); };
    { let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
    { let a_ = sym::gSndSerial; let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
    break 'f ((c.m.r16((l_cmd_7).wrapping_add(2))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_QueuePlay", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_QueueStopId(c: &mut Ctx, mut p_mask_1: i32, mut p_id_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_QueueStopId", &PROBE_N, &[(p_mask_1) as u32, (p_id_2) as u32], false); }
    let mut l_cmd_3: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); c.m.r32(a_) }) < (((32i32) as u32))) as i32)) != 0) {
        if (((((p_id_2) < (((256i32) as u32))) as i32)) != 0) {
            l_cmd_3 = (((c.m.r32(sym::gSndRpc)).wrapping_add(104)).wrapping_add(4)).wrapping_add(({ let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); c.m.r32(a_) }).wrapping_mul(12));
            { let v_ = ((1i32) as u8); let a_ = l_cmd_3; c.m.w8(a_, v_); };
            { let v_ = ((p_mask_1) as u8); let a_ = (l_cmd_3).wrapping_add(4); c.m.w8(a_, v_); };
            { let v_ = ((p_id_2) as u8); let a_ = (l_cmd_3).wrapping_add(5); c.m.w8(a_, v_); };
            { let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_QueueStopId", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_QueueStopHandle(c: &mut Ctx, mut p_handle_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_QueueStopHandle", &PROBE_N, &[(p_handle_1) as u32], false); }
    let mut l_cmd_2: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); c.m.r32(a_) }) < (((32i32) as u32))) as i32)) != 0) {
        if (((((p_handle_1) >= (0i32)) as i32)) != 0) {
            l_cmd_2 = (((c.m.r32(sym::gSndRpc)).wrapping_add(104)).wrapping_add(4)).wrapping_add(({ let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); c.m.r32(a_) }).wrapping_mul(12));
            { let v_ = ((2i32) as u8); let a_ = l_cmd_2; c.m.w8(a_, v_); };
            { let v_ = ((p_handle_1) as u16); let a_ = (l_cmd_2).wrapping_add(2); c.m.w16(a_, v_); };
            { let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_QueueStopHandle", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcCmdB(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCmdB", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_a_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_b_2; let a_ = (sym::gSndRpcBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 11i32, sym::gSndRpcBuf, 8i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCmdB", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcCmdC(c: &mut Ctx, mut p_a_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCmdC", &PROBE_N, &[(p_a_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_a_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 12i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCmdC", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcFlush(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcFlush", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::Snd_RpcCall(c, 9i32, sym::gSndRpcBuf, 4i32); };
    if ((((({ let a_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        F::FlushCache(c, 0i32);
        { let s_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); let d_ = sym::gSndRpcBuf; rt::memcpy(c, d_, s_, 388); };
        { let _ = F::Snd_RpcCall(c, 13i32, sym::gSndRpcBuf, ((388u32) as i32)); };
    }
    { let _ = { let a0_ = (c.m.r32(sym::gSndRpc)).wrapping_add(104); F::memset(c, a0_, 0i32, 388u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcFlush", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcSetMono(c: &mut Ctx, mut p_mono_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetMono", &PROBE_N, &[(p_mono_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_mono_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 5i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetMono", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcSetPause(c: &mut Ctx, mut p_mask_1: i32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetPause", &PROBE_N, &[(p_mask_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_mask_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_on_2; let a_ = (sym::gSndRpcBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 6i32, sym::gSndRpcBuf, 8i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetPause", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcStopBank(c: &mut Ctx, mut p_mask_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcStopBank", &PROBE_N, &[(p_mask_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_mask_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 10i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcStopBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcSetVolume(c: &mut Ctx, mut p_mask_1: i32, mut p_volume_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetVolume", &PROBE_N, &[(p_mask_1) as u32, (p_volume_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_mask_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_volume_2; let a_ = (sym::gSndRpcBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 7i32, sym::gSndRpcBuf, 8i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetVolume", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcCmd8(c: &mut Ctx, mut p_a_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCmd8", &PROBE_N, &[(p_a_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_a_1; let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Snd_RpcCall(c, 8i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCmd8", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SpuTransfer(c: &mut Ctx, mut p_iopAddr_1: u32, mut p_spuAddr_2: i32, mut p_size_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SpuTransfer", &PROBE_N, &[(p_iopAddr_1) as u32, (p_spuAddr_2) as u32, (p_size_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::func_002967C0(c, 1i32, 32976i32, &[((1i32) as i64 as u64), ((0i32) as i64 as u64), ((p_iopAddr_1) as u64), ((p_spuAddr_2) as i64 as u64), ((p_size_3) as i64 as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_SpuTransfer", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SpuTransferWait(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SpuTransferWait", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::func_002967C0(c, 1i32, 33008i32, &[((1i32) as i64 as u64), ((1i32) as i64 as u64)]);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_SpuTransferWait", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SpuTransferCheck(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SpuTransferCheck", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::func_002967C0(c, 1i32, 33008i32, &[((1i32) as i64 as u64), ((0i32) as i64 as u64)]);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_SpuTransferCheck", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcCmdE(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCmdE", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::Snd_RpcCall(c, 14i32, sym::gSndRpcBuf, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcCmdE", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_RpcSetFighters(c: &mut Ctx, mut p_count_1: i32, mut p_ids_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetFighters", &PROBE_N, &[(p_count_1) as u32, (p_ids_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    { let v_ = ((p_count_1) as i16); let a_ = (sym::gSndRpcBuf).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (p_count_1)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (((c.m.r32((p_ids_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)) as i16); let a_ = (sym::gSndRpcBuf).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let _ = F::Snd_RpcCall(c, 15i32, sym::gSndRpcBuf, 24i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_RpcSetFighters", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_BankIndex(c: &mut Ctx, mut p_mask_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_BankIndex", &PROBE_N, &[(p_mask_1) as u32], false); }
    let mut l_idx_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_idx_2 = 0i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((p_mask_1) & ((((1i32).wrapping_shl((l_i_3) as u32)) as u32)))) != 0) {
                    l_idx_2 = l_i_3;
                    break 'l1;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_idx_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_BankIndex", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_ScaleVolume(c: &mut Ctx, mut p_bank_1: i32, mut p_volume_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_ScaleVolume", &PROBE_N, &[(p_bank_1) as u32, (p_volume_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = (c.m.r32((sym::gSndBankVolume).wrapping_add((((p_bank_1) as u32)).wrapping_mul(4))) as i32); let n_ = (p_volume_2).wrapping_mul(r_); p_volume_2 = n_; };
    { let r_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); (c.m.r32(a_) as i32) }; let n_ = (p_volume_2).wrapping_mul(r_); p_volume_2 = n_; };
    { let r_ = 1152i32; let n_ = rt::div_i32(p_volume_2, r_); p_volume_2 = n_; };
    if (((((p_volume_2) > (127i32)) as i32)) != 0) {
        p_volume_2 = 127i32;
    }
    if (((((p_volume_2) < (0i32)) as i32)) != 0) {
        p_volume_2 = 0i32;
    }
    break 'f p_volume_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_ScaleVolume", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_ScalePan(c: &mut Ctx, mut p_pan_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_ScalePan", &PROBE_N, &[(p_pan_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = (p_pan_1).wrapping_add(64i32);
    if (((((l_v_2) > (127i32)) as i32)) != 0) {
        l_v_2 = 127i32;
    }
    if (((((l_v_2) < (0i32)) as i32)) != 0) {
        l_v_2 = 0i32;
    }
    break 'f l_v_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_ScalePan", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_ScalePitch(c: &mut Ctx, mut p_pitch_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_ScalePitch", &PROBE_N, &[(p_pitch_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let r_ = 'f: {
    l_v_2 = rt::f2i(rt::fmul(rt::i2f(p_pitch_1), F32(0x41960000)));
    if (((((l_v_2) > (1200i32)) as i32)) != 0) {
        l_v_2 = 1200i32;
    }
    if (((((l_v_2) < ((1200i32).wrapping_neg())) as i32)) != 0) {
        l_v_2 = (1200i32).wrapping_neg();
    }
    break 'f l_v_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_ScalePitch", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SetupBank(c: &mut Ctx, mut p_bank_1: u32, mut p_mask_2: u32, mut p_file_3: u32, mut p_alloc_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SetupBank", &PROBE_N, &[(p_bank_1) as u32, (p_mask_2) as u32, (p_file_3) as u32, (p_alloc_4) as u32], false); }
    let mut l_size_5: i32 = 0;
    let r_ = 'f: {
    { let v_ = (((c.m.r32((p_file_3).wrapping_add(8))).wrapping_sub(c.m.r32((p_file_3).wrapping_add(4)))) as i32); let a_ = (p_bank_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_file_3).wrapping_add((rt::div_u32(c.m.r32((p_file_3).wrapping_add(4)), ((4i32) as u32))).wrapping_mul(4)); let a_ = (p_bank_1).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = (((c.m.r32((p_file_3).wrapping_add(12))).wrapping_sub(c.m.r32((p_file_3).wrapping_add(8)))) as i32); let a_ = (p_bank_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_file_3).wrapping_add((rt::div_u32(c.m.r32((p_file_3).wrapping_add(8)), ((4i32) as u32))).wrapping_mul(4)); let a_ = (p_bank_1).wrapping_add(16); c.m.w32(a_, v_); };
    { let v_ = (((c.m.r32((p_file_3).wrapping_add(16))).wrapping_sub(c.m.r32((p_file_3).wrapping_add(12)))) as i32); let a_ = (p_bank_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_file_3).wrapping_add((rt::div_u32(c.m.r32((p_file_3).wrapping_add(12)), ((4i32) as u32))).wrapping_mul(4)); let a_ = (p_bank_1).wrapping_add(20); c.m.w32(a_, v_); };
    if ((p_alloc_4) != 0) {
        l_size_5 = (((c.m.r32((p_file_3).wrapping_add(8))).wrapping_sub(c.m.r32((p_file_3).wrapping_add(4)))) as i32);
        's1: {
            let sv6: u32 = p_mask_2;
            match sv6 {
                16u32 | 32u32 => {
                    l_size_5 = 585728i32;
                    break 's1;
                }
                8u32 => {
                    l_size_5 = 79872i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let v_ = l_size_5; let a_ = (p_bank_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::SpuHeap_Alloc(c, l_size_5); let a_ = p_bank_1; c.m.w32(a_, (v_) as u32); };
        l_size_5 = (((c.m.r32((p_file_3).wrapping_add(12))).wrapping_sub(c.m.r32((p_file_3).wrapping_add(8)))) as i32);
        's2: {
            let sv7: u32 = p_mask_2;
            match sv7 {
                16u32 | 32u32 => {
                    l_size_5 = 4096i32;
                    break 's2;
                }
                8u32 => {
                    l_size_5 = 2048i32;
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let v_ = l_size_5; let a_ = (p_bank_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::IopHeap_Alloc(c, l_size_5); let a_ = (p_bank_1).wrapping_add(8); c.m.w32(a_, v_); };
        l_size_5 = (((c.m.r32((p_file_3).wrapping_add(16))).wrapping_sub(c.m.r32((p_file_3).wrapping_add(12)))) as i32);
        's3: {
            let sv8: u32 = p_mask_2;
            match sv8 {
                16u32 | 32u32 => {
                    l_size_5 = 2048i32;
                    break 's3;
                }
                8u32 => {
                    l_size_5 = 2048i32;
                    break 's3;
                }
                _ => {
                }
            }
        }
        { let v_ = l_size_5; let a_ = (p_bank_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::IopHeap_Alloc(c, l_size_5); let a_ = (p_bank_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
    { let v_ = ((F::SndHd_GetVagMax(c, p_file_3)) as u8); let a_ = (p_bank_1).wrapping_add(56); c.m.w8(a_, v_); };
    F::SndHd_GetLoopBits(c, p_file_3, (p_bank_1).wrapping_add(57));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_SetupBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SendBank(c: &mut Ctx, mut p_bank_1: u32, mut p_sync_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SendBank", &PROBE_N, &[(p_bank_1) as u32, (p_sync_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_bank_1).wrapping_add(8)); let a1_ = c.m.r32((p_bank_1).wrapping_add(16)); let a2_ = (c.m.r32((p_bank_1).wrapping_add(28)) as i32); F::Dma_SendToIop(c, a0_, a1_, a2_) };
    { let a0_ = c.m.r32((p_bank_1).wrapping_add(4)); let a1_ = c.m.r32((p_bank_1).wrapping_add(20)); let a2_ = (c.m.r32((p_bank_1).wrapping_add(32)) as i32); F::Dma_SendToIop(c, a0_, a1_, a2_) };
    if ((p_sync_2) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(608); c.m.r32(a_) }; let a1_ = c.m.r32((p_bank_1).wrapping_add(12)); let a2_ = (c.m.r32((p_bank_1).wrapping_add(24)) as i32); F::Dma_SendToIop(c, a0_, a1_, a2_) };
        { let a0_ = { let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(608); c.m.r32(a_) }; let a1_ = (c.m.r32(p_bank_1) as i32); let a2_ = (c.m.r32((p_bank_1).wrapping_add(24)) as i32); F::Snd_SpuTransfer(c, a0_, a1_, a2_) };
        { let _ = F::Snd_SpuTransferWait(c); };
    } else {
        { let v_ = 0i32; let a_ = (p_bank_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (p_bank_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_SendBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((680u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSndMgr; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gSndMgr); F::memset(c, a0_, 0i32, 680u32) }; };
    { let v_ = F::IopHeap_Alloc(c, 673792i32); let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(608); c.m.w32(a_, v_); };
    F::SpuHeap_Init(c);
    { let _ = F::sceSdRemoteInit(c); };
    { let _ = F::func_00296B48(c, 5i32); };
    F::Snd_RpcInit(c);
    F::Snd_SetMasterVolume(c, 127i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_Term", &PROBE_N, &[], false); }
    let mut l_bank_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    F::Snd_RpcTerm(c);
    {
        { l_i_2 = 0i32; l_bank_1 = c.m.r32(sym::gSndMgr); };
        'l1: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(l_bank_1) as i32)) == (0i32)) as i32)) != 0) {
                    F::Snd_UnloadBank(c, (((1i32).wrapping_shl((l_i_2) as u32)) as u32));
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_bank_1; let n_ = (o_).wrapping_add(76); l_bank_1 = n_; }; };
        }
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(608); c.m.r32(a_) }; { let _ = F::IopHeap_Free(c, a0_); } };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(608); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32(sym::gSndMgr); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gSndMgr; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_Reset", &PROBE_N, &[], false); }
    let mut l_bank_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    F::Snd_RpcReset(c);
    { let _ = F::Snd_SpuTransferWait(c); };
    {
        { l_i_2 = 0i32; l_bank_1 = c.m.r32(sym::gSndMgr); };
        'l1: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = (l_bank_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_bank_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_bank_1; let n_ = (o_).wrapping_add(76); l_bank_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_Update", &PROBE_N, &[], false); }
    let mut l_bank_1: u32 = 0;
    let mut l_busy_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_buf_4: u32 = 0;
    let r_ = 'f: {
    l_busy_2 = 0i32;
    F::Snd_RpcFlush(c);
    {
        { l_i_3 = 0i32; l_bank_1 = c.m.r32(sym::gSndMgr); };
        'l1: loop {
            if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((c.m.r32((l_bank_1).wrapping_add(52)) as i32)) != 0) {
                    if (((((F::Snd_SpuTransferCheck(c)) == (1i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (l_bank_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (l_bank_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                    } else {
                        l_busy_2 = 1i32;
                    }
                    break 'l1;
                }
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_bank_1; let n_ = (o_).wrapping_add(76); l_bank_1 = n_; }; };
        }
    }
    if ((((!((l_busy_2) != 0)) as i32)) != 0) {
        {
            { l_i_3 = 0i32; l_bank_1 = c.m.r32(sym::gSndMgr); };
            'l3: loop {
                if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if (((c.m.r32((l_bank_1).wrapping_add(48)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (l_bank_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                        l_buf_4 = { let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(608); c.m.r32(a_) };
                        { let a1_ = c.m.r32((l_bank_1).wrapping_add(12)); let a2_ = (c.m.r32((l_bank_1).wrapping_add(24)) as i32); F::Dma_SendToIop(c, l_buf_4, a1_, a2_) };
                        { let a1_ = (c.m.r32(l_bank_1) as i32); let a2_ = (c.m.r32((l_bank_1).wrapping_add(24)) as i32); F::Snd_SpuTransfer(c, l_buf_4, a1_, a2_) };
                        break 'l3;
                    }
                }
                { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_bank_1; let n_ = (o_).wrapping_add(76); l_bank_1 = n_; }; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_LoadBank(c: &mut Ctx, mut p_mask_1: u32, mut p_file_2: u32, mut p_sync_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_LoadBank", &PROBE_N, &[(p_mask_1) as u32, (p_file_2) as u32, (p_sync_3) as u32], false); }
    let mut l_idx_4: i32 = 0;
    let mut l_bank_5: u32 = 0;
    let r_ = 'f: {
    l_idx_4 = F::Snd_BankIndex(c, p_mask_1);
    if ((((!((F::Snd_IsBankLoaded(c, p_mask_1)) != 0)) as i32)) != 0) {
        l_bank_5 = (c.m.r32(sym::gSndMgr)).wrapping_add((((l_idx_4) as u32)).wrapping_mul(76));
        if (((c.m.r32((l_bank_5).wrapping_add(48)) as i32)) != 0) {
            if (((c.m.r32((l_bank_5).wrapping_add(52)) as i32)) != 0) {
                { let _ = F::Snd_SpuTransferWait(c); };
            }
            { let v_ = 0i32; let a_ = (l_bank_5).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (l_bank_5).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        }
        F::Snd_SetupBank(c, l_bank_5, p_mask_1, p_file_2, 1i32);
        F::Snd_SendBank(c, l_bank_5, p_sync_3);
        { let a1_ = (c.m.r32(l_bank_5) as i32); let a2_ = c.m.r32((l_bank_5).wrapping_add(8)); let a3_ = c.m.r32((l_bank_5).wrapping_add(4)); F::Snd_RpcSetBank(c, ((p_mask_1) as i32), a1_, a2_, a3_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_LoadBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_ReloadBank(c: &mut Ctx, mut p_mask_1: u32, mut p_file_2: u32, mut p_sync_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_ReloadBank", &PROBE_N, &[(p_mask_1) as u32, (p_file_2) as u32, (p_sync_3) as u32], false); }
    let mut l_fixed_4: i32 = 0;
    let mut l_idx_5: i32 = 0;
    let mut l_bank_6: u32 = 0;
    let r_ = 'f: {
    's1: {
        let sv7: u32 = p_mask_1;
        match sv7 {
            8u32 | 16u32 | 32u32 => {
                l_fixed_4 = 1i32;
                break 's1;
            }
            _ => {
                l_fixed_4 = 0i32;
                break 's1;
            }
        }
    }
    l_idx_5 = F::Snd_BankIndex(c, p_mask_1);
    if ((((((l_fixed_4) != 0) && ((F::Snd_IsBankLoaded(c, p_mask_1)) != 0)) as i32)) != 0) {
        l_bank_6 = (c.m.r32(sym::gSndMgr)).wrapping_add((((l_idx_5) as u32)).wrapping_mul(76));
        if (((c.m.r32((l_bank_6).wrapping_add(48)) as i32)) != 0) {
            if (((c.m.r32((l_bank_6).wrapping_add(52)) as i32)) != 0) {
                { let _ = F::Snd_SpuTransferWait(c); };
            }
            { let v_ = 0i32; let a_ = (l_bank_6).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (l_bank_6).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        }
        F::Snd_StopBank(c, ((p_mask_1) as i32));
        F::Snd_SetupBank(c, l_bank_6, p_mask_1, p_file_2, 0i32);
        F::Snd_SendBank(c, l_bank_6, p_sync_3);
        { let a1_ = (c.m.r32(l_bank_6) as i32); let a2_ = c.m.r32((l_bank_6).wrapping_add(8)); let a3_ = c.m.r32((l_bank_6).wrapping_add(4)); F::Snd_RpcSetBank(c, ((p_mask_1) as i32), a1_, a2_, a3_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_ReloadBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_LoadBankFile(c: &mut Ctx, mut p_mask_1: u32, mut p_fileId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_LoadBankFile", &PROBE_N, &[(p_mask_1) as u32, (p_fileId_2) as u32], false); }
    let mut l_data_3: u32 = 0;
    let r_ = 'f: {
    l_data_3 = F::File_LoadSync(c, p_fileId_2, ((0i32) as u32), 0i32);
    F::Snd_LoadBank(c, p_mask_1, l_data_3, 1i32);
    F::Heap_Free(c, l_data_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_LoadBankFile", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_UnloadBank(c: &mut Ctx, mut p_mask_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_UnloadBank", &PROBE_N, &[(p_mask_1) as u32], false); }
    let mut l_idx_2: i32 = 0;
    let mut l_bank_3: u32 = 0;
    let r_ = 'f: {
    l_idx_2 = F::Snd_BankIndex(c, p_mask_1);
    if ((F::Snd_IsBankLoaded(c, p_mask_1)) != 0) {
        l_bank_3 = (c.m.r32(sym::gSndMgr)).wrapping_add((((l_idx_2) as u32)).wrapping_mul(76));
        { let a0_ = (c.m.r32(l_bank_3) as i32); F::SpuHeap_Free(c, a0_) };
        { let v_ = 0i32; let a_ = l_bank_3; c.m.w32(a_, (v_) as u32); };
        { let a0_ = c.m.r32((l_bank_3).wrapping_add(8)); { let _ = F::IopHeap_Free(c, a0_); } };
        { let v_ = ((0i32) as u32); let a_ = (l_bank_3).wrapping_add(8); c.m.w32(a_, v_); };
        { let a0_ = c.m.r32((l_bank_3).wrapping_add(4)); { let _ = F::IopHeap_Free(c, a0_); } };
        { let v_ = ((0i32) as u32); let a_ = (l_bank_3).wrapping_add(4); c.m.w32(a_, v_); };
        F::Snd_RpcFreeBank(c, ((p_mask_1) as i32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_UnloadBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_IsUploadDone(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_IsUploadDone", &PROBE_N, &[], false); }
    let mut l_bank_1: u32 = 0;
    let mut l_done_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_bank_1 = c.m.r32(sym::gSndMgr);
    l_done_2 = 1i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_bank_1).wrapping_add(48)) as i32)) == (0i32)) as i32)) != 0) {
                    break 'c2;
                }
                l_done_2 = 0i32;
                break 'l1;
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_bank_1; let n_ = (o_).wrapping_add(76); l_bank_1 = n_; }; };
        }
    }
    break 'f l_done_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_IsUploadDone", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_PushSpuMark(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_PushSpuMark", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::SpuHeap_GetFreeSize(c); let a_ = ((c.m.r32(sym::gSndMgr)).wrapping_add(616)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(612); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(612); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_PushSpuMark", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_PopSpuMark(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_PopSpuMark", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add(612); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    break 'f F::SpuHeap_GetFreeSize(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_PopSpuMark", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SetStereo(c: &mut Ctx, mut p_stereo_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SetStereo", &PROBE_N, &[(p_stereo_1) as u32], false); }
    let r_ = 'f: {
    if ((p_stereo_1) != 0) {
        F::Snd_RpcSetMono(c, 0i32);
        F::Adx_SetMono(c, 0i32);
    } else {
        F::Snd_RpcSetMono(c, 1i32);
        F::Adx_SetMono(c, 1i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_SetStereo", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SetMasterVolume(c: &mut Ctx, mut p_volume_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SetMasterVolume", &PROBE_N, &[(p_volume_1) as u32], false); }
    let r_ = 'f: {
    F::Snd_RpcSetVolume(c, 255i32, p_volume_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_SetMasterVolume", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_PlaySe(c: &mut Ctx, mut p_mask_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_PlaySe", &PROBE_N, &[(p_mask_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::Snd_PlaySeEx(c, p_mask_1, p_id_2, 64i32, 0i32, 0i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_PlaySe", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_PlaySeEx(c: &mut Ctx, mut p_mask_1: u32, mut p_id_2: i32, mut p_volume_3: i32, mut p_pan_4: i32, mut p_pitch_5: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_PlaySeEx", &PROBE_N, &[(p_mask_1) as u32, (p_id_2) as u32, (p_volume_3) as u32, (p_pan_4) as u32, (p_pitch_5) as u32], false); }
    let mut l_idx_6: i32 = 0;
    let mut l_side_7: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_idx_6 = F::Snd_BankIndex(c, p_mask_1);
                if ((((!((F::Snd_IsBankLoaded(c, p_mask_1)) != 0)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                break 'f (1i32).wrapping_neg();
            }
            2 => {
                if (({ let a_ = ((c.m.r32(sym::gSndMgr)).wrapping_add((((l_idx_6) as u32)).wrapping_mul(76))).wrapping_add(48); (c.m.r32(a_) as i32) }) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            3 => {
                bb_ = 2; continue 'fl1;
            }
            4 => {
                break 'f (1i32).wrapping_neg();
            }
            5 => {
                if (((((p_mask_1) != (((16i32) as u32))) as i32)) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
            }
            6 => {
                bb_ = 5; continue 'fl1;
            }
            7 => {
                l_side_7 = 1i32;
                if (((((p_mask_1) != (((32i32) as u32))) as i32)) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
            }
            8 => {
                if ((F::BtlScript_IsSlotBusy(c, l_side_7, 0i32)) != 0) { bb_ = 14; continue 'fl1; } else { bb_ = 15; continue 'fl1; }
            }
            9 => {
                l_side_7 = 0i32;
                bb_ = 8; continue 'fl1;
            }
            10 => {
                bb_ = 12; continue 'fl1;
            }
            11 => {
                bb_ = 8; continue 'fl1;
            }
            12 => {
                p_volume_3 = F::Snd_ScaleVolume(c, l_idx_6, p_volume_3);
                p_pan_4 = F::Snd_ScalePan(c, p_pan_4);
                p_pitch_5 = F::Snd_ScalePitch(c, p_pitch_5);
                break 'f { let a5_ = F::Snd_IsLoopSe(c, p_mask_1, p_id_2); F::Snd_QueuePlay(c, ((p_mask_1) as i32), ((p_id_2) as u32), ((p_volume_3) as u32), ((p_pan_4) as u32), p_pitch_5, a5_) };
            }
            13 => {
                bb_ = 11; continue 'fl1;
            }
            14 => {
                break 'f (1i32).wrapping_neg();
            }
            15 => {
                bb_ = 12; continue 'fl1;
            }
            16 => {
                bb_ = 15; continue 'fl1;
            }
            17 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_PlaySeEx", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_StopSe(c: &mut Ctx, mut p_mask_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_StopSe", &PROBE_N, &[(p_mask_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    if ((F::Snd_IsBankLoaded(c, p_mask_1)) != 0) {
        F::Snd_QueueStopId(c, ((p_mask_1) as i32), ((p_id_2) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_StopSe", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_StopHandle(c: &mut Ctx, mut p_handle_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_StopHandle", &PROBE_N, &[(p_handle_1) as u32], false); }
    let r_ = 'f: {
    F::Snd_QueueStopHandle(c, p_handle_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_StopHandle", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_CmdB(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_CmdB", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    F::Snd_RpcCmdB(c, p_a_1, p_b_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_CmdB", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_CmdC(c: &mut Ctx, mut p_a_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_CmdC", &PROBE_N, &[(p_a_1) as u32], false); }
    let r_ = 'f: {
    F::Snd_RpcCmdC(c, p_a_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_CmdC", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SetPause(c: &mut Ctx, mut p_mask_1: i32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SetPause", &PROBE_N, &[(p_mask_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    F::Snd_RpcSetPause(c, p_mask_1, p_on_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_SetPause", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_StopBank(c: &mut Ctx, mut p_mask_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_StopBank", &PROBE_N, &[(p_mask_1) as u32], false); }
    let r_ = 'f: {
    F::Snd_RpcStopBank(c, p_mask_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_StopBank", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_StopBankAndResume(c: &mut Ctx, mut p_mask_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_StopBankAndResume", &PROBE_N, &[(p_mask_1) as u32], false); }
    let r_ = 'f: {
    F::Snd_StopBank(c, p_mask_1);
    F::Snd_SetPause(c, p_mask_1, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_StopBankAndResume", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_Cmd8(c: &mut Ctx, mut p_a_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_Cmd8", &PROBE_N, &[(p_a_1) as u32], false); }
    let r_ = 'f: {
    F::Snd_RpcCmd8(c, p_a_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_Cmd8", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_IsLoopSe(c: &mut Ctx, mut p_mask_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_IsLoopSe", &PROBE_N, &[(p_mask_1) as u32, (p_id_2) as u32], false); }
    let mut l_idx_3: i32 = 0;
    let mut l_bank_4: u32 = 0;
    let r_ = 'f: {
    l_idx_3 = F::Snd_BankIndex(c, p_mask_1);
    if ((F::Snd_IsBankLoaded(c, p_mask_1)) != 0) {
        l_bank_4 = (c.m.r32(sym::gSndMgr)).wrapping_add((((l_idx_3) as u32)).wrapping_mul(76));
        break 'f (((((c.m.r8(((l_bank_4).wrapping_add(57)).wrapping_add(((rt::div_i32(p_id_2, 8i32)) as u32)))) as i32)).wrapping_shr((rt::rem_i32(p_id_2, 8i32)) as u32)) & (1i32));
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_IsLoopSe", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_GetVagMax(c: &mut Ctx, mut p_mask_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_GetVagMax", &PROBE_N, &[(p_mask_1) as u32], false); }
    let mut l_idx_2: i32 = 0;
    let mut l_bank_3: u32 = 0;
    let r_ = 'f: {
    l_idx_2 = F::Snd_BankIndex(c, p_mask_1);
    if ((F::Snd_IsBankLoaded(c, p_mask_1)) != 0) {
        l_bank_3 = (c.m.r32(sym::gSndMgr)).wrapping_add((((l_idx_2) as u32)).wrapping_mul(76));
        break 'f ((c.m.r8((l_bank_3).wrapping_add(56))) as i32);
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_GetVagMax", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_IsBankLoaded(c: &mut Ctx, mut p_mask_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_IsBankLoaded", &PROBE_N, &[(p_mask_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (c.m.r32(sym::gSndMgr)).wrapping_add((((F::Snd_BankIndex(c, p_mask_1)) as u32)).wrapping_mul(76)); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_IsBankLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_CmdE(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_CmdE", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Snd_RpcCmdE(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Snd_CmdE", &PROBE_N, &[], true); }
    r_
}

pub fn Snd_SendFighters(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Snd_SendFighters", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_count_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        l_count_1 = 0i32;
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (F::BtlCharApi_GetSoundCount(c, 0i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::BtlCharApi_GetSound(c, 0i32, l_i_2, (fp).wrapping_add((((l_count_1) as u32)).wrapping_mul(4)), ((0i32) as u32), ((0i32) as u32));
                    { let o_ = l_count_1; let n_ = (o_).wrapping_add(1); l_count_1 = n_; };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (F::BtlCharApi_GetSoundCount(c, 1i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    F::BtlCharApi_GetSound(c, 1i32, l_i_2, (fp).wrapping_add((((l_count_1) as u32)).wrapping_mul(4)), ((0i32) as u32), ((0i32) as u32));
                    { let o_ = l_count_1; let n_ = (o_).wrapping_add(1); l_count_1 = n_; };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::Snd_RpcSetFighters(c, l_count_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Snd_SendFighters", &PROBE_N, &[], true); }
    r_
}

pub fn SndHd_GetVagInfo(c: &mut Ctx, mut p_head_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SndHd_GetVagInfo", &PROBE_N, &[(p_head_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_head_1).wrapping_add((rt::div_u32(c.m.r32((p_head_1).wrapping_add((((12i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(4));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SndHd_GetVagInfo", &PROBE_N, &[], true); }
    r_
}

pub fn SndHd_GetVagMax(c: &mut Ctx, mut p_file_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SndHd_GetVagMax", &PROBE_N, &[(p_file_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ({ let a0_ = (p_file_1).wrapping_add((rt::div_u32(c.m.r32((p_file_1).wrapping_add(8)), ((4i32) as u32))).wrapping_mul(4)); F::SndHd_GetVagInfo(c, a0_) }).wrapping_add(12); c.m.r32(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SndHd_GetVagMax", &PROBE_N, &[], true); }
    r_
}

pub fn SndHd_GetLoopFlags(c: &mut Ctx, mut p_file_1: u32, mut p_flags_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SndHd_GetLoopFlags", &PROBE_N, &[(p_file_1) as u32, (p_flags_2) as u32], false); }
    let mut l_chunk_3: u32 = 0;
    let mut l_param_4: u32 = 0;
    let mut l_i_5: u32 = 0;
    let r_ = 'f: {
    l_chunk_3 = { let a0_ = (p_file_1).wrapping_add((rt::div_u32(c.m.r32((p_file_1).wrapping_add(8)), ((4i32) as u32))).wrapping_mul(4)); F::SndHd_GetVagInfo(c, a0_) };
    l_param_4 = ((l_chunk_3).wrapping_add(16)).wrapping_add(((c.m.r32((l_chunk_3).wrapping_add(12))).wrapping_add(((1i32) as u32))).wrapping_mul(4));
    {
        l_i_5 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_5) <= (c.m.r32((l_chunk_3).wrapping_add(12)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((c.m.r8((l_param_4).wrapping_add(6))) != 0) {
                    { let v_ = ((1i32) as u8); let a_ = p_flags_2; c.m.w8(a_, v_); };
                } else {
                    { let v_ = ((0i32) as u8); let a_ = p_flags_2; c.m.w8(a_, v_); };
                }
                { let o_ = l_param_4; let n_ = (o_).wrapping_add(8); l_param_4 = n_; };
                { let o_ = p_flags_2; let n_ = (o_).wrapping_add(1); p_flags_2 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SndHd_GetLoopFlags", &PROBE_N, &[], true); }
    r_
}

pub fn SndHd_GetLoopBits(c: &mut Ctx, mut p_file_1: u32, mut p_bits_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SndHd_GetLoopBits", &PROBE_N, &[(p_file_1) as u32, (p_bits_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_count_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        l_count_3 = F::SndHd_GetVagMax(c, p_file_1);
        F::SndHd_GetLoopFlags(c, p_file_1, fp);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (l_count_3)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((c.m.r8((fp).wrapping_add(((l_i_4) as u32)))) != 0) {
                        { let r_ = (1i32).wrapping_shl((rt::rem_i32(l_i_4, 8i32)) as u32); let a_ = (p_bits_2).wrapping_add(((rt::div_i32(l_i_4, 8i32)) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SndHd_GetLoopBits", &PROBE_N, &[], true); }
    r_
}
