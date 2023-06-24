# contracts-node-runtime

## Installation

### Requirements

Should install required packages and Rust for Substrate development

Follow the [official installation steps](https://docs.substrate.io/install/) to set up all Substrate prerequisites.

Afterwards you can build node runtime via

```bash
cargo build
```

## Usage

To run a local dev node execute

```bash
./target/debug/contracts-node 
```

A new chain in temporary directory will be created each time the command is executed. This is the
default for this node. If you want to persist chain state across runs you need to
specify a directory with `--base-path`.
