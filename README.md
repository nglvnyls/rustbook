# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [Match control](#match-control)
      - [Patterns that Bind to Values](#patterns-that-bind-to-values)

----

## Match control

**Is an extremely powerful control flow operator**.

Allows to compare a value against a series of patterns and then execute code based on which pattern matches.

Patterns can be made up of:

- literal values,
- variable names, 
- wildcards, 
- and many other things

The compiler confirms that all possible cases are handled.

First, we list the match keyword followed by an expression. 

This seems very similar to an **expression used** with if, but there’s a big difference: with if, the expression needs to return a Boolean value, but here, **it can be any type**

```rust
match coin { //Next are the match arms.
        Coin::Penny => 1, //first match arms. Finished with a comma.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
```

An arm has two parts: 
  - a pattern (Coin:Penny)
  - an operator (=>)
  - some code. (value 1)

it **compares** the resulting value against the pattern of each arm, **in order**.

If a pattern matches the value, the code associated with that pattern is executed.

it can have as many arms as it needs

If you want to run multiple lines of code in a match arm, you can use curly brackets.

#### Patterns that Bind to Values







