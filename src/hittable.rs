pub mod hittable {
    use crate::material;
    use crate::ray;
    use crate::sphere;
    use crate::vec;

    #[derive(Copy, Clone, Default)]
    pub struct HitRecord {
        pub p: vec::vec::Vec,
        pub n: vec::vec::Vec,
        pub mat: material::material::Materials,
        pub t: f32,
        pub front_face: bool,
    }

    impl HitRecord {
        pub fn set_face_norm(&mut self, r: &ray::ray::Ray, out_n: &vec::vec::Vec) {
            self.front_face = r.dir.dot(out_n) < 0.0;
            self.n = if self.front_face {
                *out_n
            } else {
                out_n.mulf(-1.0)
            };
        }
    }

    pub trait Hittable {
        fn hit(&self, r: &ray::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    }

    pub enum Hittables {
        Sphere(sphere::sphere::Sphere),
    }

    impl Hittable for Hittables {
        fn hit(&self, r: &ray::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
            match self {
                Hittables::Sphere(sphere) => sphere.hit(r, t_min, t_max, rec),
            }
        }
    }

    #[derive(Default)]
    pub struct HittableList {
        pub objects: std::vec::Vec<Hittables>,
    }

    impl Hittable for HittableList {
        fn hit(&self, r: &ray::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
            let mut temp_rec: HitRecord = Default::default();

            let mut hit = false;
            let mut closest = t_max;

            for obj in &self.objects[..] {
                if obj.hit(r, t_min, closest, &mut temp_rec) {
                    hit = true;
                    closest = temp_rec.t;
                    *rec = temp_rec;
                }
            }

            hit
        }
    }
}
