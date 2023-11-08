use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct BuildError(String);

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for BuildError {}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Add code to clone/building the C library!

    // let out_dir = env::var("OUT_DIR")
    //     .map_err(|_| BuildError("Could not determine OUT_DIR environment variable.".to_string()))?;
    let target_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| BuildError("Could not determine target environment variable.".to_string()))?;
    // let compiled_lib_path = Path::new(&out_dir).join("lib/nalib.lib");
    // let destination = Path::new(&target_dir).join("lub/nalib.lib");

    // println!(
    //     "Copying library from {:?} to {:?}",
    //     target_dir, compiled_lib_path
    // );

    // Copy the compiled .lib file to lib directory
    // fs::copy(compiled_lib_path, destination).expect("Failed to copy compiled library");

    // Tell cargo to look for native libraries in the `lib` directory
    let lib_path = Path::new(&target_dir).join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=NALib");

    Ok(())
}
