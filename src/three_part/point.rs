pub mod point {
    use std::ops;

    use super::super::three_part::ThreePart;

    pub struct Point(ThreePart);

    impl Point {
        fn origin() -> Self {
            Point(ThreePart {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            })
        }
        fn new(x: f64, y: f64, z: f64) -> Self {
            Point(ThreePart { x, y, z, w: 1.0 })
        }
    }

    use crate::three_part::vector::vector::Vector;

    impl ops::Add<Vector> for Point {
        type Output = Point;

        fn add(self, Vector(rhs): Vector) -> Self::Output {
            let Point(lhs) = self;
            Point::new(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z)
        }
    }
    impl ops::Sub for Point {
        type Output = Vector;

        fn sub(self, Point(rhs): Point) -> Self::Output {
            let Point(lhs) = self;
            Vector::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
        }
    }
    impl ops::Sub<Vector> for Point {
        type Output = Point;

        fn sub(self, Vector(rhs): Vector) -> Self::Output {
            let Point(lhs) = self;
            Point::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Point;
        use super::ThreePart;
        use super::Vector;

        fn check_w_is_one(three_part: ThreePart) {
            assert_eq!(three_part.w, 1.0);
        }

        #[test]
        fn can_be_created_direct() {
            let point = Point(ThreePart {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 1.0,
            });
            let Point(three_part) = point;
            assert_eq!(three_part.x, 4.3);
            assert_eq!(three_part.y, -4.2);
            assert_eq!(three_part.z, 3.1);
            check_w_is_one(three_part);
        }
        #[test]
        fn can_be_created_default() {
            let point = Point::origin();
            let Point(three_part) = point;
            assert_eq!(three_part.x, 0.0);
            assert_eq!(three_part.y, 0.0);
            assert_eq!(three_part.z, 0.0);
            check_w_is_one(three_part);
        }
        #[test]
        fn can_be_created_values() {
            let point = Point::new(4.3, -4.2, 3.1);
            let Point(three_part) = point;
            assert_eq!(three_part.x, 4.3);
            assert_eq!(three_part.y, -4.2);
            assert_eq!(three_part.z, 3.1);
            check_w_is_one(three_part);
        }

        #[test]
        fn can_add_point_and_vector() {
            let point = Point::new(3., -2., 5.);
            let vector = Vector::new(-2., 3., 1.);

            let Point(out) = point + vector;
            assert_eq!(
                out,
                ThreePart {
                    x: 1.,
                    y: 1.,
                    z: 6.,
                    w: 1.
                }
            );
        }

        #[test]
        fn can_add_point_and_point() {
            let point1 = Point::new(5., -2., 8.);
            let point2 = Point::new(-9., 2., 5.);

            let Point(out) = point2 + (point1 - Point::origin());
            assert_eq!(
                out,
                ThreePart {
                    x: -4.,
                    y: 0.,
                    z: 13.,
                    w: 1.
                }
            );
        }

        #[test]
        fn can_sub_point_and_vector() {
            let point = Point::new(4., -7., 3.);
            let vector = Vector::new(-8., 6., 0.);

            let Point(out) = point - vector;
            assert_eq!(
                out,
                ThreePart {
                    x: 12.,
                    y: -13.,
                    z: 3.,
                    w: 1.
                }
            );
        }

        #[test]
        fn can_sub_point_and_point() {
            let point1 = Point::new(1., 2., 3.);
            let point2 = Point::new(4., 5., 6.);

            let Vector(out) = point1 - point2;
            assert_eq!(
                out,
                ThreePart {
                    x: -3.,
                    y: -3.,
                    z: -3.,
                    w: 0.
                }
            );
        }
    }
}
