use rand::rngs::ThreadRng;
use rand::{random, thread_rng, Rng};

use crate::scene::material::dielectric::Dielectric;
use crate::scene::material::diffuse::Diffuse;
use crate::scene::material::material::Material;
use crate::scene::material::reflective::Reflective;
use crate::scene::object::sphere::Sphere;
use crate::vec3::{Color, Point3};

fn random_sphere(center: Point3, radius: f64, rng: &mut ThreadRng) -> Sphere {
  let random_material = random::<f64>();

  let material: Box<dyn Material> = match random_material {
    // Matte.
    random_material if random_material < 0.8 => {
      Box::new(Diffuse::new(Color::random() * Color::random()))
    }
    // Metal.
    random_material if random_material < 0.95 => Box::new(Reflective::new(
      Color::random_within_range(0.5, 1.0),
      rng.gen_range(0.0..0.5),
    )),
    // Glass.
    _ => Box::new(Dielectric::new(1.5)),
  };

  Sphere::new(center, radius, material)
}

pub fn random_spheres() -> Vec<Sphere> {
  const LARGE_SPHERE_RADIUS: f64 = 1.0;
  const SMALL_SPHERE_RADIUS: f64 = 0.2;

  let mut spheres = vec![
    // Ground.
    Sphere::new(
      Point3::new(0.0, -1000.0, 0.0),
      1000.0,
      Box::new(Diffuse::new(Color::new(0.5, 0.5, 0.5))), // Matte material with a gray color.
    ),
    // First large sphere.
    Sphere::new(
      Point3::new(-4.0, 1.0, 0.0),
      LARGE_SPHERE_RADIUS,
      Box::new(Diffuse::new(Color::new(0.4, 0.2, 0.1))), // Matte material with a brown color
    ),
    // Second large sphere.
    Sphere::new(
      Point3::new(0.0, 1.0, 0.0),
      LARGE_SPHERE_RADIUS,
      Box::new(Dielectric::new(1.5)), // Glass material.
    ),
    // Third large sphere.
    Sphere::new(
      Point3::new(4.0, 1.0, 0.0),
      LARGE_SPHERE_RADIUS,
      Box::new(Reflective::new(Color::new(0.7, 0.6, 0.5), 0.0)), // Metal material with a gray color and no fuzz.
    ),
  ];

  // Many random small spheres.
  let large_sphere_center = Point3::new(4.0, LARGE_SPHERE_RADIUS, 0.0);
  let mut rng = thread_rng();
  for i in -11..11 {
    for j in -11..11 {
      let x = i as f64 + (random::<f64>() * (LARGE_SPHERE_RADIUS + SMALL_SPHERE_RADIUS));
      let y = SMALL_SPHERE_RADIUS;
      let z = j as f64 + (random::<f64>() * (LARGE_SPHERE_RADIUS + SMALL_SPHERE_RADIUS));
      let small_sphere_center = Point3::new(x, y, z);

      // Only create small spheres away from the large spheres.
      if (small_sphere_center - large_sphere_center).length()
        > (LARGE_SPHERE_RADIUS + SMALL_SPHERE_RADIUS)
      {
        let random_sphere = random_sphere(small_sphere_center, SMALL_SPHERE_RADIUS, &mut rng);
        spheres.push(random_sphere);
      }
    }
  }

  spheres
}
