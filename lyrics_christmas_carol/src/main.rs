fn main() {
    println!("Print the lyrics to the Christmas carol");
    println!("-The Twelve Days of Christmas-");        
    println!("taking advantage of the repetition in the song");
    println!("");
    
    let versos = [
        "On the {} day of Christmas my true love sent to me" ,
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve Lords a leaping",
    ];
    
    let ordinal = ["first","second", "third", "fouth", "fifht", "sixth","seventh", "eight", "ninth","tenth","eleventh","twelfth" ];
        
    for n in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", ordinal[n]);
            
        if n==0 {
            println!("A partridge in a pear tree");
            println!("");
            continue;
        }
    
        for vers in (1..n+2).rev() {
            //println!("{}", n);
            println!("{}", versos[vers]);        
            
        }
        if n==11 {
            println!(",and a partridge in a pear tree.");
        }
            
        println!("");
    }
    
    
}
    

