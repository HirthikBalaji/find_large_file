use walkdir::WalkDir;
use humansize::{format_size, DECIMAL};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <directory> [top_n]");
        return;
    }

    let root = &args[1];
    let top_n: usize = args.get(2).unwrap_or(&"10".to_string())
                           .parse().unwrap_or(10);

    let mut files: Vec<(String, u64)> = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().display().to_string();
        println!("{}", path);
        if let Ok(metadata) = fs::metadata(entry.path()) {
            files.push((path, metadata.len()));
        }
    }

    // Sort by size (largest first)
    files.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nTop {} largest files in '{}':\n", top_n, root);

    for (path, size) in files.iter().take(top_n) {
        println!("{:<12}  {}", format_size(*size, DECIMAL), path);
    }
}
