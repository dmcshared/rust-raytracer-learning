pub mod modular;
pub mod phong;

use crate::gfx::primitives::color::ColorRGBA;
use std::{fmt::Debug, sync::Arc};

use self::phong::Phong;

use super::{intersection::Intersection, world_info::WorldInfo};

pub trait Material: Debug + Sync + Send {
    fn render(&self, intersection: &Intersection, world_info: Arc<WorldInfo>) -> ColorRGBA;
}

pub type Default = Phong;
