# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [use_keyboard](#use_keyboard)
    - [Creating Idiomatic use Paths](#creating-idiomatic-use-paths)
    - [Providing New Names with the as Keyword](#providing-new-names-with-the-as-keyword)
    - [Re-exporting Names with pub use](#re-exporting-names-with-pub-use)
    - [Using External Packages](#using-external-packages)
    - [Using Nested Paths to Clean Up Large use Lists](#using-nested-paths-to-clean-up-large-use-lists)
  - [The Glob Operator](#the-glob-operator)

----

## use_keyboard

We can bring a path into a scope once and then call the items in that path as if they’re local items with the use keyword.
 
Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.

### Creating Idiomatic use Paths

The idiomatic way is bringing the function’s parent module into scope with use so we have to specify the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. 

On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.

There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.


### Providing New Names with the as Keyword

There’s another solution to the problem of bringing two types of the same name into the same scope with use: after the path, we can specify as and a new local name, or alias, for the type

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```
### Re-exporting Names with pub use

When we bring a name into scope with the use keyword, the name available in the new scope is private. 

To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. 

This technique is called **re-exporting** because we’re **bringing an item into scope but also making that item available for others to bring into their scope**.

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain

### Using External Packages

To use an external package in our project, we added this line to Cargo.toml behind [dependencies].

```rust
[dependencies]
rand = "0.5.5"
```
Adding a package as a dependency in Cargo.toml tells Cargo to download the package and any dependencies from crates.io and make this available to our project.

to bring packages definitions into the scope of our package, we added a use line starting with the name of the package, and listed the items we wanted to bring into scope.

```rust
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

**standard library (std)** is also a crate that’s external to our package. 

Because the standard library is shipped with the Rust language, we **don’t need**to change Cargo.toml **to include std**. 

But we do need to refer to it with use to bring items from there into our package’s scope.

```rust
use std::collections::HashMap;
```

### Using Nested Paths to Clean Up Large use Lists

---
```rust
use std::io;
use std::cmp::Ordering;
}
```
is equal to 

```rust
use std::{cmp::Ordering, io};
}
```
----
```rust
use std::io;
use std::io::Write;
}
```
is equal to 

```rust
use std::io::{self, Write}; //brings std::io into scope and std::io::Write
}
```
----

## The Glob Operator

bring all public items defined in a path into scope

* is the glob operator

```rust
use std::collections::*;
```
Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

The glob operator is often used when testing to bring everything under test into the tests module.

The glob operator is also sometimes used as part of the prelude pattern.














