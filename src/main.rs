use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use colored::Colorize;
use walkdir::WalkDir;
use std::time::Instant;
use rayon::prelude::*;

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

// --------------------------------------------------
// CORE LOGIC & TESTS BELOW
// --------------------------------------------------

pub fn search(query: &str, contents: &str, is_regex: bool, is_colored: bool) -> Vec<(usize, String)> {
    if is_regex {
        let re = Regex::new(query).expect("Invalid regular expression provided");
        
        contents
            .lines()
            .enumerate()
            .filter(|(_, line)| re.is_match(line))
            .map(|(index, line)| {
                let formatted_line = if is_colored {
                    // Replace the exact matched regex group with a red version of itself
                    re.replace_all(line, |caps: &regex::Captures| {
                        caps[0].red().to_string()
                    }).to_string()
                } else {
                    line.to_string()
                };
                (index + 1, formatted_line)
            })
            .collect()
    } else {
        contents
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains(query))
            .map(|(index, line)| {
                let formatted_line = if is_colored {
                    // Replace the raw string with a red version
                    line.replace(query, &query.red().to_string())
                } else {
                    line.to_string()
                };
                (index + 1, formatted_line)
            })
            .collect()
    }
}

pub fn get_files(path: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if recursive && path.is_dir() {
        // Walk the directory, ignoring errors (like permission denied)
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    } else if path.is_file() {
        files.push(path.to_path_buf());
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_match_with_line_numbers() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let expected = vec![(2, String::from("safe, fast, productive."))];
        assert_eq!(expected, search(query, contents, false, false)); 
    }

    #[test]
    fn search_finds_multiple_matches() {
        let query = "ou";
        let contents = "\
One line
Two lines containing you
Three lines containing our word
Four lines";
        let expected = vec![
            (2, String::from("Two lines containing you")),
            (3, String::from("Three lines containing our word")),
            (4, String::from("Four lines")),
        ];
        assert_eq!(expected, search(query, contents, false, false));
    }

    #[test]
    fn search_with_regex() {
        let query = r"\d+"; 
        let contents = "\
Letters only
This line has 123 numbers
Another line with 42
No digits here";

        let expected = vec![
            (2, String::from("This line has 123 numbers")),
            (3, String::from("Another line with 42")),
        ];
        assert_eq!(expected, search(query, contents, true, false));
    }

    #[test]
    fn search_with_color() {
        // Force the colored crate to output ANSI codes even during background tests
        colored::control::set_override(true);

        // We simulate the `colored` crate's output. \x1b[31m is red, \x1b[0m resets it.
        let query = "fast";
        let contents = "safe, fast, productive.";
        
        let expected_line = format!("safe, \x1b[31m{}\x1b[0m, productive.", query);
        let expected = vec![(1, expected_line)];
        
        assert_eq!(expected, search(query, contents, false, true));
    }

    #[test]
    fn test_get_files_recursive() {
        // Search our own 'src' directory recursively
        let path = Path::new("src");
        let files = get_files(path, true);
        
        // It should find at least one file, and main.rs should be among them
        assert!(!files.is_empty());
        assert!(files.iter().any(|p| p.ends_with("main.rs")));
    }
}
