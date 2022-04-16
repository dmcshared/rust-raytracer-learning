use super::Rotation;

pub struct Degree(pub f64);

impl From<Degree> for Rotation {
    fn from(deg: Degree) -> Self {
        Rotation {
            val: deg.0 * std::f64::consts::PI * (1.0 / 180.0),
        }
    }
}

impl From<Rotation> for Degree {
    fn from(rot: Rotation) -> Self {
        Degree {
            0: rot.val * 180.0 * std::f64::consts::FRAC_1_PI,
        }
    }
}
