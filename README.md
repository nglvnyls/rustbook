# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [- The _ Placeholder](#ullithe-_-placeholderliul)
  - [Match control](#match-control)
      - [Patterns that Bind to Values](#patterns-that-bind-to-values)
    - [Matching with Option<T>](#matching-with-optiont)
      - [Matches Are Exhaustive](#matches-are-exhaustive)
  - [The _ Placeholder](#the-_-placeholder)
----

## Match control

**Is an extremely powerful control flow operator**.

Allows to compare a value against a series of patterns and then execute code based on which pattern matches.

Patterns can be made up of:

- literal values,
- variable names, 
- wildcards, 
- and many other things

The **compiler confirms** that **all possible cases are handled**.

First, we list the match keyword followed by an expression. 

Next are the match arms

arms are:

- has **3 parts**: 
  - a pattern,
  - an operator,
  - some code.
- **pattern** is a **value**
- the **=> operator** that **separates** the pattern and the code to run
- The **code** associated with each arm is an expression(that is a resulting **value**)
- Each arm **is separated** from the next with a **comma**
- can have **as many arms as we need**
- they **can bind** to the **parts of the values** that match the pattern
  
When the match expression executes, 

- **it compares** the resulting value against the pattern of each arm, **in order**
- If a pattern matches the value, the code associated with that pattern is executed.
- The resulting value in the matching arm is the value that gets returned for the entire match expression.

This seems very similar to an **expression used with if**, but there’s a **big difference**: with if, the expression needs to return a Boolean value, but here, **it can be any type**

Curly brackets typically aren’t used if the match arm code is short.If you want to run multiple lines of code in a match arm, you can use curly brackets.


```rust
match coin { //Next are the match arms.
        Coin::Penny => 1, //first match arms. Finished with a comma.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
```

#### Patterns that Bind to Values

we can extract values out of enum variants, because an useful feature of match arms is that they can bind to the parts of the values that match the pattern.

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

The binding for state sometimes will be the value UsState::Alaska. When it occurs it can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.

### Matching with Option<T>

we wanted to get the inner T value out of the Some case when using Option<T>.

It handle Option<T> using match and we’ll compare the variants of Option<T>, because an useful feature of match arms is that they can bind to the parts of the values that match the pattern.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

```
**Combining match and enums is useful in many situations**. You’ll see this pattern a lot in Rust code: **match against an enum, bind a variable to the data inside**, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.

#### Matches Are Exhaustive

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier.

## The _ Placeholder

It ss a pattern we can use when we **don’t want to list all possible values**

The _ pattern will match any value.

By **putting it after our other arms**, the _ will match all the possible cases that aren’t specified before it.







