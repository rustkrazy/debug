use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut f = File::create("/debug.txt")?;
    f.write_all("boot successful".as_bytes())?;

    Ok(())
}
