use s7_antialiasing::{
    camera::Camera,
    util::random_f64,
    color::write_color,
    ray::Ray,
    vec3::{dot, unit_vector, Color, Point3, Vec3},
    sphere::Sphere,
    hittable::{HitRecord, Hittable, Shape},
};
use std::fs::File;
use std::io::{self, Write};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 384;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

const COUNT_MAX: usize = IMAGE_HEIGHT as usize * IMAGE_WIDTH as usize;

fn ray_color(r: Ray, world: &Vec<Hittable<Sphere>>) -> Color {
    let mut rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };

    if world.iter().any(|h| h.shape.hit(r, 0.0, f64::INFINITY, &mut rec)) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let mut out_str = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let world = vec![
        Hittable::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Hittable::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let cam = Camera::new();

    let mut data_vector = vec![String::from(""); COUNT_MAX];
    let mut index = 0;

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }
            data_vector[index] = write_color(pixel_color, SAMPLES_PER_PIXEL);
            index += 1;
        }
    }

    out_str += &data_vector.join("\n");

    let mut file = File::create("a.ppm").unwrap();
    file.write_fmt(format_args!("{}", out_str))?;
    Ok(())
}
