use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use indicatif::ProgressBar;
use walkdir::WalkDir;
use std::io::Read;
use indicatif::ProgressStyle;

mod algorithms;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    path: PathBuf,
    #[arg(long, short, default_value_t=false)]
    quiet: bool,
    #[arg(long, short, value_enum, default_value_t=algorithms::Algorithm::Sha256)]
    algorithm: algorithms::Algorithm,
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

    files.sort();

    let mut hasher: Box<dyn algorithms::DigestAlgorithm> = match args.algorithm {
        algorithms::Algorithm::Sha256 => Box::new(algorithms::Sha256Algorithm::new()),
        algorithms::Algorithm::Sha512 => Box::new(algorithms::Sha512Algorithm::new()),
        algorithms::Algorithm::Md5 => Box::new(algorithms::Md5Algorithm::new()),
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
        if let Some(filename_str) = file_path.file_name().expect("Illegal file name").to_str(){
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
