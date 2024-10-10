use std::{option, path::PathBuf};

use colored::Colorize;
use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", author = "hr.zhou@mail.utoronto.ca", disable_help_flag = true)]
struct Config {
    // path: Option<PathBuf>,
    // first value is the string to be searched
    paths: Vec<PathBuf>,

    /// Case-insensitive search
    #[arg(short='i')]
    i: bool,

    /// Print line numbers
    #[arg(short='n')]
    n: bool,

    /// Invert match (exclude lines that match the pattern)
    #[arg(short='v')]
    v: bool,

    /// Recursive directory search
    #[arg(short='r')]
    r: bool,

    /// Print filenames
    #[arg(short='f')]
    f: bool,

    /// Enable colored output
    #[arg(short='c')]
    c: bool,

    #[arg(short = 'h', long = "help", action = clap::ArgAction::SetTrue)]
    h: bool
}

fn help_msg() {
    println!(
        "Usage: grep [OPTIONS] <pattern> <files...>\n\
        Options:\n\
        -i                Case-insensitive search\n\
        -n                Print line numbers\n\
        -v                Invert match (exclude lines that match the pattern)\n\
        -r                Recursive directory search\n\
        -f                Print filenames\n\
        -c                Enable colored output\n\
        -h, --help        Show help information"
    );
}

fn main() {
    let config = Config::parse();
    if config.h || config.paths.is_empty() {
        help_msg();
        return;
    }

    for path in config.paths {
        if path.exists() {
            println!("Using file: {:?}", path);
        }
        else {
            eprintln!("File does not exist: {:?}", path);
        }
    }
}
