# Smart contract interper with Rust

This repository contains the codes and the contents for [createlang.rs](https://github.com/ehsanmok/create-your-own-lang-with-rust)

## Requirements

Make sure you have

1. [Rust toolchain installed](https://www.rust-lang.org/tools/install)
2. Cloned this repository (follow the instructions in each chapter)
3. LLVM installed to run and test locally `cargo test --tests`
    * Easiest option is LLVM v14.0 ([Debian/Ubuntu](https://apt.llvm.org/) or [macOS](https://formulae.brew.sh/formula/llvm))
    * Otherwise, in `Cargo.toml` you'd need to change the `inkwell = { ..., branch = "master", features = ["your-llvm-version"] }` with LLVM version on your system (output of `llvm-config --version`)
