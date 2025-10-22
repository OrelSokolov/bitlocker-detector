use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const PATTERN: &[u8] = b"FVE-FS";
const CHUNK_SIZE: usize = 16 * 1024 * 1024; // 16 MB

fn main() -> std::io::Result<()> {
    let device = "/dev/nvme0n1"; // при необходимости поменяй

    let mut file = File::open(device)?;
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut global_offset: u64 = 0;

    eprintln!("[*] Scanning {} for BitLocker signatures (FVE-FS)...", device);

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        if let Some(indices) = find_all(&buffer[..bytes_read], PATTERN) {
            for idx in indices {
                let abs_offset = global_offset + idx as u64;
                println!("{}", abs_offset);
            }
        }

        global_offset += bytes_read as u64;
    }

    eprintln!("[+] Done.");
    Ok(())
}

fn find_all(buf: &[u8], pat: &[u8]) -> Option<Vec<usize>> {
    let mut positions = Vec::new();
    let mut start = 0;
    while let Some(pos) = twoway::find_bytes(&buf[start..], pat) {
        positions.push(start + pos);
        start += pos + 1;
    }
    if positions.is_empty() { None } else { Some(positions) }
}

