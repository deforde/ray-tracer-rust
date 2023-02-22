use std::fs::File;
use std::io::prelude::*;

mod camera;
mod dielectric;
mod hittable;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod util;
mod vec;

use dielectric::dielectric::Dielectric;
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
use util::util::randmm_f32;
use util::util::write_colour;
use vec::vec::rand;
use vec::vec::randmm;
use vec::vec::Colour;
use vec::vec::Point;
use vec::vec::Vec;

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

fn random_scene() -> HittableList {
    let mut world = HittableList {
        objects: std::vec::Vec::new(),
    };

    let mat_gnd = Materials::Lambertian(Lambertian {
        albedo: Colour {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    });
    world.objects.push(Hittables::Sphere(Sphere {
        c: Point {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        r: 1000.0,
        mat: mat_gnd,
    }));

    for i in -11..11 {
        for j in -11..11 {
            let choose = rand_f32();
            let centre = Point {
                x: i as f32 + 0.9 * rand_f32(),
                y: 0.2,
                z: j as f32 + 0.9 * rand_f32(),
            };
            if (centre.sub(&[Point {
                x: 4.0,
                y: 0.2,
                z: 0.0,
            }]))
            .len()
                > 0.9
            {
                if choose < 0.8 {
                    let albedo = rand().mul(&[rand()]);
                    let mat = Materials::Lambertian(Lambertian { albedo });
                    world.objects.push(Hittables::Sphere(Sphere {
                        c: centre,
                        r: 0.2,
                        mat,
                    }));
                } else if choose < 0.96 {
                    let albedo = randmm(0.5, 1.0);
                    let fuzz = randmm_f32(0.0, 0.5);
                    let mat = Materials::Metal(Metal { albedo, fuzz });
                    world.objects.push(Hittables::Sphere(Sphere {
                        c: centre,
                        r: 0.2,
                        mat,
                    }));
                } else {
                    let mat = Materials::Dielectric(Dielectric { ir: 1.5 });
                    world.objects.push(Hittables::Sphere(Sphere {
                        c: centre,
                        r: 0.2,
                        mat,
                    }));
                }
            }
        }
    }

    let mut mat = Materials::Dielectric(Dielectric { ir: 1.5 });
    world.objects.push(Hittables::Sphere(Sphere {
        c: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        r: 1.0,
        mat,
    }));

    mat = Materials::Lambertian(Lambertian {
        albedo: Colour {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    });
    world.objects.push(Hittables::Sphere(Sphere {
        c: Point {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        r: 1.0,
        mat,
    }));

    mat = Materials::Metal(Metal {
        albedo: Colour {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });
    world.objects.push(Hittables::Sphere(Sphere {
        c: Point {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        r: 1.0,
        mat,
    }));

    return world;
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let n_samples = 100;
    let max_depth = 50;

    let world = random_scene();

    let lf = Point {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let la = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vup = Vec {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let fd = 10.0;
    let ap = 0.1;
    let cam = camera::camera::init(&lf, &la, &vup, 20.0, aspect_ratio, ap, fd);

    let mut f = File::create("img.ppm")?;
    f.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for j in (0..image_height).rev() {
        print!("\rscan lines remaining: {j}  ");
        std::io::stdout().flush().unwrap();
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
