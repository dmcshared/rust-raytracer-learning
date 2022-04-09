pub mod vector {
    use std::ops;

    use super::super::three_part::ThreePart;

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

    #[cfg(test)]
    mod tests {

        fn check_w_is_zero(three_part: super::ThreePart) {
            assert_eq!(three_part.w, 0.0);
        }

        #[test]
        fn can_be_created_direct() {
            let vector = super::Vector(super::ThreePart {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 1.0,
            });
            let super::Vector(three_part) = vector;
            assert_eq!(three_part.x, 4.3);
            assert_eq!(three_part.y, -4.2);
            assert_eq!(three_part.z, 3.1);
            check_w_is_zero(three_part);
        }
        #[test]
        fn can_be_created_default() {
            let vector = super::Vector::origin();
            let super::Vector(three_part) = vector;
            assert_eq!(three_part.x, 0.0);
            assert_eq!(three_part.y, 0.0);
            assert_eq!(three_part.z, 0.0);
            check_w_is_zero(three_part);
        }
        #[test]
        fn can_be_created_values() {
            let vector = super::Vector::new(4.3, -4.2, 3.1);
            let super::Vector(three_part) = vector;
            assert_eq!(three_part.x, 4.3);
            assert_eq!(three_part.y, -4.2);
            assert_eq!(three_part.z, 3.1);
            check_w_is_zero(three_part);
        }
    }
}
