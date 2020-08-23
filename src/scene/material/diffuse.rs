use crate::scene::material::material::Material;
use crate::tracer::hit_point::HitPoint;
use crate::tracer::ray::{Ray, ScatteredRay};
use crate::vec3::{Color, Vec3};

// An object with a diffuse material (matte) doesn't emit light. It takes on colors from the
// surroundings and modulates them with their own intrinsic color. Light that reflects of a diffuse
// surface has it's direction randomized. So if we send three rays into a crack between two diffuse
// surfaces they will each have their own random behavior. They might also be absorbed rather than
// reflected. The darker the surface, the more likely absorption is. This random behavior is called
// "diffuse reflection" or "light scattering". Absorption is called "light attenuation".
//
// There are two unit radius spheres tangent to the hit point P of a surface. These two spheres
// have a center of: (P+n) and (P-n) where n is the normal of the surface (surface normal). The
// sphere with a center at (P-n) is considered _inside_ the surface. Whereas the sphere with center
// (P+n) is considered _outside_ the surface. Select the tangent unit radius sphere that is on the
// same side of the surface as the ray origin. Pick a random point, S, inside this unit radius
// sphere and send a ray from the hit point P to the random point S. This is the vector (S-P).
//
// Ultimately, multiplying incoming light with how the object reflects it, in given angles, is what
// gives the object its perceived color.
pub struct Diffuse {
  albedo: Color, // "Whiteness". Measure of light reflection for a surface.
}

impl Diffuse {
  pub fn new(albedo: Color) -> Diffuse {
    Diffuse { albedo }
  }
}

// Below are three different ways of rendering reflections on diffuse materials. Try them out in
// different worlds of varying complexity to gain insight on how they affect the lightning of the
// world.
impl Material for Diffuse {
  // // Rejection algorithm / Rejection sampling. Pick a random point in the unit cube where (x,y,z)
  // // all range from -1 to +1. If the point is outside the unit sphere, reject it, and try again.
  // // This corresponds to picking directions on a hemisphere with a high probability of being close
  // // to the surface normal and a lower probability of scattering rays at grazing angles (almost 90
  // // degrees).
  // let point = hit_point.point();
  // let surface_normal = hit_point.surface_normal();
  // let random_bounce_ray_target = point + surface_normal + Vec3::random_in_unit_sphere(); // Approximation of Lambertian reflection.

  // // Drop-in replacement for random_in_unit_sphere(), which is more intuitive than Lambertian
  // // reflection. This achieves uniform scattering direction for all angles away from the hit point,
  // // with no dependence from the normal. Many of the first raytracing papers used this diffuse
  // // method before adopting Lambertian diffuse.
  // let point = hit_point.point();
  // let surface_normal = hit_point.surface_normal();
  // let random_bounce_ray_target = point + Vec3::random_in_hemisphere(surface_normal)

  // Drop-in replacement for random_in_unit_sphere(), which achieves true Lambertian reflection.
  // It's higher probability for ray scattering close to the surface normal, but the distribution
  // is more uniform. This is achieved by picking points on the unit sphere, offset along the
  // surface normal. Picking points on the sphere can be achieved by picking points in the unit
  // sphere and then normalizing those. Because the scattering is more uniform, fewer rays are
  // scattering towards the surface normal. This means that for diffuse objects, they will appear
  // lighter. For its shadows, less rays bounce straight up, so the surface underneath will appear
  // brighter.
  fn scatter(&self, _ray: &Ray, hit_point: &HitPoint) -> ScatteredRay {
    let point = hit_point.point();
    let scatter_direction = hit_point.surface_normal() + Vec3::random_unit_vector(); // True Lambertian reflection.
    return ScatteredRay::new(point, scatter_direction, self.albedo);
  }
}
