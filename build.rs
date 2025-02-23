use std::io::{Read, Seek, Write};

fn main() -> anyhow::Result<()>{
    println!("cargo:rerun-if-changed=linker.ld");
    let mut fd = std::fs::OpenOptions::new().read(true).write(true).open("linker.ld")?;
    let mut buffer = String::new();
    fd.read_to_string(&mut buffer)?;
    if cfg!(feature = "virt") {
        if buffer.contains("0x8fe00000") {
            buffer = buffer.replace("0x8fe00000", "0x80000000");
        }
    }
    else{
        if buffer.contains("0x80000000") {
            buffer = buffer.replace("0x80000000", "0x8fe00000");
        }
    };
    fd.seek(std::io::SeekFrom::Start(0))?;
    fd.write_all(buffer.as_bytes())?;
    Ok(())
}