# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [103 - Validating References with Lifetimes](#103---validating-references-with-lifetimes)
    - [Preventing Dangling References with Lifetimes](#preventing-dangling-references-with-lifetimes)
    - [The Borrow Checker](#the-borrow-checker)
    - [Generic Lifetimes in Functions](#generic-lifetimes-in-functions)
      - [Lifetime Annotation Syntax](#lifetime-annotation-syntax)
    - [Lifetime Annotations in Function Signatures](#lifetime-annotations-in-function-signatures)
    - [Lifetime Elision](#lifetime-elision)
      - [Rules for elision](#rules-for-elision)
----

## 103 - Validating References with Lifetimes

Every reference has a lifetime.

Lifetime is the scope for which that reference is valid.

Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.

We must **annotate types when multiple types are possible**.

we **must annotate lifetimes when the lifetimes** of references could be related in a few **different ways**.

Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

### Preventing Dangling References with Lifetimes

**The main aim of lifetimes** is to **prevent** dangling references, which cause a program **to reference data other than the data it’s intended to reference**.

### The Borrow Checker

It is how it determine the code is valid or not.

It compares scopes to determine whether all borrows are valid.

### Generic Lifetimes in Functions

Generic lifetime parameters define the relationship between the references so the borrow checker can perform its analysis.

#### Lifetime Annotation Syntax

Lifetime annotations don’t change how long any of the references live.

Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types. 

Most people use the name 'a. 

We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other

### Lifetime Annotations in Function Signatures

It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.

As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct.

```rust
struct ImportantExcerpt<'a> {
```
we can use the lifetime parameter in the body of the struct definition

### Lifetime Elision

In some cases **some functions** compiles **without lifetime **annotations.

**The reason is historical**. 

After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.

The **patterns programmed** into Rust’s analysis of references are called the **lifetime elision rules**.

They’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

**input lifetimes** are Lifetimes on function or **method parameters**.

**output lifetimes** are lifetimes on return values.

#### Rules for elision

**1- Each parameter that is a reference gets its own lifetime parameter**

a function with **1 parameter**:
-  gets **1 lifetime** parameter: 
   -  fn foo<'a>(x: &'a i32);

a function with **2 parameters**:  
-  gets **2 separate lifetime** parameters:
   -  fn foo<'a, 'b>(x: &'a i32, y: &'b i32); 
and so on.

**2- if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters**

```rust
foo<'a>(x: &'a i32) -> &'a i32
```

**3- (ONLY APPLIES IN METHOD SIGNATURES) if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.**

Here is an example where the third lifetime elision rule applies:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.







