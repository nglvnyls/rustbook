# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [Packages and Crates](#packages-and-crates)
    - [Crates](#crates)
    - [Packages](#packages)
  - [What cargo new does](#what-cargo-new-does)

----

## Packages and Crates

### Crates

A **crate** is a **binary or library**.

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects.

It cans use that functionality in our own projects by bringing the crate into our project’s scope.

All the functionality provided by the crate is accessible through the crate’s name.

Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined and prevents potential conflicts.




### Packages

A **package** is **one or more crates** that provide a set of functionality.

A package must contain:

- 0 or 1 library crates. No more.
- 1 or many binary crates. At least one.


## What cargo new does

```rust
cargo new something
```
When cargo news run it does:

- Created a Cargo.toml file, giving us a package
- Create src/main.rs file, following a convention that this is the crate root of a crate with the same name as a a package.
- If src/lib.rs exist, there is 1 library crate there and this is its crate root.
- passes the crate root files to rustc to build the library or binary.







