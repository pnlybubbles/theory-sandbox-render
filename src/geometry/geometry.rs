use super::intersection::Intersection;
use math::Vector3;
use ray::Ray;
use sample::{pdf, Sample};

pub trait Geometry {
  fn intersect(&self, &Ray) -> Option<Intersection>;
  fn area(&self) -> f32;
  fn sample(&self) -> Sample<Vector3, pdf::Area>;
}
