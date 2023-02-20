pub mod sphere {
    use crate::hittable;
    use crate::ray;
    use crate::vec;

    pub struct Sphere {
        pub c: vec::vec::Point,
        pub r: f32,
    }

    impl hittable::hittable::Hittable for Sphere {
        fn hit(
            &self,
            r: &ray::ray::Ray,
            t_min: f32,
            t_max: f32,
            rec: &mut hittable::hittable::HitRecord,
        ) -> bool {
            let oc = r.orig.sub(&[self.c]);
            let a = r.dir.len_sqrd();
            let b = oc.dot(&r.dir);
            let c = oc.len_sqrd() - self.r * self.r;

            let discriminant = b * b - a * c;
            if discriminant < 0.0 {
                return false;
            }
            let sqrtd = discriminant.sqrt();

            let mut root = (-b - sqrtd) / a;
            if root < t_min || root > t_max {
                root = (-b + sqrtd) / a;
                if root < t_min || root > t_max {
                    return false;
                }
            }

            rec.t = root;
            rec.p = r.at(rec.t);
            let out_n = rec.p.sub(&[self.c]).divf(self.r);
            rec.set_face_norm(r, &out_n);

            true
        }
    }
}
