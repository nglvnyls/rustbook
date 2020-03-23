# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## functions

Functions are pervasive in Rust code.
Rust code **uses snake case** as the conventional style for **function and variable names**.
Function definitions in Rust **start with fn** and have a **set of parentheses** after the function name. The **curly brackets** tell the compiler where the **function body begins and ends**.

We can call any function we’ve defined by entering its name followed by a set of parentheses.

Rust doesn’t care where you define your functions, only that they’re defined somewhere. The lines execute in the order in which they appear in the main function.

### Function Parameters

Parameters are special variables that are part of a function’s signature. Technically, the concrete values **are called arguments**,


### Function Bodies Contain Statements and Expressions

Rust is an expression-based language.
Let’s look at what statements and expressions are and how their differences affect the bodies of functions.

**Statements** are instructions that **perform some action and do not return a value**

**Expressions** evaluate to a **resulting value**

##### Statements. 

- Function definition
```
fn main() {
   ....
}
```

- Creating a variable and assigning a value to it with the let keyword is a statement too

```
let y = 6;
```
 In other languages, you can write 
 ```
  x = y = 6 
  ```
  and have both x and y have the value 6; that is not the case in Rust. 
  y = 6 statement does not return a value, so there isn’t anything for x to bind to. you’ll get an error

##### Expressions.

- A simple math operation, such as 5 + 6
- Calling a function
- Calling a macro
- The block that we use to create new scopes, {}

 Expressions do **not include ending semicolons**. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

### Functions with Return Values

Functions can return values to the code that calls them.
We don’t name return values, but we do **declare their type after an arrow**. 
The **return value** of the function is synonymous with **the value of the final expression in the block** of the body of a function.
You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

If you do not return a value (so an expression), you'll get an error.

```
fn plus_one(x: i32) -> i32 {
    x + 1;
}
```
**; changes it from an expression to a statement**, we’ll get an error.


