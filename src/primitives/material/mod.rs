pub mod phong;
use dyn_clone::DynClone;

use crate::gfx::primitives::color::ColorRGBA;
use std::fmt::Debug;

use self::phong::Phong;

use super::{intersection::Intersection, light::Lights};

pub trait Material: Debug + DynClone + Sync {
    fn render(&self, intersection: &Intersection, lights: &Lights) -> ColorRGBA;
}

pub type Default = Phong;
