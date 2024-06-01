use core::mem::transmute;

use bitflags::bitflags;

pub struct Font<'a> {
    header: Psf2Header,
    glyphs: &'a [u8],
    unicode_table: &'a [u8],
}
impl<'a> Font<'a> {
    pub fn parse(data: &[u8]) -> Font {
        let mut header = [0; 32];
        header.copy_from_slice(&data[..32]);
        let header = unsafe { transmute::<[u8; 32], Psf2Header>(header) };
        assert_eq!(header.magic, [0x72, 0xb5, 0x4a, 0x86]);
        assert_eq!(header.version, 0);
        assert_eq!(header.flags, Psf2Flags::UNICODE_TABLE);

        let glyphs_start = header.header_size as usize;
        let glyphs_end = glyphs_start + header.length as usize * header.char_size as usize;
        let unicode_start = glyphs_end;

        let glyphs = &data[glyphs_start..glyphs_end];
        let unicode_table = &data[unicode_start..];

        Font {
            header,
            glyphs,
            unicode_table,
        }
    }

    pub fn width(&self) -> usize {
        self.header.width as usize
    }

    pub fn height(&self) -> usize {
        self.header.height as usize
    }

    pub fn get_char(&self, c: char) -> Option<&'a [u8]> {
        let index = self.unicode_index(c)?;
        let start = index * self.header.char_size as usize;
        let end = start + self.header.char_size as usize;
        Some(&self.glyphs[start..end])
    }

    pub fn get_char_fallback(&self, c: char) -> &'a [u8] {
        self.get_char(c).unwrap_or(
            self.get_char('\u{fffd}')
                .unwrap_or(self.get_char('?').unwrap()),
        )
    }

    fn unicode_index(&self, c: char) -> Option<usize> {
        let mut utf8 = [0; 4];
        c.encode_utf8(&mut utf8);
        let ut8_len = c.len_utf8();

        let table = self.unicode_table.iter();

        let mut index = 0;
        let mut matched = 0;
        for byte in table {
            match byte {
                0xff => {
                    index += 1;
                }
                byte => {
                    if *byte == utf8[matched] {
                        matched += 1;

                        if matched == ut8_len {
                            return Some(index);
                        }
                    } else {
                        matched = 0;
                    }
                }
            }
        }
        None
    }
}

#[repr(C)]
struct Psf2Header {
    magic: [u8; 4],
    version: u32,
    header_size: u32, // offset to start of bitmaps
    flags: Psf2Flags,
    length: u32,    // number of glyphs
    char_size: u32, // number of bytes for each glyph
    height: u32,
    width: u32,
}

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    struct Psf2Flags: u32 {
        const UNICODE_TABLE = 1;
    }
}
