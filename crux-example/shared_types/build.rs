use crux_core::typegen::TypeGen;
use shared::App;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../shared");

    let mut generator = TypeGen::new();

    generator.register_app::<App>()?;

    let output_root = PathBuf::from("./generated");

    generator.swift("SharedTypes", output_root.join("swift"))?;

    generator.java("com.example.counter.shared_types", output_root.join("java"))?;

    generator.typescript("shared_types", output_root.join("typescript"))?;

    Ok(())
}
