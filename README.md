# bitlocker-detector

GPT table gone and you are note able to recovery by `R-studio`, `photorec` because of encryption?
Not a problem! Detect BitLocker partition using this tool.

Original problem: `grep` is being killed anytime you use search by grep. This tool is just grep for bitlocker but without memory leak. 


Unlike `grep`, `ripgrep` or `strings`, this tool:

✅ Reads the disk **in fixed-size chunks (default: 16 MB)**  
✅ Does **not load entire disk into memory**  
✅ Outputs **clean byte offsets** of `FVE-FS` matches (BitLocker headers / metadata)  
✅ Can be safely **stopped and resumed**  
✅ Suitable for forensic tasks or BitLocker partition recovery

---

## ⚙️ Build

```bash
git clone https://github.com/OrelSokolov/bitlocker-detector.git
cd bitlocker-detector
cargo build --release
```

Binary will appear at:

```
target/release/bitlocker-detector
```

## 🚀 Usage

- ⚠️ Run as root.
- ⚠️ Always use the raw disk (e.g. /dev/nvme0n1), not a partition.

```
sudo ./target/release/bitlocker-detect /dev/nvme0n1 | tee fve_offsets.txt

Example output:

1245677265
29886856521
529563451393
...

```

Each number = absolute byte offset on disk where FVE-FS was found.
This allows reconstructing approximate start/end of a BitLocker volume.

## 🛠 How it works

- The program scans the disk chunk-by-chunk (default 16 MB).
- In every chunk it searches for the ASCII signature FVE-FS.
- Found positions are printed as absolute disk offsets in bytes.
- No binary data or shred are output – only raw offsets.


## ✅ Requirements

- Rust 1.70+
- Linux / macOS
- Root privileges to read raw disks

## ⚠️ Warning

Scanning large NVMe/SSD devices can take time.
Never write to the source disk if you are trying to recover data.
Always store resulting images (bitlocker.img) to a separate external disk with enough free space.

## 📜 License

Apache 2.0