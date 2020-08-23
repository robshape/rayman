use crate::vec3::Point3;

pub struct Config {
  pub aspect_ratio: f64,                  // Image / camera aspect ratio.
  pub camera_aperture: f64,               // To simulate depth of field.
  pub camera_focus_distance: f64,         // To simulate depth of field.
  pub camera_look_at: Point3,             // The point that the camera will look at.
  pub camera_look_from: Point3,           // The point that the camera will look from.
  pub camera_vertical_field_of_view: f64, // In degrees.
  pub image_width: u16,                   // In pixels.
  pub samples_per_pixel: u16,             // For anti-aliasing.
}

impl Default for Config {
  fn default() -> Config {
    Config {
      aspect_ratio: 3.0 / 2.0,
      camera_aperture: 0.1,
      camera_focus_distance: 10.0,
      camera_look_at: Point3::new(0.0, 0.0, 0.0),
      camera_look_from: Point3::new(13.0, 2.0, 3.0),
      camera_vertical_field_of_view: 20.0,
      image_width: 1200,
      samples_per_pixel: 500,
    }
  }
}
