//! The game's vertex programs served by GPU shaders instead of the VU1 interpreter (with the GPU renderer only).
//! The game uploads nine programs; the ones that draw almost everything (the fighters, the stage, the shadows,
//! the debris) are recognised by their size and their batches handed to gpu.rs with the program's constants,
//! where shaders/vu0.vert, vu4.vert and vu6.vert do the per-vertex work. Anything else still runs in vu1.rs.
//! Rust version of the C port's gs_vu1.c (hle_program0 .. hle_program78, GsVu1_Call). BT3_VU_INTERP=1 turns it off.
use super::Gs;

impl Gs {
    fn vu_q(&self, q: u32) -> [u8; 16] {
        let o = ((q & 0x3FF) * 16) as usize;
        self.vu.mem[o..o + 16].try_into().unwrap()
    }

    fn vu_tag(&self, q: u32) -> u64 {
        u64::from_le_bytes(self.vu_q(q)[..8].try_into().unwrap())
    }

    fn vu_floats(&self, q: u32, n: usize) -> Vec<f32> {
        let o = ((q & 0x3FF) * 16) as usize;
        self.vu.mem[o..o + n * 4].chunks_exact(4).map(|b| f32::from_le_bytes(b.try_into().unwrap())).collect()
    }

    /// The A+D packet of a batch (GIF tag at q, its data at d) sent to the GS.
    fn vu_ad(&mut self, q: u32, d: u32) {
        let mut pkt = [0u8; 32];
        pkt[..16].copy_from_slice(&self.vu_q(q));
        pkt[16..].copy_from_slice(&self.vu_q(d));
        self.gif(&pkt);
    }

    fn vu_draw(&mut self, which: u32, layer: u32, ctx: usize, verts: &[f32], count: usize, consts: &[f32]) {
        let mut g = self.gpu.take().unwrap();
        match which {
            0 => g.draw_vu0(self, layer, ctx, verts, count, consts),
            4 => g.draw_vu4(self, ctx, verts, count, consts),
            _ => g.draw_vu6(self, ctx, verts, count, consts),
        }
        self.gpu = Some(g);
    }

    /// Program 0 (the fighters' models; 127 instructions): +0 A+D tag, +1 / +2 TEX0 of the texture and toon
    /// layers, +3 / +4 the two layers' primitive tags, +5 the vertices (three quadwords each).
    fn hle_program0(&mut self) -> bool {
        let top = self.vu.top & 0x3FF;
        let count = (self.vu_tag(top + 3) & 0x7FFF) as u32;
        if count < 3 || top + 5 + count * 3 > 1024 {
            return false;
        }
        let verts = self.vu_floats(top + 5, count as usize * 12);
        let consts = self.vu_floats(0, 96);
        for layer in 0..2u32 {
            self.vu_ad(top, top + 1 + layer);
            let tag = self.vu_tag(top + 3 + layer);
            self.reg_write(0, (tag >> 47) & 0x7FF);
            self.vu_draw(0, layer, ((tag >> 56) & 1) as usize, &verts, count as usize, &consts);
        }
        self.vu.stat_kicks += 1;
        true
    }

    /// Program 1 (124 instructions): program 0's batch; layer 1 takes its texture from constant 26 and its
    /// coordinates from the normal (vu0.vert layer 5). A batch whose first normal has 0 in the low 16 bits of its
    /// 4th word is not drawn.
    fn hle_program1(&mut self) -> bool {
        let top = self.vu.top & 0x3FF;
        let count = (self.vu_tag(top + 3) & 0x7FFF) as u32;
        if count < 3 || top + 5 + count * 3 > 1024 {
            return false;
        }
        let q = self.vu_q(top + 6);
        if u32::from_le_bytes(q[12..16].try_into().unwrap()) & 0xFFFF == 0 {
            return true;
        }
        let verts = self.vu_floats(top + 5, count as usize * 12);
        let consts = self.vu_floats(0, 96);
        for layer in 0..2u32 {
            self.vu_ad(top, if layer == 0 { top + 1 } else { 26 });
            let tag = self.vu_tag(top + 3 + layer);
            self.reg_write(0, (tag >> 47) & 0x7FF);
            self.vu_draw(0, if layer == 0 { 0 } else { 5 }, ((tag >> 56) & 1) as usize, &verts, count as usize, &consts);
        }
        self.vu.stat_kicks += 1;
        true
    }

    /// Program 4 (the stage; 399 instructions): +0 A+D tag, +1 the texture, +2 the primitive tag, +3 the vertices.
    fn hle_program4(&mut self) -> bool {
        let top = self.vu.top & 0x3FF;
        let tag = self.vu_tag(top + 2);
        let count = (tag & 0x7FFF) as u32;
        if count < 3 || top + 3 + count * 3 > 1024 {
            return false;
        }
        self.vu_ad(top, top + 1);
        self.reg_write(0, (tag >> 47) & 0x7FF);
        let verts = self.vu_floats(top + 3, count as usize * 12);
        let consts = self.vu_floats(0, 16);
        self.vu_draw(4, 0, ((tag >> 56) & 1) as usize, &verts, count as usize, &consts);
        self.vu.stat_kicks += 1;
        true
    }

    /// Programs 2a / 2b (99 / 101 instructions; a fighter in one flat colour for the shadow): program 0's batch
    /// with the constants in other places: 0..9 bones and pivots, 10 the colour, 11..14 the screen matrix.
    fn hle_program2(&mut self) -> bool {
        let top = self.vu.top & 0x3FF;
        let tag = self.vu_tag(top + 4);
        let count = (tag & 0x7FFF) as u32;
        if count < 3 || top + 5 + count * 3 > 1024 {
            return false;
        }
        let mem = self.vu_floats(0, 60);
        let mut consts = [0f32; 96];
        consts[..40].copy_from_slice(&mem[..40]);
        consts[56..72].copy_from_slice(&mem[44..60]);
        consts[88..92].copy_from_slice(&mem[40..44]);
        self.vu_ad(top, top + 1);
        self.reg_write(0, (tag >> 47) & 0x7FF);
        // the context of the PRIM in force (the shadow passes run with PRMODECONT.AC = 0)
        let ctx = ((self.s.prim >> 9) & 1) as usize;
        let verts = self.vu_floats(top + 5, count as usize * 12);
        self.vu_draw(0, 2, ctx, &verts, count as usize, &consts);
        self.vu.stat_kicks += 1;
        true
    }

    /// Programs 8 (animated stage objects; 94 instructions, layer 3) and 7 (debris; 104, layer 4): the stage's
    /// batch through the fighter shader.
    fn hle_program78(&mut self, layer: u32) -> bool {
        let top = self.vu.top & 0x3FF;
        let tag = self.vu_tag(top + 2);
        let count = (tag & 0x7FFF) as u32;
        if count < 3 || top + 3 + count * 3 > 1024 {
            return false;
        }
        let mem = self.vu_floats(0, 56);
        let mut consts = [0f32; 96];
        if layer == 3 {
            consts[..40].copy_from_slice(&mem[..40]);
            consts[56..72].copy_from_slice(&mem[40..56]);
        } else {
            consts[..16].copy_from_slice(&mem[..16]);
            consts[16..32].copy_from_slice(&mem[..16]);
            consts[56..72].copy_from_slice(&mem[16..32]);
            consts[91] = mem[12 * 4 + 3];
        }
        self.vu_ad(top, top + 1);
        self.reg_write(0, (tag >> 47) & 0x7FF);
        let verts = self.vu_floats(top + 3, count as usize * 12);
        self.vu_draw(0, layer, ((tag >> 56) & 1) as usize, &verts, count as usize, &consts);
        self.vu.stat_kicks += 1;
        true
    }

    /// Program 6 (the ground shadow; 451 instructions): +2 the primitive tag, +3 the vertices, four quadwords each.
    fn hle_program6(&mut self) -> bool {
        let top = self.vu.top & 0x3FF;
        let tag = self.vu_tag(top + 2);
        let count = (tag & 0x7FFF) as u32;
        if count < 3 || top + 3 + count * 4 > 1024 {
            return false;
        }
        self.reg_write(0, (tag >> 47) & 0x7FF);
        let ctx = ((self.s.prim >> 9) & 1) as usize;
        let verts = self.vu_floats(top + 3, count as usize * 16);
        let consts = self.vu_floats(0, 0x11 * 4);
        self.vu_draw(6, 0, ctx, &verts, count as usize, &consts);
        self.vu.stat_kicks += 1;
        true
    }

    /// MSCAL / MSCALF (addr = instruction address) or MSCNT (addr < 0).
    pub(super) fn vu_call(&mut self, addr: i32) {
        if self.gpu.is_none() || self.vu_interp {
            self.vu.call(addr);
            return;
        }
        self.vu.swap_buffers();
        let size = self.vu.prog_size;
        let mut resume: Option<u32> = None;
        match size {
            399 => {
                if addr >= 0 {
                    self.hle_p4 = 0; // MSCALF 0: constants only
                    return;
                }
                self.hle_p4 += 1;
                if self.hle_p4 == 1 || self.hle_program4() {
                    return; // (the first MSCNT is the object header: only the clipper used it)
                }
                // a batch the shader path cannot take: the interpreter needs what the skipped calls would have left
                self.vu.run(0);
                resume = Some(21);
            }
            451 => {
                if addr >= 0 {
                    self.hle_p6 = 0;
                    return;
                }
                self.hle_p6 += 1;
                if self.hle_p6 > 1 {
                    self.hle_program6(); // (fewer than 3 vertices: nothing to draw either)
                }
                return;
            }
            94 | 104 => {
                if addr >= 0 || self.hle_program78(if size == 94 { 3 } else { 4 }) {
                    return;
                }
                self.vu.run(0);
                resume = Some(if size == 94 { 15 } else { 52 });
            }
            99 | 101 => {
                if addr >= 0 || self.hle_program2() {
                    return;
                }
                self.vu.run(0);
                resume = Some(16);
            }
            124 => {
                if addr >= 0 || self.hle_program1() {
                    return;
                }
                self.vu.run(0);
                resume = Some(15);
            }
            127 => {
                if addr >= 0 || self.hle_program0() {
                    return;
                }
            }
            _ => {}
        }
        let start = match resume {
            Some(pc) => pc,
            None if addr < 0 => self.vu.pc,
            None => addr as u32,
        };
        self.vu.run(start);
    }
}
