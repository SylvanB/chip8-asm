use std::fmt::Display;

/// Represents the entire base instruction set of the Chip8 interpreter.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    CLS,
    RET,
    JP_NNN(u16),
    CALL_Addr(u16),
    SE_Vx_Byte(u8, u8),
    SNE_Vx_Byte(u8, u8),
    SE_Vx_Vy(u8, u8),
    LD_Vx_Byte(u8, u8),
    ADD_Vx_Byte(u8, u8),
    LD_Vx_Vy(u8, u8),
    OR_Vx_Vy(u8, u8),
    AND_Vx_Vy(u8, u8),
    XOR_Vx_Vy(u8, u8),
    ADD_Vx_Vy(u8, u8),
    SUB_Vx_Vy(u8, u8),
    SHR_Vx_Vy(u8, u8),
    SUBN_Vx_Vy(u8, u8),
    SHL_Vx_Vy(u8, u8),
    SNE_Vx_Vy(u8, u8),
    LD_I_Addr(u16),
    JP_V0_Addr(u16),
    RND_Vx_Byte(u8, u8),
    DRW_Vx_Vy_N(u8, u8, u8),
    SKP_Vx(u8),
    SKNP_Vx(u8),
    LD_Vx_Dt(u8),
    LD_Vx_K(u8),
    LD_Dt_Vx(u8),
    LD_St_Vx(u8),
    ADD_I_Vx(u8),
    LD_F_Vx(u8),
    LD_B_Vx(u8),
    LD_I_Vx(u8),
    LD_Vx_I(u8),
    DATA(u16),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::CLS => write!(f, "CLS"),
            Token::RET => write!(f, "RET"),
            Token::JP_NNN(nnn) => write!(f, "JP ${}", nnn),
            Token::CALL_Addr(addr) => write!(f, "CALL ${}", addr),
            Token::SE_Vx_Byte(vx, byte) => write!(f, "SE V{} {:#02x}", vx, byte),
            Token::SNE_Vx_Byte(vx, byte) => write!(f, "SNE V{} {:#02x}", vx, byte),
            Token::SE_Vx_Vy(vx, vy) => write!(f, "SE V{} V{}", vx, vy),
            Token::LD_Vx_Byte(vx, byte) => write!(f, "LD V{} {:#02x}", vx, byte),
            Token::ADD_Vx_Byte(vx, byte) => write!(f, "ADD V{} {:#02x}", vx, byte),
            Token::LD_Vx_Vy(vx, vy) => write!(f, "LD V{} V{}", vx, vy),
            Token::OR_Vx_Vy(vx, vy) => write!(f, "OR V{} V{}", vx, vy),
            Token::AND_Vx_Vy(vx, vy) => write!(f, "AND V{} V{}", vx, vy),
            Token::XOR_Vx_Vy(vx, vy) => write!(f, "XOR V{} V{}", vx, vy),
            Token::ADD_Vx_Vy(vx, vy) => write!(f, "ADD V{} V{}", vx, vy),
            Token::SUB_Vx_Vy(vx, vy) => write!(f, "SUB V{} V{}", vx, vy),
            Token::SHR_Vx_Vy(vx, vy) => write!(f, "SHR V{} V{}", vx, vy),
            Token::SUBN_Vx_Vy(vx, vy) => write!(f, "SUBN V{} V{}", vx, vy),
            Token::SHL_Vx_Vy(vx, vy) => write!(f, "SHL V{} V{}", vx, vy),
            Token::SNE_Vx_Vy(vx, vy) => write!(f, "SNE V{} V{}", vx, vy),
            Token::LD_I_Addr(addr) => write!(f, "LD I ${}", addr),
            Token::JP_V0_Addr(addr) => write!(f, "JP V0 ${}", addr),
            Token::RND_Vx_Byte(vx, byte) => write!(f, "RND V{} {:#02x}", vx, byte),
            Token::DRW_Vx_Vy_N(vx, vy, n) => write!(f, "DRW V{} V{} {:#02x}", vx, vy, n),
            Token::SKP_Vx(vx) => write!(f, "SKP V{}", vx),
            Token::SKNP_Vx(vx) => write!(f, "SKNP V{}", vx),
            Token::LD_Vx_Dt(vx) => write!(f, "LD V{} D", vx),
            Token::LD_Vx_K(vx) => write!(f, "LD V{} K", vx),
            Token::LD_Dt_Vx(vx) => write!(f, "LD D V{}", vx),
            Token::LD_St_Vx(vx) => write!(f, "LD S V{}", vx),
            Token::ADD_I_Vx(vx) => write!(f, "ADD I V{}", vx),
            Token::LD_F_Vx(vx) => write!(f, "LD F V{}", vx),
            Token::LD_B_Vx(vx) => write!(f, "LD B V{}", vx),
            Token::LD_I_Vx(vx) => write!(f, "LD I V{}", vx),
            Token::LD_Vx_I(vx) => write!(f, "LD V{} I", vx),
            Token::DATA(d) => write!(f, "DATA {:#02x}", d),
        }
    }
}
