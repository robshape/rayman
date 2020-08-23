use crate::tracer::ray::Ray;
use crate::vec3::Color;

pub fn background_color(ray: &Ray) -> Color {
  let blue_color = Color::new(0.5, 0.7, 1.0); // Sky'ish blue.
  let white_color = Color::new(1.0, 1.0, 1.0);

  let ray_direction = ray.direction();
  let normalized_ray_direction = ray_direction.unit_vector();
  let t = (normalized_ray_direction.y() + 1.0) * 0.5;

  // When t is 1.0 return blue. When 0.0 return white. Blend in-between. This is called
  // linear blend / linear interpolation / "lerp".
  (white_color * (1.0 - t)) + (blue_color * t)
}
