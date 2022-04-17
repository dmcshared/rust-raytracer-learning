use super::Vector;

impl Vector {
    pub fn reflect_across(&self, normal: Vector) -> Vector {
        let incoming = *self;
        let reflected = incoming - normal * (incoming * normal) * 2.0;

        reflected
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn reflect_across() {
        let normal = Vector::new(0.0, 1.0, 0.0);
        let incoming = Vector::new(1.0, -1.0, 0.0);
        let reflected = incoming.reflect_across(normal);

        assert_eq!(reflected, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_across_slant() {
        let incoming = Vector::new(0.0, -1.0, 0.0);
        let normal = Vector::new(1.0, 1.0, 0.0).normalize();
        let reflected = incoming.reflect_across(normal);

        assert_eq!(reflected, Vector::new(1.0, 0.0, 0.0));
    }
}
