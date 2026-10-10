# MathFP

[![CI & Docs](https://github.com/VarunVF/mathfp-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/VarunVF/mathfp-rs/actions/workflows/rust.yml)

MathFP is a functional programming language written in Rust. It aims to look and feel like math while being practical to use.

```mathfp
fibonacci := n |-> match {
    n < 2  => n,
    n >= 2 => fibonacci(n-1) + fibonacci(n-2)
};

println(map(fibonacci, range(0, 10)))  // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

Everything in MathFP is an expression: `if` conditionals, `match` conditionals, lambdas, functions, and more are all equally treated as values.

MathFP provides detailed scanner and parser error messages with line and column tracking to help you catch mistakes easily.

```mathfp
>>> y := (2 * (x + 3);
Parser errors:
[Line 1, Col 18] Expected ')' after parenthesised expression, found ';'
```

## Getting Started

### Building

Install [Rust](https://www.rust-lang.org/tools/install), then clone the repository and build the project using Cargo. (Pass the `-r` flag to build or run an optimised release build.)

```bash
git clone https://github.com/varunvf/mathfp-rs.git
cd mathfp-rs/
cargo build
```

Run the REPL to start evaluating expressions, or run a script by passing the filename as an argument.

```bash
cargo run  # Start the REPL
cargo run script.mfp  # Run a script
```

### Editor Support

A simple syntax highlighting extension for VS Code / VSCodium is available at [`editors/vscode/`](https://github.com/VarunVF/mathfp-rs/tree/main/editors/vscode#mathfp-vs-code-extension). Most editor themes should work alongside with this extension.

## Syntax

The below is an explanation of the language syntax. If you prefer reading code or want more examples, you can find several scripts at [`examples/`](https://github.com/VarunVF/mathfp-rs/tree/main/examples).

### Variables

Variables are declared using the `:=` operator, and most variables can be modified using the `=` operator.
A variable can only be declared once in the same scope.

```mathfp
x := 10;        // x is declared as 10
y := x * 5;     // y is declared as 50
x = 2 * y;      // x is set to 100
```

### Types

Variables can be of any of the following types:
- Numbers
- Strings
- Booleans
- Functions
- Lists (using square brackets: `[4, 5, 6]`)
- Nil (the type of the `nil` value)

### Conditionals

Any expression can be used in the `then` or `else` clause of an `if`-expression.

```mathfp
if y then (z := 1) else (z := 2)
```

If you omit the `else` branch but the condition is false, `nil` is implicitly returned.
```mathfp
res := if 0 then 5;  // res is set to nil
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

Functions are single-argument lambdas defined using the `|->` (maps-to) operator:

```mathfp
f := x |-> x * x;
f(2)  // 4
```

Functions can contain multiple statements wrapped in braces `{...}`; the value of the last expression is implicitly returned. Variables declared inside a function are locally scoped.

#### Currying & Multi-Argument Functions

All functions take exactly one argument; multi-argument functions are defined by 'chaining' single-argument lambdas (this technique is called currying). In the snippet below, `hypotenuse` takes an argument of `3` and returns another function, which in turn takes an argument of `4`:

```mathfp
hypotenuse := a |-> b |-> {
    a2 := a * a;
    b2 := b * b;
    sqrt(a2 + b2)
};
hypotenuse(3)(4)  // 5
```

For convenience, you may also pass comma-separated arguments, which translate directly into curried applications at parse time: `hypotenuse(3, 4)` is syntax sugar for `hypotenuse(3)(4)`.

Supplying fewer arguments than a function expects returns a new, specialised function waiting for the remaining arguments. In the below, `filter` is partially applied to create a specialised `take_positive` function, which is only later applied to a list.

```mathfp
items := [2, 5, -1, -3]
take_positive := filter(x |-> x > 0)
take_positive(items)  // [2, 5]
```

#### Builtins

Common math functions like `sin` / `sqrt` and several other utilities are built-in and can be used anywhere.
You can read the full list of builtins at [`src/stdlib.mfp`](https://github.com/VarunVF/mathfp-rs/blob/main/src/stdlib.mfp).

```mathfp
square := x |-> x * x;
f := x |-> square(sin(x)) + square(cos(x));
```

## Development

### Running Tests

Run the testing suite with cargo:

```bash
cargo test
```

### Documentation

The [project documentation](https://varunvf.github.io/mathfp-rs) is automatically updated on every push to `main`. You can also view the local version by running:
```bash
cargo doc --open
```
