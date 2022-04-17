use crate::{
    gfx::primitives::color::ColorRGBA,
    primitives::{ray::Ray, three_part::vector::Vector},
};

use super::Light;

pub struct DirectionalLight {
    pub direction: Vector,
    pub intensity: ColorRGBA,
}

impl DirectionalLight {
    pub fn new(direction: Vector, intensity: ColorRGBA) -> Self {
        Self {
            direction: direction.normalize(),
            intensity,
        }
    }
}

impl Light for DirectionalLight {
    fn light_effectiveness(&self, r: Ray) -> ColorRGBA {
        let direction = self.direction;
        let cosine = -(r.direction * direction);
        // if cosine > 0.0 {
        //     1.0 / (4.0 * std::f64::consts::PI) * cosine * cosine
        // } else {
        //     0.0
        // }
        if cosine > 0.0 {
            self.intensity.mul_all(cosine)
        } else {
            ColorRGBA::blank()
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        assert_fuzzy_eq, primitives::three_part::point::Point,
        util::fuzzy_comparison::FuzzyPartialEq,
    };

    use super::*;

    #[test]
    fn directional_light_new() {
        let direction = Vector::new(1.0, 2.0, 3.0);
        let intensity = ColorRGBA::new(1.0, 1.0, 1.0, 1.0);
        let l = DirectionalLight::new(direction, intensity);
        assert_eq!(l.direction, direction.normalize());
        assert_eq!(l.intensity, intensity);
    }

    #[test]
    fn directional_light_should_not_care_about_origin() {
        let direction = Vector::new(0.0, -1.0, 0.0);
        let intensity = ColorRGBA::new(1.0, 1.0, 1.0, 1.0);

        let l = DirectionalLight::new(direction, intensity);

        let r = Ray::new(Point::new(2.0, 7.0, 4.0), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(l.light_effectiveness(r), intensity);
        let r = Ray::new(Point::new(98.0, 4.0, 6.0), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(l.light_effectiveness(r), intensity);
    }

    #[test]
    fn directional_light_45deg() {
        let direction = Vector::new(0.0, -1.0, 0.0);
        let intensity = ColorRGBA::new(1.0, 1.0, 1.0, 1.0);

        let l = DirectionalLight::new(direction, intensity);

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 1.0));
        assert_fuzzy_eq!(
            l.light_effectiveness(r),
            intensity * ((2.0 as f64).sqrt() / 2.0)
        );
    }
}
