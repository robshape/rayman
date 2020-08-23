use crate::scene::material::material::Material;
use crate::tracer::ray::Ray;
use crate::vec3::{Point3, Vec3};

// RUST: 'a reads as "the lifetime of a". Any reference to a HitPoint cannot outlive the reference
// to material.
pub struct HitPoint<'a> {
  front_facing: bool,         // Is the surface normal of the object facing the ray?
  material: &'a dyn Material, // The type of material hit by the ray.
  point: Point3,              // Position of intersection point / hit point.
  surface_normal: Vec3, // "Normal" vector (perpendicular to surface). Not the same as unit vector (normalized)!
  t: f64,               // Distance along the ray to the intersection point.
}

impl<'a> HitPoint<'a> {
  pub fn new(
    point: Point3,
    t: f64,
    surface_normal: Vec3,
    ray: &Ray,
    material: &'a dyn Material,
  ) -> HitPoint<'a> {
    let mut hit_point = HitPoint {
      front_facing: true, // Has to be calculated using set_front_facing()!
      point,
      surface_normal,
      t,
      material,
    };

    hit_point.set_front_facing(&ray);

    hit_point
  }

  pub fn front_facing(&self) -> bool {
    self.front_facing
  }

  pub fn material(&self) -> &'a dyn Material {
    self.material
  }

  pub fn point(&self) -> Point3 {
    self.point
  }

  pub fn surface_normal(&self) -> Vec3 {
    self.surface_normal
  }

  pub fn t(&self) -> f64 {
    self.t
  }

  fn set_front_facing(&mut self, ray: &Ray) {
    let ray_direction = ray.direction();
    let surface_normal_faces_ray = ray_direction.dot(self.surface_normal) < 0.0;

    self.front_facing = surface_normal_faces_ray;

    self.surface_normal = // RUST: There is no ternary operator!
      if surface_normal_faces_ray { // Ray is outside the object.
        self.surface_normal
      } else { // Ray is inside the object.
        -self.surface_normal
      }
  }
}
