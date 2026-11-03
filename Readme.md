# Challenge
We'd like for you to build ocgrep, a command line tool for searching through text across multiple files; supporting a small subset of the features found in the popular tool grep.
You may use any programming language you like, either as a way of gaining deeper knowledge of efficient string handling in a language you're already familiar with, or as a way of learning or becoming more familiar with a new language.
 
The challenge has been split into several discrete goals
- Plain text search with one input file
- Plain text search with multiple input files
- Outputting matching line numbers with -n flag
- Searching for regular expressions with -e
- Searching recursively through all files in a directory with -r flag
- Adding colour highlighting to matches in a line with --colour flag
- Optional performance benchmarking with -b
- Optional Overlapping context lines with (-C n to show n lines surrounding the match)

### running all the unit tests
```make test```

## running the cli tool
### single file search
#### with string and line-number
```
make run ARGS="-n 'fn search' src/main.rs"
```
#### with regex and line-number
```
make run ARGS="-n -e '^pub fn' src/main.rs"
```
#### with colours
```
make run ARGS="-n -c 'fn search' src/main.rs"
```
#### with regenx, line-number and colours
```
make run ARGS="-n -c -e 'pub fn.*' src/main.rs"
```
#### recursive search
```
make run ARGS="-r -n -c 'pub' /workspace"
```
#### benchmarking
```
make run ARGS="-r -b -e 'root' /usr"
```

## makeing release binary
```
make release
```
### using the release binary
```
target/release/ocgrep -n -c -r -b "days" ../odyssey
```
