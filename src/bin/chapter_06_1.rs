use indicatif::ProgressBar;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use raytracer::{
    gfx::{
        canvas::Canvas,
        image_formats::{png::PNGImage, Image},
        primitives::color::{default_palettes, ColorRGBA},
    },
    primitives::{
        body::{scene::Scene, sphere::Sphere, Body, BodyBuilder},
        intersection::IntersectionList,
        light::{point_light::PointLight, Lights},
        material::phong::Phong,
        matrix::Matrix4f,
        ray::Ray,
        three_part::point::Point,
        world_info::{Limits, WorldInfo},
    },
    util::NewAsArc,
};
use std::{fs::write, sync::Arc, sync::Mutex};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let virtual_canvas_pos_z = 5.0;
    let virtual_canvas_size = 10.0;
    let mut canvas = Canvas::new(512, 512);
    let virtual_pixel_size = virtual_canvas_size / (canvas.width as f64);

    let material = Phong::default()
        .with_diffuse(ColorRGBA::new(0.5, 0.5, 0.5, 1.0))
        .with_shininess(30.0);

    let material_red = Phong::default()
        .with_diffuse(ColorRGBA::new(1.0, 0.5, 0.5, 1.0))
        .with_shininess(30.0);

    // let scene = Sphere::new(Matrix4f::identity()).with_material(Arc::new(material));

    let scene = Scene::new(vec![
        Arc::new(Sphere::new(Matrix4f::identity()).with_material(Arc::new(material))),
        Arc::new(
            Sphere::new(Matrix4f::translate_raw(0.0, 1.0, 0.0))
                .with_material(Arc::new(material_red)),
        ),
    ]);

    let lights = Lights::new(vec![Arc::new(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        ColorRGBA::new(1.0, 1.0, 1.0, 270.0), // The intensity should be the minimum distance to the scene squared
    ))])
    .as_arc();

    let world_info = WorldInfo {
        root_object: scene.clone(),
        lights: lights.clone(),
        limits: Limits {
            max_light_bounces: 5,
        },
    }
    .as_arc();

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

            let intersections = scene.intersect(&ray);

            let canv_opt = canvas_mutex.lock();

            if canv_opt.is_err() {
                return;
            }

            let mut canv = canv_opt.unwrap();
            if let Some(hit) = intersections.hit() {
                (*canv).set_color_at(
                    x,
                    y,
                    hit.object.get_material().render(hit, world_info.clone()),
                );
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
