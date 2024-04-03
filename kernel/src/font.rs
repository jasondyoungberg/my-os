use crate::graphics::{color, Color, PixelBuffer};

#[derive(Clone, Copy)]
pub struct CharIcon(u128);

pub fn get_char_icon(c: char) -> Option<CharIcon> {
    DATA.iter().find(|(ch, _)| *ch == c).map(|(_, icon)| *icon)
}

impl PixelBuffer for CharIcon {
    fn size(&self) -> (usize, usize) {
        (8, 16)
    }

    fn set_pixel(&mut self, _pos: (usize, usize), _color: &Color) {
        unimplemented!()
    }

    fn get_pixel(&self, pos: (usize, usize)) -> Color {
        let shift = 127 - (pos.0 + pos.1 * 8);
        if (self.0 >> shift) & 1 == 1 {
            color::WHITE
        } else {
            color::BLACK
        }
    }
}

// https://github.com/susam/pcface/blob/main/out/moderndos-8x16/fontmap.js
const DATA: [(char, CharIcon); 256] = [
    ('\u{FFFD}', CharIcon(0x00000000000000000000000000000000)),
    ('\u{263A}', CharIcon(0x00007e81a58181bd9981817e00000000)),
    ('\u{263B}', CharIcon(0x00007effdbffffc3e7ffff7e00000000)),
    ('\u{2665}', CharIcon(0x000000006ceefefefe7c381000000000)),
    ('\u{2666}', CharIcon(0x0000000010387cfefe7c381000000000)),
    ('\u{2663}', CharIcon(0x000000183c3c5affff5a183c00000000)),
    ('\u{2660}', CharIcon(0x00000010387cfefeee54103800000000)),
    ('\u{2022}', CharIcon(0x000000000000183c3c18000000000000)),
    ('\u{25D8}', CharIcon(0xffffffffffffe7c3c3e7ffffffffffff)),
    ('\u{25CB}', CharIcon(0x00000000003c664242663c0000000000)),
    ('\u{25D9}', CharIcon(0xffffffffffc399bdbd99c3ffffffffff)),
    ('\u{2642}', CharIcon(0x00001e0e1a3078cccccccc7800000000)),
    ('\u{2640}', CharIcon(0x00003c666666663c187e181800000000)),
    ('\u{266A}', CharIcon(0x00080c0a0a0a08080878f87000000000)),
    ('\u{266B}', CharIcon(0x00101814121a16121272f2620e1e1c00)),
    ('\u{263C}', CharIcon(0x00000010927c6cc66c7c921000000000)),
    ('\u{25BA}', CharIcon(0x00000080c0e0f8fef8e0c08000000000)),
    ('\u{25C4}', CharIcon(0x00000002060e3efe3e0e060200000000)),
    ('\u{2195}', CharIcon(0x000010387cd61010d67c381000000000)),
    ('\u{203C}', CharIcon(0x00006666666666666600666600000000)),
    ('\u{00B6}', CharIcon(0x00007fdbdbdbdb7b1b1b1b1b00000000)),
    ('\u{00A7}', CharIcon(0x007cc660386cc6c66c380cc67c000000)),
    ('\u{25AC}', CharIcon(0x0000000000000000fefefefe00000000)),
    ('\u{21A8}', CharIcon(0x000010387cd61010d67c3810fe000000)),
    ('\u{2191}', CharIcon(0x000010387cd610101010101000000000)),
    ('\u{2193}', CharIcon(0x0000101010101010d67c381000000000)),
    ('\u{2192}', CharIcon(0x0000000010180cfe0c18100000000000)),
    ('\u{2190}', CharIcon(0x00000000103060fe6030100000000000)),
    ('\u{221F}', CharIcon(0x000000000000c0c0c0fe000000000000)),
    ('\u{2194}', CharIcon(0x00000000002442ff4224000000000000)),
    ('\u{25B2}', CharIcon(0x000000001038387c7cfefe0000000000)),
    ('\u{25BC}', CharIcon(0x00000000fefe7c7c3838100000000000)),
    (' ', CharIcon(0x00000000000000000000000000000000)),
    ('!', CharIcon(0x0000183c3c3c18181800181800000000)),
    ('"', CharIcon(0x00666622220000000000000000000000)),
    ('#', CharIcon(0x000000006c6cfe6c6cfe6c6c00000000)),
    ('$', CharIcon(0x0010107cd6d670381cd6d67c10100000)),
    ('%', CharIcon(0x00006092966c18306cd2920c00000000)),
    ('&', CharIcon(0x0000386c6c383076dccccc7600000000)),
    ('\'', CharIcon(0x00181808100000000000000000000000)),
    ('(', CharIcon(0x00000c18303030303030180c00000000)),
    (')', CharIcon(0x000030180c0c0c0c0c0c183000000000)),
    ('*', CharIcon(0x0000000000663cff3c66000000000000)),
    ('+', CharIcon(0x000000000018187e1818000000000000)),
    (',', CharIcon(0x00000000000000000000303010200000)),
    ('-', CharIcon(0x00000000000000fe0000000000000000)),
    ('.', CharIcon(0x00000000000000000000303000000000)),
    ('/', CharIcon(0x00000002060c183060c0800000000000)),
    ('0', CharIcon(0x00007cc6c6cedef6e6c6c67c00000000)),
    ('1', CharIcon(0x00001838781818181818187e00000000)),
    ('2', CharIcon(0x00007cc6060c183060c0c6fe00000000)),
    ('3', CharIcon(0x00007cc606063c060606c67c00000000)),
    ('4', CharIcon(0x00000c1c3c6cccccfe0c0c1e00000000)),
    ('5', CharIcon(0x0000fec0c0c0fc060606c67c00000000)),
    ('6', CharIcon(0x00007cc6c0c0fcc6c6c6c67c00000000)),
    ('7', CharIcon(0x0000fec606060c183030303000000000)),
    ('8', CharIcon(0x00007cc6c6c67cc6c6c6c67c00000000)),
    ('9', CharIcon(0x00007cc6c6c6c67e0606c67c00000000)),
    (':', CharIcon(0x00000000181800000018180000000000)),
    (';', CharIcon(0x00000000181800000018180810000000)),
    ('<', CharIcon(0x000000060c18306030180c0600000000)),
    ('=', CharIcon(0x00000000007e00007e00000000000000)),
    ('>', CharIcon(0x0000006030180c060c18306000000000)),
    ('?', CharIcon(0x00007cc6c6060c181800181800000000)),
    ('@', CharIcon(0x0000003c429da5a5adb6403c00000000)),
    ('A', CharIcon(0x0000386cc6c6c6fec6c6c6c600000000)),
    ('B', CharIcon(0x0000fc6666667c66666666fc00000000)),
    ('C', CharIcon(0x00007cc6c6c0c0c0c0c6c67c00000000)),
    ('D', CharIcon(0x0000fc6666666666666666fc00000000)),
    ('E', CharIcon(0x0000fe6662687878686266fe00000000)),
    ('F', CharIcon(0x0000fe6662687878686060f000000000)),
    ('G', CharIcon(0x00007cc6c6c0c0cec6c6c67e00000000)),
    ('H', CharIcon(0x0000c6c6c6c6fec6c6c6c6c600000000)),
    ('I', CharIcon(0x00003c18181818181818183c00000000)),
    ('J', CharIcon(0x00001e0c0c0c0c0c0ccccc7800000000)),
    ('K', CharIcon(0x0000e666666c78786c6666e600000000)),
    ('L', CharIcon(0x0000f06060606060606266fe00000000)),
    ('M', CharIcon(0x000082c6eefefed6c6c6c6c600000000)),
    ('N', CharIcon(0x000086c6e6f6fedecec6c6c600000000)),
    ('O', CharIcon(0x00007cc6c6c6c6c6c6c6c67c00000000)),
    ('P', CharIcon(0x0000fc666666667c606060f000000000)),
    ('Q', CharIcon(0x00007cc6c6c6c6c6c6d6de7c06000000)),
    ('R', CharIcon(0x0000fc666666667c6c6666e600000000)),
    ('S', CharIcon(0x00007cc6c660380c06c6c67c00000000)),
    ('T', CharIcon(0x00007e7e5a1818181818183c00000000)),
    ('U', CharIcon(0x0000c6c6c6c6c6c6c6c6c67c00000000)),
    ('V', CharIcon(0x0000c6c6c6c6c6c6c66c381000000000)),
    ('W', CharIcon(0x0000c6c6c6c6c6d6feeec68200000000)),
    ('X', CharIcon(0x0000c6c66c7c38387c6cc6c600000000)),
    ('Y', CharIcon(0x0000666666663c181818183c00000000)),
    ('Z', CharIcon(0x0000fec6860c183060c2c6fe00000000)),
    ('[', CharIcon(0x00003c30303030303030303c00000000)),
    ('\\', CharIcon(0x00000080c06030180c06020000000000)),
    (']', CharIcon(0x00003c0c0c0c0c0c0c0c0c3c00000000)),
    ('^', CharIcon(0x000010386cc600000000000000000000)),
    ('_', CharIcon(0x00000000000000000000000000ff0000)),
    ('`', CharIcon(0x00181810080000000000000000000000)),
    ('a', CharIcon(0x0000000000780c7ccccccc7600000000)),
    ('b', CharIcon(0x0000e060607c66666666667c00000000)),
    ('c', CharIcon(0x00000000007cc6c0c0c0c67c00000000)),
    ('d', CharIcon(0x00001c0c0c7ccccccccccc7600000000)),
    ('e', CharIcon(0x00000000007cc6c6fec0c67c00000000)),
    ('f', CharIcon(0x00001c36307c30303030307800000000)),
    ('g', CharIcon(0x000000000076cccccccccc7c0ccc7800)),
    ('h', CharIcon(0x0000e060606c7666666666e600000000)),
    ('i', CharIcon(0x00001818003818181818183c00000000)),
    ('j', CharIcon(0x00000c0c001c0c0c0c0c0c0ccccc7800)),
    ('k', CharIcon(0x0000e06060666c78786c66e600000000)),
    ('l', CharIcon(0x00003818181818181818183c00000000)),
    ('m', CharIcon(0x0000000000ecfed6d6d6d6c600000000)),
    ('n', CharIcon(0x0000000000dc66666666666600000000)),
    ('o', CharIcon(0x00000000007cc6c6c6c6c67c00000000)),
    ('p', CharIcon(0x0000000000dc66666666667c6060f000)),
    ('q', CharIcon(0x00000000007ccccccccccc7c0c0c1e00)),
    ('r', CharIcon(0x0000000000de7660606060f000000000)),
    ('s', CharIcon(0x00000000007cc660380cc67c00000000)),
    ('t', CharIcon(0x0000103030fc30303030341800000000)),
    ('u', CharIcon(0x0000000000cccccccccccc7600000000)),
    ('v', CharIcon(0x0000000000c6c6c6c66c381000000000)),
    ('w', CharIcon(0x0000000000c6d6d6d6d6fe6c00000000)),
    ('x', CharIcon(0x0000000000c6c66c386cc6c600000000)),
    ('y', CharIcon(0x0000000000c6c6c6c6c6c67e060cf800)),
    ('z', CharIcon(0x0000000000fe8c183060c2fe00000000)),
    ('{', CharIcon(0x00000e18181870181818180e00000000)),
    ('|', CharIcon(0x00001818181800001818181800000000)),
    ('}', CharIcon(0x0000701818180e181818187000000000)),
    ('~', CharIcon(0x000076dc000000000000000000000000)),
    ('\u{2302}', CharIcon(0x0000000010386cc6c6c6fe0000000000)),
    ('\u{00C7}', CharIcon(0x00007cc6c6c0c0c0c0c6c67c10087000)),
    ('\u{00FC}', CharIcon(0x0000cccc00cccccccccccc7600000000)),
    ('\u{00E9}', CharIcon(0x00060c10007cc6c6fec0c67c00000000)),
    ('\u{00E2}', CharIcon(0x003078cc00780c7ccccccc7600000000)),
    ('\u{00E4}', CharIcon(0x0000cccc00780c7ccccccc7600000000)),
    ('\u{00E0}', CharIcon(0x00c0601000780c7ccccccc7600000000)),
    ('\u{00E5}', CharIcon(0x0030483000780c7ccccccc7600000000)),
    ('\u{00E7}', CharIcon(0x00000000007cc6c0c0c0c67c10087000)),
    ('\u{00EA}', CharIcon(0x00183c66007cc6c6fec0c67c00000000)),
    ('\u{00EB}', CharIcon(0x0000c6c6007cc6c6fec0c67c00000000)),
    ('\u{00E8}', CharIcon(0x00c06010007cc6c6fec0c67c00000000)),
    ('\u{00EF}', CharIcon(0x00006666003818181818183c00000000)),
    ('\u{00EE}', CharIcon(0x00183c66003818181818183c00000000)),
    ('\u{00EC}', CharIcon(0x00c06010003818181818183c00000000)),
    ('\u{00C4}', CharIcon(0xc6c600386cc6c6fec6c6c6c600000000)),
    ('\u{00C5}', CharIcon(0x384438386cc6c6fec6c6c6c600000000)),
    ('\u{00C9}', CharIcon(0x0c1820fe66626878686266fe00000000)),
    ('\u{00E6}', CharIcon(0x00000000007c12729e90927c00000000)),
    ('\u{00C6}', CharIcon(0x00003e6ac8c8ccfcc8c8cace00000000)),
    ('\u{00F4}', CharIcon(0x00183c66007cc6c6c6c6c67c00000000)),
    ('\u{00F6}', CharIcon(0x0000c6c6007cc6c6c6c6c67c00000000)),
    ('\u{00F2}', CharIcon(0x00c06010007cc6c6c6c6c67c00000000)),
    ('\u{00FB}', CharIcon(0x003078cc00cccccccccccc7600000000)),
    ('\u{00F9}', CharIcon(0x00c0601000cccccccccccc7600000000)),
    ('\u{00FF}', CharIcon(0x0000c6c600c6c6c6c6c6c67e060c7800)),
    ('\u{00D6}', CharIcon(0xc6c6007cc6c6c6c6c6c6c67c00000000)),
    ('\u{00DC}', CharIcon(0xc6c600c6c6c6c6c6c6c6c67c00000000)),
    ('\u{00A2}', CharIcon(0x000010107cd6d0d0d67c101000000000)),
    ('\u{00A3}', CharIcon(0x0000386c60f060606060f2dc00000000)),
    ('\u{00A5}', CharIcon(0x00006666663c187e187e181800000000)),
    ('\u{20A7}', CharIcon(0x0000f8ccccf8c4ccdeccccc600000000)),
    ('\u{0192}', CharIcon(0x00000e1b18187e181818181818d87000)),
    ('\u{00E1}', CharIcon(0x00060c1000780c7ccccccc7600000000)),
    ('\u{00ED}', CharIcon(0x00060c10003818181818183c00000000)),
    ('\u{00F3}', CharIcon(0x00060c10007cc6c6c6c6c67c00000000)),
    ('\u{00FA}', CharIcon(0x00060c1000cccccccccccc7600000000)),
    ('\u{00F1}', CharIcon(0x000076dc00dc66666666666600000000)),
    ('\u{00D1}', CharIcon(0x76dc0086c6e6f6fedecec6c600000000)),
    ('\u{00AA}', CharIcon(0x00380c3c6c36007e0000000000000000)),
    ('\u{00BA}', CharIcon(0x00003c66663c007e0000000000000000)),
    ('\u{00BF}', CharIcon(0x0000303000303060c0c6c67c00000000)),
    ('\u{2310}', CharIcon(0x00000000000000fec0c0c00000000000)),
    ('\u{00AC}', CharIcon(0x00000000000000fe0606060000000000)),
    ('\u{00BD}', CharIcon(0x0060e0646c783060dcb60c183e000000)),
    ('\u{00BC}', CharIcon(0x0060e0646c78306cdcac3e0c0c000000)),
    ('\u{00A1}', CharIcon(0x00001818001818183c3c3c1800000000)),
    ('\u{00AB}', CharIcon(0x0000000000366cd86c36000000000000)),
    ('\u{00BB}', CharIcon(0x0000000000d86c366cd8000000000000)),
    ('\u{2591}', CharIcon(0x22882288228822882288228822882288)),
    ('\u{2592}', CharIcon(0x55aa55aa55aa55aa55aa55aa55aa55aa)),
    ('\u{2593}', CharIcon(0xdd77dd77dd77dd77dd77dd77dd77dd7f)),
    ('\u{2502}', CharIcon(0x18181818181818181818181818181818)),
    ('\u{2524}', CharIcon(0x18181818181818f81818181818181818)),
    ('\u{2561}', CharIcon(0x1818181818f818f81818181818181818)),
    ('\u{2562}', CharIcon(0x36363636363636f63636363636363636)),
    ('\u{2556}', CharIcon(0x00000000000000fe3636363636363636)),
    ('\u{2555}', CharIcon(0x0000000000f818f81818181818181818)),
    ('\u{2563}', CharIcon(0x3636363636f606f63636363636363636)),
    ('\u{2551}', CharIcon(0x36363636363636363636363636363636)),
    ('\u{2557}', CharIcon(0x0000000000fe06f63636363636363636)),
    ('\u{255D}', CharIcon(0x3636363636f606fe0000000000000000)),
    ('\u{255C}', CharIcon(0x36363636363636fe0000000000000000)),
    ('\u{255B}', CharIcon(0x1818181818f818f80000000000000000)),
    ('\u{2510}', CharIcon(0x00000000000000f81818181818181818)),
    ('\u{2514}', CharIcon(0x181818181818181f0000000000000000)),
    ('\u{2534}', CharIcon(0x18181818181818ff0000000000000000)),
    ('\u{252C}', CharIcon(0x00000000000000ff1818181818181818)),
    ('\u{251C}', CharIcon(0x181818181818181f1818181818181818)),
    ('\u{2500}', CharIcon(0x00000000000000ff0000000000000000)),
    ('\u{253C}', CharIcon(0x18181818181818ff1818181818181818)),
    ('\u{255E}', CharIcon(0x18181818181f181f1818181818181818)),
    ('\u{255F}', CharIcon(0x36363636363636373636363636363636)),
    ('\u{255A}', CharIcon(0x363636363637303f0000000000000000)),
    ('\u{2554}', CharIcon(0x00000000003f30373636363636363636)),
    ('\u{2569}', CharIcon(0x3636363636f700ff0000000000000000)),
    ('\u{2566}', CharIcon(0x0000000000ff00f73636363636363636)),
    ('\u{2560}', CharIcon(0x36363636363730373636363636363636)),
    ('\u{2550}', CharIcon(0x0000000000ff00ff0000000000000000)),
    ('\u{256C}', CharIcon(0x3636363636f700f73636363636363636)),
    ('\u{2567}', CharIcon(0x1818181818ff00ff0000000000000000)),
    ('\u{2568}', CharIcon(0x36363636363636ff0000000000000000)),
    ('\u{2564}', CharIcon(0x0000000000ff00ff1818181818181818)),
    ('\u{2565}', CharIcon(0x00000000000000ff3636363636363636)),
    ('\u{2559}', CharIcon(0x363636363636363f0000000000000000)),
    ('\u{2558}', CharIcon(0x18181818181f181f0000000000000000)),
    ('\u{2552}', CharIcon(0x00000000001f181f1818181818181818)),
    ('\u{2553}', CharIcon(0x000000000000003f3636363636363636)),
    ('\u{256B}', CharIcon(0x36363636363636ff3636363636363636)),
    ('\u{256A}', CharIcon(0x1818181818ff18ff1818181818181818)),
    ('\u{2518}', CharIcon(0x18181818181818f80000000000000000)),
    ('\u{250C}', CharIcon(0x000000000000001f1818181818181818)),
    ('\u{2588}', CharIcon(0xffffffffffffffffffffffffffffffff)),
    ('\u{2584}', CharIcon(0x00000000000000ffffffffffffffffff)),
    ('\u{258C}', CharIcon(0xf0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0)),
    ('\u{2590}', CharIcon(0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f)),
    ('\u{2580}', CharIcon(0xffffffffffffffff0000000000000000)),
    ('\u{03B1}', CharIcon(0x000000000074dcc8c8c8dc7600000000)),
    ('\u{00DF}', CharIcon(0x000078ccccd8ccc6c6c6dcc040000000)),
    ('\u{0393}', CharIcon(0x0000fe6260606060606060f000000000)),
    ('\u{03C0}', CharIcon(0x00000000027eec6c6c6c6c4800000000)),
    ('\u{03A3}', CharIcon(0x0000fec26030183060c0c2fe00000000)),
    ('\u{03C3}', CharIcon(0x00000000007ed0c8c8c8c87000000000)),
    ('\u{00B5}', CharIcon(0x0000000000ccccccccccccf880800000)),
    ('\u{03C4}', CharIcon(0x00000000007ed8181818181000000000)),
    ('\u{03A6}', CharIcon(0x000038107cd6d6d6d67c103800000000)),
    ('\u{0398}', CharIcon(0x0000007cc6c6c6fec6c6c67c00000000)),
    ('\u{03A9}', CharIcon(0x00007cc6c6c6c6c66c2828ee00000000)),
    ('\u{03B4}', CharIcon(0x00003c6230187ccccccccc7800000000)),
    ('\u{221E}', CharIcon(0x00000000006edbdbdb76000000000000)),
    ('\u{03C6}', CharIcon(0x00000002067cceded6f6e67cc0800000)),
    ('\u{03B5}', CharIcon(0x00000000003c60c0f8c0603c00000000)),
    ('\u{2229}', CharIcon(0x00007cc6c6c6c6c6c6c6c6c600000000)),
    ('\u{2261}', CharIcon(0x00000000fe0000fe0000fe0000000000)),
    ('\u{00B1}', CharIcon(0x0000000018187e18180000ff00000000)),
    ('\u{2265}', CharIcon(0x0000006030180c18306000fe00000000)),
    ('\u{2264}', CharIcon(0x0000000c18306030180c00fe00000000)),
    ('\u{2320}', CharIcon(0x00000e1b1b1818181818181818181818)),
    ('\u{2321}', CharIcon(0x181818181818181818d8d87000000000)),
    ('\u{00F7}', CharIcon(0x000000001818007e0018180000000000)),
    ('\u{2248}', CharIcon(0x000000000076dc0076dc000000000000)),
    ('\u{00B0}', CharIcon(0x00386c6c380000000000000000000000)),
    ('\u{2219}', CharIcon(0x00000000000000303000000000000000)),
    ('\u{00B7}', CharIcon(0x00000000000000003000000000000000)),
    ('\u{221A}', CharIcon(0x000f0c0c0c0c0cec6c6c3c1c00000000)),
    ('\u{207F}', CharIcon(0x00d86c6c6c6c00000000000000000000)),
    ('\u{00B2}', CharIcon(0x00384c0c18307c000000000000000000)),
    ('\u{25A0}', CharIcon(0x000000007c7c7c7c7c7c7c0000000000)),
    ('\u{00A0}', CharIcon(0x00000000000000000000000000000000)),
];