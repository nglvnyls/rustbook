fn main() {
    println!("Packages and Crates");
    println!("");

    //A crate is a binary or library

    //src/main.rs is the crate root 
    //of a binary crate with the same name as the package

    //src/lib.rs,  if the directory and the file exists it contains 
    //a library crate with the same name as the package,

    /*
    If a package contains src/main.rs and src/lib.rs, 
    it has two crates: a library and a binary, both with 
    the same name as the package.     
    */

    //src/bin, if the directory exists, each file in that 
    //directory contains a separete binary crate 

}
