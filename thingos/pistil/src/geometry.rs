pub use pistil_types::{Color, Point, Rect, Size};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform2D {
    pub const fn identity() -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 }
    }

    pub fn transform_point_f(&self, x: f32, y: f32) -> (f32, f32) {
        (self.a * x + self.c * y + self.tx, self.b * x + self.d * y + self.ty)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EdgeAA {
    #[default]
    None,
    Coverage8,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PathVerb {
    MoveTo(PointF),
    LineTo(PointF),
    QuadTo(PointF, PointF),
    CubicTo(PointF, PointF, PointF),
    Close,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Path2D {
    pub verbs: alloc::vec::Vec<PathVerb>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}
