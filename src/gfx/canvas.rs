use super::{
    image_formats::{
        ppm::{PPMP3Image, PPMP7Image},
        Image,
    },
    primitives::color::ColorRGBA,
};

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

impl From<Canvas> for PPMP3Image {
    fn from(canvas: Canvas) -> Self {
        let mut out = Self::new(canvas.width, canvas.height);

        for (pos, pixel) in canvas.pixels.iter().enumerate() {
            out.set_color_at(pos % canvas.width, pos / canvas.width, *pixel);
        }

        out
    }
}

impl From<Canvas> for PPMP7Image {
    fn from(canvas: Canvas) -> Self {
        let mut out = Self::new(canvas.width, canvas.height);

        for (pos, pixel) in canvas.pixels.iter().enumerate() {
            out.set_color_at(pos % canvas.width, pos / canvas.width, *pixel);
        }

        out
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

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = PPMP3Image::from(c);
        /* Header consisting of
         * Magic Number: P3
         * Width and Height: 5 3
         * Maximum Color Value: 255
         */
        let expected_header = String::from("P3\n5 3\n255\n").into_bytes();

        assert_eq!(ppm.as_bytes_header(), expected_header);
    }

    #[test]
    fn constructing_pam_header() {
        let c = Canvas::new(5, 3);
        let ppm = PPMP7Image::from(c);
        /* Header consisting of
         * Magic Number: P3
         * Width and Height: 5 3
         * Maximum Color Value: 255
         */
        let expected_header = String::from(
            "P7\nWIDTH 5\nHEIGHT 3\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n",
        )
        .into_bytes();

        assert_eq!(ppm.as_bytes_header(), expected_header);
    }

    #[test]
    fn constructing_ppm_image() {
        let c = Canvas::new(5, 3);
        let ppm = PPMP3Image::from(c);
        /* Header consisting of
         * Magic Number: P3
         * Width and Height: 5 3
         * Maximum Color Value: 255
         * Pixels:
         */
        let expected_image = String::from(
            r#"P3
5 3
255
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
"#,
        )
        .into_bytes();

        assert_eq!(ppm.as_bytes(), expected_image);
    }

    #[test]
    fn constructing_pam_image() {
        let c = Canvas::new(5, 3);
        let ppm = PPMP7Image::from(c);

        let expected_image_header = String::from(
            r#"P7
WIDTH 5
HEIGHT 3
DEPTH 4
MAXVAL 255
TUPLTYPE RGB_ALPHA
ENDHDR
"#,
        )
        .into_bytes();

        let expected_image_data = vec![0_u8; 60];

        let expected_image: Vec<u8> = expected_image_header
            .into_iter()
            .chain(expected_image_data)
            .collect();

        assert_eq!(ppm.as_bytes(), expected_image);
    }
}
