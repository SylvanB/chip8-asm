use crate::opcode::OpCode;
use crate::token::Token;

#[derive(Debug)]
pub(crate) struct Parser<'parser> {
    src: &'parser [u8],
    idx: usize,
    out: Vec<Token>,
}

impl<'parser> Parser<'parser> {
    pub fn initialise(src: &'parser [u8]) -> Self {
        Self {
            src,
            idx: 0,
            out: vec![],
        }
    }

    // Get the next op in the stream
    fn get_next_op(&mut self) -> Option<OpCode> {
        if self.idx + 2 <= self.src.len() {
            self.idx += 2;
            return Some(OpCode::new_from_parts(
                self.src[self.idx - 2],
                self.src[self.idx - 1],
            ));
        }

        None
    }

    // Parse the next token
    pub fn parse_next_op(&mut self) {
        if let Some(op) = self.get_next_op() {
            self.parse_op(op);
        }
    }

    // Parse a single op
    fn parse_op(&mut self, op: OpCode) {
        match op.raw() {
            0x0000..=0x0ff => match op.raw() {
                0x00e0 => self.cls(),
                0x00ee => self.ret(),
                _ => self.out.push(Token::DATA(op.raw())),
            },
            0x1000..=0x1fff => self.jp(&op),
            0x2000..=0x2fff => self.call(&op),
            0x3000..=0x3fff => self.se(&op),
            0x4000..=0x4fff => self.sne(&op),
            0x5000..=0x5fff => self.se_r(&op),
            0x6000..=0x6fff => self.ld_r(&op),
            0x7000..=0x7fff => self.add(&op),
            0x8000..=0x8fff => self.ops_8(&op),
            0x9000..=0x9fff => self.sne_xy(&op),
            0xa000..=0xafff => self.ld_i(&op),
            0xb000..=0xbfff => self.jp_v0(&op),
            0xc000..=0xcfff => self.rnd(&op),
            0xd000..=0xdfff => self.drw(&op),
            0xe000..=0xefff => self.ops_e(&op),
            0xf000..=0xffff => self.ops_f(&op),
            _ => self.out.push(Token::DATA(op.raw())),
        };
    }

    fn ops_8(&mut self, op: &OpCode) {
        if op.raw() & 0x000F == 0x0 {
            self.ld_xy(&op);
        } else if op.raw() & 0x000F == 0x1 {
            self.or_xy(&op);
        } else if op.raw() & 0x000F == 0x2 {
            self.and_xy(&op);
        } else if op.raw() & 0x000F == 0x3 {
            self.xor_xy(&op);
        } else if op.raw() & 0x000F == 0x4 {
            self.add_xy(&op);
        } else if op.raw() & 0x000F == 0x5 {
            self.sub_xy(&op);
        } else if op.raw() & 0x000F == 0x6 {
            self.shr(&op);
        } else if op.raw() & 0x000F == 0x7 {
            self.subn_yx(&op);
        } else if op.raw() & 0x000F == 0xE {
            self.shl(&op);
        } else {
        }
    }

    fn ops_f(&mut self, op: &OpCode) {
        if op.raw() & 0x00FF == 0x07 {
            self.ld_vx_dt(&op);
        } else if op.raw() & 0x00FF == 0x15 {
            self.ld_dt(&op);
        } else if op.raw() & 0x00FF == 0x18 {
            self.ld_st(&op);
        } else if op.raw() & 0x00FF == 0x0A {
            self.ld_vx_k(&op);
        } else if op.raw() & 0x00FF == 0x1E {
            self.add_i(&op);
        } else if op.raw() & 0x00FF == 0x29 {
            self.ld_f_vx(&op);
        } else if op.raw() & 0x00FF == 0x33 {
            self.ld_b(&op);
        } else if op.raw() & 0x00FF == 0x55 {
            self.ld_mem_i_vx(&op);
        } else if op.raw() & 0x00FF == 0x65 {
            self.ld_mem_vx_i(&op);
        }
    }

    fn ops_e(&mut self, op: &OpCode) {
        if op.raw() & 0x00FF == 0x9E {
            self.skp_vx(op);
        } else if op.raw() & 0x00FF == 0xA1 {
            self.sknp_vx(op);
        } else {
        }
    }

    fn cls(&mut self) {
        self.out.push(Token::CLS);
    }

    fn ret(&mut self) {
        self.out.push(Token::RET);
    }

    fn jp(&mut self, op: &OpCode) {
        self.out.push(Token::JP_NNN(op.nnn()));
    }

    fn call(&mut self, op: &OpCode) {
        self.out.push(Token::CALL_Addr(op.nnn()));
    }

    fn se(&mut self, op: &OpCode) {
        self.out.push(Token::SE_Vx_Byte(op.x(), op.kk()));
    }

    fn sne(&mut self, op: &OpCode) {
        self.out.push(Token::SNE_Vx_Byte(op.x(), op.kk()));
    }

    fn se_r(&mut self, op: &OpCode) {
        self.out.push(Token::SE_Vx_Vy(op.x(), op.y()));
    }

    fn ld_r(&mut self, op: &OpCode) {
        self.out.push(Token::LD_Vx_Byte(op.x(), op.kk()));
    }

    fn add(&mut self, op: &OpCode) {
        self.out.push(Token::ADD_Vx_Byte(op.x(), op.kk()));
    }

    fn ld_xy(&mut self, op: &OpCode) {
        self.out.push(Token::LD_Vx_Vy(op.x(), op.y()));
    }

    fn ld_f_vx(&mut self, op: &OpCode) {
        self.out.push(Token::LD_F_Vx(op.x()));
    }

    fn ld_vx_k(&mut self, op: &OpCode) {
        self.out.push(Token::LD_Vx_K(op.x()));
    }

    fn ld_vx_dt(&mut self, op: &OpCode) {
        self.out.push(Token::LD_Vx_Dt(op.x()));
    }

    fn ld_st(&mut self, op: &OpCode) {
        self.out.push(Token::LD_St_Vx(op.x()));
    }

    fn ld_dt(&mut self, op: &OpCode) {
        self.out.push(Token::LD_Dt_Vx(op.x()));
    }

    fn add_i(&mut self, op: &OpCode) {
        self.out.push(Token::ADD_I_Vx(op.x()));
    }

    fn ld_b(&mut self, op: &OpCode) {
        self.out.push(Token::LD_B_Vx(op.x()));
    }

    fn ld_mem_i_vx(&mut self, op: &OpCode) {
        self.out.push(Token::LD_I_Vx(op.x()));
    }

    fn ld_mem_vx_i(&mut self, op: &OpCode) {
        self.out.push(Token::LD_Vx_I(op.x()));
    }

    fn or_xy(&mut self, op: &OpCode) {
        self.out.push(Token::OR_Vx_Vy(op.x(), op.y()));
    }

    fn and_xy(&mut self, op: &OpCode) {
        self.out.push(Token::AND_Vx_Vy(op.x(), op.y()));
    }

    fn xor_xy(&mut self, op: &OpCode) {
        self.out.push(Token::XOR_Vx_Vy(op.x(), op.y()));
    }

    fn add_xy(&mut self, op: &OpCode) {
        self.out.push(Token::ADD_Vx_Vy(op.x(), op.y()));
    }

    fn sub_xy(&mut self, op: &OpCode) {
        self.out.push(Token::SUB_Vx_Vy(op.x(), op.y()));
    }

    fn subn_yx(&mut self, op: &OpCode) {
        self.out.push(Token::SUBN_Vx_Vy(op.x(), op.y()));
    }

    fn shr(&mut self, op: &OpCode) {
        self.out.push(Token::SHR_Vx_Vy(op.x(), op.y()));
    }

    fn shl(&mut self, op: &OpCode) {
        self.out.push(Token::SHL_Vx_Vy(op.x(), op.y()));
    }

    fn sne_xy(&mut self, op: &OpCode) {
        self.out.push(Token::SNE_Vx_Vy(op.x(), op.y()));
    }

    fn ld_i(&mut self, op: &OpCode) {
        self.out.push(Token::LD_I_Addr(op.nnn()));
    }

    fn jp_v0(&mut self, op: &OpCode) {
        self.out.push(Token::JP_V0_Addr(op.nnn()));
    }

    fn rnd(&mut self, op: &OpCode) {
        self.out.push(Token::RND_Vx_Byte(op.x(), op.kk()));
    }

    fn drw(&mut self, op: &OpCode) {
        self.out.push(Token::DRW_Vx_Vy_N(
            op.x(),
            op.y(),
            (op.raw() & 0x000F) as u8,
        ));
    }

    fn skp_vx(&mut self, op: &OpCode) {
        self.out.push(Token::SKP_Vx(op.x()));
    }

    fn sknp_vx(&mut self, op: &OpCode) {
        self.out.push(Token::SKNP_Vx(op.x()));
    }
}
