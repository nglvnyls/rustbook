# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [102 - Traits](#102---traits)
    - [Defining a Trait.](#defining-a-trait)
  - [Default Implementations](#default-implementations)
    - [use traits to define functions that accept many different types](#use-traits-to-define-functions-that-accept-many-different-types)
    - [Trait Bound Syntax](#trait-bound-syntax)
    - [Specifying Multiple Trait Bounds with the + Syntax](#specifying-multiple-trait-bounds-with-the--syntax)
    - [Clearer Trait Bounds with where Clauses](#clearer-trait-bounds-with-where-clauses)
    - [Returning Types that Implement Traits](#returning-types-that-implement-traits)

----

## 102 - Traits

A trait tells the Rust compiler about **functionality a particular type has and can share** with other types.

We use traits :
 - **to define shared behavior** in an abstract way.
 - to specify that a **generic can be any type** that has **certain behavior**.

### Defining a Trait.

**methods we can call** on that type are **type’s behavior**.

Trait definitions are **a way to group method signatures together to define a set of behaviors** necessary to accomplish some purpose.

## Default Implementations

Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.

Then, as we implement the trait on a particular type:
  - we can keep 
  - we can override 
each method’s default behavior.

### use traits to define functions that accept many different types

### Trait Bound Syntax

The impl Trait syntax is convenient and makes for more concise code in simple cases. The trait bound syntax can express more complexity in other cases. For example, we can have two parameters that implement Summary. Using the impl Trait syntax looks like this:

```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {
```
If we wanted this function to **allow item1 and item2 to have different types**, using impl Trait would be appropriate (as long as both types implement Summary). 

If we wanted to force **both parameters to have the same type**, that’s **only possible to express using a trait bound**, like this:

```rust
pub fn notify<T: Summary>(item1: T, item2: T) {
```

### Specifying Multiple Trait Bounds with the + Syntax

We can also specify more than one trait bound. 
Say we wanted notify to use:
-  display formatting on item 
-  the summarize method.

we specify in the notify definition that item must implement both Display and Summary. 

We can do so using the + syntax:

```rust
pub fn notify(item: impl Summary + Display) {
```
is also valid with trait bounds on generic types:

```rust
pub fn notify<T: Summary + Display>(item: T) {
```
With the two trait bounds specified, the body of notify can call summarize and use {} to format item.


### Clearer Trait Bounds with where Clauses

Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. So instead of writing this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```
we can use a where clause, like this:

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

### Returning Types that Implement Traits

We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait.

The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators.

Closures and iterators create types that only the compiler knows or types that are very long to specify. 

The impl Trait syntax lets you concisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.

It cans **only return a single type**.









