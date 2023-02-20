pub mod dielectric {
    use crate::hittable;
    use crate::material;
    use crate::ray;
    use crate::vec;

    #[derive(Copy, Clone)]
    pub struct Dielectric {
        pub ir: f32,
    }

    impl material::material::Material for Dielectric {
        fn scatter(
            &self,
            r: &ray::ray::Ray,
            rec: &hittable::hittable::HitRecord,
            att: &mut vec::vec::Colour,
            scattered: &mut ray::ray::Ray,
        ) -> bool {
            *att = vec::vec::Colour {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let rr = if rec.front_face {
                1.0 / self.ir
            } else {
                self.ir
            };
            let u = r.dir.unit();
            let c = u.mulf(-1.0).dot(&rec.n).min(1.0);
            let s = (1.0 - c * c).sqrt();
            let dir = if rr * s > 1.0 {
                u.reflect(&rec.n)
            } else {
                u.refract(&rec.n, rr)
            };
            *scattered = ray::ray::Ray { orig: rec.p, dir };
            true
        }
    }
}
