use clap::{Parser, Subcommand};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

fn validate_package_name(name: &str) -> Result<(), String> {
    if name.trim().len() != name.len() {
        Err(String::from(
            "package name cannot have leading and trailing space",
        ))
    } else {
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[clap(author = "Carlton Joseph", version, about)]
/// A simple package hunter
struct Arguments {
    #[clap(short, long, default_value_t=usize::MAX)]
    /// Maximum depth of the search
    max_depth: usize,
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Count how many times the package is used
    Count {
        #[clap(forbid_empty_values = true, validator = validate_package_name)]
        /// Name of the package to search
        package_name: String,
    },
    /// list all the projects
    Projects {
        #[clap(short, long, default_value_t = String::from("."),forbid_empty_values = true, validator = validate_package_name)]
        /// directory to start exploring from
        start_path: String,
        #[clap(short, long, multiple_values = true, value_delimiter = ':')]
        /// paths to exclude when searching
        exclude: Vec<String>,
    },
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
    match args.cmd {
        SubCommand::Count { package_name } => match count(package_name, args.max_depth) {
            Ok(c) => println!("{} usage count", c),
            Err(e) => println!("error! {}", e),
        },
        SubCommand::Projects {
            start_path,
            exclude,
        } => {
            println!("{:?} {:?}", start_path, exclude)
        }
    }
}

fn count(package_name: String, max_depth: usize) -> std::io::Result<usize> {
    let mut count = 0;
    let mut queue = VecDeque::new();
    queue.push_back((PathBuf::from("."), 0));
    loop {
        if queue.is_empty() {
            break;
        }
        let (path, crr_depth) = queue.pop_back().unwrap();
        if crr_depth > max_depth {
            continue;
        }
        for dir in fs::read_dir(path)? {
            let dir = dir?;
            if dir.file_type()?.is_dir() {
                if dir.file_name().to_str().unwrap() == package_name {
                    count += 1;
                } else {
                    queue.push_back((dir.path(), crr_depth + 1));
                }
            }
        }
    }
    Ok(count)
}
