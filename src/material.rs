pub mod material {
    use crate::dielectric;
    use crate::hittable;
    use crate::lambertian;
    use crate::metal;
    use crate::ray;
    use crate::vec;

    pub trait Material {
        fn scatter(
            &self,
            r: &ray::ray::Ray,
            rec: &hittable::hittable::HitRecord,
            att: &mut vec::vec::Colour,
            scattered: &mut ray::ray::Ray,
        ) -> bool;
    }

    #[derive(Copy, Clone)]
    pub enum Materials {
        MaterialNone,
        Dielectric(dielectric::dielectric::Dielectric),
        Lambertian(lambertian::lambertian::Lambertian),
        Metal(metal::metal::Metal),
    }

    impl Material for Materials {
        fn scatter(
            &self,
            r: &ray::ray::Ray,
            rec: &hittable::hittable::HitRecord,
            att: &mut vec::vec::Colour,
            scattered: &mut ray::ray::Ray,
        ) -> bool {
            match self {
                Materials::MaterialNone => false,
                Materials::Dielectric(mat) => mat.scatter(r, rec, att, scattered),
                Materials::Lambertian(mat) => mat.scatter(r, rec, att, scattered),
                Materials::Metal(mat) => mat.scatter(r, rec, att, scattered),
            }
        }
    }
}
