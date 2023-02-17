use std::fs::File;
use std::io::prelude::*;

mod util;
mod vec;
mod ray;

use ray::ray::Ray;
use util::util::write_colour;
use vec::vec::Colour;
use vec::vec::Point;
use vec::vec::Vec;

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut f = File::create("img.ppm")?;
    f.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for j in (0..image_height).rev() {
        print!("\rscan lines remaining: {j}  ");
        for i in 0..image_width {
            let c = Colour {
                x: i as f32 / (image_width - 1) as f32,
                y: j as f32 / (image_height - 1) as f32,
                z: 0.25,
            };
            write_colour(&mut f, &c)?;
        }
    }
    println!("\ndone");

    Ok(())
}
