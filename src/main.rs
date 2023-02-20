use std::fs::File;
use std::io::prelude::*;

mod camera;
mod hittable;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod util;
mod vec;

use camera::camera::Camera;
use hittable::hittable::HitRecord;
use hittable::hittable::Hittable;
use hittable::hittable::HittableList;
use hittable::hittable::Hittables;
use lambertian::lambertian::Lambertian;
use material::material::Material;
use material::material::Materials;
use metal::metal::Metal;
use ray::ray::Ray;
use sphere::sphere::Sphere;
use util::util::rand_f32;
use util::util::write_colour;
use vec::vec::rand_hemisphere;
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
        p: Vec {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        n: Vec {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        mat: Materials::MaterialNone,
        t: 0.0,
        front_face: false,
    };

    if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
        let mut scattered = Ray {
            orig: Vec {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            dir: Vec {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let mut att = Colour {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        if rec.mat.scatter(r, &rec, &mut att, &mut scattered) {
            return att.mul(&[ray_colour(&scattered, world, depth - 1)]);
        }
        return Colour {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
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

    let mat_gnd = Materials::Lambertian(Lambertian {
        albedo: Colour {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });
    let mat_centre = Materials::Lambertian(Lambertian {
        albedo: Colour {
            x: 0.7,
            y: 0.3,
            z: 0.3,
        },
    });
    let mat_left = Materials::Metal(Metal {
        albedo: Colour {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
    });
    let mat_right = Materials::Metal(Metal {
        albedo: Colour {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
    });

    let mut world = HittableList {
        objects: std::vec::Vec::new(),
    };
    world.objects.push(Hittables::Sphere(Sphere {
        c: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        r: 100.0,
        mat: mat_gnd,
    }));
    world.objects.push(Hittables::Sphere(Sphere {
        c: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        r: 0.5,
        mat: mat_centre,
    }));
    world.objects.push(Hittables::Sphere(Sphere {
        c: Point {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        r: 0.5,
        mat: mat_left,
    }));
    world.objects.push(Hittables::Sphere(Sphere {
        c: Point {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        r: 0.5,
        mat: mat_right,
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
