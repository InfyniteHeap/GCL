use std::error::Error;

use crate::{auth::*, java, path};

// First check if launch parameters file are exsited.
// If so, directly read it or re-create the file.
// Second check if Java path is valid.
// If so, pass parameters to JVM; if invalid, prompt player to set a new path or let launcher search by itself.
// The launcher will choose the proper Java version that fits to the Minecraft version you chose.
pub fn launch_mc() -> Result<(), Box<dyn Error>> {
    Ok(())
}
