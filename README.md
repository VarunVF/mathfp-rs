# MathFP

[![CI & Docs](https://github.com/VarunVF/mathfp-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/VarunVF/mathfp-rs/actions/workflows/rust.yml)

**MathFP** is a functional, expression-oriented programming language designed for mathematical modeling and rapid prototyping. Built with Rust, it prioritizes safety and mathematical correctness.

[Online Documentation](https://varunvf.github.io/mathfp-rs)

## Features

* **Expression-First:** Everything in MathFP is an expression, which allows for more elegant composition of logic.
* **First-Class Functions:** Functions are first-class values. They can be passed as arguments and returned from functions.
* **Rich Error Reporting:** Detailed scanner and parser error messages, including line and column tracking.

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)

### Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/varunvf/mathfp-rs.git
cd mathfp-rs/
cargo build --release
```

### Usage

Run the REPL to start evaluating expressions:

```bash
cargo run
```

You can also run a script by passing the filename as an argument.

```bash
cargo run -- script.mfp
```

## Language Syntax

### Variable Bindings

Variables are bound using the `:=` operator.

```mathfp
x := 10;
y := x * 5;
```

### Functions

Functions use the `|->` (maps-to) operator:

```mathfp
f := x |-> x * x;
```

## Development

### Running Tests

Run the testing suite with cargo:

```bash
cargo test
```

### Documentation

The project documentation is automatically updated on every push to `main`.
**[View the online documentation](https://varunvf.github.io/mathfp-rs)**

You can also view the local version by running:
```bash
cargo doc --open
```
