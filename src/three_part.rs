pub mod point;
pub mod vector;

#[derive(Copy, Clone, Debug)]
pub struct ThreePart(pub f64, pub f64, pub f64, pub f64);

use crate::util::f64_fuzzy_eq;
impl PartialEq for ThreePart {
    fn eq(&self, other: &Self) -> bool {
        f64_fuzzy_eq(self.0, other.0)
            && f64_fuzzy_eq(self.1, other.1)
            && f64_fuzzy_eq(self.2, other.2)
            && f64_fuzzy_eq(self.3, other.3)
    }
}
