pub mod phong;

use crate::gfx::primitives::color::ColorRGBA;
use std::fmt::Debug;

use self::phong::Phong;

use super::{intersection::Intersection, light::Lights};

pub trait Material: Debug + Sync + Send {
    fn render(&self, intersection: &Intersection, lights: &Lights) -> ColorRGBA;
}

pub type Default = Phong;
