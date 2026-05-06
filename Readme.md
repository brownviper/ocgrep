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