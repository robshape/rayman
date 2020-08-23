use rand::{thread_rng, Rng};

use crate::config::Config;
use crate::image::ppm::{print_ppm_color, print_ppm_header, print_progress};
use crate::scene::object::object::Object;
use crate::scene::object::world::World;
use crate::tracer::camera::Camera;
use crate::tracer::tracer::trace_ray_in_world;
use crate::vec3::Color;

pub fn render_image<T: Object>(config: Config, world: World<T>) {
  // Setup camera.
  let camera = Camera::new(
    config.camera_look_from,
    config.camera_look_at,
    config.camera_vertical_field_of_view,
    config.aspect_ratio,
    config.camera_aperture,
    config.camera_focus_distance,
  );

  // Setup renderer.
  const RAY_BOUNCE_LIMIT: u8 = 50; // Limit the number of times a ray can bounce off objects.
  let mut rng = thread_rng();
  let image_height = (config.image_width as f64 / config.aspect_ratio) as u16;

  // Pixels are written out in rows, left to right. Rows are written out top to bottom.
  print_ppm_header(config.image_width, image_height);
  for h in (0..image_height).rev() {
    print_progress(h);
    for w in 0..config.image_width {
      // Multisample anti-aliasing (MSAA). Traverse multiple random samples, using rays, inside a
      // pixel. Color the samples and average them out for a more representative shaded color.
      let mut color = Color::zero();
      for _ in 0..config.samples_per_pixel {
        // Use two offset vectors (u,v) to shoot the rays randomly at the pixel.
        let u = (w as f64 + rng.gen_range(0.0, 1.0)) / (config.image_width - 1) as f64;
        let v = (h as f64 + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f64;

        let ray = camera.shoot_ray_at(u, v);

        color = color + trace_ray_in_world(&ray, &world, RAY_BOUNCE_LIMIT);
      }

      print_ppm_color(color, config.samples_per_pixel);
    }
  }

  eprintln!("Voila!");
}
