# Advent of Code Solutions

This is my repo for advent of code solutions. I try and use different languages for different answers so here's rough
guide I'll try and stick to:

## Python(3)

Single python script with a flag to toggle behaviour for differnt parts

```
python3 main.py input.txt
python3 main.py input.txt --part-2
```

## Rust

```
# Run part 1 with the test input
cargo run -- --test
# Run part 2 with the real input
cargo run -- --part-2
```

## Go

For `go` solutions, I'm using a stdlib flag to toggle the part 1 or part 2 functionality. Using the stdlib flag package means
that the flags must come before the arguments:

```
go run . input.txt
go run . -part-2 input.txt
```
