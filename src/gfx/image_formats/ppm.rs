use super::Canvas;

use super::Image;

pub struct PPMP7Image<'a> {
    canvas: &'a Canvas,
    max_color: u8,
    depth: u8,
}

impl<'a> From<&'a Canvas> for PPMP7Image<'a> {
    fn from(canvas: &'a Canvas) -> Self {
        Self {
            canvas,
            max_color: 255,
            depth: 4,
        }
    }
}

impl<'a> Image for PPMP7Image<'a> {
    fn as_bytes_header(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        // header

        bytes.extend(String::from("P7\n").into_bytes());
        bytes.extend(format!("WIDTH {}\n", self.canvas.width).into_bytes());
        bytes.extend(format!("HEIGHT {}\n", self.canvas.height).into_bytes());
        bytes.extend(format!("DEPTH {}\n", self.depth).into_bytes());
        bytes.extend(format!("MAXVAL {}\n", self.max_color).into_bytes());
        bytes.extend(String::from("TUPLTYPE RGB_ALPHA\n").into_bytes());
        bytes.extend(String::from("ENDHDR\n").into_bytes());

        bytes
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.as_bytes_header();

        let color_data: Vec<u8> = self
            .canvas
            .pixels
            .iter()
            .flat_map(|c| (*c).as_bytes())
            .collect();

        bytes.extend(color_data);

        bytes
    }
}

pub struct PPMP3Image<'a> {
    canvas: &'a Canvas,
    max_color: u8,
}

impl<'a> From<&'a Canvas> for PPMP3Image<'a> {
    fn from(canvas: &'a Canvas) -> Self {
        Self {
            canvas,
            max_color: 255,
        }
    }
}

impl<'a> Image for PPMP3Image<'a> {
    fn as_bytes_header(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        // header

        bytes.extend(String::from("P3\n").into_bytes());
        bytes.extend(format!("{} {}\n", self.canvas.width, self.canvas.height).into_bytes());
        bytes.extend(format!("{}\n", self.max_color).into_bytes());

        bytes
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.as_bytes_header();

        // each pixel must be represented as R G B in ascii

        let color_data: Vec<u8> = self
            .canvas
            .pixels
            .iter()
            .flat_map(|c| {
                format!(
                    "{} {} {}\n",
                    ((c.0.clamp(0.0, 1.0) * 255.0) as u8),
                    ((c.1.clamp(0.0, 1.0) * 255.0) as u8),
                    ((c.2.clamp(0.0, 1.0) * 255.0) as u8),
                )
                .as_bytes()
                .to_vec()
            })
            .collect();

        bytes.extend(color_data);

        bytes
    }
}
