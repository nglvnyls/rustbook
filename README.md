# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [recoverable_errors](#recoverable_errors)
    - [How do we know File::open returns a Result?](#how-do-we-know-fileopen-returns-a-result)
    - [Shortcuts for Panic on Error: unwrap and expect](#shortcuts-for-panic-on-error-unwrap-and-expect)
      - [unwrap](#unwrap)
      - [expects](#expects)
    - [Propagating Errors](#propagating-errors)

----

## recoverable_errors

Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.

```rust

#![allow(unused_variables)]
fn main() {
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
}

```

The T and E are generic type parameters:

**T** represents the **type of the value** that will be returned in a **success case within the Ok variant**.

**E** represents the **type of the error** that will be returned in a **failure case within the Err variant**. 

Because Result has these generic type parameters, we can use the Result type and the functions that the standard library has defined on it in many different situations where the successful value and error value we want to return may differ.

### How do we know File::open returns a Result?

If we give f a type annotation that we know is not the return type of the function and then try to compile the code, the compiler will tell us that the types don’t match. The error message will then tell us what the type of f is. Let’s try it! We know that the return type of File::open isn’t of type u32, so let’s change the let f statement to this:

```rust
let f: u32 = File::open("hello.txt");
```
Attempting to compile now gives us the following output:

```rust
...
... found type `std::result::Result<std::fs::File, std::io::Error>`
```

This tells us the return type of the File::open function is a Result<T, E>. 

The generic parameter **T** has been filled in here with the type of the**success value**, **std::fs::File**, which is a file handle. 

The type of **E** used in the error value is **std::io::Error**.

This return type means the call to **File::open might succeed and return a file** handle that we can read from or write to.

The **function call also might fail**: for example, the file might not exist, or we might not have permission to access the file. 
 
The File::open function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information. 
 
This information is exactly what the Result enum conveys.

**If File::open succeeds**: 
  - the value in the variable **f will be an instance of Ok** that contains a file handle.

**If File::open fails**:
  - the value in **f will be an instance of Err** that contains more information about the kind of error that happened.

###Matching on Different Errors

What we want to do instead is take different actions for different failure reasons: 
- if File::open **failed because the file doesn’t exist**, we want to create the file and return the handle to the new file. 
- If File::open **failed for any other reason—for** example, because we didn’t have permission to open the file—we still want the code to panic!




### Shortcuts for Panic on Error: unwrap and expect


Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well. 


The **Result<T, E>** type has many **helper methods defined** on it to do various tasks. One of those methods, called **unwrap**, is a shortcut method that is implemented just like the match expression

#### unwrap

If the **Result value is the Ok** variant, **unwrap** will return the **value inside the Ok**.

If the **Result is the Err** variant, **unwrap** will call the panic! macro for us.

#### expects

Is similar to unwrap.

Lets us choose the panic! error message.

We use expect in the same way as unwrap:

Because this error message starts with the text we specified, Failed to open hello.txt, it will be easier to find where in the code this error message is coming from. If we use unwrap in multiple places, it can take more time to figure out exactly which unwrap is causing the panic because all unwrap calls that panic print the same message.

### Propagating Errors










