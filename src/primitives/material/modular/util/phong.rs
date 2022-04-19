use std::sync::Arc;

use crate::{
    gfx::primitives::color::ColorRGBA,
    primitives::material::modular::{
        ambient::Ambient, diffuse::Diffuse, specular::Specular, MaterialStack,
    },
};

pub fn phong(
    ambient: ColorRGBA,
    diffuse: ColorRGBA,
    specular: ColorRGBA,
    shininess: f64,
) -> MaterialStack {
    MaterialStack::new(vec![
        Arc::new(Ambient::new(ambient)),
        Arc::new(Diffuse::new(diffuse)),
        Arc::new(Specular::new(specular, shininess)),
    ])
}
