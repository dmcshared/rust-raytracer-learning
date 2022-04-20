use raytracer::prelude::essential::*;

fn main() {
    let mut canvas = Canvas::new(16 * 32, 9 * 32);

    // let material = Phong::default()
    //     .with_diffuse(ColorRGBA::new(0.5, 0.5, 0.5, 1.0))
    //     .with_shininess(30.0);
    // let material = phong(
    //     ColorRGBA::new(0.1, 0.1, 0.1, 1.0),
    //     ColorRGBA::new(0.5, 0.5, 0.5, 1.0),
    //     ColorRGBA::new(0.9, 0.9, 0.9, 1.0),
    //     30.0,
    // );

    let material = MaterialStack::new(vec![
        Ambient::new(ColorRGBA::new(0.1, 0.1, 0.1, 1.0)).as_arc(),
        // Multiply::new(vec![
        Diffuse::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0)).as_arc(),
        //     CheckerBoard::new(
        //         ColorRGBA::new(1.0, 0.0, 0.0, 1.0),
        //         ColorRGBA::new(0.0, 0.0, 1.0, 1.0),
        //     )
        //     .as_arc(),
        // ])
        // .as_arc(),
        Specular::new(ColorRGBA::new(0.9, 0.9, 0.9, 1.0), 60.0).as_arc(),
    ])
    .as_arc();

    let material_red = Phong::default()
        .with_diffuse(ColorRGBA::new(1.0, 0.5, 0.5, 1.0))
        .with_shininess(60.0)
        .as_arc();

    // let scene = Sphere::new(Matrix4f::identity()).with_material(Arc::new(material));

    let scene = Scene::new(vec![
        Sphere::new(Matrix4f::identity())
            .with_material(material.clone())
            .as_arc(),
        Sphere::new(Matrix4f::translate_raw(0.0, 1.0, 0.0))
            .with_material(material_red)
            .as_arc(),
        Sphere::new(
            Matrix4f::translate_raw(0.0, -2.0, 0.0)
                * Matrix4f::scale_raw(100.0, 0.3, 100.0)
                * Matrix4f::translate_raw(0.0, -1.0, 0.0),
        )
        .with_material(material.clone())
        .as_arc(),
    ]);

    let lights = Lights::new(vec![
        // PointLight::new(
        //     Point::new(-10.0, 10.0, -10.0),
        //     ColorRGBA::new(1.0, 1.0, 1.0, 270.0), // The intensity should be the minimum distance to the scene squared
        // )
        // .as_arc(),
        DirectionalLight::new(
            Vector::new(1.0, -1.0, 1.0).normalize(),
            ColorRGBA::new(1.0, 1.0, 1.0, 1.0), // The intensity should be the minimum distance to the scene squared
        )
        .as_arc(),
        // PointLight::new(
        //     Point::new(-10.0, 10.0, -10.0),
        //     ColorRGBA::new(1.0, 0.5, 1.0, 250.0), // The intensity should be the minimum distance to the scene squared
        // )
        // .as_arc(),
        // PointLight::new(
        //     Point::new(10.0, 10.0, -10.0),
        //     ColorRGBA::new(1.0, 1.0, 0.5, 250.0), // The intensity should be the minimum distance to the scene squared
        // )
        // .as_arc(),
    ])
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

    let cam = Camera::new(
        Point::new(0.0, 0.0, -5.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        16.0,
        9.0,
        Degree(80.0).into(),
    );

    let cam_arc = cam.as_arc();

    let canvas_mutex = Mutex::new(&mut canvas);

    let pb = ProgressBar::new((canvas_height * canvas_width).try_into().unwrap());

    pb.set_draw_rate(10);

    (0..canvas_width)
        .cartesian_product(0..canvas_height)
        .par_bridge()
        .for_each(|(x, y)| {
            let ray = cam_arc.ray_for_pos(
                (x as f64) / (canvas_width as f64),
                (y as f64) / (canvas_height as f64),
            );

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
                    // default_palettes::full_bright::WHITE
                    //     * (hit.world_pos - ray.origin).magnitude().log10()
                    //     + default_palettes::full_bright::BLACK
                    //         * (1.0 - (hit.world_pos - ray.origin).magnitude().log10()),
                    // default_palettes::full_bright::WHITE * hit.t.log10()
                    //     + default_palettes::full_bright::BLACK * (1.0 - hit.t.log10()),
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
