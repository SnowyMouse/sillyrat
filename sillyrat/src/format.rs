use core::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SyntaxDialect {
    /// Intelligent Systems Assembler (ISAS) syntax.
    ///
    /// This is the "official" syntax from the Game Boy Development Kit.
    ISAS
}

impl SyntaxDialect {
    /// Format a number as 8-bit hex.
    #[inline]
    pub const fn format_hex_u8(self, number: u8) -> SyntaxDialectFormattedNumberHex {
        SyntaxDialectFormattedNumberHex { number: number as u32, syntax: self, number_size: 8 }
    }

    /// Format a number as 16-bit hex.
    #[inline]
    pub const fn format_hex_u16(self, number: u16) -> SyntaxDialectFormattedNumberHex {
        SyntaxDialectFormattedNumberHex { number: number as u32, syntax: self, number_size: 16 }
    }

    /// Format a number as 32-bit hex.
    #[inline]
    pub const fn format_hex_u32(self, number: u32) -> SyntaxDialectFormattedNumberHex {
        SyntaxDialectFormattedNumberHex { number, syntax: self, number_size: 32 }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SyntaxDialectFormattedNumberHex {
    number: u32,
    syntax: SyntaxDialect,
    number_size: u8
}

impl SyntaxDialectFormattedNumberHex {
    /// Get the syntax dialect.
    #[inline]
    pub const fn get_syntax(self) -> SyntaxDialect {
        self.syntax
    }
}

impl Display for SyntaxDialectFormattedNumberHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let number = self.number;
        let number_size = self.number_size;
        match self.syntax {
            SyntaxDialect::ISAS => match number_size {
                8 => f.write_fmt(format_args!("{number:02X}h")),
                16 => f.write_fmt(format_args!("{number:04X}h")),
                32 => f.write_fmt(format_args!("{number:08X}h")),
                _ => unreachable!()
            }
        }
    }
}
