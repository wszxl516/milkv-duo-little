fn main() -> anyhow::Result<()>{
    println!("cargo:rerun-if-changed=linker.ld");
    Ok(())
}