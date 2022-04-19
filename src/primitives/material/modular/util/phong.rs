use crate::prelude::materials::*;
use std::sync::Arc;

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
