pub mod point;
pub mod vector;

#[derive(Copy, Clone, Debug)]
pub struct ThreePart {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

use crate::util::f64_fuzzy_eq;
impl PartialEq for ThreePart {
    fn eq(&self, other: &Self) -> bool {
        f64_fuzzy_eq(self.x, other.x)
            && f64_fuzzy_eq(self.y, other.y)
            && f64_fuzzy_eq(self.z, other.z)
            && f64_fuzzy_eq(self.w, other.w)
    }
}
