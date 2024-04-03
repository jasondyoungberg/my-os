use super::color::Color;

pub struct Bitmap<const W: usize, const H: usize>([[Color; W]; H]);

impl<const W: usize, const H: usize> Bitmap<W, H> {
    pub fn new(data: [[Color; W]; H]) -> Self {
        Self(data)
    }

    pub fn data(&self) -> &[[Color; W]; H] {
        &self.0
    }
}
