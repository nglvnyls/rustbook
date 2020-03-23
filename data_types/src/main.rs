use std::char::from_u32;

fn main() {

    /*
    Scalar Types
    A scalar type represents a single value
    */

     /*
    Integer Types
    An integer is a number without a fractional component
    */
    println!("interger types");
    println!("Signed"); //Numbers can be negative or positive, 
    //so then number needs to have a sign with it (signed)   

    let inegatiu: i8 = -128; 
    let ipositiu: i8 = 127;      
    println!("i8- from {} to {}", inegatiu, ipositiu);
    
    let inegatiu: i16 = -32_768;
    let ipositiu: i16 = 32_767;    
    println!("i16- from {} to {}", inegatiu, ipositiu);

    let inegatiu: i32 = -2_147_483_648;
    let ipositiu: i32 = 2_147_483_647;    
    println!("i32- from {} to {}", inegatiu, ipositiu);

    let inegatiu: i64 = -9_223_372_036_854_775_808;
    let ipositiu: i64 = 9_223_372_036_854_775_807;    
    println!("i64- from {} to {}", inegatiu, ipositiu);

    let inegatiu: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let ipositiu: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;    
    println!("i128- from {} to {}", inegatiu, ipositiu);

    println!("Unsigned");//Only positive numbers
    // can be represented without a sign (unsigned)

    let u: u8 = 255;      
    println!("u8- from 0 to {}", u);
    
    let u: u16 = 65_535;    
    println!("u16- from 0 to {}", u);

    let u: u32 = 4_294_967_295;    
    println!("u32- from 0 to {}", u);

    let u: u64 = 18_446_744_073_709_551_615;    
    println!("u64- from 0 to {}", u);

    let u: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;    
    println!("u128- from 0 to {}", u);

     /*
    Floating-Point Types
    They are numbers with decimal points.
    */
    println!("floating-point types");
   
    let x = 2.0; // f64. which are 64 bits in size
    println!("f64- default {} -- double precission", x);

    let y: f32 = 3.0; // f32 .which are 32 bits in size
    println!("f32- default {} -- single precission", y);

    println!("Numeric Operations");
    //Rust supports the basic mathematical operations 

    // addition
    let sum = 5 + 10;
    println!("addition: 5+10 = {}",sum);
    let sum = 0.0000000000001 + 0.00000001;
    println!("addition: 0.0000000000001 + 0.00000001 = {}",sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("subtraction:  95.5 - 4.3 = {}",difference);

    // multiplication
    let product = 4 * 30;
    println!("multiplication:   4 * 30 = {}",product);

    // division
    let quotient = 56.7 / 32.2;
    println!("division:  56.7 / 32.2 = {}",quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder:  43 % 5 = {}",remainder);

    println!("The Boolean Type");
    //has two possible values: true and false 

    let t = true;
    println!("{}",t);

    let f: bool = false; // with explicit type annotation
    println!("{}",f);

    println!("The Character Type");
    /*
    Rust supports letters too as a scalar.
    CHAR literals are specified with single quotes, 
    as opposed to string literals, which use double quotes
    Char type is 4 bytes in size.
    It represents a Unicode Scalar Value.
    it can represent: 
     - Accented letters; 
     - Chinese, 
     - Japanese, 
     - Korean characters; 
     - emoji; 
     - zero-width spaces 
    are all valid char values in Rust

    Unicode Scalar Values range from 
     - U+0000 to U+D7FF 
       and 
     - U+E000 to U+10FFFF inclusive. 
    
    However, a “character” isn’t really a concept in Unicode, 
    so your human intuition for what a “character” is may not match up 
    with what a char is in Rust.
    */
    let c = ['z','è','é','i','ò','ó','ç','à','ú']; 
    // This loop prints: 0 1 2
    for x in &c {
        println!("in minúscules, {}",x);
    }

    let z = ['ℤ','À','È','Í','Ò','Ó','Ú','Ç']; 
    for x in &z {
        println!("or MAJÚSCULES, {}",x);
    }

    let heart_eyed_cat = '😻';
    println!("emojis too. Heart_eyed_cat = {}",heart_eyed_cat);

    let martini_emoji = '\u{1F378}';
    println!("martini_emoji = {}",martini_emoji);

    let smiling_face_with_halo = '\u{1F607}';
    println!("smiling face with halo = {}",smiling_face_with_halo);


    /* Unicode Scalar Values range from 
     - U+0000 to U+D7FF 
       and 
     - U+E000 to U+10FFFF inclusive. 
     */

    for n in 0xD7F0 ..=0xD7FF  {
        println!("Unicode Scalar Values, {:X} = {}",n, from_u32(n).unwrap());
    }

    for n in 0x10FF00 ..=0x10FFFF {
        println!("Unicode Scalar Values, {:X} = {}",n, from_u32(n).unwrap());
    }

    println!("Compound Types");
    /*
    Compound types can group multiple values into one type. 
    Rust has two primitive compound types: tuples and arrays
    */
    println!("Tuple Type");
    /*
    Tuples have a fixed length: once declared, 
    they cannot grow or shrink in size.
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = tup.0;
    println!("First value of tup tuple is: {}", five_hundred);

    let six_point_four = tup.1;
    println!("Second value of tup tuple is: {}", six_point_four);

    let one = tup.2;
    println!("Third value of tup tuple is: {}", one);

    let (x, y, z) = tup;

    println!("We can desestructure the tuple into x,y,z. The value of y is: {}", y);
    /*
    We create a tuple by writing a comma-separated 
    list of values inside parentheses. 
    */
    /*
    Each position in the tuple has a type, 
    and the types of the different values in the 
    tuple don’t have to be the same
    */




}
