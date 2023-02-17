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

fn ray_colour(r: &Ray) -> Colour {
    let unit = r.dir.unit();
    let t = 0.5 * (unit.y + 1.0);
    Colour{x: 1.0, y: 1.0, z: 1.0}.mulf(1.0 - t).add(&[Colour{x: 0.5, y: 0.7, z: 1.0}.mulf(t)])
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin.sub(&[horizontal.divf(2.0), vertical.divf(2.0), Vec{x:0.0, y:0.0, z:focal_length}]);

    let mut f = File::create("img.ppm")?;
    f.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for j in (0..image_height).rev() {
        print!("\rscan lines remaining: {j}  ");
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray {
                orig: origin,
                dir: lower_left_corner.add(&[horizontal.mulf(u), vertical.mulf(v), origin.mulf(-1.0)]),
            };
            let c = ray_colour(&r);
            write_colour(&mut f, &c)?;
        }
    }
    println!("\ndone");

    Ok(())
}
