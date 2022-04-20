use crate::prelude::body::*;

use super::rotation::Rotation;

pub struct Camera {
    // transform: Matrix4f,
    position: Point,
    forward: Vector,
    up: Vector,
    hsize: f64,
    vsize: f64,
    z: f64,
}

impl Camera {
    pub fn new(
        position: Point,
        forward: Vector,
        up: Vector,
        hsize: f64,
        vsize: f64,
        fov: Rotation,
    ) -> Self {
        Self {
            // transform: Matrix4f::identity(),
            vsize,
            hsize,
            z: (1.0 / (fov.val / 2.0).tan()) * 0.5 * hsize.max(vsize),
            position,
            forward: forward.normalize(),
            up: up.normalize(),
        }
    }

    // pub fn with_transform(mut self, transform: Matrix4f) -> Self {
    //     self.transform = transform;
    //     self
    // }
    /// Takes x, y in the range [0,1] and returns a ray
    pub fn ray_for_pos(&self, x: f64, y: f64) -> Ray {
        let x = (x - 0.5) * self.hsize;
        let y = (0.5 - y) * self.vsize;

        let forward = self.forward;
        let up = self.up;
        let right = (forward / up).normalize();
        let up = (right / forward).normalize();

        let origin = self.position;
        let direction = right * x + up * y + forward * self.z;

        Ray::new(origin, direction.normalize())
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::rotation::degrees::Degree;

    use super::*;

    #[test]
    fn test_camera_new() {
        let camera = Camera::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
            1.0,
            1.0,
            Degree(90.0).into(),
        );

        assert_eq!(camera.position, Point::new(0.0, 0.0, 0.0));
        assert_eq!(camera.forward, Vector::new(0.0, 0.0, 1.0));
        assert_eq!(camera.up, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(camera.hsize, 1.0);
        assert_eq!(camera.vsize, 1.0);
        assert_eq!(camera.z, 0.5);
    }

    #[test]
    fn test_camera_ray_for_pos() {
        let camera = Camera::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
            1.0,
            1.0,
            Degree(90.0).into(),
        );
        assert_eq!(
            camera.ray_for_pos(0.5, 0.5),
            Ray::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 1.0).normalize()
            )
        );
    }
}
