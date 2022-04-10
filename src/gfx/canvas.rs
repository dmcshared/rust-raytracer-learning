use super::primitives::color::ColorRGBA;

pub struct Canvas {
    pub width: usize,
    pub height: usize,

    pub pixels: Vec<ColorRGBA>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![ColorRGBA::blank(); width * height],
        }
    }

    pub fn color_at(&self, x: usize, y: usize) -> ColorRGBA {
        self.pixels[y * self.width + x]
    }

    pub fn set_color_at(&mut self, x: usize, y: usize, color: ColorRGBA) {
        self.pixels[y * self.width + x] = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_canvas_and_check_empty_initialized() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for y in 0..canvas.height {
            for x in 0..canvas.width {
                assert_eq!(canvas.color_at(x, y), ColorRGBA::blank());
            }
        }
    }

    #[test]
    fn set_a_color_and_check_it_is_set() {
        let mut canvas = Canvas::new(10, 20);
        canvas.set_color_at(2, 3, ColorRGBA::new(1.0, 0.0, 0.0, 1.0));

        assert_eq!(canvas.color_at(2, 3), ColorRGBA::new(1.0, 0.0, 0.0, 1.0));
    }
}
