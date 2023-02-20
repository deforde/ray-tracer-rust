pub mod camera {
    use crate::ray;
    use crate::vec;

    pub struct Camera {
        pub orig: vec::vec::Point,
        pub llc: vec::vec::Point,
        pub hori: vec::vec::Vec,
        pub vert: vec::vec::Vec,
    }

    pub fn init() -> Camera {
        let ar = 16.0 / 9.0;
        let vh = 2.0;
        let vw = ar * vh;
        let fl = 1.0;

        let orig = vec::vec::Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let hori = vec::vec::Point {
            x: vw,
            y: 0.0,
            z: 0.0,
        };
        let vert = vec::vec::Point {
            x: 0.0,
            y: vh,
            z: 0.0,
        };
        let llc = orig.sub(&[
            hori.divf(2.0),
            vert.divf(2.0),
            vec::vec::Vec {
                x: 0.0,
                y: 0.0,
                z: fl,
            },
        ]);

        Camera {
            orig,
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
