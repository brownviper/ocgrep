use clap::Parser;
use std::fs;
use std::process;
use std::time::Instant;
use rayon::prelude::*;

use ocgrep::{get_files, search};

/// A simple grep clone written in Rust
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    pattern: String,

    /// The path to the file to read
    path: std::path::PathBuf,

    /// Print line numbers with output matches
    #[arg(short = 'n', long)]
    line_number: bool,

    /// Enable Regular Expression search
    #[arg(short = 'e', long)]
    regex: bool,

    /// Highlight matches in red
    #[arg(short = 'c', long = "colour")] // We can map "colour" to "color"
    color: bool,

    /// Recursively search directories
    #[arg(short = 'r', long)]
    recursive: bool,

    /// Print execution time at the end
    #[arg(short = 'b', long)]
    benchmark: bool,
}

fn main() {
    // Start the clock!
    let start_time = Instant::now();

    let args = Args::parse();

    // Get all files to search
    let files = get_files(&args.path, args.recursive);
    
    // If we are searching multiple files, we should print the filename
    let show_filename = files.len() > 1 || args.path.is_dir();

    if files.is_empty() {
        eprintln!("No files found or path is a directory without -r flag.");
        process::exit(1);
    }

    // 2. Loop through every file found
    files.into_par_iter().for_each(|file_path| {
        // Safely attempt to read the file
        let contents = match fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(_) => return, // Note: Use `return` instead of `continue` in a closure
        };

        // Search the file
        let results = search(&args.pattern, &contents, args.regex, args.color);

        // Print the results
        for (line_num, line) in results {
            let mut prefix = String::new();
            if show_filename {
                prefix.push_str(&format!("{}:", file_path.display()));
            }
            if args.line_number {
                prefix.push_str(&format!("{}:", line_num));
            }
            // println! in Rust is thread-safe. It locks stdout for the duration 
            // of the print, so our lines won't get garbled together!
            println!("{}{}", prefix, line);
        }
    });

    // Stop the clock!
    if args.benchmark {
        // eprintln! prints to standard error, keeping our standard output clean
        eprintln!("\n⏱️  Execution time: {:?}", start_time.elapsed());
    }
}
