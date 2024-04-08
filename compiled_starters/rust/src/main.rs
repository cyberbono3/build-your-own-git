use anyhow::Context;
use clap::{Parser, Subcommand};
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::ffi::CStr;
use std::io::{BufReader, BufRead};

use flate2::write::ZlibDecoder;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Debug, Subcommand)]
enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            // TODO support shortest unique object hashes
            let mut f = std::fs::File::open(format!(
                ".git/objects/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))
            .context("open in .git/objects  ")?;
            let z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);
            let mut buf: Vec<u8> = Vec::new();
            let _ = z.read_until(0, &mut buf).context("read header from .git/objects")?;
            let header = CStr::from_bytes_with_nul(&buf).expect("know there is an exact one nul and it is at the end");
            let header = header.to_str().context("git/object file header is not valid utf8")?;
            let Some((kind, size)) = header.split_once(' ') else {
                anyhow::bail!(".git/objects file header did not start with 'known type': '{header}'");
            };
            let Some(size) = header.strip_prefix("blob") else {
                anyhow::bail!(".git/objects file header did not start with 'blob': '{header}'");
            };
            let size = size.parse::<usize>().context(".git/objects file header has invalid 'size': '{size}'")?;
            buf.clear();
            buf.reserve_exact(size);
            let _  = z.read_exact(&mut buf[..]).context(".git/objects file does not match expectations")
            let n = z.read(&mut [0]).context("validte EOF in .git/object file")?;
            anyhow::ensure!(n == 0, "git/objects file has trailing bytes");
            let stdout = std::io::stdout();
            let stdout = stdout.lock();
           
            match kind {
                "blob" => 
                    stdout 
                    .write_all(&buf)
                    .context("write objects contents to stdout")?,
                _ => write!(stdout, "we do not know how to print kind"),
            }



        }
    }

    Ok(())
}
