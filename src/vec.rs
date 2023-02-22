pub mod vec {
    use crate::util;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Vec {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl Vec {
        pub fn add(&self, others: &[Vec]) -> Vec {
            let mut v = *self;
            for other in others {
                v.x += other.x;
                v.y += other.y;
                v.z += other.z;
            }
            v
        }

        pub fn sub(&self, others: &[Vec]) -> Vec {
            let mut v = *self;
            for other in others {
                v.x -= other.x;
                v.y -= other.y;
                v.z -= other.z;
            }
            v
        }

        pub fn mul(&self, others: &[Vec]) -> Vec {
            let mut v = *self;
            for other in others {
                v.x *= other.x;
                v.y *= other.y;
                v.z *= other.z;
            }
            v
        }

        pub fn div(&self, others: &[Vec]) -> Vec {
            let mut v = *self;
            for other in others {
                v.x /= other.x;
                v.y /= other.y;
                v.z /= other.z;
            }
            v
        }

        pub fn mulf(&self, val: f32) -> Vec {
            Vec {
                x: self.x * val,
                y: self.y * val,
                z: self.z * val,
            }
        }

        pub fn divf(&self, val: f32) -> Vec {
            Vec {
                x: self.x / val,
                y: self.y / val,
                z: self.z / val,
            }
        }

        pub fn dot(&self, other: &Vec) -> f32 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }

        pub fn cross(&self, other: &Vec) -> Vec {
            Vec {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }
        }

        pub fn unit(&self) -> Vec {
            self.divf(self.len())
        }

        pub fn len_sqrd(&self) -> f32 {
            self.x * self.x + self.y * self.y + self.z * self.z
        }

        pub fn len(&self) -> f32 {
            self.len_sqrd().sqrt()
        }

        pub fn near_zero(&self) -> bool {
            let s = 1e-8;
            self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
        }

        pub fn reflect(&self, v: &Vec) -> Vec {
            self.sub(&[v.mulf(2.0 * self.dot(v))])
        }

        pub fn refract(&self, n: &Vec, e: f32) -> Vec {
            let c = self.mulf(-1.0).dot(n).min(1.0);
            let u = self.add(&[n.mulf(c)]).mulf(e);
            let v = n.mulf(-1.0 * (1.0 - u.len_sqrd()).abs().sqrt());
            u.add(&[v])
        }
    }

    pub type Point = Vec;
    pub type Colour = Vec;

    pub fn rand() -> Vec {
        Vec {
            x: util::util::rand_f32(),
            y: util::util::rand_f32(),
            z: util::util::rand_f32(),
        }
    }

    pub fn randmm(min: f32, max: f32) -> Vec {
        Vec {
            x: util::util::randmm_f32(min, max),
            y: util::util::randmm_f32(min, max),
            z: util::util::randmm_f32(min, max),
        }
    }

    pub fn rand_unit_sphere() -> Vec {
        loop {
            let p = randmm(-1.0, 1.0);
            if p.len_sqrd() < 1.0 {
                return p;
            }
        }
    }

    pub fn rand_unit() -> Vec {
        rand_unit_sphere().unit()
    }

    pub fn rand_hemisphere(n: &Vec) -> Vec {
        let v = rand_unit_sphere();
        if v.dot(n) > 0.0 {
            v
        } else {
            v.mulf(-1.0)
        }
    }

    pub fn rand_unit_disk() -> Vec {
        loop {
            let p = Vec {
                x: util::util::randmm_f32(-1.0, 1.0),
                y: util::util::randmm_f32(-1.0, 1.0),
                z: 0.0,
            };
            if p.len_sqrd() < 1.0 {
                return p;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_arithmetic() {
        let v = vec::Vec {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let u = vec::Vec {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let r = vec::Vec {
            x: 7.0,
            y: 8.0,
            z: 9.0,
        };

        let mut t = v.add(&[u, r]);
        assert_eq!(
            t,
            vec::Vec {
                x: 12.0,
                y: 15.0,
                z: 18.0,
            }
        );

        t = v.sub(&[u, r]);
        assert_eq!(
            t,
            vec::Vec {
                x: -10.0,
                y: -11.0,
                z: -12.0,
            }
        );

        t = v.mul(&[u, r]);
        assert_eq!(
            t,
            vec::Vec {
                x: 28.0,
                y: 80.0,
                z: 162.0,
            }
        );

        t = v.div(&[u, r]);
        assert_eq!(
            t,
            vec::Vec {
                x: 0.035714286,
                y: 0.05,
                z: 0.055555556,
            }
        );

        assert_eq!(v.len_sqrd(), 14.0);
        assert_eq!(v.len(), 3.741657387);

        assert_eq!(
            v.mulf(2.0),
            vec::Vec {
                x: 2.0,
                y: 4.0,
                z: 6.0,
            }
        );

        assert_eq!(
            v.divf(2.0),
            vec::Vec {
                x: 0.5,
                y: 1.0,
                z: 1.5,
            }
        );

        assert_eq!(v.dot(&u), 32.0);
        assert_eq!(
            v.unit(),
            vec::Vec {
                x: 0.26726124,
                y: 0.5345225,
                z: 0.8017837,
            }
        );
        assert_eq!(
            v.cross(&u),
            vec::Vec {
                x: -3.0,
                y: 6.0,
                z: -3.0,
            }
        );
    }
}
