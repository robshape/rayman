use crate::vec3::Color;

fn clamp(val: f64, min: f64, max: f64) -> f64 {
  if val < min {
    min
  } else if val > max {
    max
  } else {
    val
  }
}

// Before printing the color, average it out based on the number of samples per pixel and then
// gamma correct for "gamma 2.0". Almost all image viewers assume that the image is gamma corrected
// so we need to correct it ourselves. Here we will approximate it to gamma 2.0, which means
// raising the color to the power of 1/gamma. Or in this case 1.0/2.0, which is just square root.
pub fn print_ppm_color(color: Color, samples_per_pixel: u16) {
  let scale = 1.0 / samples_per_pixel as f64;

  let r = (color.x() * scale).sqrt();
  let red = (clamp(r, 0.0, 0.999) * 256.0) as u16;

  let g = (color.y() * scale).sqrt();
  let green = (clamp(g, 0.0, 0.999) * 256.0) as u16;

  let b = (color.z() * scale).sqrt();
  let blue = (clamp(b, 0.0, 0.999) * 256.0) as u16;

  println!("{} {} {}", red, green, blue);
}

pub fn print_ppm_header(image_width: u16, image_height: u16) {
  const RGB_MAXIMUM_VALUE: u8 = 255;
  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("{}", RGB_MAXIMUM_VALUE);
}

pub fn print_progress(progress: u16) {
  eprintln!("\x1B[2J"); // Clear Terminal.
  eprintln!("Scanlines remaining: {}", progress);
}
