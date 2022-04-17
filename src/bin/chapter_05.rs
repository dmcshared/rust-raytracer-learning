use indicatif::ProgressBar;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use raytracer::{
    gfx::{
        canvas::Canvas,
        image_formats::{png::PNGImage, Image},
        primitives::color::default_palettes,
    },
    primitives::{
        body::{sphere::Sphere, Body},
        intersection::IntersectionList,
        matrix::Matrix4f,
        ray::Ray,
        three_part::point::Point,
    },
};
use std::{fs::write, sync::Mutex};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let virtual_canvas_pos_z = 5.0;
    let virtual_canvas_size = 10.0;
    let mut canvas = Canvas::new(512, 512);
    let virtual_pixel_size = virtual_canvas_size / (canvas.width as f64);

    let sphere = Sphere::new(Matrix4f::identity());

    let canvas_width = canvas.width;
    let canvas_height = canvas.height;

    let canvas_mutex = Mutex::new(&mut canvas);

    let pb = ProgressBar::new((canvas_height * canvas_width).try_into().unwrap());

    pb.set_draw_rate(10);

    (0..canvas_height)
        .cartesian_product(0..canvas_width)
        .par_bridge()
        .for_each(|(x, y)| {
            let half = canvas_width as f64 * 0.5;

            let wall_point = Point::new(
                (x as f64 - half) * virtual_pixel_size,
                (half - y as f64) * virtual_pixel_size,
                virtual_canvas_pos_z,
            );
            let ray = Ray::new(ray_origin, (wall_point - ray_origin).normalize());

            let intersections = sphere.intersect(&ray);

            let mut canv = canvas_mutex.lock().unwrap();
            if let Some(_) = intersections.hit() {
                (*canv).set_color_at(x, y, default_palettes::full_bright::WHITE);
            } else {
                (*canv).set_color_at(x, y, default_palettes::full_bright::BLACK);
            }
            pb.inc(1);
        });

    pb.finish_with_message("Complete.");

    // save to png output/png.png
    let png: PNGImage = (&canvas).into();
    write("./output/png.png", png.as_bytes()).expect("Could not write ouput.png to disk.");
}
