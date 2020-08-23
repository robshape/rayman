use crate::tracer::hit_point::HitPoint;
use crate::tracer::ray::{Ray, ScatteredRay};

pub trait Material {
  fn scatter(&self, ray: &Ray, hit_point: &HitPoint) -> ScatteredRay;
}
