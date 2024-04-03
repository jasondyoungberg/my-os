use super::color::Color;

// pub struct Bitmap<const W: usize, const H: usize>([[Color; W]; H]);

// impl<const W: usize, const H: usize> Bitmap<W, H> {
//     pub fn new(data: [[Color; W]; H]) -> Self {
//         Self(data)
//     }

//     pub fn data(&self) -> &[[Color; W]; H] {
//         &self.0
//     }
// }

pub trait Bitmap {
    fn size(&self) -> (usize, usize);
    fn get_pixel_unchecked(&self, pos: (usize, usize)) -> Color;

    fn get_pixel(&self, pos: (usize, usize)) -> Result<Color, &'static str> {
        if pos.0 >= self.size().0 || pos.1 >= self.size().1 {
            return Err("Out of bounds");
        }

        Ok(self.get_pixel_unchecked(pos))
    }
}

impl<const W: usize, const H: usize> Bitmap for [[Color; W]; H] {
    fn size(&self) -> (usize, usize) {
        (W, H)
    }

    fn get_pixel_unchecked(&self, pos: (usize, usize)) -> Color {
        self[pos.1][pos.0]
    }
}
