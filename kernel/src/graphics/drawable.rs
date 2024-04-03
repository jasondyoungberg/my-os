use super::{color::Color, Bitmap};

pub trait Drawable {
    fn size(&self) -> (usize, usize);
    fn set_pixel_unchecked(&mut self, pos: (usize, usize), color: &Color);

    fn set_pixel(&mut self, pos: (usize, usize), color: &Color) -> Result<(), DrawError> {
        if pos.0 >= self.size().0 || pos.1 >= self.size().1 {
            return Err(DrawError::OutOfBounds);
        }

        self.set_pixel_unchecked(pos, color);
        Ok(())
    }

    fn clear(&mut self, color: &Color) {
        let (width, height) = self.size();
        for y in 0..height {
            for x in 0..width {
                self.set_pixel_unchecked((x, y), color)
            }
        }
    }

    fn blit<const W: usize, const H: usize>(
        &mut self,
        pos: (usize, usize),
        other: Bitmap<W, H>,
    ) -> Result<(), DrawError> {
        if pos.0 + W >= self.size().0 || pos.1 + H >= self.size().1 {
            return Err(DrawError::OutOfBounds);
        }

        for (y, row) in other.data().iter().enumerate() {
            for (x, color) in row.iter().enumerate() {
                self.set_pixel_unchecked((pos.0 + x, pos.1 + y), color)
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum DrawError {
    OutOfBounds,
}
