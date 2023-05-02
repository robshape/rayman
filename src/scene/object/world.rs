use std::vec::Vec;

use crate::scene::object::object::Object;
use crate::tracer::hit_point::HitPoint;
use crate::tracer::ray::Ray;

pub struct World<T: Object> {
    objects: Vec<T>,
}

impl<T: Object> World<T> {
    pub fn new(objects: Vec<T>) -> World<T> {
        World { objects }
    }
}

impl<T: Object> Object for World<T> {
    fn is_hit_by_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitPoint> {
        let mut closest_object_so_far = t_max;
        let mut object_hit_by_ray: Option<HitPoint> = None;

        for object in &self.objects {
            // RUST: Match single pattern: Some.
            if let Some(hit_point) = object.is_hit_by_ray(&ray, t_min, closest_object_so_far) {
                closest_object_so_far = hit_point.t();
                object_hit_by_ray = Some(hit_point);
            }
        }

        object_hit_by_ray
    }
}
