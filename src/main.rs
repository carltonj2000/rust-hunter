use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

#[derive(Default, Debug)]
struct Arguments {
    package_name: String,
    max_depth: usize,
}
fn main() {
    let args = get_arguments();
    println!("{:?}", args);
    match count(args) {
        Ok(c) => println!("{} usage count", c),
        Err(e) => println!("error! {}", e),
    };
}

fn get_arguments() -> Arguments {
    let mut params = Arguments::default();
    let args: Vec<_> = std::env::args().collect();
    let len = args.len();
    if len < 2 {
        println!("Enter a -f argument");
        std::process::exit(1);
    };
    if args[1] != "-f" {
        println!("provide -f argument first");
        std::process::exit(1);
    }
    params.package_name = args[2].clone();
    let mdepth = if len > 3 {
        if args[3] != "-d" {
            println!("provide -d argument second");
            std::process::exit(1);
        }
        if len < 5 {
            println!("No -d argument provided");
            std::process::exit(1);
        };
        args[4].parse().unwrap()
    } else {
        usize::MAX
    };
    println!("{:?}", args);
    params.max_depth = mdepth;
    params
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
