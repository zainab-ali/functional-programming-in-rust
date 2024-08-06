# An introduction to functional programming

This presentation was given at the Women in Rust Lunch & Learn on 6th August 2024. It serves as an introduction to functional programming.

The slides can be found on the [corresponding website](https://zainab-ali.github.io/introduction-to-functional-programming-in-rust).

## Abstract

In this session, we'll get to the heart of functional programming. We'll explore what it is, how to structure code with it, and how it benefits us when reasoning. We'll see how Rust's language features lend itself to developing in functional style.

## Code examples

The examples from the slides are in `src/bin`:
 - `birthday1` has the bug
 - `birthday2` resolves this by separating and minimising side effects.
 - `birthday3` has a `Message` datatype to enable even more testable code.
 - `birthday4` has an `Error` datatype to enable totality.
 
You can run the examples with:
```sh
cargo run --bin birthday1 2030-08-06
```

And run the tests with:
```sh
cargo test
```
