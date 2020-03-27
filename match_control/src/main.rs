fn main() {
    println!("Match Control");
    println!("");

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {               //every time the method was called with a 
                println!("Lucky penny!");  //Coin::Penny but would still return the 
                1                          //last value of the block, 1:
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }



}
