pub mod lambertian {
    use crate::hittable;
    use crate::material;
    use crate::ray;
    use crate::vec;

    #[derive(Copy, Clone, Default)]
    pub struct Lambertian {
        pub albedo: vec::vec::Colour,
    }

    impl material::material::Material for Lambertian {
        fn scatter(
            &self,
            _r: &ray::ray::Ray,
            rec: &hittable::hittable::HitRecord,
            att: &mut vec::vec::Colour,
            scattered: &mut ray::ray::Ray,
        ) -> bool {
            let mut dir = rec.n.add(&[vec::vec::rand_unit()]);
            if dir.near_zero() {
                dir = rec.n;
            }
            *scattered = ray::ray::Ray { orig: rec.p, dir };
            *att = self.albedo;
            true
        }
    }
}
