pub mod ray {
    use crate::vec;

    pub struct Ray {
        pub orig: vec::vec::Point,
        pub dir: vec::vec::Vec,
    }

    impl Ray {
        pub fn at(&self, t: f32) -> vec::vec::Point {
            self.orig.add(&[self.dir.mulf(t)])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec;
    use super::*;
    #[test]
    fn ray_at() {
        let r = ray::Ray {
            orig: vec::vec::Vec {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            dir: vec::vec::Vec {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        };
        assert_eq!(r.at(2.0), vec::vec::Vec {
            x: 9.0,
            y: 12.0,
            z: 15.0,
        });
    }
}
