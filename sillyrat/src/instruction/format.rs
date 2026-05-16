use core::fmt::{Display, Formatter};
use super::*;
use core::stringify;

/// Display-able instruction.
pub struct InstructionDisplay<'a, F: Fn(u16) -> Option<&'a str>> {
    pub(crate) instruction: Instruction,
    pub(crate) config: &'a FormatConfig<'a, F>,
    pub(crate) pc: u16
}

enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,

    BC,
    DE,
    AF,
    HL,
    SP,

    HLi,
    HLd
}

enum Flag {
    NZ,
    Z,
    C,
    NC
}

impl<'a, F: Fn(u16) -> Option<&'a str>> Display for InstructionDisplay<'a, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        fn write_str_capsable<'a, F: Fn(u16) -> Option<&'a str>>(str: &str, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            match format_config.casing {
                Casing::Lowercase => f.write_str(str),
                Casing::Uppercase => {
                    for c in str.chars() {
                        for q in c.to_uppercase() {
                            f.write_char(q)?;
                        }
                    }
                    Ok(())
                }
            }
        }

        fn write_padded_instruction<'a, F: Fn(u16) -> Option<&'a str>>(instruction: &str, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            write_str_capsable(instruction, format_config, f)?;
            // TODO: have more stuff here
            f.write_str(" ")
        }

        fn write_deref_start<'a, F: Fn(u16) -> Option<&'a str>>(_format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.write_str("(")
        }

        fn write_deref_end<'a, F: Fn(u16) -> Option<&'a str>>(_format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.write_str(")")
        }

        fn write_register<'a, F: Fn(u16) -> Option<&'a str>>(register: Register, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            match register {
                Register::A => { write_str_capsable("a", format_config, f) }
                Register::B => { write_str_capsable("b", format_config, f) }
                Register::C => { write_str_capsable("c", format_config, f) }
                Register::D => { write_str_capsable("d", format_config, f) }
                Register::E => { write_str_capsable("e", format_config, f) }
                Register::H => { write_str_capsable("h", format_config, f) }
                Register::L => { write_str_capsable("l", format_config, f) }
                Register::BC => { write_str_capsable("bc", format_config, f) }
                Register::DE => { write_str_capsable("de", format_config, f) }
                Register::AF => { write_str_capsable("af", format_config, f) }
                Register::HL => { write_str_capsable("hl", format_config, f) }
                Register::SP => { write_str_capsable("sp", format_config, f) }
                Register::HLi => { write_str_capsable("hli", format_config, f) }
                Register::HLd => { write_str_capsable("hld", format_config, f) }
            }
        }

        fn write_flag<'a, F: Fn(u16) -> Option<&'a str>>(flag: Flag, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            match flag {
                Flag::C => write_str_capsable("c", format_config, f),
                Flag::NC => write_str_capsable("nc", format_config, f),
                Flag::Z => write_str_capsable("z", format_config, f),
                Flag::NZ => write_str_capsable("nz", format_config, f),
            }
        }

        fn write_deref_register<'a, F: Fn(u16) -> Option<&'a str>>(register: Register, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            write_deref_start(format_config, f)?;
            write_register(register, format_config, f)?;
            write_deref_end(format_config, f)
        }

        fn write_comma<'a, F: Fn(u16) -> Option<&'a str>>(_format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            // TODO: determine if a space after the comma is desired via config
            f.write_str(", ")
        }

        fn write_d16<'a, F: Fn(u16) -> Option<&'a str>>(d16: u16, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            if format_config.resolve_u16 && let Some(n) = (format_config.resolve_symbol)(d16) {
                f.write_str(n)
            }
            else {
                format_config.write_hex_u16(d16, f)
            }
        }

        fn write_a16<'a, F: Fn(u16) -> Option<&'a str>>(a16: u16, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            match (format_config.resolve_symbol)(a16) {
                Some(n) => f.write_str(n),
                None => write_d16(a16, format_config, f)
            }
        }

        fn write_deref_a16<'a, F: Fn(u16) -> Option<&'a str>>(a16: u16, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            write_deref_start(format_config, f)?;
            write_a16(a16, format_config, f)?;
            write_deref_end(format_config, f)
        }

        fn write_a8<'a, F: Fn(u16) -> Option<&'a str>>(a8: u8, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            write_a16(0xFF00 + (a8 as u16), format_config, f)
        }

        fn write_r8<'a, F: Fn(u16) -> Option<&'a str>>(r8: i8, pc: u16, instruction: Instruction, format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            let to = pc.wrapping_add_signed(r8 as i16).wrapping_add(instruction.into_binary().length as u16);
            write_a16(to, format_config, f)
        }

        fn write_d8<'a, F: Fn(u16) -> Option<&'a str>>(d8: u8, _format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            Display::fmt(&d8, f)
        }

        fn write_s8<'a, F: Fn(u16) -> Option<&'a str>>(s8: i8, _format_config: &FormatConfig<'a, F>, f: &mut Formatter<'_>) -> core::fmt::Result {
            Display::fmt(&s8, f)
        }

        macro_rules! write_instruction {
            ($i:tt) => {{
                write_str_capsable(stringify!($i), &self.config, f)
            }};

            ($i:tt reg($reg_a:tt), reg($reg_b:tt)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_register(Register::$reg_a, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_register(Register::$reg_b, &self.config, f)
            }};

            ($i:tt reg($reg:tt), d16($d16:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_register(Register::$reg, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_d16($d16, &self.config, f)
            }};

            ($i:tt reg($reg:tt)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_register(Register::$reg, &self.config, f)
            }};

            ($i:tt flag($flag:tt)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_flag(Flag::$flag, &self.config, f)
            }};

            ($i:tt d8($d8:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_d8($d8, &self.config, f)
            }};

            ($i:tt d8($d8:expr), reg($reg:tt)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_d8($d8, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_register(Register::$reg, &self.config, f)
            }};

            ($i:tt d8($d8:expr), [reg($reg:tt)]) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_d8($d8, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_deref_register(Register::$reg, &self.config, f)
            }};

            ($i:tt flag($flag:tt), a16($a16:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_flag(Flag::$flag, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_a16($a16, &self.config, f)
            }};

            ($i:tt a16($a16:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_a16($a16, &self.config, f)
            }};

            ($i:tt flag($flag:tt), r8($r8:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_flag(Flag::$flag, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_r8($r8, self.pc, self.instruction, &self.config, f)
            }};

            ($i:tt r8($r8:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_r8($r8, self.pc, self.instruction, &self.config, f)
            }};

            ($i:tt [reg($reg:tt)]) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_deref_register(Register::$reg, &self.config, f)
            }};

            ($i:tt [reg($reg_a:tt)], reg($reg_b:tt)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_deref_register(Register::$reg_a, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_register(Register::$reg_b, &self.config, f)
            }};

            ($i:tt reg($reg:tt), d8($d8:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_register(Register::$reg, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_d8($d8, &self.config, f)
            }};

            ($i:tt [a8($a8:expr)], reg($reg:tt)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_a8($a8, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_register(Register::$reg, &self.config, f)
            }};

            ($i:tt reg($reg:tt), [a8($a8:expr)]) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_register(Register::$reg, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_a8($a8, &self.config, f)
            }};

            ($i:tt reg($reg:tt), s8($s8:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_register(Register::$reg, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_s8($s8, &self.config, f)
            }};

            ($i:tt reg($reg_a:tt), [reg($reg_b:tt)]) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_register(Register::$reg_a, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_deref_register(Register::$reg_b, &self.config, f)
            }};

            ($i:tt [a16($a16:expr)], reg($reg:tt)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_deref_a16($a16, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_register(Register::$reg, &self.config, f)
            }};

            ($i:tt reg($reg:tt), [a16($a16:expr)]) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_register(Register::$reg, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_deref_a16($a16, &self.config, f)
            }};

            ($i:tt [reg($reg:tt)], d8($d8:expr)) => {{
                write_padded_instruction(stringify!($i), &self.config, f)?;
                write_deref_register(Register::$reg, &self.config, f)?;
                write_comma(&self.config, f)?;
                write_d8($d8, &self.config, f)
            }};
        }

        match self.instruction {
            Instruction::NOP => write_instruction!(nop),
            Instruction::STOP => write_instruction!(stop),
            Instruction::RRCA => write_instruction!(rrca),
            Instruction::RRA => write_instruction!(rra),
            Instruction::CPL => write_instruction!(cpl),
            Instruction::CCF => write_instruction!(ccf),
            Instruction::HALT => write_instruction!(halt),
            Instruction::DI => write_instruction!(di),
            Instruction::RET => write_instruction!(ret),
            Instruction::RETI => write_instruction!(reti),
            Instruction::EI => write_instruction!(ei),
            Instruction::RLCA => write_instruction!(rlca),
            Instruction::RLA => write_instruction!(rla),
            Instruction::DAA => write_instruction!(daa),
            Instruction::SCF => write_instruction!(scf),

            Instruction::INC_BC => write_instruction!(inc reg(BC)),
            Instruction::INC_DE => write_instruction!(inc reg(DE)),
            Instruction::INC_HL => write_instruction!(inc reg(HL)),
            Instruction::INC_SP => write_instruction!(inc reg(SP)),
            Instruction::INC_B => write_instruction!(inc reg(B)),
            Instruction::INC_D => write_instruction!(inc reg(D)),
            Instruction::INC_H => write_instruction!(inc reg(H)),
            Instruction::INC_C => write_instruction!(inc reg(C)),
            Instruction::INC_E => write_instruction!(inc reg(E)),
            Instruction::INC_L => write_instruction!(inc reg(L)),
            Instruction::INC_A => write_instruction!(inc reg(A)),
            Instruction::INC_HL_addr => write_instruction!(inc reg(A)),

            Instruction::DEC_BC => write_instruction!(dec reg(BC)),
            Instruction::DEC_DE => write_instruction!(dec reg(DE)),
            Instruction::DEC_HL => write_instruction!(dec reg(HL)),
            Instruction::DEC_SP => write_instruction!(dec reg(SP)),
            Instruction::DEC_C => write_instruction!(dec reg(C)),
            Instruction::DEC_E => write_instruction!(dec reg(E)),
            Instruction::DEC_L => write_instruction!(dec reg(L)),
            Instruction::DEC_A => write_instruction!(dec reg(A)),
            Instruction::DEC_B => write_instruction!(dec reg(B)),
            Instruction::DEC_D => write_instruction!(dec reg(D)),
            Instruction::DEC_H => write_instruction!(dec reg(H)),
            Instruction::DEC_HL_addr => write_instruction!(dec [reg(HL)]),

            Instruction::LD_BC_d16 { d16 } => write_instruction!(ld reg(BC), d16(d16)),
            Instruction::LD_DE_d16 { d16 } => write_instruction!(ld reg(DE), d16(d16)),
            Instruction::LD_HL_d16 { d16 } => write_instruction!(ld reg(HL), d16(d16)),
            Instruction::LD_SP_d16 { d16 } => write_instruction!(ld reg(SP), d16(d16)),

            Instruction::LD_BC_addr_A => write_instruction!(ld [reg(BC)], reg(A)),
            Instruction::LD_DE_addr_A => write_instruction!(ld [reg(DE)], reg(A)),
            Instruction::LD_HLi_addr_A => write_instruction!(ld [reg(HLi)], reg(A)),
            Instruction::LD_HLd_addr_A => write_instruction!(ld [reg(HLd)], reg(A)),

            Instruction::LD_A_BC_addr => write_instruction!(ld reg(A), [reg(BC)]),
            Instruction::LD_A_DE_addr => write_instruction!(ld reg(A), [reg(DE)]),
            Instruction::LD_A_HLi_addr => write_instruction!(ld reg(A), [reg(HLi)]),
            Instruction::LD_A_HLd_addr => write_instruction!(ld reg(A), [reg(HLd)]),

            Instruction::LD_C_d8 { d8 } => write_instruction!(ld reg(C), d8(d8)),
            Instruction::LD_E_d8 { d8 } => write_instruction!(ld reg(E), d8(d8)),
            Instruction::LD_L_d8 { d8 } => write_instruction!(ld reg(L), d8(d8)),
            Instruction::LD_A_d8 { d8 } => write_instruction!(ld reg(A), d8(d8)),
            Instruction::LD_B_d8 { d8 } => write_instruction!(ld reg(B), d8(d8)),
            Instruction::LD_D_d8 { d8 } => write_instruction!(ld reg(D), d8(d8)),
            Instruction::LD_H_d8 { d8 } => write_instruction!(ld reg(H), d8(d8)),

            Instruction::LD_B_B => write_instruction!(ld reg(B), reg(B)),
            Instruction::LD_D_B => write_instruction!(ld reg(D), reg(B)),
            Instruction::LD_H_B => write_instruction!(ld reg(H), reg(B)),
            Instruction::LD_B_C => write_instruction!(ld reg(B), reg(C)),
            Instruction::LD_D_C => write_instruction!(ld reg(D), reg(C)),
            Instruction::LD_H_C => write_instruction!(ld reg(H), reg(C)),
            Instruction::LD_B_D => write_instruction!(ld reg(B), reg(D)),
            Instruction::LD_D_D => write_instruction!(ld reg(D), reg(D)),
            Instruction::LD_H_D => write_instruction!(ld reg(H), reg(D)),
            Instruction::LD_B_E => write_instruction!(ld reg(B), reg(E)),
            Instruction::LD_D_E => write_instruction!(ld reg(D), reg(E)),
            Instruction::LD_H_E => write_instruction!(ld reg(H), reg(E)),
            Instruction::LD_B_H => write_instruction!(ld reg(B), reg(H)),
            Instruction::LD_D_H => write_instruction!(ld reg(D), reg(H)),
            Instruction::LD_H_H => write_instruction!(ld reg(H), reg(H)),
            Instruction::LD_B_L => write_instruction!(ld reg(B), reg(L)),
            Instruction::LD_D_L => write_instruction!(ld reg(D), reg(L)),
            Instruction::LD_H_L => write_instruction!(ld reg(H), reg(L)),
            Instruction::LD_B_A => write_instruction!(ld reg(B), reg(A)),
            Instruction::LD_D_A => write_instruction!(ld reg(D), reg(A)),
            Instruction::LD_H_A => write_instruction!(ld reg(H), reg(A)),
            Instruction::LD_C_B => write_instruction!(ld reg(C), reg(B)),
            Instruction::LD_E_B => write_instruction!(ld reg(E), reg(B)),
            Instruction::LD_L_B => write_instruction!(ld reg(L), reg(B)),
            Instruction::LD_A_B => write_instruction!(ld reg(A), reg(B)),
            Instruction::LD_C_C => write_instruction!(ld reg(C), reg(C)),
            Instruction::LD_E_C => write_instruction!(ld reg(E), reg(C)),
            Instruction::LD_L_C => write_instruction!(ld reg(L), reg(C)),
            Instruction::LD_A_C => write_instruction!(ld reg(A), reg(C)),
            Instruction::LD_C_D => write_instruction!(ld reg(C), reg(D)),
            Instruction::LD_E_D => write_instruction!(ld reg(E), reg(D)),
            Instruction::LD_L_D => write_instruction!(ld reg(L), reg(D)),
            Instruction::LD_A_D => write_instruction!(ld reg(A), reg(D)),
            Instruction::LD_C_E => write_instruction!(ld reg(C), reg(E)),
            Instruction::LD_E_E => write_instruction!(ld reg(E), reg(E)),
            Instruction::LD_L_E => write_instruction!(ld reg(L), reg(E)),
            Instruction::LD_A_E => write_instruction!(ld reg(A), reg(E)),
            Instruction::LD_C_H => write_instruction!(ld reg(C), reg(H)),
            Instruction::LD_E_H => write_instruction!(ld reg(E), reg(H)),
            Instruction::LD_L_H => write_instruction!(ld reg(L), reg(H)),
            Instruction::LD_A_H => write_instruction!(ld reg(A), reg(H)),
            Instruction::LD_C_L => write_instruction!(ld reg(C), reg(L)),
            Instruction::LD_E_L => write_instruction!(ld reg(E), reg(L)),
            Instruction::LD_L_L => write_instruction!(ld reg(L), reg(L)),
            Instruction::LD_A_L => write_instruction!(ld reg(A), reg(L)),
            Instruction::LD_C_A => write_instruction!(ld reg(C), reg(A)),
            Instruction::LD_E_A => write_instruction!(ld reg(E), reg(A)),
            Instruction::LD_L_A => write_instruction!(ld reg(L), reg(A)),
            Instruction::LD_A_A => write_instruction!(ld reg(A), reg(A)),

            Instruction::LD_HL_addr_d8 { d8 } => write_instruction!(ld [reg(HL)], d8(d8)),
            Instruction::LD_a16_addr_SP { a16 } => write_instruction!(ld [a16(a16)], reg(SP)),

            Instruction::LD_HL_addr_B => write_instruction!(ld [reg(HL)], reg(B)),
            Instruction::LD_HL_addr_C => write_instruction!(ld [reg(HL)], reg(C)),
            Instruction::LD_HL_addr_D => write_instruction!(ld [reg(HL)], reg(D)),
            Instruction::LD_HL_addr_E => write_instruction!(ld [reg(HL)], reg(E)),
            Instruction::LD_HL_addr_H => write_instruction!(ld [reg(HL)], reg(H)),
            Instruction::LD_HL_addr_A => write_instruction!(ld [reg(HL)], reg(A)),
            Instruction::LD_HL_addr_L => write_instruction!(ld [reg(HL)], reg(L)),

            Instruction::LD_B_HL_addr => write_instruction!(ld reg(B), [reg(HL)]),
            Instruction::LD_D_HL_addr => write_instruction!(ld reg(D), [reg(HL)]),
            Instruction::LD_H_HL_addr => write_instruction!(ld reg(H), [reg(HL)]),
            Instruction::LD_C_HL_addr => write_instruction!(ld reg(C), [reg(HL)]),
            Instruction::LD_E_HL_addr => write_instruction!(ld reg(E), [reg(HL)]),
            Instruction::LD_L_HL_addr => write_instruction!(ld reg(L), [reg(HL)]),
            Instruction::LD_A_HL_addr => write_instruction!(ld reg(A), [reg(HL)]),

            Instruction::ADD_HL_BC => write_instruction!(add reg(HL), reg(BC)),
            Instruction::ADD_HL_DE => write_instruction!(add reg(HL), reg(DE)),
            Instruction::ADD_HL_HL => write_instruction!(add reg(HL), reg(HL)),
            Instruction::ADD_HL_SP => write_instruction!(add reg(HL), reg(SP)),

            Instruction::POP_AF => write_instruction!(pop reg(AF)),
            Instruction::POP_BC => write_instruction!(pop reg(BC)),
            Instruction::POP_DE => write_instruction!(pop reg(DE)),
            Instruction::POP_HL => write_instruction!(pop reg(HL)),

            Instruction::PUSH_AF => write_instruction!(push reg(AF)),
            Instruction::PUSH_BC => write_instruction!(push reg(BC)),
            Instruction::PUSH_DE => write_instruction!(push reg(DE)),
            Instruction::PUSH_HL => write_instruction!(push reg(HL)),

            Instruction::CALL_a16 { a16 } => write_instruction!(call a16(a16)),
            Instruction::CALL_C_a16 { a16 } => write_instruction!(call flag(C), a16(a16)),
            Instruction::CALL_NC_a16 { a16 } => write_instruction!(call flag(NC), a16(a16)),
            Instruction::CALL_NZ_a16 { a16 } => write_instruction!(call flag(NZ), a16(a16)),
            Instruction::CALL_Z_a16 { a16 } => write_instruction!(call flag(Z), a16(a16)),

            Instruction::JR_r8 { r8 } => write_instruction!(jr r8(r8)),
            Instruction::JR_Z_r8 { r8 } => write_instruction!(jr flag(Z), r8(r8)),
            Instruction::JR_C_r8 { r8 } => write_instruction!(jr flag(C), r8(r8)),

            Instruction::JR_NZ_r8 { r8 } => write_instruction!(jr flag(NZ), r8(r8)),
            Instruction::JR_NC_r8 { r8 } => write_instruction!(jr flag(NC), r8(r8)),
            Instruction::JP_A16 { a16 } => write_instruction!(jp a16(a16)),
            Instruction::JP_C_a16 { a16 } => write_instruction!(jp flag(C), a16(a16)),
            Instruction::JP_HL => write_instruction!(jp reg(HL)),
            Instruction::JP_NC_a16 { a16 } => write_instruction!(jp flag(NC), a16(a16)),
            Instruction::JP_NZ_a16 { a16 } => write_instruction!(jp flag(NZ), a16(a16)),
            Instruction::JP_Z_a16 { a16 } => write_instruction!(jp flag(Z), a16(a16)),

            Instruction::ADC_d8 { d8 } => write_instruction!(adc d8(d8)),
            Instruction::ADD_d8 { d8 } => write_instruction!(add d8(d8)),
            Instruction::AND_d8 { d8 } => write_instruction!(and d8(d8)),
            Instruction::CP_d8 { d8 } => write_instruction!(cp d8(d8)),
            Instruction::OR_d8 { d8 } => write_instruction!(or d8(d8)),
            Instruction::SBC_d8 { d8 } => write_instruction!(sbc d8(d8)),
            Instruction::SUB_d8 { d8 } => write_instruction!(sub d8(d8)),
            Instruction::XOR_d8 { d8 } => write_instruction!(xor d8(d8)),

            Instruction::RST_0 => write_instruction!(rst d8(0)),
            Instruction::RST_1 => write_instruction!(rst d8(1)),
            Instruction::RST_2 => write_instruction!(rst d8(2)),
            Instruction::RST_3 => write_instruction!(rst d8(3)),
            Instruction::RST_4 => write_instruction!(rst d8(4)),
            Instruction::RST_5 => write_instruction!(rst d8(5)),
            Instruction::RST_6 => write_instruction!(rst d8(6)),
            Instruction::RST_7 => write_instruction!(rst d8(7)),

            Instruction::ADD_SP_s8 { s8 } => write_instruction!(add reg(SP), s8(s8)),
            
            Instruction::LD_A_a16_addr { a16 } => write_instruction!(ld reg(A), [a16(a16)]),
            Instruction::LD_a16_addr_A { a16 } => write_instruction!(ld [a16(a16)], reg(A)),
            Instruction::LDHL_SP_s8 { s8 } => match self.config.syntax_dialect {
                SyntaxDialect::ISAS => write_instruction!(ldhl reg(SP), s8(s8))
            },
            Instruction::LD_SP_HL => write_instruction!(ld reg(SP), reg(HL)),

            Instruction::LD_A_a8_addr { a8 } => write_instruction!(ld reg(A), [a8(a8)]),
            Instruction::LD_a8_addr_A { a8 } => write_instruction!(ld [a8(a8)], reg(A)),
            Instruction::LD_A_C_addr => write_instruction!(ld reg(A), [reg(C)]),
            Instruction::LD_C_addr_A => write_instruction!(ld [reg(C)], reg(A)),

            Instruction::RET_C => write_instruction!(ret flag(C)),
            Instruction::RET_NC => write_instruction!(ret flag(NC)),
            Instruction::RET_NZ => write_instruction!(ret flag(NZ)),
            Instruction::RET_Z => write_instruction!(ret flag(Z)),

            Instruction::ADC_A => write_instruction!(adc reg(A)),
            Instruction::ADC_B => write_instruction!(adc reg(B)),
            Instruction::ADC_C => write_instruction!(adc reg(C)),
            Instruction::ADC_D => write_instruction!(adc reg(D)),
            Instruction::ADC_E => write_instruction!(adc reg(E)),
            Instruction::ADC_H => write_instruction!(adc reg(H)),
            Instruction::ADC_L => write_instruction!(adc reg(L)),
            Instruction::ADC_HL_addr => write_instruction!(adc [reg(HL)]),

            Instruction::ADD_A => write_instruction!(add reg(A)),
            Instruction::ADD_B => write_instruction!(add reg(B)),
            Instruction::ADD_C => write_instruction!(add reg(C)),
            Instruction::ADD_D => write_instruction!(add reg(D)),
            Instruction::ADD_E => write_instruction!(add reg(E)),
            Instruction::ADD_H => write_instruction!(add reg(H)),
            Instruction::ADD_L => write_instruction!(add reg(L)),
            Instruction::ADD_HL_addr => write_instruction!(add [reg(HL)]),

            Instruction::AND_A => write_instruction!(and reg(A)),
            Instruction::AND_B => write_instruction!(and reg(B)),
            Instruction::AND_C => write_instruction!(and reg(C)),
            Instruction::AND_D => write_instruction!(and reg(D)),
            Instruction::AND_E => write_instruction!(and reg(E)),
            Instruction::AND_H => write_instruction!(and reg(H)),
            Instruction::AND_L => write_instruction!(and reg(L)),
            Instruction::AND_HL_addr => write_instruction!(and [reg(HL)]),

            Instruction::CP_A => write_instruction!(cp reg(A)),
            Instruction::CP_B => write_instruction!(cp reg(B)),
            Instruction::CP_C => write_instruction!(cp reg(C)),
            Instruction::CP_D => write_instruction!(cp reg(D)),
            Instruction::CP_E => write_instruction!(cp reg(E)),
            Instruction::CP_H => write_instruction!(cp reg(H)),
            Instruction::CP_L => write_instruction!(cp reg(L)),
            Instruction::CP_HL_addr => write_instruction!(cp [reg(HL)]),

            Instruction::OR_A => write_instruction!(or reg(A)),
            Instruction::OR_B => write_instruction!(or reg(B)),
            Instruction::OR_C => write_instruction!(or reg(C)),
            Instruction::OR_D => write_instruction!(or reg(D)),
            Instruction::OR_E => write_instruction!(or reg(E)),
            Instruction::OR_H => write_instruction!(or reg(H)),
            Instruction::OR_L => write_instruction!(or reg(L)),
            Instruction::OR_HL_addr => write_instruction!(or [reg(HL)]),

            Instruction::SBC_A => write_instruction!(sbc reg(A)),
            Instruction::SBC_B => write_instruction!(sbc reg(B)),
            Instruction::SBC_C => write_instruction!(sbc reg(C)),
            Instruction::SBC_D => write_instruction!(sbc reg(D)),
            Instruction::SBC_E => write_instruction!(sbc reg(E)),
            Instruction::SBC_H => write_instruction!(sbc reg(H)),
            Instruction::SBC_L => write_instruction!(sbc reg(L)),
            Instruction::SBC_HL_addr => write_instruction!(sbc [reg(HL)]),

            Instruction::SUB_A => write_instruction!(sub reg(A)),
            Instruction::SUB_B => write_instruction!(sub reg(B)),
            Instruction::SUB_C => write_instruction!(sub reg(C)),
            Instruction::SUB_D => write_instruction!(sub reg(D)),
            Instruction::SUB_E => write_instruction!(sub reg(E)),
            Instruction::SUB_H => write_instruction!(sub reg(H)),
            Instruction::SUB_L => write_instruction!(sub reg(L)),
            Instruction::SUB_HL_addr => write_instruction!(sub [reg(HL)]),

            Instruction::XOR_A => write_instruction!(xor reg(A)),
            Instruction::XOR_B => write_instruction!(xor reg(B)),
            Instruction::XOR_C => write_instruction!(xor reg(C)),
            Instruction::XOR_D => write_instruction!(xor reg(D)),
            Instruction::XOR_E => write_instruction!(xor reg(E)),
            Instruction::XOR_H => write_instruction!(xor reg(H)),
            Instruction::XOR_L => write_instruction!(xor reg(L)),
            Instruction::XOR_HL_addr => write_instruction!(xor [reg(HL)]),

            Instruction::PrefixedInstruction { instruction } => match instruction {
                PrefixedInstruction::BIT_0_A => write_instruction!(bit d8(0), reg(A)),
                PrefixedInstruction::BIT_0_B => write_instruction!(bit d8(0), reg(B)),
                PrefixedInstruction::BIT_0_C => write_instruction!(bit d8(0), reg(C)),
                PrefixedInstruction::BIT_0_D => write_instruction!(bit d8(0), reg(D)),
                PrefixedInstruction::BIT_0_E => write_instruction!(bit d8(0), reg(E)),
                PrefixedInstruction::BIT_0_H => write_instruction!(bit d8(0), reg(H)),
                PrefixedInstruction::BIT_0_HL_addr => write_instruction!(bit d8(0), [reg(HL)]),
                PrefixedInstruction::BIT_0_L => write_instruction!(bit d8(0), reg(L)),
                PrefixedInstruction::BIT_1_A => write_instruction!(bit d8(1), reg(A)),
                PrefixedInstruction::BIT_1_B => write_instruction!(bit d8(1), reg(B)),
                PrefixedInstruction::BIT_1_C => write_instruction!(bit d8(1), reg(C)),
                PrefixedInstruction::BIT_1_D => write_instruction!(bit d8(1), reg(D)),
                PrefixedInstruction::BIT_1_E => write_instruction!(bit d8(1), reg(E)),
                PrefixedInstruction::BIT_1_H => write_instruction!(bit d8(1), reg(H)),
                PrefixedInstruction::BIT_1_HL_addr => write_instruction!(bit d8(1), [reg(HL)]),
                PrefixedInstruction::BIT_1_L => write_instruction!(bit d8(1), reg(L)),
                PrefixedInstruction::BIT_2_A => write_instruction!(bit d8(2), reg(A)),
                PrefixedInstruction::BIT_2_B => write_instruction!(bit d8(2), reg(B)),
                PrefixedInstruction::BIT_2_C => write_instruction!(bit d8(2), reg(C)),
                PrefixedInstruction::BIT_2_D => write_instruction!(bit d8(2), reg(D)),
                PrefixedInstruction::BIT_2_E => write_instruction!(bit d8(2), reg(E)),
                PrefixedInstruction::BIT_2_H => write_instruction!(bit d8(2), reg(H)),
                PrefixedInstruction::BIT_2_HL_addr => write_instruction!(bit d8(2), [reg(HL)]),
                PrefixedInstruction::BIT_2_L => write_instruction!(bit d8(2), reg(L)),
                PrefixedInstruction::BIT_3_A => write_instruction!(bit d8(3), reg(A)),
                PrefixedInstruction::BIT_3_B => write_instruction!(bit d8(3), reg(B)),
                PrefixedInstruction::BIT_3_C => write_instruction!(bit d8(3), reg(C)),
                PrefixedInstruction::BIT_3_D => write_instruction!(bit d8(3), reg(D)),
                PrefixedInstruction::BIT_3_E => write_instruction!(bit d8(3), reg(E)),
                PrefixedInstruction::BIT_3_H => write_instruction!(bit d8(3), reg(H)),
                PrefixedInstruction::BIT_3_HL_addr => write_instruction!(bit d8(3), [reg(HL)]),
                PrefixedInstruction::BIT_3_L => write_instruction!(bit d8(3), reg(L)),
                PrefixedInstruction::BIT_4_A => write_instruction!(bit d8(4), reg(A)),
                PrefixedInstruction::BIT_4_B => write_instruction!(bit d8(4), reg(B)),
                PrefixedInstruction::BIT_4_C => write_instruction!(bit d8(4), reg(C)),
                PrefixedInstruction::BIT_4_D => write_instruction!(bit d8(4), reg(D)),
                PrefixedInstruction::BIT_4_E => write_instruction!(bit d8(4), reg(E)),
                PrefixedInstruction::BIT_4_H => write_instruction!(bit d8(4), reg(H)),
                PrefixedInstruction::BIT_4_HL_addr => write_instruction!(bit d8(4), [reg(HL)]),
                PrefixedInstruction::BIT_4_L => write_instruction!(bit d8(4), reg(L)),
                PrefixedInstruction::BIT_5_A => write_instruction!(bit d8(5), reg(A)),
                PrefixedInstruction::BIT_5_B => write_instruction!(bit d8(5), reg(B)),
                PrefixedInstruction::BIT_5_C => write_instruction!(bit d8(5), reg(C)),
                PrefixedInstruction::BIT_5_D => write_instruction!(bit d8(5), reg(D)),
                PrefixedInstruction::BIT_5_E => write_instruction!(bit d8(5), reg(E)),
                PrefixedInstruction::BIT_5_H => write_instruction!(bit d8(5), reg(H)),
                PrefixedInstruction::BIT_5_HL_addr => write_instruction!(bit d8(5), [reg(HL)]),
                PrefixedInstruction::BIT_5_L => write_instruction!(bit d8(5), reg(L)),
                PrefixedInstruction::BIT_6_A => write_instruction!(bit d8(6), reg(A)),
                PrefixedInstruction::BIT_6_B => write_instruction!(bit d8(6), reg(B)),
                PrefixedInstruction::BIT_6_C => write_instruction!(bit d8(6), reg(C)),
                PrefixedInstruction::BIT_6_D => write_instruction!(bit d8(6), reg(D)),
                PrefixedInstruction::BIT_6_E => write_instruction!(bit d8(6), reg(E)),
                PrefixedInstruction::BIT_6_H => write_instruction!(bit d8(6), reg(H)),
                PrefixedInstruction::BIT_6_HL_addr => write_instruction!(bit d8(6), [reg(HL)]),
                PrefixedInstruction::BIT_6_L => write_instruction!(bit d8(6), reg(L)),
                PrefixedInstruction::BIT_7_A => write_instruction!(bit d8(7), reg(A)),
                PrefixedInstruction::BIT_7_B => write_instruction!(bit d8(7), reg(B)),
                PrefixedInstruction::BIT_7_C => write_instruction!(bit d8(7), reg(C)),
                PrefixedInstruction::BIT_7_D => write_instruction!(bit d8(7), reg(D)),
                PrefixedInstruction::BIT_7_E => write_instruction!(bit d8(7), reg(E)),
                PrefixedInstruction::BIT_7_H => write_instruction!(bit d8(7), reg(H)),
                PrefixedInstruction::BIT_7_HL_addr => write_instruction!(bit d8(7), [reg(HL)]),
                PrefixedInstruction::BIT_7_L => write_instruction!(bit d8(7), reg(L)),
                PrefixedInstruction::RES_0_A => write_instruction!(res d8(0), reg(A)),
                PrefixedInstruction::RES_0_B => write_instruction!(res d8(0), reg(B)),
                PrefixedInstruction::RES_0_C => write_instruction!(res d8(0), reg(C)),
                PrefixedInstruction::RES_0_D => write_instruction!(res d8(0), reg(D)),
                PrefixedInstruction::RES_0_E => write_instruction!(res d8(0), reg(E)),
                PrefixedInstruction::RES_0_H => write_instruction!(res d8(0), reg(H)),
                PrefixedInstruction::RES_0_HL_addr => write_instruction!(res d8(0), [reg(HL)]),
                PrefixedInstruction::RES_0_L => write_instruction!(res d8(0), reg(L)),
                PrefixedInstruction::RES_1_A => write_instruction!(res d8(1), reg(A)),
                PrefixedInstruction::RES_1_B => write_instruction!(res d8(1), reg(B)),
                PrefixedInstruction::RES_1_C => write_instruction!(res d8(1), reg(C)),
                PrefixedInstruction::RES_1_D => write_instruction!(res d8(1), reg(D)),
                PrefixedInstruction::RES_1_E => write_instruction!(res d8(1), reg(E)),
                PrefixedInstruction::RES_1_H => write_instruction!(res d8(1), reg(H)),
                PrefixedInstruction::RES_1_HL_addr => write_instruction!(res d8(1), [reg(HL)]),
                PrefixedInstruction::RES_1_L => write_instruction!(res d8(1), reg(L)),
                PrefixedInstruction::RES_2_A => write_instruction!(res d8(2), reg(A)),
                PrefixedInstruction::RES_2_B => write_instruction!(res d8(2), reg(B)),
                PrefixedInstruction::RES_2_C => write_instruction!(res d8(2), reg(C)),
                PrefixedInstruction::RES_2_D => write_instruction!(res d8(2), reg(D)),
                PrefixedInstruction::RES_2_E => write_instruction!(res d8(2), reg(E)),
                PrefixedInstruction::RES_2_H => write_instruction!(res d8(2), reg(H)),
                PrefixedInstruction::RES_2_HL_addr => write_instruction!(res d8(2), [reg(HL)]),
                PrefixedInstruction::RES_2_L => write_instruction!(res d8(2), reg(L)),
                PrefixedInstruction::RES_3_A => write_instruction!(res d8(3), reg(A)),
                PrefixedInstruction::RES_3_B => write_instruction!(res d8(3), reg(B)),
                PrefixedInstruction::RES_3_C => write_instruction!(res d8(3), reg(C)),
                PrefixedInstruction::RES_3_D => write_instruction!(res d8(3), reg(D)),
                PrefixedInstruction::RES_3_E => write_instruction!(res d8(3), reg(E)),
                PrefixedInstruction::RES_3_H => write_instruction!(res d8(3), reg(H)),
                PrefixedInstruction::RES_3_HL_addr => write_instruction!(res d8(3), [reg(HL)]),
                PrefixedInstruction::RES_3_L => write_instruction!(res d8(3), reg(L)),
                PrefixedInstruction::RES_4_A => write_instruction!(res d8(4), reg(A)),
                PrefixedInstruction::RES_4_B => write_instruction!(res d8(4), reg(B)),
                PrefixedInstruction::RES_4_C => write_instruction!(res d8(4), reg(C)),
                PrefixedInstruction::RES_4_D => write_instruction!(res d8(4), reg(D)),
                PrefixedInstruction::RES_4_E => write_instruction!(res d8(4), reg(E)),
                PrefixedInstruction::RES_4_H => write_instruction!(res d8(4), reg(H)),
                PrefixedInstruction::RES_4_HL_addr => write_instruction!(res d8(4), [reg(HL)]),
                PrefixedInstruction::RES_4_L => write_instruction!(res d8(4), reg(L)),
                PrefixedInstruction::RES_5_A => write_instruction!(res d8(5), reg(A)),
                PrefixedInstruction::RES_5_B => write_instruction!(res d8(5), reg(B)),
                PrefixedInstruction::RES_5_C => write_instruction!(res d8(5), reg(C)),
                PrefixedInstruction::RES_5_D => write_instruction!(res d8(5), reg(D)),
                PrefixedInstruction::RES_5_E => write_instruction!(res d8(5), reg(E)),
                PrefixedInstruction::RES_5_H => write_instruction!(res d8(5), reg(H)),
                PrefixedInstruction::RES_5_HL_addr => write_instruction!(res d8(5), [reg(HL)]),
                PrefixedInstruction::RES_5_L => write_instruction!(res d8(5), reg(L)),
                PrefixedInstruction::RES_6_A => write_instruction!(res d8(6), reg(A)),
                PrefixedInstruction::RES_6_B => write_instruction!(res d8(6), reg(B)),
                PrefixedInstruction::RES_6_C => write_instruction!(res d8(6), reg(C)),
                PrefixedInstruction::RES_6_D => write_instruction!(res d8(6), reg(D)),
                PrefixedInstruction::RES_6_E => write_instruction!(res d8(6), reg(E)),
                PrefixedInstruction::RES_6_H => write_instruction!(res d8(6), reg(H)),
                PrefixedInstruction::RES_6_HL_addr => write_instruction!(res d8(6), [reg(HL)]),
                PrefixedInstruction::RES_6_L => write_instruction!(res d8(6), reg(L)),
                PrefixedInstruction::RES_7_A => write_instruction!(res d8(7), reg(A)),
                PrefixedInstruction::RES_7_B => write_instruction!(res d8(7), reg(B)),
                PrefixedInstruction::RES_7_C => write_instruction!(res d8(7), reg(C)),
                PrefixedInstruction::RES_7_D => write_instruction!(res d8(7), reg(D)),
                PrefixedInstruction::RES_7_E => write_instruction!(res d8(7), reg(E)),
                PrefixedInstruction::RES_7_H => write_instruction!(res d8(7), reg(H)),
                PrefixedInstruction::RES_7_HL_addr => write_instruction!(res d8(7), [reg(HL)]),
                PrefixedInstruction::RES_7_L => write_instruction!(res d8(7), reg(L)),
                PrefixedInstruction::SET_0_A => write_instruction!(set d8(0), reg(A)),
                PrefixedInstruction::SET_0_B => write_instruction!(set d8(0), reg(B)),
                PrefixedInstruction::SET_0_C => write_instruction!(set d8(0), reg(C)),
                PrefixedInstruction::SET_0_D => write_instruction!(set d8(0), reg(D)),
                PrefixedInstruction::SET_0_E => write_instruction!(set d8(0), reg(E)),
                PrefixedInstruction::SET_0_H => write_instruction!(set d8(0), reg(H)),
                PrefixedInstruction::SET_0_HL_addr => write_instruction!(set d8(0), [reg(HL)]),
                PrefixedInstruction::SET_0_L => write_instruction!(set d8(0), reg(L)),
                PrefixedInstruction::SET_1_A => write_instruction!(set d8(1), reg(A)),
                PrefixedInstruction::SET_1_B => write_instruction!(set d8(1), reg(B)),
                PrefixedInstruction::SET_1_C => write_instruction!(set d8(1), reg(C)),
                PrefixedInstruction::SET_1_D => write_instruction!(set d8(1), reg(D)),
                PrefixedInstruction::SET_1_E => write_instruction!(set d8(1), reg(E)),
                PrefixedInstruction::SET_1_H => write_instruction!(set d8(1), reg(H)),
                PrefixedInstruction::SET_1_HL_addr => write_instruction!(set d8(1), [reg(HL)]),
                PrefixedInstruction::SET_1_L => write_instruction!(set d8(1), reg(L)),
                PrefixedInstruction::SET_2_A => write_instruction!(set d8(2), reg(A)),
                PrefixedInstruction::SET_2_B => write_instruction!(set d8(2), reg(B)),
                PrefixedInstruction::SET_2_C => write_instruction!(set d8(2), reg(C)),
                PrefixedInstruction::SET_2_D => write_instruction!(set d8(2), reg(D)),
                PrefixedInstruction::SET_2_E => write_instruction!(set d8(2), reg(E)),
                PrefixedInstruction::SET_2_H => write_instruction!(set d8(2), reg(H)),
                PrefixedInstruction::SET_2_HL_addr => write_instruction!(set d8(2), [reg(HL)]),
                PrefixedInstruction::SET_2_L => write_instruction!(set d8(2), reg(L)),
                PrefixedInstruction::SET_3_A => write_instruction!(set d8(3), reg(A)),
                PrefixedInstruction::SET_3_B => write_instruction!(set d8(3), reg(B)),
                PrefixedInstruction::SET_3_C => write_instruction!(set d8(3), reg(C)),
                PrefixedInstruction::SET_3_D => write_instruction!(set d8(3), reg(D)),
                PrefixedInstruction::SET_3_E => write_instruction!(set d8(3), reg(E)),
                PrefixedInstruction::SET_3_H => write_instruction!(set d8(3), reg(H)),
                PrefixedInstruction::SET_3_HL_addr => write_instruction!(set d8(3), [reg(HL)]),
                PrefixedInstruction::SET_3_L => write_instruction!(set d8(3), reg(L)),
                PrefixedInstruction::SET_4_A => write_instruction!(set d8(4), reg(A)),
                PrefixedInstruction::SET_4_B => write_instruction!(set d8(4), reg(B)),
                PrefixedInstruction::SET_4_C => write_instruction!(set d8(4), reg(C)),
                PrefixedInstruction::SET_4_D => write_instruction!(set d8(4), reg(D)),
                PrefixedInstruction::SET_4_E => write_instruction!(set d8(4), reg(E)),
                PrefixedInstruction::SET_4_H => write_instruction!(set d8(4), reg(H)),
                PrefixedInstruction::SET_4_HL_addr => write_instruction!(set d8(4), [reg(HL)]),
                PrefixedInstruction::SET_4_L => write_instruction!(set d8(4), reg(L)),
                PrefixedInstruction::SET_5_A => write_instruction!(set d8(5), reg(A)),
                PrefixedInstruction::SET_5_B => write_instruction!(set d8(5), reg(B)),
                PrefixedInstruction::SET_5_C => write_instruction!(set d8(5), reg(C)),
                PrefixedInstruction::SET_5_D => write_instruction!(set d8(5), reg(D)),
                PrefixedInstruction::SET_5_E => write_instruction!(set d8(5), reg(E)),
                PrefixedInstruction::SET_5_H => write_instruction!(set d8(5), reg(H)),
                PrefixedInstruction::SET_5_HL_addr => write_instruction!(set d8(5), [reg(HL)]),
                PrefixedInstruction::SET_5_L => write_instruction!(set d8(5), reg(L)),
                PrefixedInstruction::SET_6_A => write_instruction!(set d8(6), reg(A)),
                PrefixedInstruction::SET_6_B => write_instruction!(set d8(6), reg(B)),
                PrefixedInstruction::SET_6_C => write_instruction!(set d8(6), reg(C)),
                PrefixedInstruction::SET_6_D => write_instruction!(set d8(6), reg(D)),
                PrefixedInstruction::SET_6_E => write_instruction!(set d8(6), reg(E)),
                PrefixedInstruction::SET_6_H => write_instruction!(set d8(6), reg(H)),
                PrefixedInstruction::SET_6_HL_addr => write_instruction!(set d8(6), [reg(HL)]),
                PrefixedInstruction::SET_6_L => write_instruction!(set d8(6), reg(L)),
                PrefixedInstruction::SET_7_A => write_instruction!(set d8(7), reg(A)),
                PrefixedInstruction::SET_7_B => write_instruction!(set d8(7), reg(B)),
                PrefixedInstruction::SET_7_C => write_instruction!(set d8(7), reg(C)),
                PrefixedInstruction::SET_7_D => write_instruction!(set d8(7), reg(D)),
                PrefixedInstruction::SET_7_E => write_instruction!(set d8(7), reg(E)),
                PrefixedInstruction::SET_7_H => write_instruction!(set d8(7), reg(H)),
                PrefixedInstruction::SET_7_HL_addr => write_instruction!(set d8(7), [reg(HL)]),
                PrefixedInstruction::SET_7_L => write_instruction!(set d8(7), reg(L)),

                PrefixedInstruction::RLC_A => write_instruction!(rlc reg(A)),
                PrefixedInstruction::RLC_B => write_instruction!(rlc reg(B)),
                PrefixedInstruction::RLC_C => write_instruction!(rlc reg(C)),
                PrefixedInstruction::RLC_D => write_instruction!(rlc reg(D)),
                PrefixedInstruction::RLC_E => write_instruction!(rlc reg(E)),
                PrefixedInstruction::RLC_H => write_instruction!(rlc reg(H)),
                PrefixedInstruction::RLC_HL_addr => write_instruction!(rlc [reg(HL)]),
                PrefixedInstruction::RLC_L => write_instruction!(rlc reg(L)),
                PrefixedInstruction::RRC_A => write_instruction!(rrc reg(A)),
                PrefixedInstruction::RRC_B => write_instruction!(rrc reg(B)),
                PrefixedInstruction::RRC_C => write_instruction!(rrc reg(C)),
                PrefixedInstruction::RRC_D => write_instruction!(rrc reg(D)),
                PrefixedInstruction::RRC_E => write_instruction!(rrc reg(E)),
                PrefixedInstruction::RRC_H => write_instruction!(rrc reg(H)),
                PrefixedInstruction::RRC_HL_addr => write_instruction!(rrc [reg(HL)]),
                PrefixedInstruction::RRC_L => write_instruction!(rrc reg(L)),
                PrefixedInstruction::SLA_A => write_instruction!(sla reg(A)),
                PrefixedInstruction::SLA_B => write_instruction!(sla reg(B)),
                PrefixedInstruction::SLA_C => write_instruction!(sla reg(C)),
                PrefixedInstruction::SLA_D => write_instruction!(sla reg(D)),
                PrefixedInstruction::SLA_E => write_instruction!(sla reg(E)),
                PrefixedInstruction::SLA_H => write_instruction!(sla reg(H)),
                PrefixedInstruction::SLA_HL_addr => write_instruction!(sla [reg(HL)]),
                PrefixedInstruction::SLA_L => write_instruction!(sla reg(L)),
                PrefixedInstruction::SRA_A => write_instruction!(sra reg(A)),
                PrefixedInstruction::SRA_B => write_instruction!(sra reg(B)),
                PrefixedInstruction::SRA_C => write_instruction!(sra reg(C)),
                PrefixedInstruction::SRA_D => write_instruction!(sra reg(D)),
                PrefixedInstruction::SRA_E => write_instruction!(sra reg(E)),
                PrefixedInstruction::SRA_H => write_instruction!(sra reg(H)),
                PrefixedInstruction::SRA_HL_addr => write_instruction!(sra [reg(HL)]),
                PrefixedInstruction::SRA_L => write_instruction!(sra reg(L)),
                PrefixedInstruction::SRL_A => write_instruction!(srl reg(A)),
                PrefixedInstruction::SRL_B => write_instruction!(srl reg(B)),
                PrefixedInstruction::SRL_C => write_instruction!(srl reg(C)),
                PrefixedInstruction::SRL_D => write_instruction!(srl reg(D)),
                PrefixedInstruction::SRL_E => write_instruction!(srl reg(E)),
                PrefixedInstruction::SRL_H => write_instruction!(srl reg(H)),
                PrefixedInstruction::SRL_HL_addr => write_instruction!(srl [reg(HL)]),
                PrefixedInstruction::SRL_L => write_instruction!(srl reg(L)),

                PrefixedInstruction::RL_A => write_instruction!(rl reg(A)),
                PrefixedInstruction::RL_B => write_instruction!(rl reg(B)),
                PrefixedInstruction::RL_C => write_instruction!(rl reg(C)),
                PrefixedInstruction::RL_D => write_instruction!(rl reg(D)),
                PrefixedInstruction::RL_E => write_instruction!(rl reg(E)),
                PrefixedInstruction::RL_H => write_instruction!(rl reg(H)),
                PrefixedInstruction::RL_HL_addr => write_instruction!(rl [reg(HL)]),
                PrefixedInstruction::RL_L => write_instruction!(rl reg(L)),
                PrefixedInstruction::RR_A => write_instruction!(rr reg(A)),
                PrefixedInstruction::RR_B => write_instruction!(rr reg(B)),
                PrefixedInstruction::RR_C => write_instruction!(rr reg(C)),
                PrefixedInstruction::RR_D => write_instruction!(rr reg(D)),
                PrefixedInstruction::RR_E => write_instruction!(rr reg(E)),
                PrefixedInstruction::RR_H => write_instruction!(rr reg(H)),
                PrefixedInstruction::RR_HL_addr => write_instruction!(rr [reg(HL)]),
                PrefixedInstruction::RR_L => write_instruction!(rr reg(L)),

                PrefixedInstruction::SWAP_A => write_instruction!(swap reg(A)),
                PrefixedInstruction::SWAP_B => write_instruction!(swap reg(B)),
                PrefixedInstruction::SWAP_C => write_instruction!(swap reg(C)),
                PrefixedInstruction::SWAP_D => write_instruction!(swap reg(D)),
                PrefixedInstruction::SWAP_E => write_instruction!(swap reg(E)),
                PrefixedInstruction::SWAP_H => write_instruction!(swap reg(H)),
                PrefixedInstruction::SWAP_HL_addr => write_instruction!(swap [reg(HL)]),
                PrefixedInstruction::SWAP_L => write_instruction!(swap reg(L)),
            }
        }
    }
}
