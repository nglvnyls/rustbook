# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## data types

### scalar types

A scalar type represents **a single value**. 

Rust has four primary scalar types: 

- integers, 
- floating-point numbers, 
- Booleans, 
- characters. 

#### integer Type

An integer is a **number without a fractional component**. 

There are 6 variants

- 8
- 16
- 32
- 64
- 128
- size


Each variant can be either **signed or unsigned** and has an explicit size. 

##### Signed and unsigned 

refer to whether it’s possible for the **number to be negative or positive**. 

In other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned). It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign. Signed numbers are stored using two’s complement representation.

Each **signed variant can store** numbers from 

> -(2n - 1) to 2n - 1 - 1 inclusive

where n is the number of bits that variant uses. So an :

i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. 

**Unsigned variants can store** numbers from 

>0 to 2n - 1

so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.


Additionally, the **isize and usize** types depend on the kind of computer your program is running on: 

> 64 bits if you’re on a 64-bit architecture and 
> 32 bits if you’re on a 32-bit architecture.

#### integer literals 

All number literals except the byte literal allow a type suffix, such as 57u8, and _ as a visual separator, such as 1_000.

#### So how do you know which type of integer to use? 

If you’re unsure, Rust’s defaults are generally good choices, and **integer types default to i32** This type is generally the fastest, even on 64-bit systems. 

The primary situation in which you’d use isize or usize is when indexing some sort of collection.


#### Floating-Point Type

Rust also has two primitive types for floating-point numbers, **which are numbers with decimal points**. 

Rust’s floating-point types are: 

- f32 - 32 bits in size
- f64 - 64 bits in size

The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.

#### Numeric Operations

Rust supports the basic mathematical operations you’d expect for all of the number types: 

- addition 
- subtraction 
- multiplication 
- division 
- and remainder 


