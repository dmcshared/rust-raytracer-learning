use std::sync::Arc;

use super::{body::Body, light::Light};

pub struct WorldInfo {
    pub root_object: Arc<dyn Body>,
    pub lights: Arc<dyn Light>,
    pub limits: Limits,
}

pub struct Limits {
    pub max_light_bounces: usize,
}
