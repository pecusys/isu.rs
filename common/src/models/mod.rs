pub mod demo;
pub mod section;
pub mod step;

pub use demo::Demo;
pub use step::Step;
pub use section::Section;

use serde_derive::*;
use serde::{Serialize, Deserialize};

pub trait PointPair {}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct IntCoords { pub x: u32, pub y: u32 }
#[derive(Serialize, Deserialize, PartialEq)]
pub struct FloatCoords { pub x: f32, pub y: f32 }
#[derive(Serialize, Deserialize, PartialEq)]
pub struct IntDims { pub w: u32, pub h: u32 }
#[derive(Serialize, Deserialize, PartialEq)]
pub struct FloatDims { pub w: f32, pub h: f32 }

impl PointPair for IntCoords {}
impl PointPair for FloatCoords {}
impl PointPair for IntDims {}
impl PointPair for FloatDims {}

impl From<(u32, u32)> for IntDims {
    fn from(tup: (u32, u32)) -> IntDims {
        IntDims { w: tup.0, h: tup.1}
    }
}

impl From<(u32, u32)> for IntCoords {
    fn from(tup: (u32, u32)) -> IntCoords {
        IntCoords { x: tup.0, y: tup.1}
    }
}

impl From<(f32, f32)> for FloatDims {
    fn from(tup: (f32, f32)) -> FloatDims {
        FloatDims { w: tup.0, h: tup.1}
    }
}

impl From<(f32, f32)> for FloatCoords {
    fn from(tup: (f32, f32)) -> FloatCoords {
        FloatCoords { x: tup.0, y: tup.1}
    }
}
