mod geometry;
mod parse;
mod pattern;
mod physical;
mod render;

mod prelude {
    pub use crate::geometry::*;
    pub use crate::physical::*;
    pub use crate::render::*;
}

use render::render_from_yaml;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).expect("No file path given");
    let target = std::env::args().nth(2).expect("No target name given");

    render_from_yaml(&path, &target)?;

    println!("All done! Created a new file: {}.ppm", target);

    Ok(())
}
