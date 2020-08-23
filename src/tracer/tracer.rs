use crate::scene::background::background_color;
use crate::scene::object::object::Object;
use crate::tracer::ray::Ray;
use crate::vec3::Color;

pub fn trace_ray_in_world<T: Object>(ray: &Ray, world: &T, maximum_ray_bounce_depth: u8) -> Color {
  // No light at the maximum bounce depth. Could be a ray bouncing around inside of a crack of the
  // object.
  if maximum_ray_bounce_depth <= 0 {
    return Color::zero();
  }

  // Shadow acne is a visual artefact that appears in the shape of small black dots on the surface
  // of objects. This is because some of the reflected rays aren't reflecting of at t=0 but at an
  // approximation, such as -0.0000001 or 0.0000001. Therefore we need to ignore hits very near to
  // zero.
  const T_MIN: f64 = 0.001;
  const T_MAX: f64 = f64::INFINITY;
  match world.is_hit_by_ray(&ray, T_MIN, T_MAX) {
    // RUST: Match multiple patterns: None, Some.
    None => {}

    Some(hit_point) => {
      let ray_bounce_depth = maximum_ray_bounce_depth - 1;

      let scatter = hit_point.material().scatter(&ray, &hit_point);
      let scattered_ray = scatter.ray();
      let light_attenuation = scatter.light_attenuation();

      return trace_ray_in_world(&scattered_ray, world, ray_bounce_depth) * light_attenuation;
    }
  }

  background_color(&ray)
}
