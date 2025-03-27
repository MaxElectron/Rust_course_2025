#![forbid(unsafe_code)]

use std::io::BufRead;

fn read_file(filename: &str) -> std::collections::HashSet<String> {
    let file = std::fs::File::open(filename).expect("error: unable to open file");
    let reader = std::io::BufReader::new(file);

    let mut lines = std::collections::HashSet::new();
    for line in reader.lines() {
        let line = line.expect("error: unable to read line");
        lines.insert(line);
    }

    lines
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("error: two filenames expected, got {}", args[0]);
        std::process::exit(1);
    }

    let file_lines1 = read_file(&args[1]);
    let file_lines2 = read_file(&args[2]);

    let common_lines = file_lines1.intersection(&file_lines2);

    for line in common_lines {
        println!("{}", line);
    }
}
