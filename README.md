# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [panic or not to panic](#panic-or-not-to-panic)
    - [Examples, Prototype Code, and Tests](#examples-prototype-code-and-tests)
    - [Cases in Which You Have More Information Than the Compiler](#cases-in-which-you-have-more-information-than-the-compiler)
      - [Guidelines for Error Handling](#guidelines-for-error-handling)
    - [Creating Custom Types for Validation](#creating-custom-types-for-validation)

----

## panic or not to panic

Call panic or Result?

When code **panics**, there’s **no way to recover**.

Return a **Result value**, you give the calling code options rather than making the decision for it. When failure is expected, it’s more appropriate to return a Result than to make a panic! call

The calling code could choose to:

- attempt to recover in a way that’s appropriate for its situation

- it could decide that an Err value in this case is unrecoverable, so it can call panic! 

Therefore, **returning Result is a good default choice when you’re defining a function that might fail**.

### Examples, Prototype Code, and Tests

The **unwrap and expect** methods are very **handy when prototyping**, before you’re ready to decide how to handle errors.

If a method call fails in a test, you’d want the whole test to fail, even if that method isn’t the functionality under test. Because panic! is how a test is marked as a failure, calling unwrap or expect is exactly what should happen.

### Cases in Which You Have More Information Than the Compiler

It would also be appropriate to call unwrap when you have some other logic that ensures the Result will have an Ok value, but the logic isn’t something the compiler understands.

If **you can ensure by manually inspecting** the code that you’ll **never have an Err** variant, it’s perfectly **acceptable to call unwrap**.

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

#### Guidelines for Error Handling

A **bad state** is when some 
 - assumption, 
 - guarantee, 
 - contract, 
 - or invariant 
**has been broken**, such as when: 
 - invalid values, 
 - contradictory values, 
 - or missing values 
are passed to your code—plus one or more of the following:

  - The bad state is not something that’s expected to happen occasionally.
 - Your code after this point needs to rely on not being in this bad state.
 - There’s not a good way to encode this information in the types you use.

If someone calls your code and **passes in values that don’t make sense**, the best choice might be to **call panic!** and alert the person using your library to the bug in their code so they can fix it during development.

**panic!** is often **appropriate** if you’re **calling external code** that is out of your control and **it returns an invalid state** that you have no way of fixing.

having **lots of error checks** in all of your functions would be **verbose and annoying**. Fortunately, you can use **Rust’s type system** (and thus the type checking the compiler does) to **do many of the checks** for you.

### Creating Custom Types for Validation

One way to do validation is to parse the guess as a type desired, and then add a check with if expression.

Other way,  we **make a new type** and **put the validations in a function to create an instance of the type rather than repeating the validations everywhere**. 

That way, it’s safe for functions to use the new type in their signatures and confidently use the values they receive.










