#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn List_Copy(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_Copy", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = c.m.r32(p_src_2); let a_ = p_dst_1; c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_src_2).wrapping_add(4)); let a_ = (p_dst_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_src_2).wrapping_add(8)) as i32); let a_ = (p_dst_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "List_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn SList_Copy(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_Copy", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = c.m.r32(p_src_2); let a_ = p_dst_1; c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_src_2).wrapping_add(4)); let a_ = (p_dst_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_src_2).wrapping_add(8)) as i32); let a_ = (p_dst_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn List_Init(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_Init", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = p_list_1; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (p_list_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "List_Init", &PROBE_N, &[], true); }
    r_
}

pub fn List_GetHead(c: &mut Ctx, mut p_list_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_GetHead", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    l_node_2 = ((0i32) as u32);
    if (((((p_list_1) != (((0i32) as u32))) as i32)) != 0) {
        l_node_2 = c.m.r32(p_list_1);
    }
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_GetHead", &PROBE_N, &[], true); }
    r_
}

pub fn List_GetTail(c: &mut Ctx, mut p_list_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_GetTail", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    l_node_2 = ((0i32) as u32);
    if (((((p_list_1) != (((0i32) as u32))) as i32)) != 0) {
        l_node_2 = c.m.r32((p_list_1).wrapping_add(4));
    }
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_GetTail", &PROBE_N, &[], true); }
    r_
}

pub fn List_GetCount(c: &mut Ctx, mut p_list_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_GetCount", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let r_ = 'f: {
    l_count_2 = 0i32;
    if (((((p_list_1) != (((0i32) as u32))) as i32)) != 0) {
        l_count_2 = (c.m.r32((p_list_1).wrapping_add(8)) as i32);
    }
    break 'f l_count_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_GetCount", &PROBE_N, &[], true); }
    r_
}

pub fn List_PushFront(c: &mut Ctx, mut p_list_1: u32, mut p_node_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_PushFront", &PROBE_N, &[(p_list_1) as u32, (p_node_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_list_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_node_2; let a_ = c.m.r32(p_list_1); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = p_node_2; c.m.w32(a_, v_); };
        { let v_ = c.m.r32(p_list_1); let a_ = (p_node_2).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = p_node_2; let a_ = p_list_1; c.m.w32(a_, v_); };
    } else {
        { let v_ = p_node_2; let a_ = p_list_1; c.m.w32(a_, v_); };
        { let v_ = p_node_2; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = p_node_2; c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_node_2).wrapping_add(4); c.m.w32(a_, v_); };
    }
    { let a_ = (p_list_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "List_PushFront", &PROBE_N, &[], true); }
    r_
}

pub fn List_PushBack(c: &mut Ctx, mut p_list_1: u32, mut p_node_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_PushBack", &PROBE_N, &[(p_list_1) as u32, (p_node_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_list_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_node_2; let a_ = (c.m.r32((p_list_1).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((p_list_1).wrapping_add(4)); let a_ = p_node_2; c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_node_2).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = p_node_2; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
    } else {
        { let v_ = p_node_2; let a_ = p_list_1; c.m.w32(a_, v_); };
        { let v_ = p_node_2; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = p_node_2; c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_node_2).wrapping_add(4); c.m.w32(a_, v_); };
    }
    { let a_ = (p_list_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "List_PushBack", &PROBE_N, &[], true); }
    r_
}

pub fn List_PrependList(c: &mut Ctx, mut p_list_1: u32, mut p_other_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_PrependList", &PROBE_N, &[(p_list_1) as u32, (p_other_2) as u32], false); }
    let mut l_head_3: u32 = 0;
    let mut l_tail_4: u32 = 0;
    let mut l_count_5: i32 = 0;
    let r_ = 'f: {
    l_head_3 = c.m.r32(p_other_2);
    l_tail_4 = c.m.r32((p_other_2).wrapping_add(4));
    l_count_5 = (c.m.r32((p_other_2).wrapping_add(8)) as i32);
    if (((((l_count_5) != (0i32)) as i32)) != 0) {
        if (((((c.m.r32(p_list_1)) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_tail_4; let a_ = c.m.r32(p_list_1); c.m.w32(a_, v_); };
            { let v_ = c.m.r32(p_list_1); let a_ = (l_tail_4).wrapping_add(4); c.m.w32(a_, v_); };
            { let v_ = l_head_3; let a_ = p_list_1; c.m.w32(a_, v_); };
            { let r_ = l_count_5; let a_ = (p_list_1).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            F::List_Copy(c, p_list_1, p_other_2);
        }
        F::List_Init(c, p_other_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "List_PrependList", &PROBE_N, &[], true); }
    r_
}

pub fn List_AppendList(c: &mut Ctx, mut p_list_1: u32, mut p_other_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_AppendList", &PROBE_N, &[(p_list_1) as u32, (p_other_2) as u32], false); }
    let mut l_head_3: u32 = 0;
    let mut l_tail_4: u32 = 0;
    let mut l_count_5: i32 = 0;
    let r_ = 'f: {
    l_head_3 = c.m.r32(p_other_2);
    l_tail_4 = c.m.r32((p_other_2).wrapping_add(4));
    l_count_5 = (c.m.r32((p_other_2).wrapping_add(8)) as i32);
    if (((((l_count_5) != (0i32)) as i32)) != 0) {
        if (((((c.m.r32(p_list_1)) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_head_3; let a_ = (c.m.r32((p_list_1).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
            { let v_ = c.m.r32((p_list_1).wrapping_add(4)); let a_ = l_head_3; c.m.w32(a_, v_); };
            { let v_ = l_tail_4; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
            { let r_ = l_count_5; let a_ = (p_list_1).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            F::List_Copy(c, p_list_1, p_other_2);
        }
        F::List_Init(c, p_other_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "List_AppendList", &PROBE_N, &[], true); }
    r_
}

pub fn List_InsertBefore(c: &mut Ctx, mut p_list_1: u32, mut p_pos_2: u32, mut p_node_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_InsertBefore", &PROBE_N, &[(p_list_1) as u32, (p_pos_2) as u32, (p_node_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_pos_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32(p_list_1)) == (p_pos_2)) as i32)) != 0) {
            F::List_PushFront(c, p_list_1, p_node_3);
        } else {
            { let v_ = c.m.r32(p_pos_2); let a_ = p_node_3; c.m.w32(a_, v_); };
            { let v_ = p_pos_2; let a_ = (p_node_3).wrapping_add(4); c.m.w32(a_, v_); };
            if (((((c.m.r32(p_pos_2)) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = p_node_3; let a_ = (c.m.r32(p_pos_2)).wrapping_add(4); c.m.w32(a_, v_); };
            }
            { let v_ = p_node_3; let a_ = p_pos_2; c.m.w32(a_, v_); };
            { let a_ = (p_list_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_InsertBefore", &PROBE_N, &[], true); }
    r_
}

pub fn List_InsertAfter(c: &mut Ctx, mut p_list_1: u32, mut p_pos_2: u32, mut p_node_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_InsertAfter", &PROBE_N, &[(p_list_1) as u32, (p_pos_2) as u32, (p_node_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_pos_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32((p_list_1).wrapping_add(4))) == (p_pos_2)) as i32)) != 0) {
            F::List_PushBack(c, p_list_1, p_node_3);
        } else {
            { let v_ = p_pos_2; let a_ = p_node_3; c.m.w32(a_, v_); };
            { let v_ = c.m.r32((p_pos_2).wrapping_add(4)); let a_ = (p_node_3).wrapping_add(4); c.m.w32(a_, v_); };
            if (((((c.m.r32((p_pos_2).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = p_node_3; let a_ = c.m.r32((p_pos_2).wrapping_add(4)); c.m.w32(a_, v_); };
            }
            { let v_ = p_node_3; let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, v_); };
            { let a_ = (p_list_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_InsertAfter", &PROBE_N, &[], true); }
    r_
}

pub fn List_Remove(c: &mut Ctx, mut p_list_1: u32, mut p_node_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_Remove", &PROBE_N, &[(p_list_1) as u32, (p_node_2) as u32], false); }
    let mut l_prev_3: u32 = 0;
    let mut l_next_4: u32 = 0;
    let r_ = 'f: {
    if (((((p_node_2) != (((0i32) as u32))) as i32)) != 0) {
        l_prev_3 = c.m.r32(p_node_2);
        l_next_4 = c.m.r32((p_node_2).wrapping_add(4));
        if (((((l_prev_3) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_next_4; let a_ = p_list_1; c.m.w32(a_, v_); };
        } else {
            { let v_ = l_next_4; let a_ = (l_prev_3).wrapping_add(4); c.m.w32(a_, v_); };
        }
        if (((((l_next_4) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_prev_3; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
        } else {
            { let v_ = l_prev_3; let a_ = l_next_4; c.m.w32(a_, v_); };
        }
        { let v_ = ((0i32) as u32); let a_ = (p_node_2).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = p_node_2; c.m.w32(a_, v_); };
        { let a_ = (p_list_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    break 'f p_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_Remove", &PROBE_N, &[], true); }
    r_
}

pub fn List_PopFront(c: &mut Ctx, mut p_list_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_PopFront", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = c.m.r32(p_list_1); F::List_Remove(c, p_list_1, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_PopFront", &PROBE_N, &[], true); }
    r_
}

pub fn List_PopBack(c: &mut Ctx, mut p_list_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_PopBack", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = c.m.r32((p_list_1).wrapping_add(4)); F::List_Remove(c, p_list_1, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_PopBack", &PROBE_N, &[], true); }
    r_
}

pub fn List_GetPrev(c: &mut Ctx, mut p_node_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_GetPrev", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_prev_2: u32 = 0;
    let r_ = 'f: {
    l_prev_2 = ((0i32) as u32);
    if (((((p_node_1) != (((0i32) as u32))) as i32)) != 0) {
        l_prev_2 = c.m.r32(p_node_1);
    }
    break 'f l_prev_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_GetPrev", &PROBE_N, &[], true); }
    r_
}

pub fn List_GetNext(c: &mut Ctx, mut p_node_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_GetNext", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_next_2: u32 = 0;
    let r_ = 'f: {
    l_next_2 = ((0i32) as u32);
    if (((((p_node_1) != (((0i32) as u32))) as i32)) != 0) {
        l_next_2 = c.m.r32((p_node_1).wrapping_add(4));
    }
    break 'f l_next_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_GetNext", &PROBE_N, &[], true); }
    r_
}

pub fn List_Contains(c: &mut Ctx, mut p_list_1: u32, mut p_node_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "List_Contains", &PROBE_N, &[(p_list_1) as u32, (p_node_2) as u32], false); }
    let mut l_found_3: i32 = 0;
    let mut l_cur_4: u32 = 0;
    let r_ = 'f: {
    l_found_3 = 0i32;
    {
        l_cur_4 = F::List_GetHead(c, p_list_1);
        'l1: loop {
            if !(((((l_cur_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_cur_4) == (p_node_2)) as i32)) != 0) {
                    l_found_3 = 1i32;
                    break 'l1;
                }
            }
            l_cur_4 = F::List_GetNext(c, l_cur_4);
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "List_Contains", &PROBE_N, &[], true); }
    r_
}

pub fn SList_Init(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_Init", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = p_list_1; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (p_list_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_Init", &PROBE_N, &[], true); }
    r_
}

pub fn SList_GetHead(c: &mut Ctx, mut p_list_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_GetHead", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    l_node_2 = ((0i32) as u32);
    if (((((p_list_1) != (((0i32) as u32))) as i32)) != 0) {
        l_node_2 = c.m.r32(p_list_1);
    }
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_GetHead", &PROBE_N, &[], true); }
    r_
}

pub fn SList_GetTail(c: &mut Ctx, mut p_list_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_GetTail", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    l_node_2 = ((0i32) as u32);
    if (((((p_list_1) != (((0i32) as u32))) as i32)) != 0) {
        l_node_2 = c.m.r32((p_list_1).wrapping_add(4));
    }
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_GetTail", &PROBE_N, &[], true); }
    r_
}

pub fn SList_GetCount(c: &mut Ctx, mut p_list_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_GetCount", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let r_ = 'f: {
    l_count_2 = 0i32;
    if (((((p_list_1) != (((0i32) as u32))) as i32)) != 0) {
        l_count_2 = (c.m.r32((p_list_1).wrapping_add(8)) as i32);
    }
    break 'f l_count_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_GetCount", &PROBE_N, &[], true); }
    r_
}

pub fn SList_PushFront(c: &mut Ctx, mut p_list_1: u32, mut p_node_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_PushFront", &PROBE_N, &[(p_list_1) as u32, (p_node_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_list_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = c.m.r32(p_list_1); let a_ = p_node_2; c.m.w32(a_, v_); };
    } else {
        { let v_ = p_node_2; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = p_node_2; c.m.w32(a_, v_); };
    }
    { let v_ = p_node_2; let a_ = p_list_1; c.m.w32(a_, v_); };
    { let a_ = (p_list_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_PushFront", &PROBE_N, &[], true); }
    r_
}

pub fn SList_PushBack(c: &mut Ctx, mut p_list_1: u32, mut p_node_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_PushBack", &PROBE_N, &[(p_list_1) as u32, (p_node_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_list_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_node_2; let a_ = c.m.r32((p_list_1).wrapping_add(4)); c.m.w32(a_, v_); };
    } else {
        { let v_ = p_node_2; let a_ = p_list_1; c.m.w32(a_, v_); };
    }
    { let v_ = ((0i32) as u32); let a_ = p_node_2; c.m.w32(a_, v_); };
    { let v_ = p_node_2; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let a_ = (p_list_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_PushBack", &PROBE_N, &[], true); }
    r_
}

pub fn SList_PrependList(c: &mut Ctx, mut p_list_1: u32, mut p_other_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_PrependList", &PROBE_N, &[(p_list_1) as u32, (p_other_2) as u32], false); }
    let mut l_head_3: u32 = 0;
    let mut l_tail_4: u32 = 0;
    let mut l_count_5: i32 = 0;
    let r_ = 'f: {
    l_head_3 = c.m.r32(p_other_2);
    l_tail_4 = c.m.r32((p_other_2).wrapping_add(4));
    l_count_5 = (c.m.r32((p_other_2).wrapping_add(8)) as i32);
    if (((((l_count_5) != (0i32)) as i32)) != 0) {
        if (((((c.m.r32(p_list_1)) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r32(p_list_1); let a_ = l_tail_4; c.m.w32(a_, v_); };
            { let v_ = l_head_3; let a_ = p_list_1; c.m.w32(a_, v_); };
            { let r_ = l_count_5; let a_ = (p_list_1).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            F::SList_Copy(c, p_list_1, p_other_2);
        }
        F::SList_Init(c, p_other_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_PrependList", &PROBE_N, &[], true); }
    r_
}

pub fn SList_AppendList(c: &mut Ctx, mut p_list_1: u32, mut p_other_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_AppendList", &PROBE_N, &[(p_list_1) as u32, (p_other_2) as u32], false); }
    let mut l_head_3: u32 = 0;
    let mut l_tail_4: u32 = 0;
    let mut l_count_5: i32 = 0;
    let r_ = 'f: {
    l_head_3 = c.m.r32(p_other_2);
    l_tail_4 = c.m.r32((p_other_2).wrapping_add(4));
    l_count_5 = (c.m.r32((p_other_2).wrapping_add(8)) as i32);
    if (((((l_count_5) != (0i32)) as i32)) != 0) {
        if (((((c.m.r32((p_list_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_head_3; let a_ = c.m.r32((p_list_1).wrapping_add(4)); c.m.w32(a_, v_); };
            { let v_ = l_tail_4; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
            { let r_ = l_count_5; let a_ = (p_list_1).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            F::SList_Copy(c, p_list_1, p_other_2);
        }
        F::SList_Init(c, p_other_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_AppendList", &PROBE_N, &[], true); }
    r_
}

pub fn SList_PopFront(c: &mut Ctx, mut p_list_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_PopFront", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    l_node_2 = ((0i32) as u32);
    if (((((c.m.r32(p_list_1)) != (((0i32) as u32))) as i32)) != 0) {
        l_node_2 = c.m.r32(p_list_1);
        { let v_ = c.m.r32(l_node_2); let a_ = p_list_1; c.m.w32(a_, v_); };
        if (((((c.m.r32(p_list_1)) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
        }
        { let v_ = ((0i32) as u32); let a_ = l_node_2; c.m.w32(a_, v_); };
        { let a_ = (p_list_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_PopFront", &PROBE_N, &[], true); }
    r_
}

pub fn SList_GetNext(c: &mut Ctx, mut p_node_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_GetNext", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_next_2: u32 = 0;
    let r_ = 'f: {
    l_next_2 = ((0i32) as u32);
    if (((((p_node_1) != (((0i32) as u32))) as i32)) != 0) {
        l_next_2 = c.m.r32(p_node_1);
    }
    break 'f l_next_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_GetNext", &PROBE_N, &[], true); }
    r_
}

pub fn SList_Contains(c: &mut Ctx, mut p_list_1: u32, mut p_node_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SList_Contains", &PROBE_N, &[(p_list_1) as u32, (p_node_2) as u32], false); }
    let mut l_found_3: i32 = 0;
    let mut l_cur_4: u32 = 0;
    let r_ = 'f: {
    l_found_3 = 0i32;
    {
        l_cur_4 = F::SList_GetHead(c, p_list_1);
        'l1: loop {
            if !(((((l_cur_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_cur_4) == (p_node_2)) as i32)) != 0) {
                    l_found_3 = 1i32;
                    break 'l1;
                }
            }
            l_cur_4 = F::SList_GetNext(c, l_cur_4);
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SList_Contains", &PROBE_N, &[], true); }
    r_
}
