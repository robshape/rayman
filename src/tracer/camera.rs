use crate::tracer::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
  angle_of_view_horizontal: Vec3,
  angle_of_view_vertical: Vec3,
  lens_radius: f64,
  lower_left_corner: Point3,
  origin: Point3,
  u: Vec3,
  v: Vec3,
}

impl Camera {
  pub fn new(
    look_from: Point3,
    look_at: Point3,
    vertical_field_of_view: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_distance: f64,
  ) -> Camera {
    let view_up: Vec3 = Vec3::new(0.0, 1.0, 0.0); // View-up vector. Used to describe the rotation of the camera at the look_from point. In this case, we keep it horizontally leveled.

    // Size of the camera viewport (our window to the world!).
    let theta = vertical_field_of_view.to_radians();
    let half_height = (theta / 2.0).tan();
    let half_width = aspect_ratio * half_height;

    // Orthonormal basis that describes the camera's orientation.
    let w = (look_from - look_at).unit_vector(); // Vector in the direction of the Z axis.
    let u = view_up.cross(w).unit_vector(); // Vector in direction of the X axis.
    let v = w.cross(u); // Vector in direction of the Y axis.

    // Focus distance is the distance between the projection point and the plane where everything
    // is in perfect focus. Not the same as the focal length, which is the distance between the
    // projection point and the image plane (the sensor).
    //
    // In a physical camera, the focus distance is controlled by the distance between the lens and
    // the sensor. While the aperture effectively controls how big the lens is.
    //
    // In a virtual camera, we have a "perfect sensor" so aperture is only needed when when we want
    // defocus blur / depth of field. Here we say that the lens size will be half of the defined
    // aperture. A larger aperture will give us a larger lens!
    let angle_of_view_horizontal = focus_distance * (u * (half_width * 2.0));
    let angle_of_view_vertical = focus_distance * (v * (half_height * 2.0));
    let lower_left_corner = look_from
      - (angle_of_view_horizontal / 2.0)
      - (angle_of_view_vertical / 2.0)
      - (focus_distance * w);

    Camera {
      angle_of_view_horizontal,
      angle_of_view_vertical,
      lens_radius: aperture / 2.0,
      lower_left_corner,
      origin: look_from,
      u,
      v,
    }
  }

  // In real life, light rays originate from a light source. They bounce off objects in the world
  // and then and go into our eyes. With ray tracing, we shoot out rays into the world instead. We
  // use them to scan the world for objects, and then color those objects using hit points.
  pub fn shoot_ray_at(&self, s: f64, t: f64) -> Ray {
    // Normally, all rays originate at the look_from / origin point. To simulate depth of field, we
    // generate random rays from inside a unit disk centered at the look_from / origin point. The
    // intensity of the randomness is controlled by the lens radius. The larger the radius, the
    // greater the defocus blur (and vice versa).
    let random_in_unit_disk = Vec3::random_in_unit_disk() * self.lens_radius;
    let offset = (random_in_unit_disk.x() * self.u) + (random_in_unit_disk.y() * self.v);

    let ray_origin = offset + self.origin;
    let ray_direction = self.lower_left_corner
      + (s * self.angle_of_view_horizontal)
      + (t * self.angle_of_view_vertical)
      - self.origin
      - offset;

    Ray::new(ray_origin, ray_direction)
  }
}
