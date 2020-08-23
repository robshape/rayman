use crate::tracer::hit_point::HitPoint;
use crate::tracer::ray::Ray;

pub trait Object {
  fn is_hit_by_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitPoint>;
}
