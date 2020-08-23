use crate::scene::material::material::Material;
use crate::tracer::hit_point::HitPoint;
use crate::tracer::ray::{Ray, ScatteredRay};
use crate::vec3::{Color, Vec3};

// There are different types of reflective materials:
// - Polished (undisturbed reflection; a ray won't be scattered randomly)
// - Blurry (tiny random bumps on the surface cause the reflection to become blurry)
// - Metallic (highlights and reflections retain the color of the reflective object)
pub struct Reflective {
  albedo: Color,
  fuzz: f64, // 0.0 to 1.0.
}

impl Reflective {
  pub fn new(albedo: Color, fuzz: f64) -> Reflective {
    Reflective { albedo, fuzz }
  }

  // The reflected ray here is just (v+2b). (Conventionally, the incident vector [incoming ray]
  // would be originating from a light source [it'd be pointing at a light source].)
  pub fn reflect(ray_direction: Vec3, surface_normal: Vec3) -> Vec3 {
    ray_direction - ((ray_direction.dot(surface_normal) * surface_normal) * 2.0)
  }
}

impl Material for Reflective {
  // To simulate blurriness, we randomize the endpoint of the scattered ray instead of reflecting
  // it perfectly. This endpoint will be within a unit sphere; the bigger the sphere, the more
  // fuzziness. A problem however is that if the sphere is big enough, or if the scattered ray is
  // grazing the surface of the object, then the scattered ray may end up underneath the surface!
  fn scatter(&self, ray: &Ray, hit_point: &HitPoint) -> ScatteredRay {
    let normalized_ray_direction = ray.direction().unit_vector();
    let point = hit_point.point();
    let surface_normal = hit_point.surface_normal();

    let scatter_direction = Reflective::reflect(normalized_ray_direction, surface_normal);
    let fuzzy_scatter_direction = scatter_direction + (Vec3::random_in_unit_sphere() * self.fuzz);

    return ScatteredRay::new(point, fuzzy_scatter_direction, self.albedo);
  }
}
