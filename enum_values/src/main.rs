fn main() {
    println!("Enum Values");
    println!("");

    /*
     Say we need to work with IP addresses. 
     Currently, two major standards are used for IP addresses: 
     version four and version six. 
     These are the only possibilities for an IP address that 
     our program will come across: we can enumerate all possible variants, 
     which is where enumeration gets its name.

     Any IP address can be either a version four or a version six address, 
     but not both at the same time. That property of IP addresses makes 
     the enum data structure appropriate, because enum values can only be 
     one of its variants.    

    */

    let four = IpAddrKind::V4; //We can create instances of each of the two variants of IpAddrKind
    let six = IpAddrKind::V6; //the variants of the enum are namespaced under its identifier
                              //both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind

    fn route(ip_kind: IpAddrKind) { } //We can then, for instance, define a function that takes any IpAddrKind:

    route(IpAddrKind::V4);//And we can call this function with either variant:
    route(IpAddrKind::V6);

    /*
    at the moment we don’t have a way to store the actual IP address data; 
    we only know what kind it is. Given that you just learned about structs 
    in Chapter 5, you might tackle this problem as shown in
    */

    enum IpAddrKind {
        V4,
        V6,
    }
    
    struct IpAddr {
        kind: IpAddrKind, //type defined by enum
        address: String,
    }
    
    let home = IpAddr { //instances of IpAddr struct.
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr { //instances of IpAddr struct.
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    /*We’ve used a struct to bundle the kind and address values 
    together, so now the variant is associated with the value.
    
    We can represent the same concept in a more concise way using 
    just an enum, rather than an enum inside a struct, by putting 
    data directly into each enum variant
    */

    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    
    let loopback = IpAddr2::V6(String::from("::1"));

    /*
    There’s another advantage to using an enum rather than a struct: 
    each variant can have different types and amounts of associated data. 
    Version four type IP addresses will always have four numeric components 
    that will have values between 0 and 255. 
    If we wanted to store V4 addresses as four u8 values but still express 
    V6 addresses as one String value, we wouldn’t be able to with a struct.
    */

    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr3::V4(127, 0, 0, 1);
    
    let loopback = IpAddr3::V6(String::from("::1"));

    /*
    This one has a wide variety of types embedded in its variants.
    */

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    /*
    This enum has four variants with different types:

        - Quit has no data associated with it at all.
        - Move includes an anonymous struct inside it.
        - Write includes a single String.
        - ChangeColor includes three i32 values.
    */

    impl Message { //we add a function to Message enum called "call"
        fn call(&self) {
            // method body would be defined here
        }
    }
    
    let m = Message::Write(String::from("hello"));
    
    m.call(); // self= Message::Write(String::from("hello"))

    //Option<T>

    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    //let y: Option<i8> = Some(5); gets an error because, 
                                // Option<T> and T are different types

    //Only when we have an Option<T>  do we have to worry about possibly 
    //not having a value, and the compiler will make sure we handle that 
    //case before using the value.In other words, you have to convert 
    //an Option<T> to a T before you can perform T operations with it                            

    /*
    Everywhere that a value has a type that isn’t an Option<T>, 
    you can safely assume that the value isn’t null. 
    This was a deliberate design decision for Rust to limit null’s 
    pervasiveness and increase the safety of Rust code.
    */

    
}
