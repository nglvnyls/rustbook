# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [111 - How to Write Tests](#111---how-to-write-tests)
    - [The Anatomy of a Test Function](#the-anatomy-of-a-test-function)
      - [Simplest function](#simplest-function)
      - [Checking Results with the assert! Macro](#checking-results-with-the-assert-macro)
      - [Adding Custom Failure Messages](#adding-custom-failure-messages)
      - [Checking for Panics with should_panic](#checking-for-panics-with-should_panic)
----

## 111 - How to Write Tests

Tests **are Rust functions**.

The bodies of test functions **typically perform these** actions:
  - Set up any needed data or state.
  - Run the code you want to test.
  - Assert the results are what you expect.

### The Anatomy of a Test Function

#### Simplest function

A function that’s annotated with the test attribute
To change a function into a test function, **add #[test] on the line before fn**

```rust
#[test]
    fn test_add() {
        assert_eq!(...
    }
```
#### Checking Results with the assert! Macro

The assert! macro, provided by the standard library, is useful when you want **to ensure that some condition in a test evaluates to true**.

assert! macro needs an **argument that evaluates to a Boolean**. 

If the value is:
 - TRUE:   does nothing and the test passes.
 - FALSE: calls the panic! macro, which causes the test to fail

#### Adding Custom Failure Messages

You can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros.

Custom **messages are useful to document** what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.


#### Checking for Panics with should_panic








