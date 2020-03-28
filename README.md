# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

------------------------------------------------------------
- [rustbook](#rustbook)
  - [- Concise control Flow with if let](#ulliconcise-control-flow-with-if-letliul)
  - [Concise control Flow with if let](#concise-control-flow-with-if-let)
------------------------------------------------------------

## Concise control Flow with if let

It **combines if and let** into a less verbose way to **handle values that match one pattern** while **ignoring the rest**.

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

Instead, we could write this in a shorter way using if let

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

The syntax if let takes a:
- pattern
- equal sign (=) in the middle
- expression

It **works the same way as a match**, where the expression is given to the match and the pattern is its first arm.

Using if let means less typing, less indentation, and less boilerplate code

However, you lose the exhaustive checking that match enforces

We **can include an else** with an if let.

If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let is in your Rust toolbox as well.

When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.

Creating custom types to use in your API ensures type safety: the compiler will make certain your functions get only values of the type each function expects.