use clap::Parser;
use std::process;
use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::io::{BufRead, BufReader};

extern crate exitcode;


#[derive(Parser)]
struct CLIArguments {
    #[clap(short = 'p', long = "path")]
    path: String,
    #[clap(short = 's', long = "size")]
    size: i32
}


fn read_file(path: String) -> Result<Vec<String>, String> {
    let file_path = Path::new(&path);
    if !file_path.exists() {
        let err = format!("Filepath {} does not exist", file_path.display());
        return Err(err);
    }

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut words: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split(" ").collect();
        for part in parts {
            let vector: Vec<&str> = part.split(
                |ch: char| !(ch.is_alphanumeric() || ch == '\'')
            ).filter(|s| !s.is_empty()).collect();
            for w in vector {
                words.push(w.to_string());
            }
        }
    }
    Ok(words)
}

fn create_ngrams(data: Vec<String>,  data_size: i32, size: i32) -> Vec<String> {
    let mut ngrams: Vec<String> = Vec::new();

    let mut pos = 0;
    while pos < data_size {
        let mut entry: Vec<String> = Vec::new();
        if size > 1 {
            for modifier in 0..size {
                let index = (pos + modifier) % size;
                entry.push(data[index as usize].to_string());
            }
        } else {
            entry.push(data[pos as usize].to_string());
        }
        ngrams.push(entry.join(",").to_string());
        pos += 1;
    }
    ngrams
}

fn main() {
    let args = CLIArguments::parse();
    if args.path.is_empty() || args.size <= 0 {
        println!("Path to file and ngram size are required");
        process::exit(exitcode::USAGE);
    }

    let data = match read_file(args.path){
        Ok(text) => text,
        Err(e) => return println!("{}", e)
    };

    let data_size = data.len().try_into().unwrap();
    if data_size < args.size {
        println!("Ngram size {} cannot be larger than data size {}", args.size, data_size);
        process::exit(exitcode::USAGE);
    }

    let ngrams = create_ngrams(data, data_size, args.size);
    println!("Resulting ngrams: {}", ngrams.len());
    for n in ngrams {
        println!("{}", n);
    }
}
