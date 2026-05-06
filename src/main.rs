use clap::Parser;
use regex::Regex;
use std::fs;
use std::process;

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
    regex: bool, // <-- Add this flag
}

fn main() {
    // 1. Parse CLI arguments
    let args = Args::parse();

    // 2. Read the file
    let contents = fs::read_to_string(&args.path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {err}");
        process::exit(1);
    });

    // 3. Search the file
    let results = search(&args.pattern, &contents, args.regex);

    // 4. Print the results
    for (line_num, line) in results {
        if args.line_number {
            println!("{line_num}:{line}");
        } else {
            println!("{line}");
        }
    }
}

// --------------------------------------------------
// CORE LOGIC & TESTS BELOW
// --------------------------------------------------

// returns a Vector of Tuples: (Line Number, Line Text)
pub fn search<'a>(query: &str, contents: &'a str, is_regex: bool) -> Vec<(usize, &'a str)> {
    if is_regex {
        // Compile the regex once. If the user provides an invalid regex, 
        // we panic cleanly with expect().
        let re = Regex::new(query).expect("Invalid regular expression provided");
        
        contents
            .lines()
            .enumerate()
            .filter(|(_, line)| re.is_match(line)) // Use regex matching
            .map(|(index, line)| (index + 1, line))
            .collect()
    } else {
        // Fast, plain text matching
        contents
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains(query))
            .map(|(index, line)| (index + 1, line))
            .collect()
    }
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
        let expected = vec![(2, "safe, fast, productive.")];
        assert_eq!(expected, search(query, contents, false)); 
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
            (2, "Two lines containing you"),
            (3, "Three lines containing our word"),
            (4, "Four lines"),
        ];
        assert_eq!(expected, search(query, contents, false));
    }

    #[test]
    fn search_with_regex() {
        // \d+ means "one or more digits"
        let query = r"\d+"; 
        let contents = "\
Letters only
This line has 123 numbers
Another line with 42
No digits here";

        let expected = vec![
            (2, "This line has 123 numbers"),
            (3, "Another line with 42"),
        ];
        assert_eq!(expected, search(query, contents, true));
    }
}
