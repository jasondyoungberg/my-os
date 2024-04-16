use core::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Hexdump<'a>(pub &'a [u8]);

impl Display for Hexdump<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Convert a byte to a printable ASCII character,
        // or a '.' if it's not printable.
        fn byte2char(byte: u8) -> char {
            if (32..=126).contains(&byte) {
                byte as char
            } else {
                '.'
            }
        }

        let lines = self.0.len() / 16;
        let remainder = self.0.len() % 16;

        for i in 0..lines {
            let offset = i * 16;
            write!(f, "{offset:08x}: ")?;

            let data = &self.0[offset..offset + 16];

            for byte in data {
                write!(f, "{byte:02x} ")?;
            }

            write!(f, " ")?;

            for &byte in data {
                write!(f, "{}", byte2char(byte))?;
            }

            if !(i + 1 == lines && remainder == 0) {
                writeln!(f)?
            };
        }

        if remainder == 0 {
            return Ok(());
        }

        let offset = lines * 16;
        write!(f, "{offset:08x}: ")?;

        for byte in &self.0[offset..] {
            write!(f, "{byte:02x} ")?;
        }

        write!(f, " ")?;

        for _ in 0..(16 - remainder) {
            write!(f, "   ")?;
        }

        for byte in &self.0[offset..] {
            let c = if *byte >= 32 && *byte <= 126 {
                *byte as char
            } else {
                '.'
            };

            write!(f, "{c}")?;
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
        assert_eq!(format!("{}", Hexdump(&[0; 0])), "");
    }

    #[test]
    fn buffer_hello() {
        assert_eq!(
            format!("{}", Hexdump(b"Hello, world!")),
            "00000000: 48 65 6c 6c 6f 2c 20 77 6f 72 6c 64 21           Hello, world!"
        );
    }

    #[test]
    fn buffer_lipsum() {
        assert_eq!(
            format!("{}", Hexdump(b"Lorem ipsum dolor sit amet")),
            "\
00000000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem ipsum dolo
00000010: 72 20 73 69 74 20 61 6d 65 74                    r sit amet"
        );
    }

    #[test]
    fn buffer_len_1() {
        assert_eq!(
            format!("{}", Hexdump(&[0; 1])),
            "00000000: 00                                               ."
        );
    }

    #[test]
    fn buffer_len_8() {
        assert_eq!(
            format!("{}", Hexdump(&[0; 8])),
            "00000000: 00 00 00 00 00 00 00 00                          ........"
        );
    }

    #[test]
    fn buffer_len_15() {
        assert_eq!(
            format!("{}", Hexdump(&[0; 15])),
            "00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00     ..............."
        );
    }

    #[test]
    fn buffer_len_16() {
        assert_eq!(
            format!("{}", Hexdump(&[0; 16])),
            "00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................"
        );
    }

    #[test]
    fn buffer_len_17() {
        assert_eq!(
            format!("{}", Hexdump(&[0; 17])),
            "\
00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
00000010: 00                                               ."
        );
    }

    #[test]
    fn buffer_len_24() {
        assert_eq!(
            format!("{}", Hexdump(&[0; 24])),
            "\
00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
00000010: 00 00 00 00 00 00 00 00                          ........"
        );
    }

    #[test]
    fn buffer_len_31() {
        assert_eq!(
            format!("{}", Hexdump(&[0; 31])),
            "\
00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
00000010: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00     ..............."
        );
    }

    #[test]
    fn buffer_len_32() {
        assert_eq!(
            format!("{}", Hexdump(&[0; 32])),
            "\
00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
00000010: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................"
        );
    }
}
