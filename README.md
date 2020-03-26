# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## Slice

Slice does not have ownership.

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

If we try to get a part (a slice) of a text, we could search for it in the variable an get it through its index, but if when we have the index, the variable is erased or changed, the index will be pointing to an wrong adress. 

Having to worry about the index in word getting out of sync with the data in variable is tedious and error prone! 

Rust has a solution to this problem: string slices.

## String Slices

A string slice is a **reference to part of a String**. 

We can create slices using a range within brackets by specifying **/[starting_index..ending_index]**, where 

- starting_index is **the first position** in the slice  
- ending_index is **one more than the last position** in the slice

```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];// are equal

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..]; // are equal


let slice = &s[0..len]; // all string
let slice = &s[..]; // all string
```

### String Literals Are Slices

String Literals are some slice pointing to that specific point of the binary. 

This is also why string **literals are immutable**; 
**&str** is an immutable reference.

### String Slices as Parameters

If we have a string slice, we can pass that directly. 
If we have a String, we can pass a slice of the entire String.

Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality

### Array Slices

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3]; //it refers to part of an array

/*This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length. You’ll use this kind of slice for all sorts of other collections*/
```

The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work.
