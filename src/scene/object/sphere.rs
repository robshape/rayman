use crate::scene::material::material::Material;
use crate::scene::object::object::Object;
use crate::tracer::hit_point::HitPoint;
use crate::tracer::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
  center: Point3,              // Sphere center position.
  material: Box<dyn Material>, // RUST: Box<> allocates the value on the heap. For owned trait objects.
  radius: f64,
}

impl Sphere {
  pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
    Sphere {
      center,
      material,
      radius,
    }
  }
}

impl Object for Sphere {
  // The equation for a sphere in vector form is: (P-C)*(P-C)=r2. Any point P that satisfies this
  // equation is on the sphere.
  //
  // Ray: P(t)=A+tB
  // Sphere: (P-C)*(P-C)=r2
  //
  // If a ray hits the sphere, there is some t for which P(t) satisfies the sphere equation. So we
  // are looking for any t where this is true: (P(t)-C)*(P(t)-C)=r2. Or in expanded form:
  // (A+tB-C)*(A+tB-C)=r2
  //
  // Having t_min and t_max helps in other parts of the code, to decide when to do certain
  // calculations or help to avoid visual artifacts due to computational limitations (shadow acne,
  // etc.).
  fn is_hit_by_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitPoint> {
    let ray_direction = ray.direction();

    let oc = ray.origin() - self.center;
    let a = ray_direction.length_squared();
    let half_b = oc.dot(ray_direction);
    let c = oc.length_squared() - (self.radius * self.radius);
    let discriminant = (half_b * half_b) - (a * c); // It's two, one, or no solutions!

    // The ray touches the sphere in one point (0, tangent) or two points (>0, intersected). Return
    // the smaller, positive, t (closest). This should be the first intersection when a ray, facing
    // the sphere, hits it.
    if discriminant > 0.0 {
      let discriminant_square_root = discriminant.sqrt();

      let mut t = (-half_b - discriminant_square_root) / a; // Distance (along the ray), t0, to intersection.
      if t > t_min && t < t_max {
        // Create a "surface normal" so that we can shade the sphere and get a sense of 3D. This is
        // an outward surface normal because the ray hits the outside of an _opaque_ sphere.
        let point = ray.point_at(t);
        let outward_surface_normal = (point - self.center) / self.radius;

        let hit_point = HitPoint::new(point, t, outward_surface_normal, &ray, &*self.material);
        return Some(hit_point);
      }

      t = (-half_b + discriminant_square_root) / a;
      if t > t_min && t < t_max {
        let point = ray.point_at(t);
        let outward_surface_normal = (point - self.center) / self.radius;

        let hit_point = HitPoint::new(point, t, outward_surface_normal, &ray, &*self.material);
        return Some(hit_point);
      }
    }

    None // (discriminant < 0.0) means that the ray doesn't touch the sphere.
  }
}
