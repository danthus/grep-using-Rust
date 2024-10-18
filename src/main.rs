use std::{fs, path::PathBuf};

use colored::Colorize;
use walkdir::WalkDir;
use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(version = "1.0", author = "hr.zhou@mail.utoronto.ca", disable_help_flag = true)]
struct Config {
    // target word
    #[arg(default_value("<NaN>"))]
    target: String,

    // file path or paths
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

impl Config {
    fn run(&self) {
        for path in &self.paths {
            if path.exists() {
                if self.r {
                    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                        let file_name = entry.path();
                        if file_name.is_file() {
                            let contents = fs::read_to_string(file_name).expect("Failed to read the file");
                            let result = self.search(&contents);
                            self.custom_display(file_name.to_str().expect("Failed converting file name to str"), &result);
                        }
                    }
                }
                else {
                    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                        let file_name = entry.path();
                        if file_name.is_file() {
                            let contents = fs::read_to_string(file_name).expect("Failed to read the file");
                            let result = self.search(&contents);
                            self.custom_display(file_name.to_str().expect("Failed converting file name to str"), &result);
                        }
                    }
                }
            }
        }
    }

    fn custom_display(&self, file_name: &str, result: &Vec<(usize, &str)>) {
        if self.f && self.n {
            for item in result {
                if self.c && !self.v {
                    let colored_sentence = self.colorize(item.1);
                    println!("{}: {}: {}", file_name, item.0, colored_sentence);
                }
                else {
                    println!("{}: {}: {}", file_name, item.0, item.1);   
                }
            }
        }
        else if self.f {
            for item in result {
                if self.c && !self.v {
                    let colored_sentence = self.colorize(item.1);
                    println!("{}: {}", file_name, colored_sentence);
                }
                else {
                    println!("{}: {}", file_name, item.1);
                }
            }
        }
        else if self.n {
            for item in result {
                if self.c && !self.v {
                    let colored_sentence = self.colorize(item.1);
                    println!("{}: {}", item.0, colored_sentence);
                }
                else {
                    println!("{}: {}", item.0, item.1);
                }
            }
        }
        else {
            for item in result {
                if self.c && !self.v {
                    let colored_sentence = self.colorize(item.1);
                    println!("{}", colored_sentence);
                }
                else {
                    println!("{}", item.1);
                }
            }
        }
    }

    fn colorize(&self, sentence: &str) -> String {
        if self.i {
            let pattern = Regex::new(&format!(r"(?i){}", regex::escape(&self.target))).unwrap();
    
            // Replace all occurrences of the target word with its colorized version
            let colored = pattern.replace_all(sentence, |caps: &regex::Captures| {
                let matched = &caps[0]; // Get the matched word
                format!("{}", matched.red()) // Colorize the matched word
            });
        
            colored.to_string()
        }
        else {
            let pattern = Regex::new(&regex::escape(&self.target)).unwrap();
    
            // Replace all occurrences of the target word with its colored version
            let colored = pattern.replace_all(sentence, |caps: &regex::Captures| {
                let matched = &caps[0]; // Get the matched word
                format!("{}", matched.red()) // Colorize the matched word
            });
        
            colored.to_string() // Convert the result back to a String
        }
    }

    fn search<'a>(&self, contents: &'a String) -> Vec<(usize, &'a str)> {
        if self.i && self.v {
            let target_low = self.target.to_lowercase();
            contents.lines()
            .enumerate()
            .filter(|(_,line)| !line.to_lowercase().contains(&target_low))
            .collect()
        }
        else if self.i {
            let target_low = self.target.to_lowercase();
            contents.lines()
            .enumerate()
            .filter(|(_,line)| line.to_lowercase().contains(&target_low))
            .collect()
        }
        else if self.v {
            contents.lines()
            .enumerate()
            .filter(|(_,line)| !line.contains(&self.target))
            .collect()
        }
        else {
            contents.lines()
            .enumerate()
            .filter(|(_,line)| line.contains(&self.target))
            .collect()
        }
    }
}

fn main() {
    let config = Config::parse();
    if config.h || config.paths.is_empty() || config.target=="<NaN>" {
        help_msg();
        return;
    }

    config.run();
}
