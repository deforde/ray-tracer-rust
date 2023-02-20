pub mod hittable {
    use crate::ray;
    use crate::vec;

    #[derive(Copy, Clone)]
    pub struct HitRecord {
        pub p: vec::vec::Vec,
        pub n: vec::vec::Vec,
        pub t: f32,
        pub front_face: bool,
    }

    impl HitRecord {
        pub fn set_face_norm(&mut self, r: &ray::ray::Ray, out_n: &vec::vec::Vec) {
            self.front_face = r.dir.dot(out_n) < 0.0;
            self.n = if self.front_face {
                out_n.clone()
            } else {
                out_n.mulf(-1.0)
            };
        }
    }

    pub trait Hittable {
        fn hit(&self, r: &ray::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    }

    pub struct HittableList {
        pub objects: std::vec::Vec<Box<dyn Hittable>>,
    }

    impl Hittable for HittableList {
        fn hit(&self, r: &ray::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
            let mut temp_rec = HitRecord {
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
