# AdventOfCode 2021 - Learning Rust :heart: again ^^

This is my take on the [advent of code challenges](https://adventofcode.com/2021/) of 2021.

I already tried doing them in Rust [last year](https://github.com/bew/adventofcode-2020), but didn't go too far, because (for learning
purpose) I wanted to not use any libs, so I learned about cumbersome parsing and having to
re-write a simple generic error wrapper :)

Now I want to actually do somthing in Rust, so I'm using a few libraries, mainly:
- [`anyhow`](https://crates.io/crates/anyhow) for error wrapping, without having to deal with specific errors
- [`chumsky`](https://crates.io/crates/chumsky) for input parsing using combinators that build incrementally the AST of the input :heart:. I discovered this lib not long ago, and I fell in love, it's time to try it _for real_ now!


## Workflow

* `cargo run last`: Run the last available day, display results.
  It's very handy when I'm working on the next day :smiley:

* `cargo run all`: Run all days, display results

* `cargo run list`: List available days

* `cargo run dayNN`: Run specific day
