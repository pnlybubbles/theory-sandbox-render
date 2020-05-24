use math::*;
use std::ops::Mul;

pub trait Measure {}

pub struct SolidAngle(pub f32);
pub struct Area(pub f32);

impl Measure for SolidAngle {}
impl Measure for Area {}

impl Mul<f32> for SolidAngle {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    let SolidAngle(lhs) = self;
    SolidAngle(rhs * lhs)
  }
}

impl Mul<f32> for Area {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    let Area(lhs) = self;
    Area(rhs * lhs)
  }
}

impl SolidAngle {
  pub fn into_area(self, x: Vector3, x2: Vector3, n2: Vector3) -> Area {
    let path = x2 - x;
    let wi = path.normalize();
    Area((-wi).dot(n2) / path.sqr_norm())
  }
}
