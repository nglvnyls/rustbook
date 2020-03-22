fn main() {

    println!("interger types");
    println!("Signed");

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

    println!("Unsigned");

    
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

    println!("floating-point types");
   
    let x = 2.0; // f64
    println!("f64- default {} -- double precission", x);
    let y: f32 = 3.0; // f32
    println!("f32- default {} -- single precission", y);

    println!("Numeric Operations");

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



}
