pub mod phong;
use dyn_clone::DynClone;

use crate::gfx::primitives::color::ColorRGBA;
use std::fmt::Debug;

use self::phong::Phong;

use super::intersection::Intersection;

pub trait Material: Debug + DynClone + Sync {
    fn render(&self, intersection: Intersection) -> ColorRGBA;
}

pub type Default = Phong;
