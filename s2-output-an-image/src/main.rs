use std::fs::File;
use std::io::{self, stdout, Write};

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;
const COUNT_MAX: usize = IMAGE_HEIGHT as usize * IMAGE_WIDTH as usize;

fn main() -> io::Result<()> {
    let mut out_str = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut data_vector = vec![String::from(""); COUNT_MAX];
    let mut index = 0;

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Progress: {} / {}    \r", j, IMAGE_HEIGHT);
        stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            const A: f64 = 255.999;
            let ir = (A * r) as u8;
            let ig = (A * g) as u8;
            let ib = (A * b) as u8;

            data_vector[index] = format!("{} {} {}", ir, ig, ib);
            index += 1;
        }
    }

    println!("Writing to file...");

    out_str += &data_vector.join("\n");

    let mut file = File::create("2-output-an-image.ppm").unwrap();
    file.write_fmt(format_args!("{}", out_str))?;
    println!("Done!");
    Ok(())
}
