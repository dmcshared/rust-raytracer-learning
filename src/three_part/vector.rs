use std::ops;

use super::ThreePart;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(pub ThreePart);

impl Vector {
    pub fn origin() -> Self {
        Vector(ThreePart {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        })
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector(ThreePart { x, y, z, w: 0.0 })
    }

    pub fn magnitude(&self) -> f64 {
        let Vector(self_part) = self;
        (self_part.x.powi(2) + self_part.y.powi(2) + self_part.z.powi(2)).sqrt()
    }

    pub fn sqr_magnitude(&self) -> f64 {
        let Vector(self_part) = self;
        (self_part.x.powi(2) + self_part.y.powi(2) + self_part.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        *self / self.magnitude()
    }
}

// use crate::three_part::point::point::Point;

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, Vector(rhs): Vector) -> Self::Output {
        let Vector(lhs) = self;
        Vector::new(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z)
    }
}
impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, Vector(rhs): Vector) -> Self::Output {
        let Vector(lhs) = self;
        Vector::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
    }
}
impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        let Vector(lhs) = self;
        Vector::new(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs)
    }
}
impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        let Vector(lhs) = self;
        Vector::new(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs)
    }
}
impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        let Vector(lhs) = self;
        Vector::new(-lhs.x, -lhs.y, -lhs.z)
    }
}

// Dot Product
impl ops::Mul for Vector {
    type Output = f64;

    fn mul(self, Vector(rhs): Vector) -> Self::Output {
        let Vector(lhs) = self;
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }
}

// Cross product
impl ops::Div for Vector {
    type Output = Vector;

    fn div(self, Vector(rhs): Vector) -> Self::Output {
        let Vector(lhs) = self;
        Vector::new(
            lhs.y * rhs.z - lhs.z * rhs.y,
            lhs.z * rhs.x - lhs.x * rhs.z,
            lhs.x * rhs.y - lhs.y * rhs.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{ThreePart, Vector};

    #[test]
    fn can_be_created_direct() {
        let vector = Vector(ThreePart {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        });
        let Vector(three_part) = vector;

        assert_eq!(
            three_part,
            ThreePart {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 0.0,
            }
        );
    }
    #[test]
    fn can_be_created_default() {
        let vector = Vector::origin();
        let Vector(three_part) = vector;

        assert_eq!(
            three_part,
            ThreePart {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0
            }
        );
    }
    #[test]
    fn can_be_created_values() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        let Vector(three_part) = vector;

        assert_eq!(
            three_part,
            ThreePart {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 0.0
            }
        );
    }

    #[test]
    fn can_be_added() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        let vector2 = Vector::new(1.2, 3.4, -5.6);
        let vector3 = vector + vector2;
        let Vector(three_part) = vector3;

        assert_eq!(
            three_part,
            ThreePart {
                x: 5.5,
                y: -0.8,
                z: -2.5,
                w: 0.0
            }
        );
    }

    #[test]
    fn can_be_subtracted() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        let vector2 = Vector::new(1.2, 3.4, -5.6);
        let vector3 = vector - vector2;
        let Vector(three_part) = vector3;

        assert_eq!(
            three_part,
            ThreePart {
                x: 3.1,
                y: -7.6,
                z: 8.7,
                w: 0.0
            }
        );
    }

    #[test]
    fn can_be_multiplied_by_a_scalar() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        let vector2 = vector * 2.0;
        let Vector(three_part) = vector2;

        assert_eq!(
            three_part,
            ThreePart {
                x: 8.6,
                y: -8.4,
                z: 6.2,
                w: 0.0
            }
        );
    }

    #[test]
    fn can_be_divided_by_a_scalar() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        let vector2 = vector / 2.0;
        let Vector(three_part) = vector2;

        assert_eq!(
            three_part,
            ThreePart {
                x: 2.15,
                y: -2.1,
                z: 1.55,
                w: 0.0
            }
        );
    }

    #[test]
    fn can_be_negated() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        let vector2 = -vector;
        let Vector(three_part) = vector2;

        assert_eq!(
            three_part,
            ThreePart {
                x: -4.3,
                y: 4.2,
                z: -3.1,
                w: 0.0
            }
        );
    }

    #[test]
    fn get_magnitude_of_unit_vector() {
        let vector = Vector::new(1., 0., 0.);
        let magnitude = vector.magnitude();
        let expected_magnitude = 1.0;

        assert_eq!(magnitude, expected_magnitude);
    }

    #[test]
    fn get_magnitude_of_1_2_3_vector() {
        let vector = Vector::new(1., 2., 3.);
        let magnitude = vector.magnitude();
        let expected_magnitude = (14.0_f64).sqrt();

        assert_eq!(magnitude, expected_magnitude);
    }

    #[test]
    fn get_magnitude_of_negative_1_2_3_vector() {
        let vector = Vector::new(-1., -2., -3.);
        let magnitude = vector.magnitude();
        let expected_magnitude = (14.0_f64).sqrt();

        assert_eq!(magnitude, expected_magnitude);
    }

    #[test]
    fn normalize_simple_vector() {
        let vector = Vector::new(4., 0., 0.);
        let normalized_vector = vector.normalize();

        assert_eq!(normalized_vector, Vector::new(1., 0., 0.));
    }

    #[test]
    fn normalize_1_2_3_vector() {
        let vector = Vector::new(1., 2., 3.);
        let normalized_vector = vector.normalize();

        assert_eq!(
            normalized_vector,
            Vector::new(
                1.0 / (14.0_f64).sqrt(),
                2.0 / (14.0_f64).sqrt(),
                3.0 / (14.0_f64).sqrt()
            )
        );
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let vector = Vector::new(1., 2., 3.);
        let vector2 = Vector::new(4., 5., 6.);
        let dot_product = vector * vector2;

        assert_eq!(dot_product, 32.);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let vector = Vector::new(1., 2., 3.);
        let vector2 = Vector::new(4., 5., 6.);
        let cross_product = vector / vector2;

        assert_eq!(cross_product, Vector::new(-3., 6., -3.));
    }
}
