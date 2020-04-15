fn main() {
    println!("11.0 Automated Test");
    println!("");
    let add_to_3 = add_two(3);
    println!("add_two(3) = {}", add_to_3);
    println!("");
    println!("with assert_eq!(add_two(3), 5)
     we can run cargo test an test if function 
     returns is correct");
    assert_eq!(add_two(3), 5);
    
}

fn add_two (x: u32) -> u32 {
    x+2
}
