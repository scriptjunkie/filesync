use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::env;
fn main() -> std::io::Result<()> {
    let mut buf = [0; 16384];
    let mut args = env::args();
    if args.len() < 3 {
        eprintln!("Usage: filesync srcfile dstfile");
        return Ok(());
    }
    let mut file = File::open(args.nth(1).unwrap())?;
    let destfname = args.next().unwrap();
    let mut output = OpenOptions::new().read(true).write(true).create(true).open(&destfname)?;
    let mut desthname = destfname.clone();
    desthname.push_str(".hashes");
    println!("Syncing to {} (hashfile {})", destfname, desthname);
    let mut offset: u64 = 0;
    let mut hashoffset: u64 = 0;
    let mut hashfile = OpenOptions::new().read(true).write(true).create(true).open(&desthname)?;
    while let Ok(s) = file.read(&mut buf) {
        if s == 0 {
            break;
        }
        offset += s as u64;
        let hash = blake3::hash(&buf[..s]);
        let mut partialhash = [0; 32];
        if if let Ok(_) = hashfile.read_exact(&mut partialhash) {
            blake3::Hash::from(partialhash) != hash
        } else {true} {
            println!("{} bytes hash mismatch at {}", s, offset - s as u64);
            output.seek(SeekFrom::Start(offset))?;
            output.write_all(&buf[..s])?;
            hashfile.seek(SeekFrom::Start(hashoffset))?;
            hashfile.write_all(hash.as_bytes())?;
            println!("Wrote {} bytes and hash", s);
        }
        hashoffset += partialhash.len() as u64;
    }
    Ok(())
}