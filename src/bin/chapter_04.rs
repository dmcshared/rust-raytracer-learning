extern crate raytracer;

use std::fs::write;

use raytracer::{
    gfx::{
        canvas::Canvas,
        image_formats::{png::PNGImage, Image},
        primitives::color::ColorRGBA,
    },
    primitives::{matrix::Matrix4f, rotation::degrees::Degree, three_part::point::Point},
};

enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds { x: f64, y: f64 },
}

impl Pixel {
    pub fn from_point_for_canvas(point: Point, canvas: &Canvas) -> Pixel {
        // 1. Convert from floating point space to integer space
        // Completely ignoring z-order and z-value for this now
        let rx = point.0 .0.round();
        let ry = point.0 .1.round();

        if rx.is_sign_negative() || ry.is_sign_negative() {
            return Pixel::OutOfBounds { x: rx, y: ry };
        }

        let ux = rx as usize;
        let uy = ry as usize;

        if ux > canvas.width || uy > canvas.height {
            return Pixel::OutOfBounds { x: rx, y: ry };
        }

        // 2. Invert y axis to fit Screen space as the (0,0) coordinate is top left
        //    and not bottom left
        let screen_x = ux;
        let screen_y = canvas.height - uy;

        Pixel::Coordinate {
            x: screen_x,
            y: screen_y,
        }
    }
}

fn main() {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;

    let mut canvas: Canvas = Canvas::new(WIDTH, HEIGHT);
    let color = ColorRGBA::new(1.0, 1.0, 0.0, 1.0);

    let new_origin = Point::new((WIDTH / 2) as f64, (HEIGHT / 2) as f64, 0.0);

    let origin_transform = Matrix4f::translate(new_origin - Point::origin());

    for hour in 0..12 {
        let r = 200.0;
        let rotation_transform = Matrix4f::rotate_around_z(Degree(30.0 * (hour as f64)).into());
        let point = Point::new(0.0, r, 0.0);

        let transformed_point = origin_transform * rotation_transform * point;

        println!("Point: {:?}", transformed_point);

        match Pixel::from_point_for_canvas(transformed_point, &canvas) {
            Pixel::Coordinate { x, y } => canvas.set_color_at(x, y, color),
            Pixel::OutOfBounds { x, y } => panic!(
                "Could not map point to screen/canvas: Out of bounds: {:?} x {:?}",
                x, y
            ),
        }
    }

    println!("Writing ./output.png");
    let png = PNGImage::from(&canvas);
    write("./output/png.png", png.as_bytes()).expect("Could not write ouput.png to disk.");

    println!("Everything done.");
}
