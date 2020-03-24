use std::io;

fn main() {
    println!("FIBONACCI NUMERS!");
    println!("");
    println!("Input how many numbers of FIBONACCI SERIE do you want to output?");
    
    loop{

        let mut numbers = String::new();
    
        println!("Select(from 1 to 186)");

        io::stdin().read_line(&mut numbers)
        .expect("Failed to read Numbers");     

        let numbers: u128 = numbers.trim().parse().unwrap();

        if numbers> 186 {
            println!("I said (from 1 to 186)...");
            continue;
        }

        let mut number1: u128= 0;
        let mut number2: u128 = 1;
        let mut number3: u128 = 1;

        print!("{},{},",number1,number2);

        for _n in 2..numbers {
        
            print!("{},",number3);
            number1= number2;
            number2= number3;
            number3= number1+number2;
        } 
        println!("");
        println!("-------------------");
        println!("");
    }
    

}
