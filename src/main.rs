use std::fs::File;
use std::io::{self, Write};

use sys_mount::{Mount, Unmount, UnmountFlags};

fn main() -> io::Result<()> {
    let _mount = Mount::builder()
        .fstype("fat")
        .mount("/dev/disk/by-partuuid/00000000-01", "/boot")
        .expect("can't mount boot partition");

    let _mount = _mount.into_unmount_drop(UnmountFlags::DETACH);

    let mut f = File::create("/boot/debug.txt")?;
    f.write_all("boot successful".as_bytes())?;

    Ok(())
}
