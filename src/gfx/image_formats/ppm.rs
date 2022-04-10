use crate::gfx::primitives::color::ColorRGBA;

use super::Image;

pub struct PPMP7Image {
    width: usize,
    height: usize,
    max_color: u8,
    depth: u8,
    pixels: Vec<ColorRGBA>,
}

impl PPMP7Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            max_color: 255,
            depth: 4,
            pixels: vec![ColorRGBA::blank(); width * height],
        }
    }
}

impl Image for PPMP7Image {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn color_at(&self, x: usize, y: usize) -> crate::gfx::primitives::color::ColorRGBA {
        self.pixels[y * self.width + x]
    }

    fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    fn set_color_at(
        &mut self,
        x: usize,
        y: usize,
        color: crate::gfx::primitives::color::ColorRGBA,
    ) {
        self.pixels[y * self.width + x] = color;
    }

    fn as_bytes_header(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        // header

        bytes.extend(String::from("P7\n").into_bytes());
        bytes.extend(format!("WIDTH {}\n", self.width).into_bytes());
        bytes.extend(format!("HEIGHT {}\n", self.height).into_bytes());
        bytes.extend(format!("DEPTH {}\n", self.depth).into_bytes());
        bytes.extend(format!("MAXVAL {}\n", self.max_color).into_bytes());
        bytes.extend(String::from("TUPLTYPE RGB_ALPHA\n").into_bytes());
        bytes.extend(String::from("ENDHDR\n").into_bytes());

        bytes
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.as_bytes_header();

        let color_data: Vec<u8> = self.pixels.iter().flat_map(|c| (*c).as_bytes()).collect();

        bytes.extend(color_data);

        bytes
    }
}

pub struct PPMP3Image {
    width: usize,
    height: usize,
    max_color: u8,
    pixels: Vec<ColorRGBA>,
}

impl PPMP3Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            max_color: 255,
            pixels: vec![ColorRGBA::blank(); width * height],
        }
    }
}

impl Image for PPMP3Image {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn color_at(&self, x: usize, y: usize) -> ColorRGBA {
        self.pixels[y * self.width + x]
    }

    fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    fn set_color_at(&mut self, x: usize, y: usize, color: ColorRGBA) {
        self.pixels[y * self.width + x] = color;
    }

    fn as_bytes_header(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        // header

        bytes.extend(String::from("P3\n").into_bytes());
        bytes.extend(format!("{} {}\n", self.width, self.height).into_bytes());
        bytes.extend(format!("{}\n", self.max_color).into_bytes());

        bytes
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.as_bytes_header();

        // each pixel must be represented as R G B in ascii

        let color_data: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|c| format!("{} {} {}\n", c.0, c.1, c.2).as_bytes().to_vec())
            .collect();

        bytes.extend(color_data);

        bytes
    }
}
