pub mod metal {
    use crate::hittable;
    use crate::material;
    use crate::ray;
    use crate::vec;

    #[derive(Copy, Clone)]
    pub struct Metal {
        pub albedo: vec::vec::Colour,
    }

    impl material::material::Material for Metal {
        fn scatter(
            &self,
            r: &ray::ray::Ray,
            rec: &hittable::hittable::HitRecord,
            att: &mut vec::vec::Colour,
            scattered: &mut ray::ray::Ray,
        ) -> bool {
            let refl = r.dir.unit().reflect(&rec.n);
            *scattered = ray::ray::Ray {
                orig: rec.p,
                dir: refl,
            };
            *att = self.albedo;
            scattered.dir.dot(&rec.n) > 0.0
        }
    }
}
