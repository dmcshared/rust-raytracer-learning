use crate::{
    primitives::{
        intersection::Intersection, material::Material, ray::Ray, three_part::point::Point,
    },
    util::Defaultable,
};

use super::{transform::TransformedBody, Body, BodyBuilder};

#[derive(Debug)]
pub struct RawSphere {
    pub material: Box<dyn Material>,
}

impl Clone for RawSphere {
    fn clone(&self) -> Self {
        Self {
            material: dyn_clone::clone_box(&*self.material),
        }
    }
}

impl Body for RawSphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Point::origin(); // (0, 0, 5)
        let a = ray.direction.sqr_magnitude(); // 1
        let b = 2.0 * (ray.direction * sphere_to_ray); // 10
        let c = sphere_to_ray.sqr_magnitude() - 1.0; // 24

        // let discriminant = (ray.direction * sphere_to_ray).powi(2) - c; // 5 - 24 = -19
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else if discriminant == 0.0 {
            let t = -b / (2.0 * a);
            vec![Intersection::new(t, Box::new((*self).clone()), *ray)]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![
                Intersection::new(t1, Box::new((*self).clone()), *ray),
                Intersection::new(t2, Box::new((*self).clone()), *ray),
            ]
        }
    }

    fn normal_raw(&self, x: f64, y: f64, z: f64) -> crate::primitives::three_part::vector::Vector {
        (Point::new(x, y, z) - Point::origin()).normalize()
    }

    fn get_material(&self) -> Box<dyn crate::primitives::material::Material> {
        dyn_clone::clone_box(&*self.material)
    }
}

impl BodyBuilder for RawSphere {
    fn with_material(&self, material: Box<dyn Material>) -> Self {
        RawSphere { material }
    }
}

impl Defaultable for RawSphere {
    fn default() -> Self {
        RawSphere {
            material: Box::new(crate::primitives::material::Default::default()),
        }
    }
}

pub type Sphere = TransformedBody<RawSphere>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{
        intersection::IntersectionList, matrix::Matrix4f, rotation::degrees::Degree,
        three_part::vector::Vector,
    };
    use crate::primitives::{ray::Ray, three_part::point::Point};

    #[test]
    fn test_sphere_intersect() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_sphere_hits_negative() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn test_sphere_misses() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_sphere_behind() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_sphere_inside() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn test_sphere_tangent() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 1.0, -1.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
    }

    #[test]
    fn test_translated_sphere() {
        let s = Sphere::new(Matrix4f::translate_raw(2.0, 0.0, 0.0));
        let r = Ray::new(Point::new(2.0, 0.0, -3.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 2.0);
        assert_eq!(xs[1].t, 4.0);
    }

    #[test]
    fn test_hit_method() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);

        let x = xs
            .hit()
            .expect("This should contain 2 intersections, so a hit should always exist.");

        assert_eq!(x.t, 4.0);
    }

    #[test]
    fn test_diagonal_should_miss() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(
            Point::new(0.0, 0.0, -5.0),
            Vector::new(5.0, -5.0, 10.0).normalize(),
        );
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::new(Matrix4f::identity());
        let n = s.normal(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_scaled_and_rotated_sphere() {
        let s = Sphere::new(
            Matrix4f::scale_raw(1.0, 0.5, 1.0) * Matrix4f::rotate_around_z(Degree(36.0).into()),
        );
        let sqrt2_over_2 = 2.0f64.sqrt() / 2.0;
        let p = Point::new(0.0, sqrt2_over_2, -sqrt2_over_2);
        let n = s.normal(p);

        let expected_result = Vector::new(0.0, 0.97014, -0.24254);
        assert_eq!(n, expected_result);
    }

    #[test]
    fn normal_should_be_normalized() {
        let s = Sphere::new(
            Matrix4f::scale_raw(1.0, 0.5, 1.0) * Matrix4f::rotate_around_z(Degree(36.0).into()),
        );
        let sqrt2_over_2 = 2.0f64.sqrt() / 2.0;
        let p = Point::new(0.0, sqrt2_over_2, -sqrt2_over_2);
        let n = s.normal(p);

        assert_eq!(n.sqr_magnitude(), 1.0);
        assert_eq!(n, n.normalize());
    }
}
