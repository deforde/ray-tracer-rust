pub mod metal {
    use crate::hittable;
    use crate::material;
    use crate::ray;
    use crate::vec;

    #[derive(Copy, Clone, Debug, Default)]
    pub struct Metal {
        pub albedo: vec::vec::Colour,
        pub fuzz: f32,
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
                dir: refl.add(&[vec::vec::rand_unit_sphere().mulf(self.fuzz)]),
            };
            *att = self.albedo;
            scattered.dir.dot(&rec.n) > 0.0
        }
    }
}
