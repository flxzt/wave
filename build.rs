use std::env;

fn main() -> anyhow::Result<()> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)?.write_to_file("wave.h");

    Ok(())
}
