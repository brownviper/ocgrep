use clap::Parser;
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
    let results = search(&args.pattern, &contents);

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
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()                  // Gives us (0, "line text")
        .filter(|(_, line)| line.contains(query))
        .map(|(index, line)| (index + 1, line)) // Convert 0-indexed to 1-indexed
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
            (4, "Four lines"), // <-- Add this line!
        ];
        assert_eq!(expected, search(query, contents));
    }
}
