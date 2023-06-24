# Rust Smart Contracts Execution Engine

Simple testing project for building a smart contract execution engine in Rust

## Requirements
Build a smart contract execution engine in Rust that
- can interpret and execute simple smart contracts,
- including features like state management (memory) where users can update variable values
- handle function calls and return values
- managing contract scope.
- create a CLI or set of APIs to interact with the execution engine.
- Define a simple DSL to represent the smart contracts.

## Development
### Contract Interpreter
Simple interperter base on inkwell for smart contract interpretation.
Follow this guide in [`contract-interpreter`](https://github.com/black-wyvern-dev/simple-rust-contract-exeuction-engine-test/blob/main/contract-interperter/README.md#requirements)

### Contract runtime node template
Simple node runtime template for smart contract execution
Follow this guide in [`contracts-node`](https://github.com/black-wyvern-dev/simple-rust-contract-exeuction-engine-test/blob/main/contracts-node/README.md#installation)

## Inspired by
This repository contains Substrate's Node Runtime [`contracts-node`](https://codespaces.new/paritytech/substrate-contracts-node) \
And smart contract interpreter based on inkwell [`contract-interperter`](https://createlang.rs)
‒ a simple calculator operation module.

This repository is folking Substrate's Node Runtime Engine [`Substrate`](https://github.com/paritytech/substrate).
* [Substrate Node Template](https://github.com/paritytech/substrate/tree/master/bin/node-template)
