use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

fn main() {
    let (str, mdepth) = get_arguments();
    println!("{}", str);
    let cnt = count(&str[..], mdepth).unwrap();
    println!("{}", cnt);
}

fn get_arguments() -> (String, usize) {
    let args: Vec<_> = std::env::args().collect();
        let mdepth = if args.len() > 2 {
        args[2].parse().unwrap()
    } else {
        usize::MAX
    };
    println!("{:?}", args);
    (args[1].clone(), mdepth)
}

fn count(name: &str, max_depth: usize) -> std::io::Result<usize> {
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
                if dir.file_name() == name {
                    count += 1;
                } else {
                    queue.push_back((dir.path(), crr_depth + 1));
                }
            }
        }
    }
    Ok(count)
}
