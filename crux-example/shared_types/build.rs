use crux_core::typegen::TypeGen;
use shared::Counter;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../shared");

    let mut generator = TypeGen::new();

    generator.register_app::<Counter>()?;

    let output_root = PathBuf::from("./generated");

    generator.swift("SharedTypes", output_root.join("swift"))?;

    generator.java("com.crux.example.simple_counter", output_root.join("java"))?;

    generator.typescript("shared_types", output_root.join("typescript"))?;

    Ok(())
}
