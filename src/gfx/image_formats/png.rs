use super::Canvas;

use super::Image;

pub struct PNGImage<'a> {
    canvas: &'a Canvas,
}

impl<'a> From<&'a Canvas> for PNGImage<'a> {
    fn from(canvas: &'a Canvas) -> Self {
        Self { canvas }
    }
}

impl<'a> Image for PNGImage<'a> {
    fn as_bytes_header(&self) -> Vec<u8> {
        todo!()
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        let mut encoder =
            png::Encoder::new(&mut v, self.canvas.width as u32, self.canvas.height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.canvas.as_bytes()).unwrap();
        drop(writer);

        v
    }
}
