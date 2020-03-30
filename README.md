# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [collection_string](#collection_string)
    - [What Is a String?](#what-is-a-string)
    - [Creating a New String](#creating-a-new-string)
    - [Updating a String](#updating-a-string)
      - [Appending to a String with push_str and push](#appending-to-a-string-with-push_str-and-push)
    - [Concatenation with the + Operator or the format! Macro](#concatenation-with-the--operator-or-the-format-macro)
    - [Indexing into Strings](#indexing-into-strings)
        - [Internal Representation](#internal-representation)
        - [Bytes and Scalar Values and Grapheme Clusters! Oh My!](#bytes-and-scalar-values-and-grapheme-clusters-oh-my)
        - [Slicing Strings](#slicing-strings)
        - [Methods for Iterating Over Strings](#methods-for-iterating-over-strings)

----

## collection_string

New Rustaceans commonly get stuck on strings for a combination of three reasons:

- Rust’s propensity for exposing possible errors.
- strings being a more complicated data structure than many programmers give them credit for
- UTF-8

strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text

String are different from the other collections. Indexing into a String is complicated by the differences between how people and computers interpret String data.

### What Is a String?

Rust has only **one string type** in the core language, which is the string slice **str** that is usually seen in its **borrowed form &str**

The String type, which **is provided by Rust’s standard library** rather than coded into the core language, 
is a:
- growable, 
- mutable, 
- owned, 
- UTF-8 encoded string type.

When Rustaceans refer to “strings” in Rust, they usually mean :
 - the String 
 - the string slice &str types,

not just one of those types.

both **String** and **string slices** are **UTF-8** encoded.

Rust’s standard library also includes a number of other string types:

- OsString, (owned variant)
- OsStr, (borrowed variant)
- CString, (owned variant)
- CStr. (borrowed variant)

Library crates can provide even more options for storing string data.

 These string types can store text in different encodings or be represented in memory in a different way.

### Creating a New String

```rust
let mut s = String::new(); //with the new function

let data = "initial contents";  
let s =               data.to_string(); //with the to_string method
let s = "initial contents".to_string(); //with the to_string method on a literal directly

let s = String::from("initial contents"); // with function String::from
```


### Updating a String

A String can grow in size and its contents can change.

It cans **push** more data into it.
It cans conveniently use the **+ operator**
It cans conveniently use the **format! macro**

to concatenate String values.

#### Appending to a String with push_str and push

it cans grow a String by using the **push_str method to append a string slice,**

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

### Concatenation with the + Operator or the format! Macro

Often, you’ll want to combine two existing strings.ç

One way is to use **the + operator**.
The + operator uses the add method, whose signature looks something like this:

```rust
fn add(self, s: &str) -> String {
```
This signature gives us the clues we need to understand the tricky bits of the + operator.

First, second parameter  has an &, meaning that we’re adding a reference of the second string to the first string because of the first parameter in the add function: we can only add a &str to a String; we can’t add two String values together. But wait—the type of secon parameter (&str) is &String, not &str, as specified in the second parameter to add.

The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.

Second, we can see in the signature that add takes ownership of self, because self does not have an &. This means s1 in Listing 8-18 will be moved into the add call and no longer be valid after that. So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.

### Indexing into Strings

if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.

Rust strings don’t support indexing. But why not? To answer that question, we need to discuss how Rust stores strings in memory.

##### Internal Representation

Some unicode scalar value takes 2 bytes of storage.Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.

To avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

##### Bytes and Scalar Values and Grapheme Clusters! Oh My!

there are actually three relevant ways to look at strings from Rust’s perspective:

- as bytes
- as scalar Values
- grapheme clusters

##### Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. Therefore, Rust asks you to be more specific if you really need to use indices to create string slices.

You should use ranges to create string slices with caution, because doing so can crash your program.

##### Methods for Iterating Over Strings

Fortunately, you can access elements in a string in other ways.

If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the chars method. Calling chars on “नमस्ते” separates out and returns six values of type char, and you can iterate over the result to access each element:











