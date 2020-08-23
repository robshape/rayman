mod config;
mod image;
mod scene;
mod tracer;
mod vec3;

use crate::config::Config;
use crate::image::render::render_image;
use crate::scene::generator::random_spheres;
use crate::scene::object::world::World;

fn main() {
  // Configure ray tracer.
  let config = Config::default();
  // let config = Config {
  //   image_width: 300,
  //   samples_per_pixel: 100,
  //   ..Config::default()
  // }; // For faster rendering.

  // Setup world.
  let spheres = random_spheres();
  let world = World::new(spheres);

  // Trace rays!
  render_image(config, world);
}
