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

Rust is a bit more complicated to run so I have done the different parts as separate binaries with a shared library.
Your can use `cargo` to run them with the `--bin` flag:

```
cargo run --bin part-1 input.txt
cargo run --bin part-2 input.txt
```

## Go

For `go` solutions, I'm using a stdlib flag to toggle the part 1 or part 2 functionality. Using the stdlib flag package means
that the flags must come before the arguments:

```
go run . input.txt
go run . -part-2 input.txt
```
