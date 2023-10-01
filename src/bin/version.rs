use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let path = &PathBuf::from(
        std::env::var("TRUNK_SOURCE_DIR").expect("Environment variable TRUNK_SOURCE_DIR"),
    )
    .join("target")
    .join("generated");
    if !path.is_dir() {
        std::fs::create_dir(path)?;
    }
    std::fs::write(path.join("version.html"), env!("CARGO_PKG_VERSION"))
}
