extern crate raytracer;

use std::fs::write;

use raytracer::{
    gfx::{
        canvas::Canvas,
        image_formats::{png::PNGImage, Image},
        primitives::color::ColorRGBA,
    },
    primitives::three_part::{point::Point, vector::Vector},
};

#[derive(Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Projectile { position, velocity }
    }
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Environment { gravity, wind }
    }
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile::new(
        projectile.position + projectile.velocity * 0.1,
        projectile.velocity + (environment.gravity + environment.wind) * 0.1,
    )
}

enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds,
}

impl Pixel {
    pub fn from_point_for_canvas(Point(point): Point, canvas: &Canvas) -> Pixel {
        // 1. Convert from floating point space to integer space
        // Completely ignoring z-order and z-value for this now
        let rx = point.0.round();
        let ry = point.1.round();

        let ux = rx as usize;
        let uy = ry as usize;

        if rx.is_sign_negative() || ry.is_sign_negative() || ux > canvas.width || uy > canvas.height
        {
            return Pixel::OutOfBounds;
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
    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.02, 0.0, 0.0));
    let projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    );

    let mut canvas = Canvas::new(900, 500);
    let color = ColorRGBA::new(1.0, 1.0, 0.0, 1.0);

    println!("{:?}", environment);

    let mut current = projectile;
    let mut iteration: i32 = 0;
    while current.position.0 .1 > 0.0 {
        println!("{}: {:?}", iteration, current);

        match Pixel::from_point_for_canvas(current.position, &canvas) {
            Pixel::Coordinate { x, y } => {
                canvas.set_color_at(x, y, color);
            }
            Pixel::OutOfBounds => {}
        }

        current = tick(&environment, &current);
        iteration += 1;
    }
    println!("FINISHED => {}: {:?}", iteration, current);

    println!("Writing ./output/png.png");
    let ppm = PNGImage::from(&canvas);
    write("./output/png.png", ppm.as_bytes()).expect("Could not write png.png to disk.");

    println!("Everything done.");
}
