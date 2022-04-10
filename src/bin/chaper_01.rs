extern crate raytracer;

use raytracer::three_part::{point::Point, vector::Vector};

fn main() {
    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.0001, 0.0, 0.0));
    let projectile = Projectile::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.02, 0.0, 0.0));

    println!("{:?}", environment);

    let mut current = projectile;
    let mut iteration: i32 = 0;
    while current.position.0 .1 > 0.0 {
        println!("{}: {:?}", iteration, current);
        current = tick(&environment, &current);
        iteration += 1;
    }
    println!("FINISHED => {}: {:?}", iteration, current);
}

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
        projectile.position + projectile.velocity,
        projectile.velocity + environment.gravity + environment.wind,
    )
}
