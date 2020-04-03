fn main() {
    println!("91 Unrecoverable Errors with panic!");
    println!("");

    //panic!("crash and burn");
    //The call to panic! causes an error message 

    let v = vec![1, 2, 3];

    v[99];

    //line 10 causes an error.
    //if you call un your console 
    // $RUST_BACKTRACE=1 cargo run. 
    //Baktrace were running-


}
