use clap::Parser;
use clap::ValueEnum;
use std::path::PathBuf;
use std::fs::File;
use indicatif::ProgressBar;
use walkdir::WalkDir;
use sha2::{Sha256, Sha512, Digest};
use std::io::Read;
use indicatif::ProgressStyle;
use md5::Context;

fn to_hex(data: &[u8]) -> String {
    let mut out = String::new();
    for x in data {
        out += &String::from(format!("{:x}", x));
    }
    out
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    Sha256,
    Sha512,
    Md5
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    path: PathBuf,
    #[arg(long, short, default_value_t=false)]
    quiet: bool,
    #[arg(long, short, value_enum, default_value_t=Algorithm::Sha256)]
    algorithm: Algorithm,
}


pub trait DigestAlgorithm {
    fn finalize(&mut self) -> String;
    fn update(&mut self, data: &[u8]);
}

struct Sha256Algorithm{
    hasher: Sha256,
}

struct Sha512Algorithm{
    hasher: Sha512,
}

struct Md5Algorithm{
    context: Context,
}

impl DigestAlgorithm for Md5Algorithm {
    fn update(&mut self, data: &[u8]){
        self.context.consume(data);
    }

    fn finalize(&mut self) -> String{
        to_hex(&self.context.clone().compute().as_slice())
    }
}

impl DigestAlgorithm for Sha256Algorithm {
    fn update(&mut self, data: &[u8]){
        self.hasher.update(data);
    }

    fn finalize(&mut self) -> String{
        to_hex(&self.hasher.clone().finalize())
    }
}

impl DigestAlgorithm for Sha512Algorithm {
    fn update(&mut self, data: &[u8]){
        self.hasher.update(data);
    }

    fn finalize(&mut self) -> String{
        to_hex(&self.hasher.clone().finalize())
    }
}

impl Sha256Algorithm {
    fn new() -> Self {
        Sha256Algorithm {
            hasher: Sha256::new()
        }
    }
}

impl Sha512Algorithm {
    fn new() -> Self {
        Sha512Algorithm {
            hasher: Sha512::new()
        }
    }
}

impl Md5Algorithm {
    fn new() -> Self {
        Md5Algorithm {
            context: Context::new()
        }
    }
}

fn main(){
    let args = Cli::parse();
    let mut files: Vec<PathBuf> = Vec::new();
    let spin: Option<ProgressBar> = match args.quiet {
        false => {
            let temp = ProgressBar::new_spinner().with_message("Reading file tree");
            temp.enable_steady_tick(std::time::Duration::from_millis(100));
            Some(temp)
        },
        true => None
    };
    
    for x in WalkDir::new(args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) {
            files.push(x.into_path());
        }
    if let Some(ref x) = spin {
        x.finish();
    }

    let mut hasher: Box<dyn DigestAlgorithm> = match args.algorithm {
        Algorithm::Sha256 => Box::new(Sha256Algorithm::new()),
        Algorithm::Sha512 => Box::new(Sha512Algorithm::new()),
        Algorithm::Md5 => Box::new(Md5Algorithm::new()),
    };

    let bar: Option<ProgressBar> = match args.quiet {
        false => {
            let temp = ProgressBar::new(files.len().try_into().unwrap());
            temp.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"));
            Some(temp)
        },
        true => None
    };

    for file_path in files{
        if let Some(filename_str) = file_path.file_name().expect("REASON").to_str(){
            if let Some(ref x) = bar {
                x.set_message(format!("File: {}", filename_str));
            }
        }
        let mut file = match File::open(&file_path) {
            Ok(f) => f,
            Err(x) => {
                eprintln!("Error opening file: {} ({})", file_path.display(), x);
                continue;
            }
        };
        let mut buffer = [0u8; 1024];
        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(bytes_read) => hasher.update(&buffer[0..bytes_read]),
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    break;
                }
            }
        }
        
        if let Some(ref x) = bar {
            x.inc(1);
        }
    }

    if let Some(ref x) = bar {
        x.finish();
    }
    println!("{}", hasher.finalize());
}
