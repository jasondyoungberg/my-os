use super::Color;

pub trait PixelBuffer {
    fn set_pixel(&mut self, pos: (usize, usize), color: &Color);
    fn get_pixel(&self, pos: (usize, usize)) -> Color;
    fn size(&self) -> (usize, usize);

    fn clear(&mut self, color: &Color) {
        let (width, height) = self.size();
        for y in 0..height {
            for x in 0..width {
                self.set_pixel((x, y), color);
            }
        }
    }
}
