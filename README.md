# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [91 - Unrecoverable_errors.](#91---unrecoverable_errors)
    - [Using a panic! Backtrace](#using-a-panic-backtrace)

----

## 91 - Unrecoverable_errors.

When the panic! macro executes, your program will:
- print a failure message, 
- unwind the stack, 
- clean up the stack, 
- and then quit.

This  occurs when a bug of some kind has been detected and it’s not clear how to handle the error.

**unwinding,** which means Rust walks back up the stack and cleans up the data from each function it encounters.

This walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up.

If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:

```rust
[profile.release]
panic = 'abort'
```

### Using a panic! Backtrace

```rust
RUST_BACKTRACE=1 cargo run
```
A backtrace is a **list of all the functions** that **have been called** to get to this point.

Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote.

In order to get backtraces with this information, **debug symbols must be enabled.**

Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here.








