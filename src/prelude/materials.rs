pub use crate::primitives::material::{
    modular::{
        ambient::Ambient, combinators::multiply::Multiply, diffuse::Diffuse,
        special::checkerboard::CheckerBoard, specular::Specular, MaterialStack,
    },
    phong::Phong,
};

pub use crate::gfx::primitives::color::ColorRGBA;
