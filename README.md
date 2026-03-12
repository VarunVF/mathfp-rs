# MathFP

[![CI & Docs](https://github.com/VarunVF/mathfp-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/VarunVF/mathfp-rs/actions/workflows/rust.yml)

**MathFP** is a functional, expression-oriented programming language designed for mathematical modeling and rapid prototyping. Built with Rust, it prioritizes safety and mathematical correctness.

[Online Documentation](https://varunvf.github.io/mathfp-rs)

## Features

* **Expression-First:** Everything in MathFP is an expression, which allows for more elegant composition of logic.
* **First-Class Functions:** Functions are first-class values. They can be passed as arguments and returned from functions or other expressions.
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

### Editor Support

A simple syntax highlighting extension for VS Code is available (located in `editors/vscode/`).
Most editor themes should work alongside with this extension.

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

Variables are declared using the `:=` operator.
A variable can only be declared once in the same scope.

```mathfp
x := 10; y := x * 5;
```

Most variables can be modified using the `=` operator.
```mathfp
x = 2 * y;
```

### Conditionals

Any expression can be used in the `then` and `else` branches of an `if`-expression.

```mathfp
if y then (z := 1) else (z := 2)
```

If you omit the `else` branch but the condition is false, `nil` is implicitly returned.
```mathfp
res := if 0 then 5;  // res is now nil
```

`match` expressions can be used to define case-by-case logic or piecewise functions.

```mathfp
a := 5;
b := 7;
x := match {
    b == 0 => nil,
    b != 0 => a / b,
};
```

### Functions

Functions use the `|->` (maps-to) operator:

```mathfp
f := x |-> x * x;
f(2)
```

Functions can have a more complex body with multiple statements.

The last expression is implicitly returned. Bindings created inside a function are locally scoped and do not affect their outer scope.

```mathfp
hypotenuse := a |-> b |-> {
    a2 := a * a;
    b2 := b * b;
    sqrt(a2 + b2)
};
hypotenuse(3)(4)
```

#### Builtins

Common math functions like `sin` and `sqrt` are defined as native functions, and can be used anywhere.

```mathfp
square := x |-> x * x;
square(sin(9)) + square(cos(9))
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
