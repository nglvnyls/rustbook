# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

<hr>

  - [structs](#structs)
      - [Using the Field Init Shorthand when Variables and Fields Have the Same Name](#using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name)
      - [Creating Instances From Other Instances With Struct Update Syntax](#creating-instances-from-other-instances-with-struct-update-syntax)
      - [Using Tuple Structs without Named Fields to Create Different Types](#using-tuple-structs-without-named-fields-to-create-different-types)
  

<hr>

## structs
Structs are similar to tuples.

- the pieces of a struct **can be different types**
- **you’ll name each piece of data** so it’s clear what the values mean

we use **struct** a name  and curly brackets. 
Inside curly brackets we define **fields** 
Fields are **the name and the type** of the pieces of data.

```bash
struct User {
    username: String, 
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
To use a struct after we’ve defined it, we create an **instance** of that struct by specifying **concrete values for each of the fields**

the **struct** definition is like a general **template for the type**, and **instances** fill in that template with **particular data** to create values of the type

To get a specific value from a struct, **we can use dot notation**. 

Instance is immutable, but with mut entire instance can be mutable.

Rust doesn’t allow us to mark only certain fields as mutable, entire instance must be mutable.

#### Using the Field Init Shorthand when Variables and Fields Have the Same Name

If parameter names and the struct field names are exactly the same, **we can use the field init shorthand syntax **

#### Creating Instances From Other Instances With Struct Update Syntax

**The syntax ..** specifies that the **remaining fields** not explicitly set should have the **same value as** the fields in the **given instance**.

#### Using Tuple Structs without Named Fields to Create Different Types

Tuple structs have the **added** meaning the **struct name** provides but **don’t** have names **associated with their fields**; rather, they just have the types of the fields.

Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples


To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

//black and origin values are different types
```
#### Unit-Like Structs Without Any Fields

**unit-like structs  don’t have any fields!**

Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself













