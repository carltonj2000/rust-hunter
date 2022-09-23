use clap::Parser;
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

#[derive(Parser, Default, Debug)]
#[clap(author = "Carlton Joseph", version, about)]
/// A simple package hunter
struct Arguments {
    #[clap(forbid_empty_values = true, validator = validate_package_name)]
    /// Name of the package to search
    package_name: String,
    #[clap(short, long, default_value_t=usize::MAX)]
    /// Maximum depth of the search
    max_depth: usize,
}
fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
    match count(args) {
        Ok(c) => println!("{} usage count", c),
        Err(e) => println!("error! {}", e),
    };
}

fn count(args: Arguments) -> std::io::Result<usize> {
    let mut count = 0;
    let mut queue = VecDeque::new();
    queue.push_back((PathBuf::from("."), 0));
    loop {
        if queue.is_empty() {
            break;
        }
        let (path, crr_depth) = queue.pop_back().unwrap();
        if crr_depth > args.max_depth {
            continue;
        }
        for dir in fs::read_dir(path)? {
            let dir = dir?;
            if dir.file_type()?.is_dir() {
                if dir.file_name().to_str().unwrap() == args.package_name {
                    count += 1;
                } else {
                    queue.push_back((dir.path(), crr_depth + 1));
                }
            }
        }
    }
    Ok(count)
}
