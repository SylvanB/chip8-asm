use crate::opcode::OpCode;
use crate::token::Token;

#[derive(Debug)]
pub(crate) struct Parser<'parser> {
    src: &'parser [u8],
    idx: usize,
    out: Vec<Token>,
}

impl<'parser> Parser<'parser> {
    pub fn new(src: &'parser [u8]) -> Self {
        Self {
            src,
            idx: 0,
            out: vec![],
        }
    }

    pub fn parse(mut self) -> Vec<Token> {
        while self.idx < self.src.len() {
            let op = self.get_next_op().unwrap();
            self.parse_op(op);
        }

        self.out
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

    // Parse a single op
    fn parse_op(&mut self, op: OpCode) {
        let token = match op.raw() {
            0x0000..=0x0ff => match op.raw() {
                0x00e0 => Token::CLS,
                0x00ee => Token::RET,
                _ => Token::DATA(op.raw()),
            },
            0x1000..=0x1fff => Token::JP_NNN(op.nnn()),
            0x2000..=0x2fff => Token::CALL_Addr(op.nnn()),
            0x3000..=0x3fff => Token::SE_Vx_Byte(op.x(), op.kk()),
            0x4000..=0x4fff => Token::SNE_Vx_Byte(op.x(), op.kk()),
            0x5000..=0x5fff => Token::SE_Vx_Vy(op.x(), op.y()),
            0x6000..=0x6fff => Token::LD_Vx_Byte(op.x(), op.kk()),
            0x7000..=0x7fff => Token::ADD_Vx_Byte(op.x(), op.kk()),
            0x8000..=0x8fff => self.ops_8(&op),
            0x9000..=0x9fff => Token::SNE_Vx_Vy(op.x(), op.y()),
            0xa000..=0xafff => Token::LD_I_Addr(op.nnn()),
            0xb000..=0xbfff => Token::JP_V0_Addr(op.nnn()),
            0xc000..=0xcfff => Token::RND_Vx_Byte(op.x(), op.kk()),
            0xd000..=0xdfff => Token::DRW_Vx_Vy_N(op.x(), op.y(), (op.raw() & 0x000F) as u8),
            0xe000..=0xefff => self.ops_e(&op),
            0xf000..=0xffff => self.ops_f(&op),
            _ => Token::DATA(op.raw()),
        };

        self.out.push(token);
    }

    fn ops_8(&mut self, op: &OpCode) -> Token {
        if op.raw() & 0x000F == 0x0 {
            return Token::LD_Vx_Vy(op.x(), op.y());
        } else if op.raw() & 0x000F == 0x1 {
            return Token::OR_Vx_Vy(op.x(), op.y());
        } else if op.raw() & 0x000F == 0x2 {
            return Token::AND_Vx_Vy(op.x(), op.y());
        } else if op.raw() & 0x000F == 0x3 {
            return Token::XOR_Vx_Vy(op.x(), op.y());
        } else if op.raw() & 0x000F == 0x4 {
            return Token::ADD_Vx_Vy(op.x(), op.y());
        } else if op.raw() & 0x000F == 0x5 {
            return Token::SUB_Vx_Vy(op.x(), op.y());
        } else if op.raw() & 0x000F == 0x6 {
            return Token::SHR_Vx_Vy(op.x(), op.y());
        } else if op.raw() & 0x000F == 0x7 {
            return Token::SUBN_Vx_Vy(op.x(), op.y());
        } else if op.raw() & 0x000F == 0xE {
            return Token::SHL_Vx_Vy(op.x(), op.y());
        } else {
            return Token::DATA(op.raw());
        }
    }

    fn ops_f(&mut self, op: &OpCode) -> Token {
        if op.raw() & 0x00FF == 0x07 {
            return Token::LD_Vx_Dt(op.x());
        } else if op.raw() & 0x00FF == 0x15 {
            return Token::LD_Dt_Vx(op.x());
        } else if op.raw() & 0x00FF == 0x18 {
            return Token::LD_St_Vx(op.x());
        } else if op.raw() & 0x00FF == 0x0A {
            return Token::LD_Vx_K(op.x());
        } else if op.raw() & 0x00FF == 0x1E {
            return Token::ADD_I_Vx(op.x());
        } else if op.raw() & 0x00FF == 0x29 {
            return Token::LD_F_Vx(op.x());
        } else if op.raw() & 0x00FF == 0x33 {
            return Token::LD_B_Vx(op.x());
        } else if op.raw() & 0x00FF == 0x55 {
            return Token::LD_I_Vx(op.x());
        } else if op.raw() & 0x00FF == 0x65 {
            return Token::LD_Vx_I(op.x());
        } else {
            return Token::DATA(op.raw());
        }
    }

    fn ops_e(&mut self, op: &OpCode) -> Token {
        if op.raw() & 0x00FF == 0x9E {
            return Token::SKP_Vx(op.x());
        } else if op.raw() & 0x00FF == 0xA1 {
            return Token::SKNP_Vx(op.x());
        } else {
            return Token::DATA(op.raw());
        }
    }
}
