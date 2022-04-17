use crate::{
    gfx::primitives::color::ColorRGBA,
    primitives::{ray::Ray, three_part::point::Point},
};

use super::Light;

pub struct PointLight {
    pub position: Point,
    pub intensity: ColorRGBA,
}

impl PointLight {
    pub fn new(position: Point, intensity: ColorRGBA) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl Light for PointLight {
    fn light_effectiveness(&self, r: Ray) -> ColorRGBA {
        let direction = self.position - r.origin;
        let distance = direction.magnitude();
        let direction = direction.normalize();
        let cosine = r.direction * direction;
        // if cosine > 0.0 {
        //     1.0 / (4.0 * std::f64::consts::PI * distance * distance) * cosine * cosine
        // } else {
        //     0.0
        // }
        if cosine > 0.0 {
            self.intensity.mul_all(cosine / (distance * distance))
        } else {
            ColorRGBA::blank()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::three_part::vector::Vector;

    use super::*;

    #[test]
    fn point_light_new() {
        let position = Point::new(1.0, 2.0, 3.0);
        let intensity = ColorRGBA::new(1.0, 1.0, 1.0, 1.0);
        let l = PointLight::new(position, intensity);
        assert_eq!(l.position, position);
        assert_eq!(l.intensity, intensity);
    }

    #[test]
    fn point_light_light_effectiveness() {
        let position = Point::new(0.0, 0.0, 0.0);
        let intensity = ColorRGBA::new(1.0, 1.0, 1.0, 1.0);
        let l = PointLight::new(position, intensity);

        let r = Ray::new(Point::new(0.0, 0.0, -1.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(l.light_effectiveness(r), intensity.mul_all(1.0));
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(l.light_effectiveness(r), intensity.mul_all(0.0));
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), Vector::new(0.0, 0.0, -1.0));
        assert_eq!(l.light_effectiveness(r), intensity.mul_all(0.0));
    }
}
