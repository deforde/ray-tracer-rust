pub mod util {
    use std::fs::File;
    use std::io::prelude::*;
    use crate::vec;

    pub fn write_colour(f: &mut File, c: &vec::vec::Colour) -> std::io::Result<()> {
        let ir = (255.999 * c.x) as i32;
        let ig = (255.999 * c.y) as i32;
        let ib = (255.999 * c.z) as i32;
        f.write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
        Ok(())
    }
}
