use crate::vec3::Color;
use crate::util::clamp;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;
    const A: f64 = 256.0;
    format!(
        "{} {} {}",
        (A * clamp(r, 0.0, 0.999)) as u8,
        (A * clamp(g, 0.0, 0.999)) as u8,
        (A * clamp(b, 0.0, 0.999)) as u8,
    )
}
