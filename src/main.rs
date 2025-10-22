use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

const PATTERN: &[u8] = b"FVE-FS";
const CHUNK_SIZE: usize = 16 * 1024 * 1024; // 16 MB

fn main() {
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| String::from("bitlocker-detector"));

    let command = match parse_command(args) {
        Ok(cmd) => cmd,
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!();
            print_usage(&program);
            process::exit(1);
        }
    };

    match command {
        Command::Help => {
            print_usage(&program);
        }
        Command::Run(config) => {
            match run(&config) {
                Ok(summary) => {
                    eprintln!("[+] Done.");
                    if config.show_size {
                        let total_mb = summary.bytes_scanned as f64 / (1024.0 * 1024.0);
                        eprintln!("[+] Total bytes scanned: {:.2} MB", total_mb);
                    }
                }
                Err(err) => {
                    eprintln!("IO error: {}", err);
                    process::exit(1);
                }
            }
        }
    }
}

struct Config {
    device: String,
    show_size: bool,
}

enum Command {
    Run(Config),
    Help,
}

struct RunSummary {
    bytes_scanned: u64,
}

fn parse_command(mut args: env::Args) -> Result<Command, String> {
    let mut show_size = false;
    let mut device: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--show-size" => show_size = true,
            "--help" | "-h" => return Ok(Command::Help),
            _ if arg.starts_with('-') => {
                return Err(format!("Unknown option: {}", arg));
            }
            _ => {
                if device.is_some() {
                    return Err("Multiple disk paths provided".to_string());
                }
                device = Some(arg);
            }
        }
    }

    let device = device.ok_or_else(|| String::from("Missing disk path"))?;
    Ok(Command::Run(Config { device, show_size }))
}

fn run(config: &Config) -> std::io::Result<RunSummary> {
    let mut file = File::open(&config.device)?;
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut global_offset: u64 = 0;

    eprintln!(
        "[*] Scanning {} for BitLocker signatures (FVE-FS)...",
        config.device
    );

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

    Ok(RunSummary {
        bytes_scanned: global_offset,
    })
}

fn print_usage(program: &str) {
    eprintln!("Usage: sudo {} [--show-size] <disk_path>", program);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --show-size   Print total bytes scanned once the scan completes");
    eprintln!("  --help        Show this message");
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

