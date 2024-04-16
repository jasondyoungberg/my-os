use super::{color::Color, Bitmap};

pub trait Drawable {
    fn size(&self) -> (usize, usize);
    fn set_pixel_unchecked(&mut self, pos: (usize, usize), color: Color);

    fn set_pixel(&mut self, pos: (usize, usize), color: Color) -> Result<(), &'static str> {
        if pos.0 >= self.size().0 || pos.1 >= self.size().1 {
            return Err("Out of bounds");
        }

        self.set_pixel_unchecked(pos, color);
        Ok(())
    }

    fn clear(&mut self, color: Color) {
        let (width, height) = self.size();
        for y in 0..height {
            for x in 0..width {
                self.set_pixel_unchecked((x, y), color)
            }
        }
    }

    fn blit<T: Bitmap>(&mut self, pos: (usize, usize), other: T) -> Result<(), &'static str> {
        if pos.0 + other.size().0 >= self.size().0 || pos.1 + other.size().1 >= self.size().1 {
            return Err("out of bounds");
        }

        for y in 0..other.size().1 {
            for x in 0..other.size().0 {
                self.set_pixel_unchecked((pos.0 + x, pos.1 + y), other.get_pixel((x, y))?)
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum DrawError {
    OutOfBounds,
}
