# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [path_to_item](#path_to_item)
    - [privacy boundary](#privacy-boundary)
    - [Exposing Paths with the pub Keyword](#exposing-paths-with-the-pub-keyword)
    - [Starting Relative Paths with super](#starting-relative-paths-with-super)
    - [Making Structs and Enum public](#making-structs-and-enum-public)
----

## path_to_item

To show Rust **where to find an item** in a module tree.

it cans take 2 forms:

- **ABSOLUTE PATH**: from crate 
- **RELATIVE PATH**: from current module path

**Followed by one or more identifiers** separated by double colons (::).

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

```

It cans use the **crate** keyword to start an **absolute path**, because the two functions are **defined in the same crate**.

In relative path, the path starts at the same level of module tree (eat_at_restaurant), so it begins with "front_of_house".

**Our preference is to specify absolute paths** because it’s more likely to move code definitions and item calls independently of each other.

Modules aren’t useful only for organizing your code. They also **define Rust’s privacy boundary**

### privacy boundary

if you want to **make an item** like a function or struct **private**, you **put it in a module**.

all items:

 - functions, 
 - methods, 
 - structs, 
 - enums, 
 - modules, 
 - and constants
  
**are private by default**.

Items in:

- parent module can’t use the private items inside child modules.Child modules wrap and hide their implementation details
- child modules can use the items in their ancestor modules. Child modules can see the context in which they’re defined.


### Exposing Paths with the pub Keyword

it cans **expose inner parts** of child modules code to outer ancestor modules by **using the pub keyword** to make an item public.

**pub keyword** marks an item as public, in order to have public acces to its inner code. But the contents of inner items are still private; So if want acces to inner items you have to make public inner items too.

```rust
mod front_of_house { //usefut for organizing your code
    pub mod hosting { // is public
        fn add_to_waitlist() {} //is private
    }
}
```

```rust
mod front_of_house { //usefut for organizing your code
    pub mod hosting { // is public
        pub fn add_to_waitlist() {} //is also public
    }
}
```

### Starting Relative Paths with super

relative paths can be build  **super** when the paht begins in parent module.

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // goes to parent module, which in this case is 'crate'
    }

    fn cook_order() {}
}
fn main() {}

```

### Making Structs and Enum public


It cans use pub to designate **structs and enums as public**, but there are a **few extra details**: from

- if we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis

**enum public**, has **all** of its **variants public**.

Enums aren’t very useful unless their variants are public.

Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.





















