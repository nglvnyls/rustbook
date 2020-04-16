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
      - [Using Result<T, E> in Tests](#using-resultt-e-in-tests)
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

It’s also important to **check that our code handles error conditions as we expect**.

We do this by adding another attribute, should_panic, to our test function.

We place the #[should_panic] attribute after the #[test] attribute and before the test function it applies to

To **make should_panic tests more precise**, we can add an optional **expected parameter** to the should_panic attribute

In case of panics returns an string message a substring of the panic message is enough to ensure that pass the test.

#### Using Result<T, E> in Tests

We can also write tests that use Result<T, E>!

In the body of the function, rather than calling the assert_eq! macro, we return Ok(()) when the test passes and an Err with a String inside when the test fails.

Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.

You **can’t use the #[should_panic] annotation on tests that use Result<T, E>**. Instead, you should return an Err value directly when the test should fail.




