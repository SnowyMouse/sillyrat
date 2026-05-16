mod format;

pub use format::InstructionDisplay;

use core::fmt::{Debug, Display, Formatter, Write};
use crate::format::SyntaxDialect;

/// All possible SM83 instructions in binary form.
#[allow(nonstandard_style, missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Instruction {
    NOP = 0x00,
    STOP = 0x10,

    JR_NZ_r8 { r8: i8 } = 0x20,
    JR_NC_r8 { r8: i8 } = 0x30,

    LD_BC_d16 { d16: u16 } = 0x01,
    LD_DE_d16 { d16: u16 } = 0x11,
    LD_HL_d16 { d16: u16 } = 0x21,
    LD_SP_d16 { d16: u16 } = 0x31,

    LD_BC_addr_A = 0x02,
    LD_DE_addr_A = 0x12,
    LD_HLi_addr_A = 0x22,
    LD_HLd_addr_A = 0x32,

    INC_BC = 0x03,
    INC_DE = 0x13,
    INC_HL = 0x23,
    INC_SP = 0x33,

    INC_B = 0x04,
    INC_D = 0x14,
    INC_H = 0x24,
    INC_HL_addr = 0x34,

    DEC_B = 0x05,
    DEC_D = 0x15,
    DEC_H = 0x25,
    DEC_HL_addr = 0x35,

    LD_B_d8 { d8: u8 } = 0x06,
    LD_D_d8 { d8: u8 } = 0x16,
    LD_H_d8 { d8: u8 } = 0x26,
    LD_HL_addr_d8 { d8: u8 } = 0x36,

    RLCA = 0x07,
    RLA = 0x17,
    DAA = 0x27,
    SCF = 0x37,

    LD_a16_addr_SP { a16: u16 } = 0x08,
    JR_r8 { r8: i8 } = 0x18,
    JR_Z_r8 { r8: i8 } = 0x28,
    JR_C_r8 { r8: i8 } = 0x38,

    ADD_HL_BC = 0x09,
    ADD_HL_DE = 0x19,
    ADD_HL_HL = 0x29,
    ADD_HL_SP = 0x39,

    LD_A_BC_addr = 0x0A,
    LD_A_DE_addr = 0x1A,
    LD_A_HLi_addr = 0x2A,
    LD_A_HLd_addr = 0x3A,

    DEC_BC = 0x0B,
    DEC_DE = 0x1B,
    DEC_HL = 0x2B,
    DEC_SP = 0x3B,

    INC_C = 0x0C,
    INC_E = 0x1C,
    INC_L = 0x2C,
    INC_A = 0x3C,

    DEC_C = 0x0D,
    DEC_E = 0x1D,
    DEC_L = 0x2D,
    DEC_A = 0x3D,

    LD_C_d8 { d8: u8 } = 0x0E,
    LD_E_d8 { d8: u8 } = 0x1E,
    LD_L_d8 { d8: u8 } = 0x2E,
    LD_A_d8 { d8: u8 } = 0x3E,

    RRCA = 0x0F,
    RRA = 0x1F,
    CPL = 0x2F,
    CCF = 0x3F,

    LD_B_B = 0x40,
    LD_D_B = 0x50,
    LD_H_B = 0x60,
    LD_HL_addr_B = 0x70,

    LD_B_C = 0x41,
    LD_D_C = 0x51,
    LD_H_C = 0x61,
    LD_HL_addr_C = 0x71,

    LD_B_D = 0x42,
    LD_D_D = 0x52,
    LD_H_D = 0x62,
    LD_HL_addr_D = 0x72,

    LD_B_E = 0x43,
    LD_D_E = 0x53,
    LD_H_E = 0x63,
    LD_HL_addr_E = 0x73,

    LD_B_H = 0x44,
    LD_D_H = 0x54,
    LD_H_H = 0x64,
    LD_HL_addr_H = 0x74,

    LD_B_L = 0x45,
    LD_D_L = 0x55,
    LD_H_L = 0x65,
    LD_HL_addr_L = 0x75,

    LD_B_HL_addr = 0x46,
    LD_D_HL_addr = 0x56,
    LD_H_HL_addr = 0x66,
    HALT = 0x76,

    LD_B_A = 0x47,
    LD_D_A = 0x57,
    LD_H_A = 0x67,
    LD_HL_addr_A = 0x77,

    LD_C_B = 0x48,
    LD_E_B = 0x58,
    LD_L_B = 0x68,
    LD_A_B = 0x78,

    LD_C_C = 0x49,
    LD_E_C = 0x59,
    LD_L_C = 0x69,
    LD_A_C = 0x79,

    LD_C_D = 0x4A,
    LD_E_D = 0x5A,
    LD_L_D = 0x6A,
    LD_A_D = 0x7A,

    LD_C_E = 0x4B,
    LD_E_E = 0x5B,
    LD_L_E = 0x6B,
    LD_A_E = 0x7B,

    LD_C_H = 0x4C,
    LD_E_H = 0x5C,
    LD_L_H = 0x6C,
    LD_A_H = 0x7C,

    LD_C_L = 0x4D,
    LD_E_L = 0x5D,
    LD_L_L = 0x6D,
    LD_A_L = 0x7D,

    LD_C_HL_addr = 0x4E,
    LD_E_HL_addr = 0x5E,
    LD_L_HL_addr = 0x6E,
    LD_A_HL_addr = 0x7E,

    LD_C_A = 0x4F,
    LD_E_A = 0x5F,
    LD_L_A = 0x6F,
    LD_A_A = 0x7F,

    ADD_B = 0x80,
    SUB_B = 0x90,
    AND_B = 0xA0,
    OR_B = 0xB0,

    ADD_C = 0x81,
    SUB_C = 0x91,
    AND_C = 0xA1,
    OR_C = 0xB1,

    ADD_D = 0x82,
    SUB_D = 0x92,
    AND_D = 0xA2,
    OR_D = 0xB2,

    ADD_E = 0x83,
    SUB_E = 0x93,
    AND_E = 0xA3,
    OR_E = 0xB3,

    ADD_H = 0x84,
    SUB_H = 0x94,
    AND_H = 0xA4,
    OR_H = 0xB4,

    ADD_L = 0x85,
    SUB_L = 0x95,
    AND_L = 0xA5,
    OR_L = 0xB5,

    ADD_HL_addr = 0x86,
    SUB_HL_addr = 0x96,
    AND_HL_addr = 0xA6,
    OR_HL_addr = 0xB6,

    ADD_A = 0x87,
    SUB_A = 0x97,
    AND_A = 0xA7,
    OR_A = 0xB7,

    ADC_B = 0x88,
    SBC_B = 0x98,
    XOR_B = 0xA8,
    CP_B = 0xB8,

    ADC_C = 0x89,
    SBC_C = 0x99,
    XOR_C = 0xA9,
    CP_C = 0xB9,

    ADC_D = 0x8A,
    SBC_D = 0x9A,
    XOR_D = 0xAA,
    CP_D = 0xBA,

    ADC_E = 0x8B,
    SBC_E = 0x9B,
    XOR_E = 0xAB,
    CP_E = 0xBB,

    ADC_H = 0x8C,
    SBC_H = 0x9C,
    XOR_H = 0xAC,
    CP_H = 0xBC,

    ADC_L = 0x8D,
    SBC_L = 0x9D,
    XOR_L = 0xAD,
    CP_L = 0xBD,

    ADC_HL_addr = 0x8E,
    SBC_HL_addr = 0x9E,
    XOR_HL_addr = 0xAE,
    CP_HL_addr = 0xBE,

    ADC_A = 0x8F,
    SBC_A = 0x9F,
    XOR_A = 0xAF,
    CP_A = 0xBF,

    RET_NZ = 0xC0,
    RET_NC = 0xD0,
    LD_a8_addr_A { a8: u8 } = 0xE0,
    LD_A_a8_addr { a8: u8 } = 0xF0,

    POP_BC = 0xC1,
    POP_DE = 0xD1,
    POP_HL = 0xE1,
    POP_AF = 0xF1,

    JP_NZ_a16 { a16: u16 } = 0xC2,
    JP_NC_a16 { a16: u16 } = 0xD2,
    LD_C_addr_A = 0xE2,
    LD_A_C_addr = 0xF2,

    JP_A16 { a16: u16 } = 0xC3,
    // 0xD3
    // 0xE3
    DI = 0xF3,

    CALL_NZ_a16 { a16: u16 } = 0xC4,
    CALL_NC_a16 { a16: u16 } = 0xD4,
    // 0xE4
    // 0xF4

    PUSH_BC = 0xC5,
    PUSH_DE = 0xD5,
    PUSH_HL = 0xE5,
    PUSH_AF = 0xF5,

    ADD_d8 { d8: u8 } = 0xC6,
    SUB_d8 { d8: u8 } = 0xD6,
    AND_d8 { d8: u8 } = 0xE6,
    OR_d8 { d8: u8 } = 0xF6,

    RST_0 = 0xC7,
    RST_2 = 0xD7,
    RST_4 = 0xE7,
    RST_6 = 0xF7,

    RET_Z = 0xC8,
    RET_C = 0xD8,
    ADD_SP_s8 { s8: i8 } = 0xE8,
    LDHL_SP_s8 { s8: i8 } = 0xF8,

    RET = 0xC9,
    RETI = 0xD9,
    JP_HL = 0xE9,
    LD_SP_HL = 0xF9,

    JP_Z_a16 { a16: u16 } = 0xCA,
    JP_C_a16 { a16: u16 } = 0xDA,
    LD_a16_addr_A { a16: u16 } = 0xEA,
    LD_A_a16_addr { a16: u16 } = 0xFA,

    PrefixedInstruction { instruction: PrefixedInstruction } = 0xCB,
    // 0xDB
    // 0xEB
    EI = 0xFB,

    CALL_Z_a16 { a16: u16 } = 0xCC,
    CALL_C_a16 { a16: u16 } = 0xDC,
    // 0xEC
    // 0xFC

    CALL_a16 { a16: u16 } = 0xCD,
    // 0xDD
    // 0xED
    // 0xFD

    ADC_d8 { d8: u8 } = 0xCE,
    SBC_d8 { d8: u8 } = 0xDE,
    XOR_d8 { d8: u8 } = 0xEE,
    CP_d8 { d8: u8 } = 0xFE,

    RST_1 = 0xCF,
    RST_3 = 0xDF,
    RST_5 = 0xEF,
    RST_7 = 0xFF,
}

impl Instruction {
    /// Return the opcode of the instruction.
    #[must_use]
    #[inline]
    pub const fn opcode(self) -> u8 {
        // SAFETY: Opcode is repr(u8), so reading the first 8 bits to get the discriminant is safe.
        unsafe { *(&self as *const _ as *const u8) }
    }

    /// Parse the binary.
    ///
    /// It may be variable-length (i.e. if it takes an immediate value or if it is prefixed). All
    /// instructions can be assumed to be between 1 and 3 bytes.
    pub fn from_binary(binary: &[u8]) -> Result<Self, ReadInstructionError> {
        let Some(base_opcode) = binary.first().copied() else {
            return Err(ReadInstructionError::EmptyOpcode)
        };

        let mut variable_length = false;

        macro_rules! immediate_d8 {
            () => {{
                variable_length = true;
                let &[_, u8] = binary else {
                    return Err(ReadInstructionError::InvalidLength { given: binary.len(), expected: 2 })
                };
                Ok(u8)
            }};
        }

        macro_rules! immediate_s8 {
            () => {{
                variable_length = true;
                let &[_, u8] = binary else {
                    return Err(ReadInstructionError::InvalidLength { given: binary.len(), expected: 2 })
                };
                Ok(u8 as i8)
            }};
        }

        macro_rules! immediate_d16 {
            () => {{
                variable_length = true;
                let &[_, low, high] = binary else {
                    return Err(ReadInstructionError::InvalidLength { given: binary.len(), expected: 3 })
                };
                Ok(u16::from_le_bytes([low, high]))
            }};
        }

        let instruction = match base_opcode {
            0x00 => Ok(Self::NOP),
            0x10 => Ok(Self::STOP),
            0x20 => Ok(Self::JR_NZ_r8 { r8: immediate_s8!()? }),
            0x30 => Ok(Self::JR_NC_r8 { r8: immediate_s8!()? }),
            0x01 => Ok(Self::LD_BC_d16 { d16: immediate_d16!()? }),
            0x11 => Ok(Self::LD_DE_d16 { d16: immediate_d16!()? }),
            0x21 => Ok(Self::LD_HL_d16 { d16: immediate_d16!()? }),
            0x31 => Ok(Self::LD_SP_d16 { d16: immediate_d16!()? }),
            0x02 => Ok(Self::LD_BC_addr_A),
            0x12 => Ok(Self::LD_DE_addr_A),
            0x22 => Ok(Self::LD_HLi_addr_A),
            0x32 => Ok(Self::LD_HLd_addr_A),
            0x03 => Ok(Self::INC_BC),
            0x13 => Ok(Self::INC_DE),
            0x23 => Ok(Self::INC_HL),
            0x33 => Ok(Self::INC_SP),
            0x04 => Ok(Self::INC_B),
            0x14 => Ok(Self::INC_D),
            0x24 => Ok(Self::INC_H),
            0x34 => Ok(Self::INC_HL_addr),
            0x05 => Ok(Self::DEC_B),
            0x15 => Ok(Self::DEC_D),
            0x25 => Ok(Self::DEC_H),
            0x35 => Ok(Self::DEC_HL_addr),
            0x06 => Ok(Self::LD_B_d8 { d8: immediate_d8!()? }),
            0x16 => Ok(Self::LD_D_d8 { d8: immediate_d8!()? }),
            0x26 => Ok(Self::LD_H_d8 { d8: immediate_d8!()? }),
            0x36 => Ok(Self::LD_HL_addr_d8 { d8: immediate_d8!()? }),
            0x07 => Ok(Self::RLCA),
            0x17 => Ok(Self::RLA),
            0x27 => Ok(Self::DAA),
            0x37 => Ok(Self::SCF),
            0x08 => Ok(Self::LD_a16_addr_SP { a16: immediate_d16!()? }),
            0x18 => Ok(Self::JR_r8 { r8: immediate_s8!()? }),
            0x28 => Ok(Self::JR_Z_r8 { r8: immediate_s8!()? }),
            0x38 => Ok(Self::JR_C_r8 { r8: immediate_s8!()? }),
            0x09 => Ok(Self::ADD_HL_BC),
            0x19 => Ok(Self::ADD_HL_DE),
            0x29 => Ok(Self::ADD_HL_HL),
            0x39 => Ok(Self::ADD_HL_SP),
            0x0A => Ok(Self::LD_A_BC_addr),
            0x1A => Ok(Self::LD_A_DE_addr),
            0x2A => Ok(Self::LD_A_HLi_addr),
            0x3A => Ok(Self::LD_A_HLd_addr),
            0x0B => Ok(Self::DEC_BC),
            0x1B => Ok(Self::DEC_DE),
            0x2B => Ok(Self::DEC_HL),
            0x3B => Ok(Self::DEC_SP),
            0x0C => Ok(Self::INC_C),
            0x1C => Ok(Self::INC_E),
            0x2C => Ok(Self::INC_L),
            0x3C => Ok(Self::INC_A),
            0x0D => Ok(Self::DEC_C),
            0x1D => Ok(Self::DEC_E),
            0x2D => Ok(Self::DEC_L),
            0x3D => Ok(Self::DEC_A),
            0x0E => Ok(Self::LD_C_d8 { d8: immediate_d8!()? }),
            0x1E => Ok(Self::LD_E_d8 { d8: immediate_d8!()? }),
            0x2E => Ok(Self::LD_L_d8 { d8: immediate_d8!()? }),
            0x3E => Ok(Self::LD_A_d8 { d8: immediate_d8!()? }),
            0x0F => Ok(Self::RRCA),
            0x1F => Ok(Self::RRA),
            0x2F => Ok(Self::CPL),
            0x3F => Ok(Self::CCF),
            0x40 => Ok(Self::LD_B_B),
            0x50 => Ok(Self::LD_D_B),
            0x60 => Ok(Self::LD_H_B),
            0x70 => Ok(Self::LD_HL_addr_B),
            0x41 => Ok(Self::LD_B_C),
            0x51 => Ok(Self::LD_D_C),
            0x61 => Ok(Self::LD_H_C),
            0x71 => Ok(Self::LD_HL_addr_C),
            0x42 => Ok(Self::LD_B_D),
            0x52 => Ok(Self::LD_D_D),
            0x62 => Ok(Self::LD_H_D),
            0x72 => Ok(Self::LD_HL_addr_D),
            0x43 => Ok(Self::LD_B_E),
            0x53 => Ok(Self::LD_D_E),
            0x63 => Ok(Self::LD_H_E),
            0x73 => Ok(Self::LD_HL_addr_E),
            0x44 => Ok(Self::LD_B_H),
            0x54 => Ok(Self::LD_D_H),
            0x64 => Ok(Self::LD_H_H),
            0x74 => Ok(Self::LD_HL_addr_H),
            0x45 => Ok(Self::LD_B_L),
            0x55 => Ok(Self::LD_D_L),
            0x65 => Ok(Self::LD_H_L),
            0x75 => Ok(Self::LD_HL_addr_L),
            0x46 => Ok(Self::LD_B_HL_addr),
            0x56 => Ok(Self::LD_D_HL_addr),
            0x66 => Ok(Self::LD_H_HL_addr),
            0x76 => Ok(Self::HALT),
            0x47 => Ok(Self::LD_B_A),
            0x57 => Ok(Self::LD_D_A),
            0x67 => Ok(Self::LD_H_A),
            0x77 => Ok(Self::LD_HL_addr_A),
            0x48 => Ok(Self::LD_C_B),
            0x58 => Ok(Self::LD_E_B),
            0x68 => Ok(Self::LD_L_B),
            0x78 => Ok(Self::LD_A_B),
            0x49 => Ok(Self::LD_C_C),
            0x59 => Ok(Self::LD_E_C),
            0x69 => Ok(Self::LD_L_C),
            0x79 => Ok(Self::LD_A_C),
            0x4A => Ok(Self::LD_C_D),
            0x5A => Ok(Self::LD_E_D),
            0x6A => Ok(Self::LD_L_D),
            0x7A => Ok(Self::LD_A_D),
            0x4B => Ok(Self::LD_C_E),
            0x5B => Ok(Self::LD_E_E),
            0x6B => Ok(Self::LD_L_E),
            0x7B => Ok(Self::LD_A_E),
            0x4C => Ok(Self::LD_C_H),
            0x5C => Ok(Self::LD_E_H),
            0x6C => Ok(Self::LD_L_H),
            0x7C => Ok(Self::LD_A_H),
            0x4D => Ok(Self::LD_C_L),
            0x5D => Ok(Self::LD_E_L),
            0x6D => Ok(Self::LD_L_L),
            0x7D => Ok(Self::LD_A_L),
            0x4E => Ok(Self::LD_C_HL_addr),
            0x5E => Ok(Self::LD_E_HL_addr),
            0x6E => Ok(Self::LD_L_HL_addr),
            0x7E => Ok(Self::LD_A_HL_addr),
            0x4F => Ok(Self::LD_C_A),
            0x5F => Ok(Self::LD_E_A),
            0x6F => Ok(Self::LD_L_A),
            0x7F => Ok(Self::LD_A_A),
            0x80 => Ok(Self::ADD_B),
            0x90 => Ok(Self::SUB_B),
            0xA0 => Ok(Self::AND_B),
            0xB0 => Ok(Self::OR_B),
            0x81 => Ok(Self::ADD_C),
            0x91 => Ok(Self::SUB_C),
            0xA1 => Ok(Self::AND_C),
            0xB1 => Ok(Self::OR_C),
            0x82 => Ok(Self::ADD_D),
            0x92 => Ok(Self::SUB_D),
            0xA2 => Ok(Self::AND_D),
            0xB2 => Ok(Self::OR_D),
            0x83 => Ok(Self::ADD_E),
            0x93 => Ok(Self::SUB_E),
            0xA3 => Ok(Self::AND_E),
            0xB3 => Ok(Self::OR_E),
            0x84 => Ok(Self::ADD_H),
            0x94 => Ok(Self::SUB_H),
            0xA4 => Ok(Self::AND_H),
            0xB4 => Ok(Self::OR_H),
            0x85 => Ok(Self::ADD_L),
            0x95 => Ok(Self::SUB_L),
            0xA5 => Ok(Self::AND_L),
            0xB5 => Ok(Self::OR_L),
            0x86 => Ok(Self::ADD_HL_addr),
            0x96 => Ok(Self::SUB_HL_addr),
            0xA6 => Ok(Self::AND_HL_addr),
            0xB6 => Ok(Self::OR_HL_addr),
            0x87 => Ok(Self::ADD_A),
            0x97 => Ok(Self::SUB_A),
            0xA7 => Ok(Self::AND_A),
            0xB7 => Ok(Self::OR_A),
            0x88 => Ok(Self::ADC_B),
            0x98 => Ok(Self::SBC_B),
            0xA8 => Ok(Self::XOR_B),
            0xB8 => Ok(Self::CP_B),
            0x89 => Ok(Self::ADC_C),
            0x99 => Ok(Self::SBC_C),
            0xA9 => Ok(Self::XOR_C),
            0xB9 => Ok(Self::CP_C),
            0x8A => Ok(Self::ADC_D),
            0x9A => Ok(Self::SBC_D),
            0xAA => Ok(Self::XOR_D),
            0xBA => Ok(Self::CP_D),
            0x8B => Ok(Self::ADC_E),
            0x9B => Ok(Self::SBC_E),
            0xAB => Ok(Self::XOR_E),
            0xBB => Ok(Self::CP_E),
            0x8C => Ok(Self::ADC_H),
            0x9C => Ok(Self::SBC_H),
            0xAC => Ok(Self::XOR_H),
            0xBC => Ok(Self::CP_H),
            0x8D => Ok(Self::ADC_L),
            0x9D => Ok(Self::SBC_L),
            0xAD => Ok(Self::XOR_L),
            0xBD => Ok(Self::CP_L),
            0x8E => Ok(Self::ADC_HL_addr),
            0x9E => Ok(Self::SBC_HL_addr),
            0xAE => Ok(Self::XOR_HL_addr),
            0xBE => Ok(Self::CP_HL_addr),
            0x8F => Ok(Self::ADC_A),
            0x9F => Ok(Self::SBC_A),
            0xAF => Ok(Self::XOR_A),
            0xBF => Ok(Self::CP_A),
            0xC0 => Ok(Self::RET_NZ),
            0xD0 => Ok(Self::RET_NC),
            0xE0 => Ok(Self::LD_a8_addr_A { a8: immediate_d8!()? }),
            0xF0 => Ok(Self::LD_A_a8_addr { a8: immediate_d8!()? }),
            0xC1 => Ok(Self::POP_BC),
            0xD1 => Ok(Self::POP_DE),
            0xE1 => Ok(Self::POP_HL),
            0xF1 => Ok(Self::POP_AF),
            0xC2 => Ok(Self::JP_NZ_a16 { a16: immediate_d16!()? }),
            0xD2 => Ok(Self::JP_NC_a16 { a16: immediate_d16!()? }),
            0xE2 => Ok(Self::LD_C_addr_A),
            0xF2 => Ok(Self::LD_A_C_addr),
            0xC3 => Ok(Self::JP_A16 { a16: immediate_d16!()? }),
            0xF3 => Ok(Self::DI),
            0xC4 => Ok(Self::CALL_NZ_a16 { a16: immediate_d16!()? }),
            0xD4 => Ok(Self::CALL_NC_a16 { a16: immediate_d16!()? }),
            0xC5 => Ok(Self::PUSH_BC),
            0xD5 => Ok(Self::PUSH_DE),
            0xE5 => Ok(Self::PUSH_HL),
            0xF5 => Ok(Self::PUSH_AF),
            0xC6 => Ok(Self::ADD_d8 { d8: immediate_d8!()? }),
            0xD6 => Ok(Self::SUB_d8 { d8: immediate_d8!()? }),
            0xE6 => Ok(Self::AND_d8 { d8: immediate_d8!()? }),
            0xF6 => Ok(Self::OR_d8 { d8: immediate_d8!()? }),
            0xC7 => Ok(Self::RST_0),
            0xD7 => Ok(Self::RST_2),
            0xE7 => Ok(Self::RST_4),
            0xF7 => Ok(Self::RST_6),
            0xC8 => Ok(Self::RET_Z),
            0xD8 => Ok(Self::RET_C),
            0xE8 => Ok(Self::ADD_SP_s8 { s8: immediate_s8!()? }),
            0xF8 => Ok(Self::LDHL_SP_s8 { s8: immediate_s8!()? }),
            0xC9 => Ok(Self::RET),
            0xD9 => Ok(Self::RETI),
            0xE9 => Ok(Self::JP_HL),
            0xF9 => Ok(Self::LD_SP_HL),
            0xCA => Ok(Self::JP_Z_a16 { a16: immediate_d16!()? }),
            0xDA => Ok(Self::JP_C_a16 { a16: immediate_d16!()? }),
            0xEA => Ok(Self::LD_a16_addr_A { a16: immediate_d16!()? }),
            0xFA => Ok(Self::LD_A_a16_addr { a16: immediate_d16!()? }),
            0xCB => Ok(Self::PrefixedInstruction { instruction: PrefixedInstruction::from_opcode(immediate_d8!()?) }),
            0xFB => Ok(Self::EI),
            0xCC => Ok(Self::CALL_Z_a16 { a16: immediate_d16!()? }),
            0xDC => Ok(Self::CALL_C_a16 { a16: immediate_d16!()? }),
            0xCD => Ok(Self::CALL_a16 { a16: immediate_d16!()? }),
            0xCE => Ok(Self::ADC_d8 { d8: immediate_d8!()? }),
            0xDE => Ok(Self::SBC_d8 { d8: immediate_d8!()? }),
            0xEE => Ok(Self::XOR_d8 { d8: immediate_d8!()? }),
            0xFE => Ok(Self::CP_d8 { d8: immediate_d8!()? }),
            0xCF => Ok(Self::RST_1),
            0xDF => Ok(Self::RST_3),
            0xEF => Ok(Self::RST_5),
            0xFF => Ok(Self::RST_7),

            invalid => Err(ReadInstructionError::InvalidOpcode { opcode: invalid })
        }?;

        if !variable_length && binary.len() != 1 {
            return Err(ReadInstructionError::InvalidLength { expected: 1, given: binary.len() })
        }

        Ok(instruction)
    }

    /// Convert the instruction into binary.
    #[must_use]
    pub const fn into_binary(self) -> BinaryInstruction {
        let opcode = self.opcode();

        match self {
            Self::NOP
            | Self::STOP
            | Self::LD_BC_addr_A
            | Self::LD_DE_addr_A
            | Self::LD_HLi_addr_A
            | Self::LD_HLd_addr_A
            | Self::INC_BC
            | Self::INC_DE
            | Self::INC_HL
            | Self::INC_SP
            | Self::INC_B
            | Self::INC_D
            | Self::INC_H
            | Self::INC_HL_addr
            | Self::DEC_B
            | Self::DEC_D
            | Self::DEC_H
            | Self::DEC_HL_addr
            | Self::RLCA
            | Self::RLA
            | Self::DAA
            | Self::SCF
            | Self::ADD_HL_BC
            | Self::ADD_HL_DE
            | Self::ADD_HL_HL
            | Self::ADD_HL_SP
            | Self::LD_A_BC_addr
            | Self::LD_A_DE_addr
            | Self::LD_A_HLi_addr
            | Self::LD_A_HLd_addr
            | Self::DEC_BC
            | Self::DEC_DE
            | Self::DEC_HL
            | Self::DEC_SP
            | Self::INC_C
            | Self::INC_E
            | Self::INC_L
            | Self::INC_A
            | Self::DEC_C
            | Self::DEC_E
            | Self::DEC_L
            | Self::DEC_A
            | Self::RRCA
            | Self::RRA
            | Self::CPL
            | Self::CCF
            | Self::LD_B_B
            | Self::LD_D_B
            | Self::LD_H_B
            | Self::LD_HL_addr_B
            | Self::LD_B_C
            | Self::LD_D_C
            | Self::LD_H_C
            | Self::LD_HL_addr_C
            | Self::LD_B_D
            | Self::LD_D_D
            | Self::LD_H_D
            | Self::LD_HL_addr_D
            | Self::LD_B_E
            | Self::LD_D_E
            | Self::LD_H_E
            | Self::LD_HL_addr_E
            | Self::LD_B_H
            | Self::LD_D_H
            | Self::LD_H_H
            | Self::LD_HL_addr_H
            | Self::LD_B_L
            | Self::LD_D_L
            | Self::LD_H_L
            | Self::LD_HL_addr_L
            | Self::LD_B_HL_addr
            | Self::LD_D_HL_addr
            | Self::LD_H_HL_addr
            | Self::HALT
            | Self::LD_B_A
            | Self::LD_D_A
            | Self::LD_H_A
            | Self::LD_HL_addr_A
            | Self::LD_C_B
            | Self::LD_E_B
            | Self::LD_L_B
            | Self::LD_A_B
            | Self::LD_C_C
            | Self::LD_E_C
            | Self::LD_L_C
            | Self::LD_A_C
            | Self::LD_C_D
            | Self::LD_E_D
            | Self::LD_L_D
            | Self::LD_A_D
            | Self::LD_C_E
            | Self::LD_E_E
            | Self::LD_L_E
            | Self::LD_A_E
            | Self::LD_C_H
            | Self::LD_E_H
            | Self::LD_L_H
            | Self::LD_A_H
            | Self::LD_C_L
            | Self::LD_E_L
            | Self::LD_L_L
            | Self::LD_A_L
            | Self::LD_C_HL_addr
            | Self::LD_E_HL_addr
            | Self::LD_L_HL_addr
            | Self::LD_A_HL_addr
            | Self::LD_C_A
            | Self::LD_E_A
            | Self::LD_L_A
            | Self::LD_A_A
            | Self::ADD_B
            | Self::SUB_B
            | Self::AND_B
            | Self::OR_B
            | Self::ADD_C
            | Self::SUB_C
            | Self::AND_C
            | Self::OR_C
            | Self::ADD_D
            | Self::SUB_D
            | Self::AND_D
            | Self::OR_D
            | Self::ADD_E
            | Self::SUB_E
            | Self::AND_E
            | Self::OR_E
            | Self::ADD_H
            | Self::SUB_H
            | Self::AND_H
            | Self::OR_H
            | Self::ADD_L
            | Self::SUB_L
            | Self::AND_L
            | Self::OR_L
            | Self::ADD_HL_addr
            | Self::SUB_HL_addr
            | Self::AND_HL_addr
            | Self::OR_HL_addr
            | Self::ADD_A
            | Self::SUB_A
            | Self::AND_A
            | Self::OR_A
            | Self::ADC_B
            | Self::SBC_B
            | Self::XOR_B
            | Self::CP_B
            | Self::ADC_C
            | Self::SBC_C
            | Self::XOR_C
            | Self::CP_C
            | Self::ADC_D
            | Self::SBC_D
            | Self::XOR_D
            | Self::CP_D
            | Self::ADC_E
            | Self::SBC_E
            | Self::XOR_E
            | Self::CP_E
            | Self::ADC_H
            | Self::SBC_H
            | Self::XOR_H
            | Self::CP_H
            | Self::ADC_L
            | Self::SBC_L
            | Self::XOR_L
            | Self::CP_L
            | Self::ADC_HL_addr
            | Self::SBC_HL_addr
            | Self::XOR_HL_addr
            | Self::CP_HL_addr
            | Self::ADC_A
            | Self::SBC_A
            | Self::XOR_A
            | Self::CP_A
            | Self::RET_NZ
            | Self::RET_NC
            | Self::POP_BC
            | Self::POP_DE
            | Self::POP_HL
            | Self::POP_AF
            | Self::LD_C_addr_A
            | Self::LD_A_C_addr
            | Self::DI
            | Self::PUSH_BC
            | Self::PUSH_DE
            | Self::PUSH_HL
            | Self::PUSH_AF
            | Self::RST_0
            | Self::RST_2
            | Self::RST_4
            | Self::RST_6
            | Self::RET_Z
            | Self::RET_C
            | Self::RET
            | Self::RETI
            | Self::JP_HL
            | Self::LD_SP_HL
            | Self::EI
            | Self::RST_1
            | Self::RST_3
            | Self::RST_5
            | Self::RST_7 => BinaryInstruction::single_byte(opcode),

            Self::PrefixedInstruction { instruction } => BinaryInstruction::with_immediate8(opcode, instruction.opcode()),

            Self::CALL_a16 { a16: d16 }
            | Self::CALL_Z_a16 { a16: d16 }
            | Self::CALL_C_a16 { a16: d16 }
            | Self::JP_Z_a16 { a16: d16 }
            | Self::JP_C_a16 { a16: d16 }
            | Self::LD_a16_addr_A { a16: d16 }
            | Self::LD_A_a16_addr { a16: d16 }
            | Self::LD_a16_addr_SP { a16: d16 }
            | Self::JP_NZ_a16 { a16: d16 }
            | Self::JP_NC_a16 { a16: d16 }
            | Self::CALL_NZ_a16 { a16: d16 }
            | Self::JP_A16 { a16: d16 }
            | Self::CALL_NC_a16 { a16: d16 }
            | Self::LD_BC_d16 { d16 }
            | Self::LD_DE_d16 { d16 }
            | Self::LD_HL_d16 { d16 }
            | Self::LD_SP_d16 { d16 } => BinaryInstruction::with_immediate16(opcode, d16),

            Self::ADD_SP_s8 { s8 }
            | Self::LDHL_SP_s8 { s8 }
            | Self::JR_NZ_r8 { r8: s8 }
            | Self::JR_NC_r8 { r8: s8 }
            | Self::JR_r8 { r8: s8 }
            | Self::JR_Z_r8 { r8: s8 }
            | Self::JR_C_r8 { r8: s8 } => BinaryInstruction::with_immediate_signed8(opcode, s8),

            Self::ADC_d8 { d8 }
            | Self::SBC_d8 { d8 }
            | Self::XOR_d8 { d8 }
            | Self::CP_d8 { d8 }
            | Self::LD_B_d8 { d8 }
            | Self::LD_D_d8 { d8 }
            | Self::LD_H_d8 { d8 }
            | Self::LD_HL_addr_d8 { d8 }
            | Self::LD_C_d8 { d8 }
            | Self::LD_E_d8 { d8 }
            | Self::LD_L_d8 { d8 }
            | Self::LD_A_d8 { d8 }
            | Self::LD_a8_addr_A { a8: d8 }
            | Self::LD_A_a8_addr { a8: d8 }
            | Self::ADD_d8 { d8 }
            | Self::SUB_d8 { d8 }
            | Self::AND_d8 { d8 }
            | Self::OR_d8 { d8 } => BinaryInstruction::with_immediate8(opcode, d8),
        }
    }

    /// Parse a single line.
    #[expect(unused)]
    pub fn parse_line(string: &str, syntax_dialect: SyntaxDialect) -> Result<Self, InstructionParseError> {
        todo!()
    }

    /// Return an object that can display this instruction.
    #[must_use]
    pub fn display<'a, F: Fn(u16) -> Option<&'a str>>(self, config: &'a FormatConfig<'a, F>, pc: u16) -> InstructionDisplay<'a, F> {
        InstructionDisplay {
            instruction: self,
            config,
            pc
        }
    }
}


#[derive(Copy, Clone, PartialEq)]
pub struct BinaryInstruction {
    bytes: [u8; 3],
    length: u8
}

impl Debug for BinaryInstruction {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self.get_bytes(), f)
    }
}

impl BinaryInstruction {
    #[inline]
    #[must_use]
    pub const fn get_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.bytes.as_ptr(), self.length as usize) }
    }

    #[inline]
    #[must_use]
    const fn single_byte(opcode: u8) -> BinaryInstruction {
        BinaryInstruction {
            bytes: [opcode, 0, 0],
            length: 1
        }
    }

    #[inline]
    #[must_use]
    const fn with_immediate8(opcode: u8, immediate: u8) -> BinaryInstruction {
        BinaryInstruction {
            bytes: [opcode, immediate, 0],
            length: 2
        }
    }

    #[inline]
    #[must_use]
    const fn with_immediate_signed8(opcode: u8, immediate: i8) -> BinaryInstruction {
        BinaryInstruction {
            bytes: [opcode, immediate as u8, 0],
            length: 2
        }
    }

    #[inline]
    #[must_use]
    const fn with_immediate16(opcode: u8, immediate: u16) -> BinaryInstruction {
        let [low, high] = immediate.to_le_bytes();

        BinaryInstruction {
            bytes: [opcode, low, high],
            length: 3
        }
    }
}

/// Instruction read error.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReadInstructionError {
    /// An invalid opcode was given.
    InvalidOpcode {
        /// The invalid opcode.
        opcode: u8
    },

    /// A certain number of bytes are needed for this opcode.
    InvalidLength {
        /// Given number of bytes.
        given: usize,

        /// Expected number of bytes (between 1 and 3).
        expected: u8
    },

    /// No bytes were given for the opcode.
    EmptyOpcode
}

/// These are 16-bit instructions prefixed with `0xCB`.
#[allow(nonstandard_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum PrefixedInstruction {
    RLC_B = 0o000,
    RLC_C = 0o001,
    RLC_D = 0o002,
    RLC_E = 0o003,
    RLC_H = 0o004,
    RLC_L = 0o005,
    RLC_HL_addr = 0o006,
    RLC_A = 0o007,

    RRC_B = 0o010,
    RRC_C = 0o011,
    RRC_D = 0o012,
    RRC_E = 0o013,
    RRC_H = 0o014,
    RRC_L = 0o015,
    RRC_HL_addr = 0o016,
    RRC_A = 0o017,

    RL_B = 0o020,
    RL_C = 0o021,
    RL_D = 0o022,
    RL_E = 0o023,
    RL_H = 0o024,
    RL_L = 0o025,
    RL_HL_addr = 0o026,
    RL_A = 0o027,

    RR_B = 0o030,
    RR_C = 0o031,
    RR_D = 0o032,
    RR_E = 0o033,
    RR_H = 0o034,
    RR_L = 0o035,
    RR_HL_addr = 0o036,
    RR_A = 0o037,

    SLA_B = 0o040,
    SLA_C = 0o041,
    SLA_D = 0o042,
    SLA_E = 0o043,
    SLA_H = 0o044,
    SLA_L = 0o045,
    SLA_HL_addr = 0o046,
    SLA_A = 0o047,

    SRA_B = 0o050,
    SRA_C = 0o051,
    SRA_D = 0o052,
    SRA_E = 0o053,
    SRA_H = 0o054,
    SRA_L = 0o055,
    SRA_HL_addr = 0o056,
    SRA_A = 0o057,

    SWAP_B = 0o060,
    SWAP_C = 0o061,
    SWAP_D = 0o062,
    SWAP_E = 0o063,
    SWAP_H = 0o064,
    SWAP_L = 0o065,
    SWAP_HL_addr = 0o066,
    SWAP_A = 0o067,

    SRL_B = 0o070,
    SRL_C = 0o071,
    SRL_D = 0o072,
    SRL_E = 0o073,
    SRL_H = 0o074,
    SRL_L = 0o075,
    SRL_HL_addr = 0o076,
    SRL_A = 0o077,

    BIT_0_B = 0o100,
    BIT_0_C = 0o101,
    BIT_0_D = 0o102,
    BIT_0_E = 0o103,
    BIT_0_H = 0o104,
    BIT_0_L = 0o105,
    BIT_0_HL_addr = 0o106,
    BIT_0_A = 0o107,

    BIT_1_B = 0o110,
    BIT_1_C = 0o111,
    BIT_1_D = 0o112,
    BIT_1_E = 0o113,
    BIT_1_H = 0o114,
    BIT_1_L = 0o115,
    BIT_1_HL_addr = 0o116,
    BIT_1_A = 0o117,

    BIT_2_B = 0o120,
    BIT_2_C = 0o121,
    BIT_2_D = 0o122,
    BIT_2_E = 0o123,
    BIT_2_H = 0o124,
    BIT_2_L = 0o125,
    BIT_2_HL_addr = 0o126,
    BIT_2_A = 0o127,

    BIT_3_B = 0o130,
    BIT_3_C = 0o131,
    BIT_3_D = 0o132,
    BIT_3_E = 0o133,
    BIT_3_H = 0o134,
    BIT_3_L = 0o135,
    BIT_3_HL_addr = 0o136,
    BIT_3_A = 0o137,

    BIT_4_B = 0o140,
    BIT_4_C = 0o141,
    BIT_4_D = 0o142,
    BIT_4_E = 0o143,
    BIT_4_H = 0o144,
    BIT_4_L = 0o145,
    BIT_4_HL_addr = 0o146,
    BIT_4_A = 0o147,

    BIT_5_B = 0o150,
    BIT_5_C = 0o151,
    BIT_5_D = 0o152,
    BIT_5_E = 0o153,
    BIT_5_H = 0o154,
    BIT_5_L = 0o155,
    BIT_5_HL_addr = 0o156,
    BIT_5_A = 0o157,

    BIT_6_B = 0o160,
    BIT_6_C = 0o161,
    BIT_6_D = 0o162,
    BIT_6_E = 0o163,
    BIT_6_H = 0o164,
    BIT_6_L = 0o165,
    BIT_6_HL_addr = 0o166,
    BIT_6_A = 0o167,

    BIT_7_B = 0o170,
    BIT_7_C = 0o171,
    BIT_7_D = 0o172,
    BIT_7_E = 0o173,
    BIT_7_H = 0o174,
    BIT_7_L = 0o175,
    BIT_7_HL_addr = 0o176,
    BIT_7_A = 0o177,

    RES_0_B = 0o200,
    RES_0_C = 0o201,
    RES_0_D = 0o202,
    RES_0_E = 0o203,
    RES_0_H = 0o204,
    RES_0_L = 0o205,
    RES_0_HL_addr = 0o206,
    RES_0_A = 0o207,

    RES_1_B = 0o210,
    RES_1_C = 0o211,
    RES_1_D = 0o212,
    RES_1_E = 0o213,
    RES_1_H = 0o214,
    RES_1_L = 0o215,
    RES_1_HL_addr = 0o216,
    RES_1_A = 0o217,

    RES_2_B = 0o220,
    RES_2_C = 0o221,
    RES_2_D = 0o222,
    RES_2_E = 0o223,
    RES_2_H = 0o224,
    RES_2_L = 0o225,
    RES_2_HL_addr = 0o226,
    RES_2_A = 0o227,

    RES_3_B = 0o230,
    RES_3_C = 0o231,
    RES_3_D = 0o232,
    RES_3_E = 0o233,
    RES_3_H = 0o234,
    RES_3_L = 0o235,
    RES_3_HL_addr = 0o236,
    RES_3_A = 0o237,

    RES_4_B = 0o240,
    RES_4_C = 0o241,
    RES_4_D = 0o242,
    RES_4_E = 0o243,
    RES_4_H = 0o244,
    RES_4_L = 0o245,
    RES_4_HL_addr = 0o246,
    RES_4_A = 0o247,

    RES_5_B = 0o250,
    RES_5_C = 0o251,
    RES_5_D = 0o252,
    RES_5_E = 0o253,
    RES_5_H = 0o254,
    RES_5_L = 0o255,
    RES_5_HL_addr = 0o256,
    RES_5_A = 0o257,

    RES_6_B = 0o260,
    RES_6_C = 0o261,
    RES_6_D = 0o262,
    RES_6_E = 0o263,
    RES_6_H = 0o264,
    RES_6_L = 0o265,
    RES_6_HL_addr = 0o266,
    RES_6_A = 0o267,

    RES_7_B = 0o270,
    RES_7_C = 0o271,
    RES_7_D = 0o272,
    RES_7_E = 0o273,
    RES_7_H = 0o274,
    RES_7_L = 0o275,
    RES_7_HL_addr = 0o276,
    RES_7_A = 0o277,

    SET_0_B = 0o300,
    SET_0_C = 0o301,
    SET_0_D = 0o302,
    SET_0_E = 0o303,
    SET_0_H = 0o304,
    SET_0_L = 0o305,
    SET_0_HL_addr = 0o306,
    SET_0_A = 0o307,

    SET_1_B = 0o310,
    SET_1_C = 0o311,
    SET_1_D = 0o312,
    SET_1_E = 0o313,
    SET_1_H = 0o314,
    SET_1_L = 0o315,
    SET_1_HL_addr = 0o316,
    SET_1_A = 0o317,

    SET_2_B = 0o320,
    SET_2_C = 0o321,
    SET_2_D = 0o322,
    SET_2_E = 0o323,
    SET_2_H = 0o324,
    SET_2_L = 0o325,
    SET_2_HL_addr = 0o326,
    SET_2_A = 0o327,

    SET_3_B = 0o330,
    SET_3_C = 0o331,
    SET_3_D = 0o332,
    SET_3_E = 0o333,
    SET_3_H = 0o334,
    SET_3_L = 0o335,
    SET_3_HL_addr = 0o336,
    SET_3_A = 0o337,

    SET_4_B = 0o340,
    SET_4_C = 0o341,
    SET_4_D = 0o342,
    SET_4_E = 0o343,
    SET_4_H = 0o344,
    SET_4_L = 0o345,
    SET_4_HL_addr = 0o346,
    SET_4_A = 0o347,

    SET_5_B = 0o350,
    SET_5_C = 0o351,
    SET_5_D = 0o352,
    SET_5_E = 0o353,
    SET_5_H = 0o354,
    SET_5_L = 0o355,
    SET_5_HL_addr = 0o356,
    SET_5_A = 0o357,

    SET_6_B = 0o360,
    SET_6_C = 0o361,
    SET_6_D = 0o362,
    SET_6_E = 0o363,
    SET_6_H = 0o364,
    SET_6_L = 0o365,
    SET_6_HL_addr = 0o366,
    SET_6_A = 0o367,

    SET_7_B = 0o370,
    SET_7_C = 0o371,
    SET_7_D = 0o372,
    SET_7_E = 0o373,
    SET_7_H = 0o374,
    SET_7_L = 0o375,
    SET_7_HL_addr = 0o376,
    SET_7_A = 0o377,
}

impl PrefixedInstruction {
    /// Return the opcode of the instruction.
    #[must_use]
    #[inline]
    pub const fn opcode(self) -> u8 {
        self as u8
    }

    /// Convert the opcode to its instruction.
    ///
    /// This is the byte after the `0xCB`.
    #[must_use]
    #[inline]
    pub const fn from_opcode(opcode: u8) -> Self {
        // SAFETY: Every u8 value is accounted for.
        unsafe { core::mem::transmute(opcode) }
    }
}

/// Iterator that takes a byte iterator.
///
/// ## Behavior of `self.next()`
///
/// The iterator will call `inner.next()` as many times as necessary until it returns `None` or
/// returns enough bytes for a complete instruction. The instruction buffer will persist between
/// calls, thus it is valid for `inner.next()` to return `None` temporarily even mid-instruction.
///
/// The only possible values are:
/// * `None` - `inner.next()` returned `None`, or a decoding error occurred previously.
/// * `Some(Ok(Instruction))` - An instruction was decoded.
/// * `Some(Err(ReadInstructionError::InvalidOpcode))` - Invalid instruction; all future calls to
///   this method will return `None`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstructionDecoderByteIterator<B: Iterator<Item = u8>> {
    inner: B,
    buffer: [u8; 3],
    buffer_length: usize,
    last_error: Option<ReadInstructionError>
}

impl<I: Iterator<Item = u8>> InstructionDecoderByteIterator<I> {
    /// Wrap an existing byte iterator.
    #[inline]
    pub const fn new(inner: I) -> Self {
        Self {
            inner, buffer_length: 0, buffer: [0u8; 3], last_error: None
        }
    }

    /// Return `Some` with the last error.
    #[inline]
    pub const fn get_error(&self) -> Option<ReadInstructionError> {
        self.last_error
    }

    /// Get the inner iterator back.
    #[inline]
    pub fn into_inner(self) -> I {
        self.inner
    }
}

impl<I: Iterator<Item = u8>> From<I> for InstructionDecoderByteIterator<I> {
    fn from(inner: I) -> Self {
        Self::new(inner)
    }
}

impl<I: Iterator<Item = u8>> Iterator for InstructionDecoderByteIterator<I> {
    type Item = Result<Instruction, ReadInstructionError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.last_error.is_some() {
            return None
        }

        loop {
            self.buffer[self.buffer_length] = self.inner.next()?;
            self.buffer_length += 1;

            match Instruction::from_binary(&self.buffer[..self.buffer_length]) {
                Ok(n) => {
                    self.buffer_length = 0;
                    return Some(Ok(n))
                }
                Err(ReadInstructionError::EmptyOpcode) => unreachable!(),
                Err(ReadInstructionError::InvalidLength { .. }) => continue,
                Err(e) => {
                    self.last_error = Some(e);
                    return Some(Err(e))
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InstructionParseError {

}

/// Format config for formatting instructions.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FormatConfig<'a, F: Fn(u16) -> Option<&'a str>> {
    /// Syntax to use.
    pub syntax_dialect: SyntaxDialect,

    /// Preferred casing of all instructions.
    pub casing: Casing,

    /// Symbol resolver.
    pub resolve_symbol: F,

    /// Prefer to prefix hex with `0x` (may break compatibility with other tools)
    pub prefix_hex: bool,

    /// If the instruction name (e.g. POP) is fewer than this number of characters, pad it out with
    /// spaces.
    pub instruction_spacing: usize,

    /// Also attempt to resolve all 16-bit values.
    pub resolve_u16: bool
}

impl<'a, F: Fn(u16) -> Option<&'a str>> FormatConfig<'a, F> {
    pub fn default_with_symbol_resolver(resolver: F) -> FormatConfig<'a, F> {
        Self {
            syntax_dialect: SyntaxDialect::ISAS,
            casing: Casing::Lowercase,
            resolve_symbol: resolver,
            prefix_hex: false,
            instruction_spacing: 0,
            resolve_u16: false
        }
    }

    #[expect(unused)]
    pub(crate) fn write_hex_u8(&self, u8: u8, f: &mut Formatter) -> core::fmt::Result {
        if self.prefix_hex {
            f.write_fmt(format_args!("0x{u8:02X}"))
        }
        else {
            Display::fmt(&self.syntax_dialect.format_hex_u8(u8), f)
        }
    }

    pub(crate) fn write_hex_u16(&self, u16: u16, f: &mut Formatter) -> core::fmt::Result {
        if self.prefix_hex {
            f.write_fmt(format_args!("0x{u16:04X}"))
        }
        else {
            Display::fmt(&self.syntax_dialect.format_hex_u16(u16), f)
        }
    }

    #[expect(unused)]
    pub(crate) fn write_hex_u32(&self, u32: u32, f: &mut Formatter) -> core::fmt::Result {
        if self.prefix_hex {
            f.write_fmt(format_args!("0x{u32:08X}"))
        }
        else {
            Display::fmt(&self.syntax_dialect.format_hex_u32(u32), f)
        }
    }
}

impl Default for FormatConfig<'static, fn(u16) -> Option<&'static str>> {
    fn default() -> Self {
        Self::default_with_symbol_resolver(|_| None)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Casing {
    Lowercase,
    Uppercase
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::instruction::{Instruction, ReadInstructionError};

    #[test]
    fn round_trip_conversion() {
        for opcode in 0..0xFFu8 {
            let expected_number_of_bytes = match Instruction::from_binary(&[opcode]) {
                Ok(_) => 1,
                Err(ReadInstructionError::InvalidOpcode { .. }) => {
                    continue
                }
                Err(ReadInstructionError::InvalidLength { expected, .. }) => expected as usize,
                Err(ReadInstructionError::EmptyOpcode) => unreachable!()
            };

            let max_operand = match expected_number_of_bytes {
                1 => 0,
                2 => 255,
                3 => 65535,
                _ => unreachable!()
            };

            for operand in 0u16..=max_operand {
                let bytes = operand.to_le_bytes();
                let bytes = [opcode, bytes[0], bytes[1]];
                let bytes = &bytes[..expected_number_of_bytes];

                let Ok(instruction) = Instruction::from_binary(bytes) else {
                    continue;
                };

                assert_eq!(instruction.into_binary().get_bytes(), bytes, "{instruction:?} failed");
            }
        }
    }
}
