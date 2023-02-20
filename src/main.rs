use std::fs::File;
use std::io::prelude::*;

mod camera;
mod hittable;
mod ray;
mod sphere;
mod util;
mod vec;

use camera::camera::Camera;
use hittable::hittable::HitRecord;
use hittable::hittable::Hittable;
use hittable::hittable::HittableList;
use ray::ray::Ray;
use sphere::sphere::Sphere;
use util::util::rand_f32;
use util::util::write_colour;
use vec::vec::rand_unit_sphere;
use vec::vec::Colour;
use vec::vec::Point;
use vec::vec::Vec;

fn hit_sphere(centre: &Point, radius: f32, r: &Ray) -> f32 {
    let oc = r.orig.sub(&[*centre]);
    let a = r.dir.len_sqrd();
    let b = oc.dot(&r.dir);
    let c = oc.len_sqrd() - radius * radius;
    let discriminant = b * b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-b - discriminant.sqrt()) / a
}

fn ray_colour(r: &Ray, world: &HittableList, depth: i32) -> Colour {
    if depth <= 0 {
        return Colour {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    let mut rec = HitRecord {
        p: vec::vec::Vec {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        n: vec::vec::Vec {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        front_face: false,
    };

    if world.hit(r, 0.0, std::f32::MAX, &mut rec) {
        let target = rec.p.add(&[rec.n, rand_unit_sphere()]);
        return ray_colour(
            &Ray {
                orig: rec.p,
                dir: target.sub(&[rec.p]),
            },
            world,
            depth - 1,
        )
        .mulf(0.5);
    }

    let unit = r.dir.unit();
    let t = 0.5 * (unit.y + 1.0);
    Colour {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }
    .mulf(1.0 - t)
    .add(&[Colour {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    }
    .mulf(t)])
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let n_samples = 100;
    let max_depth = 50;

    let mut world = HittableList {
        objects: std::vec::Vec::new(),
    };
    world.objects.push(Box::new(Sphere {
        c: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        r: 0.5,
    }));
    world.objects.push(Box::new(Sphere {
        c: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        r: 100.0,
    }));

    let cam = camera::camera::init();

    let mut f = File::create("img.ppm")?;
    f.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for j in (0..image_height).rev() {
        print!("\rscan lines remaining: {j}  ");
        for i in 0..image_width {
            let mut c = Colour {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _ in 0..n_samples {
                let u = (i as f32 + rand_f32()) / (image_width - 1) as f32;
                let v = (j as f32 + rand_f32()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                c = c.add(&[ray_colour(&r, &world, max_depth)]);
            }
            write_colour(&mut f, &c, n_samples)?;
        }
    }
    println!("\ndone");

    Ok(())
}
