use std::fs::OpenOptions;

use boot::meta::BootSectorMeta;

const GB: u32 = 1024 * 1024 * 1024;
const MB: u32 = 1024 * 1024;
const KB: u16 = 1024;

const DEFAULT_BOUNDARY_ALIGNEMENT: u32 = 1024 * 1024;

pub mod boot;
pub mod disk;
pub mod error;

pub struct ExFat;

fn main() {
    let size: u64 = 256 * MB as u64;
    let bytes_per_sector = 512;
    // default cluster size based on sector size
    let cluster_size = if size <= 256 * MB as u64 {
        4 * KB
    } else if size <= 32 * GB as u64 {
        32 * KB
    } else {
        128 * KB
    } as u32;

    let boot_sector_meta = BootSectorMeta::try_new(
        0,
        bytes_per_sector,
        cluster_size,
        size,
        DEFAULT_BOUNDARY_ALIGNEMENT,
        false,
    )
    .unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .truncate(true)
        .create(true)
        .open("test")
        .unwrap();

    boot_sector_meta.write(&mut file).unwrap();
    println!("done");
}
