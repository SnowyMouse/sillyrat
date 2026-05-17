use crate::format::SyntaxDialect;
use crate::instruction::Instruction;
use core::fmt::Display;

const MAX_INSTRUCTION_LEN: usize = 4;

/// Parse the instruction.
///
/// Note that there is very little preprocessing here. For example `ld hl, 0xFF00+5` is not valid,
/// but `ld hl, 0xFF05` is.
///
/// If a symbol is found, it will be returned as the second value of the tuple.
pub fn parse_instruction(instruction: &str, syntax_dialect: SyntaxDialect) -> Result<(Instruction, Option<&'_ str>), InstructionParseError<'_>> {
    let mut instruction = instruction.trim();

    if !instruction.is_ascii() {
        return Err(InstructionParseError::NonAsciiInstruction(instruction));
    }

    let mut operands: [Operand; 2] = [Operand::Number { number: -1337, dereferenced: false }, Operand::Number { number: 1337, dereferenced: false }];

    let mut split = instruction.split_whitespace();
    let Some(command_name) = split.next() else { return Err(InstructionParseError::EmptyInstruction) };

    if command_name.len() > MAX_INSTRUCTION_LEN {
        return Err(InstructionParseError::InvalidInstruction(command_name));
    }

    let command_name_end = command_name.as_bytes().as_ptr_range().end;
    let instruction_range = instruction.as_bytes().as_ptr_range();

    let remainder = unsafe {
        // TODO: use slice::from_ptr_range when it is stabilized

        let len = instruction_range.end as usize - command_name_end as usize;

        // SAFETY: ASCII is 8 bits per character and also UTF-8, so we're good here
        core::str::from_utf8_unchecked(
            // SAFETY: instruction_range should contain command_name_end
            core::slice::from_raw_parts(command_name_end, len)
        ).trim()
    };

    let args: &[Operand];
    if remainder.is_empty() {
        args = &[];
    }
    else {
        match remainder.split_once(',') {
            Some((a,b)) => {
                if b.contains(',') {
                    return Err(InstructionParseError::InvalidInstruction(instruction));
                }

                operands[0] = Operand::from_str(a.trim())?;
                operands[1] = Operand::from_str(b.trim())?;

                if operands[0].is_dereferenced() && operands[1].is_dereferenced() {
                    return Err(InstructionParseError::InvalidInstruction(instruction))
                }

                args = operands.as_slice();
            }
            None => {
                operands[0] = Operand::from_str(remainder)?;
                args = &operands[..1];
            }
        }
    }

    // Make it lowercase so we can efficiently find what command it is
    let command_name_buffer = &mut [0u8; MAX_INSTRUCTION_LEN][..command_name.len()];
    command_name_buffer.copy_from_slice(command_name.as_bytes());
    command_name_buffer.make_ascii_lowercase();

    // SAFETY: We checked it was ASCII earlier.
    let command_name = unsafe { core::str::from_utf8_unchecked(command_name_buffer) };





    todo!()
}

#[derive(Copy, Clone, Debug)]
enum Operand<'a> {
    Symbol { symbol: &'a str, dereferenced: bool },
    Number { number: i32, dereferenced: bool },
}

impl<'a> Operand<'a> {
    pub fn from_str(s: &'_ str) -> Result<Operand<'_>, InstructionParseError<'_>> {
        let error = InstructionParseError::InvalidInstruction(s);
        if !s.is_ascii() {
            return Err(error);
        }

        let dereferenced;

        let inner = if s.starts_with('[') {
            dereferenced = true;
            if !s.ends_with(']') {
                return Err(error)
            }
            &s[1..s.len() - 1]
        }
        else {
            dereferenced = false;
            s
        }.trim();

        if inner.is_empty() {
            return Err(error);
        }

        // Is it a number?

        if inner.starts_with('$') {
            return Ok(Operand::Number { number: u16::from_str_radix(&inner[1..], 16).map_err(|_| error)? as i32, dereferenced });
        }

        if inner.starts_with('@') {
            return Ok(Operand::Number { number: u16::from_str_radix(&inner[1..], 8).map_err(|_| error)? as i32, dereferenced });
        }

        if inner.starts_with('%') {
            return Ok(Operand::Number { number: u16::from_str_radix(&inner[1..], 2).map_err(|_| error)? as i32, dereferenced });
        }

        if inner.starts_with("0x") || inner.starts_with("0X") {
            return Ok(Operand::Number { number: u16::from_str_radix(&inner[2..], 16).map_err(|_| error)? as i32, dereferenced });
        }

        if inner.starts_with("0o") || inner.starts_with("0O") {
            return Ok(Operand::Number { number: u16::from_str_radix(&inner[2..], 8).map_err(|_| error)? as i32, dereferenced });
        }

        if inner.starts_with("0b") || inner.starts_with("0B") {
            return Ok(Operand::Number { number: u16::from_str_radix(&inner[2..], 2).map_err(|_| error)? as i32, dereferenced });
        }

        if inner.ends_with("h") {
            return Ok(Operand::Number { number: u16::from_str_radix(&inner[..inner.len() - 1], 16).map_err(|_| error)? as i32, dereferenced });
        }

        if inner.chars().next().expect("should have a character here as it's not empty...").is_numeric() {
            return Ok(Operand::Number { number: i32::from_str_radix(inner, 10).map_err(|_| error)?, dereferenced });
        }

        // must be alphanumeric or have underscore or dots
        if !inner.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.') {
            return Err(error);
        }

        Ok(Operand::Symbol { symbol: inner, dereferenced })
    }

    pub fn is_dereferenced(self) -> bool {
        match self {
            Operand::Symbol { dereferenced, .. } | Operand::Number { dereferenced, .. } => dereferenced,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum InstructionParseError<'a> {
    EmptyInstruction,
    NonAsciiInstruction(&'a str),
    InvalidInstruction(&'a str),
    InvalidOperand(&'a str)
}
