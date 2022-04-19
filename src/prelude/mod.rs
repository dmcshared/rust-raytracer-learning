pub mod bodies;
pub mod body;
pub mod export;
pub mod general;
pub mod light;
pub mod lights;
/// For usage with making new materials.
pub mod material;
/// For usage with utilizing existing material structs.
pub mod materials;

pub mod all {
    pub use super::essential::*;
    pub use super::{body::*, material::*};
}

pub mod essential {
    pub use super::body::{IntersectionList, Matrix4f, Point};
    pub use super::{bodies::*, export::*, general::*, lights::*, materials::*};
}
