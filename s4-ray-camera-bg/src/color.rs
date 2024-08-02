use crate::vec3::Color;

pub fn write_color(pixel_color: Color) -> String {
    const A: f64 = 255.999;
    format!(
        "{} {} {}",
        (A * pixel_color.x()) as u8,
        (A * pixel_color.y()) as u8,
        (A * pixel_color.z()) as u8,
    )
}
