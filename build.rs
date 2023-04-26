fn main() -> anyhow::Result<()> {
    wit_deps::lock_sync!("crates/wasi-http/wit")?;

    println!("cargo:rerun-if-changed=wit/deps");
    println!("cargo:rerun-if-changed=wit/deps.lock");
    println!("cargo:rerun-if-changed=wit/deps.toml");

    Ok(())
}
