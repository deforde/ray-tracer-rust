pub mod camera {
    use crate::ray;
    use crate::vec;

    pub struct Camera {
        pub orig: vec::vec::Point,
        pub llc: vec::vec::Point,
        pub hori: vec::vec::Vec,
        pub vert: vec::vec::Vec,
    }

    pub fn init(
        lf: &vec::vec::Point,
        la: &vec::vec::Point,
        vup: &vec::vec::Vec,
        vfov: f32,
        ar: f32,
    ) -> Camera {
        let theta = std::f32::consts::PI * vfov / 180.0;
        let h = (theta / 2.0).tan();
        let vh = 2.0 * h;
        let vw = ar * vh;

        let w = lf.sub(&[*la]).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let hori = u.mulf(vw);
        let vert = v.mulf(vh);
        let llc = lf.sub(&[hori.divf(2.0), vert.divf(2.0), w]);

        Camera {
            orig: *lf,
            llc,
            hori,
            vert,
        }
    }

    impl Camera {
        pub fn get_ray(&self, u: f32, v: f32) -> ray::ray::Ray {
            ray::ray::Ray {
                orig: self.orig,
                dir: self
                    .llc
                    .add(&[self.hori.mulf(u), self.vert.mulf(v), self.orig.mulf(-1.0)]),
            }
        }
    }
}
