# Installation

For MacOS and WSL (Windows Subsystem)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Verification

```sh
rustc --version
cargo --version
```

# How to compile?

1. Compile
2. Run the file

```sh
rustc test.rs
./test
```
# Cargo

Create a source folder and configuration file

```
cargo init
```

Run the code

```
cargo run 
```

or just building

```
cargo build
cargo build --release
```


# Questions

1. What is a trait?