use super::Rotation;

pub struct Radian(pub f64);

impl From<Radian> for Rotation {
    fn from(rad: Radian) -> Self {
        Rotation { val: rad.0 }
    }
}

impl From<Rotation> for Radian {
    fn from(rot: Rotation) -> Self {
        Radian { 0: rot.val }
    }
}
