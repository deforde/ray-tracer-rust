pub mod camera {
    use crate::ray;
    use crate::vec;

    #[derive(Default)]
    pub struct Camera {
        pub orig: vec::vec::Point,
        pub llc: vec::vec::Point,
        pub hori: vec::vec::Vec,
        pub vert: vec::vec::Vec,
        pub w: vec::vec::Vec,
        pub u: vec::vec::Vec,
        pub v: vec::vec::Vec,
        pub lr: f32,
    }

    pub fn init(
        lf: &vec::vec::Point,
        la: &vec::vec::Point,
        vup: &vec::vec::Vec,
        vfov: f32,
        ar: f32,
        ap: f32,
        fd: f32,
    ) -> Camera {
        let theta = std::f32::consts::PI * vfov / 180.0;
        let h = (theta / 2.0).tan();
        let vh = 2.0 * h;
        let vw = ar * vh;

        let w = lf.sub(&[*la]).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let hori = u.mulf(vw * fd);
        let vert = v.mulf(vh * fd);
        let llc = lf.sub(&[hori.divf(2.0), vert.divf(2.0), w.mulf(fd)]);
        let lr = ap / 2.0;

        Camera {
            orig: *lf,
            llc,
            hori,
            vert,
            w,
            u,
            v,
            lr,
        }
    }

    impl Camera {
        pub fn get_ray(&self, s: f32, t: f32) -> ray::ray::Ray {
            let rd = vec::vec::rand_unit_disk().mulf(self.lr);
            let offset = self.u.mulf(rd.x).add(&[self.v.mulf(rd.y)]);
            ray::ray::Ray {
                orig: self.orig.add(&[offset]),
                dir: self.llc.add(&[
                    self.hori.mulf(s),
                    self.vert.mulf(t),
                    self.orig.mulf(-1.0),
                    offset.mulf(-1.0),
                ]),
            }
        }
    }
}
