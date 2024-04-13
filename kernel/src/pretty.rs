use core::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Hexdump<'a>(&'a [u8]);

impl Display for Hexdump<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lines = self.0.len() / 16;

        for i in 0..lines {
            let offset = i * 16;
            write!(f, "{:08x}:", offset)?;

            for byte in &self.0[offset..offset + 16] {
                write!(f, " {:02x}", byte)?;
            }

            writeln!(f)?;
        }

        let remainder = self.0.len() % 16;
        if remainder != 0 {
            let offset = lines * 16;
            write!(f, "{:08x}:", offset)?;

            for byte in &self.0[offset..] {
                write!(f, " {:02x}", byte)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn buffer_empty() {
        let buffer = Hexdump(&[]);
        let expected = "";

        assert_eq!(expected, format!("{}", buffer));
    }

    #[test]
    fn buffer_20_zeros() {
        let buffer = Hexdump(&[0; 20]);
        let expected = "\
00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00000010: 00 00 00 00";

        assert_eq!(expected, format!("{}", buffer));
    }
}
