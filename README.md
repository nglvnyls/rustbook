# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

------------------------------------------------------------

- [rustbook](#rustbook)
  - [Enum values](#enum-values)
    - [methods in enum](#methods-in-enum)
    - [The Option Enum and Its Advantages Over Null Values](#the-option-enum-and-its-advantages-over-null-values)
      - [The <T> syntax](#the-t-syntax)


------------------------------------------------------------
## Enum values

Define a type by enumerating its possible variants

elements only can be one of its variants. Grouped in an enum lets be treated them as the same type.

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

enum can take data directly into each enum variant.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```
it cans put any kind of data inside an enum variant: 

- strings, 
- numeric types, 
- structs, .
- another enumen
- ...

### methods in enum

We're able to define methods on enum.
Like structs we use **impl**

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));

m.call(); // self= Message::Write(String::from("hello"))
```

### The Option Enum and Its Advantages Over Null Values

**Option**, is another enum **defined by the standard library**.

The Option type **is used** in many places because it encodes the very common scenario in which **a value could be something or it could be nothing**.

Rust **doesn’t have the null feature**. Null is a value that means there is no value there.

The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

**Option<T>** is an enum that can encode the concept of a value being present or absent.

it's defined by the [standard library](https://doc.rust-lang.org/std/option/enum.Option.html) as follows:

```rust
enum Option<T> {
    Some(T),
    None,
}

```

- doesn’t need to bring it into scope explicitly.
- so are its variants: you can use Some and None directly without the Option:: prefix
- is still just a regular enum
- Some(T) and None are still variants of type Option<T>
  
#### The <T> syntax 

It’s a **generic type parameter**
<T> means the Some variant of the Option enum can hold one piece of data of any type

```rust
let some_number = Some(5); // here data type is numeric
let some_string = Some("a string"); // here is a String

let absent_number: Option<i32> = None;
/*
If we use None rather than Some, we need to tell Rust what type of Option<T> we have, because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.
*/
```


















