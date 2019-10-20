// build.rs
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use vergen::*;

/// Compiles and runs prior to the main project.
/// Useful for codegen and custom build features.
fn main() {
  // trigger codegen of git commit
  vergen(SHORT_SHA).unwrap();

  // generate file in output directory
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("codegen.rs");
  let mut f = File::create(&dest_path).unwrap();

  f.write_fmt(format_args!("

    pub fn version() -> &'static str {{
      // return project version compiled in from Toml file.
      \"{}\"
    }}

    pub fn description() -> &'static str {{
      // return project description compiled in from Toml file.
      \"{}\"
    }}

    ", VERSION, DESCRIPTION)).unwrap();
}
