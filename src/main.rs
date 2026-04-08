mod scanner;
mod patterns;
mod models;

use scanner::scan_directory;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: config-checker-rs <path>");
        return;
    }

    let path = &args[1];

    println!("Scanning: {}\n", path);

    let findings = scan_directory(path.to_string());

    if findings.is_empty() {
        println!("No issues found.");
        return;
    }

    for f in findings {
        println!(
            "[{:?}] {}:{} → {}",
            f.severity, f.file, f.line, f.title
        );
    }
}