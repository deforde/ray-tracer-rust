pub mod util {
    use crate::vec;
    use rand::Rng;
    use std::fs::File;
    use std::io::prelude::*;

    pub fn rand_f32() -> f32 {
        rand::thread_rng().gen::<f32>()
    }

    pub fn randmm_f32(min: f32, max: f32) -> f32 {
        min + (max - min) * rand_f32()
    }

    pub fn write_colour(f: &mut File, c: &vec::vec::Colour, n_samples: i32) -> std::io::Result<()> {
        let scale = 1.0 / n_samples as f32;

        let r = (c.x * scale).sqrt();
        let g = (c.y * scale).sqrt();
        let b = (c.z * scale).sqrt();

        let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
        let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
        let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;
        f.write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
        Ok(())
    }
}
