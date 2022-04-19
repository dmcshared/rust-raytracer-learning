pub mod directional_light;
pub mod point_light;

use crate::prelude::light::*;
use std::sync::Arc;

pub trait Light: Sync + Send {
    fn light_effectiveness(&self, r: Ray) -> ColorRGBA;
    fn light_effectiveness_exp(&self, r: Ray, shininess: f64) -> ColorRGBA;
}

pub struct Lights {
    pub lights: Vec<Arc<dyn Light>>,
}

impl Lights {
    pub fn new(lights: Vec<Arc<dyn Light>>) -> Self {
        Self { lights }
    }
}

impl Light for Lights {
    fn light_effectiveness(&self, r: Ray) -> ColorRGBA {
        let mut color = ColorRGBA::blank();
        for light in &self.lights {
            let lf = light.light_effectiveness(r);
            color.0 += lf.0;
            color.1 += lf.1;
            color.2 += lf.2;
            color.3 += lf.3;
        }

        // convert from (ra,ba,ga,a) to (r,g,b,a)
        color.0 /= color.3;
        color.1 /= color.3;
        color.2 /= color.3;

        color
    }

    fn light_effectiveness_exp(&self, r: Ray, shininess: f64) -> ColorRGBA {
        let mut color = ColorRGBA::blank();
        for light in &self.lights {
            let lf = light.light_effectiveness_exp(r, shininess);
            color.0 += lf.0;
            color.1 += lf.1;
            color.2 += lf.2;
            color.3 += lf.3;
        }

        // convert from (ra,ba,ga,a) to (r,g,b,a)
        color.0 /= color.3;
        color.1 /= color.3;
        color.2 /= color.3;

        color
    }
}

// for phong you would need to do light_effectiveness(r) * light_effectiveness(r.reflect_over(n))
