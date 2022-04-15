use std::ops;

use super::ThreePart;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub ThreePart);

impl Point {
    pub fn origin() -> Self {
        Point(ThreePart(0.0, 0.0, 0.0, 1.0))
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point(ThreePart(x, y, z, 1.0))
    }
}

use crate::primitives::three_part::vector::Vector;

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, Vector(rhs): Vector) -> Self::Output {
        let Point(lhs) = self;
        Point::new(lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
    }
}
impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, Point(rhs): Point) -> Self::Output {
        let Point(lhs) = self;
        Vector::new(lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
    }
}
impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, Vector(rhs): Vector) -> Self::Output {
        let Point(lhs) = self;
        Point::new(lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use super::ThreePart;
    use super::Vector;

    #[test]
    fn can_be_created_direct() {
        let point = Point(ThreePart(4.3, -4.2, 3.1, 1.0));
        let Point(three_part) = point;
        assert_eq!(three_part, ThreePart(4.3, -4.2, 3.1, 1.));
    }
    #[test]
    fn can_be_created_default() {
        let point = Point::origin();
        let Point(three_part) = point;
        assert_eq!(three_part, ThreePart(0., 0., 0., 1.));
    }
    #[test]
    fn can_be_created_values() {
        let point = Point::new(4.3, -4.2, 3.1);
        let Point(three_part) = point;

        assert_eq!(three_part, ThreePart(4.3, -4.2, 3.1, 1.));
    }

    #[test]
    fn can_add_point_and_vector() {
        let point = Point::new(3., -2., 5.);
        let vector = Vector::new(-2., 3., 1.);

        let Point(out) = point + vector;

        assert_eq!(out, ThreePart(1., 1., 6., 1.));
    }

    #[test]
    fn can_add_point_and_point() {
        let point1 = Point::new(5., -2., 8.);
        let point2 = Point::new(-9., 2., 5.);

        let Point(out) = point2 + (point1 - Point::origin());
        assert_eq!(out, ThreePart(-4., 0., 13., 1.));
    }

    #[test]
    fn can_sub_point_and_vector() {
        let point = Point::new(4., -7., 3.);
        let vector = Vector::new(-8., 6., 0.);

        let Point(out) = point - vector;
        assert_eq!(out, ThreePart(12., -13., 3., 1.));
    }

    #[test]
    fn can_sub_point_and_point() {
        let point1 = Point::new(1., 2., 3.);
        let point2 = Point::new(4., 5., 6.);

        let Vector(out) = point1 - point2;
        assert_eq!(out, ThreePart(-3., -3., -3., 0.));
    }
}
