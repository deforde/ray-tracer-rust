use std::fs::File;
use std::io::prelude::*;

mod vec;

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut file = File::create("img.ppm")?;
    file.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for j in (0..image_height).rev() {
        print!("\rscan lines remaining: {j}  ");
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            file.write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
        }
    }
    println!("\ndone");

    Ok(())
}
