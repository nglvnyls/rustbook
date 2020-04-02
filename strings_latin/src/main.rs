fn main() {
    println!("Strings to pig latin");
    println!("");

    println!("Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!");
    println!("");

    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words, no new string is allocated
    println!("Words in pangram");

    let mut pig_latin_result = String::new();
   
    for word in pangram.split_whitespace() {
        println!("> {}", word);

        let mut w_rev= word
            .chars()
            .rev()
            .collect::<String>();
        println!("word_rev: {:?}",w_rev);
        
        let first_ch: String = w_rev
            .pop()
            .unwrap()
            .to_string();
        println!("first_ch: {:?}", first_ch);
        println!("word_rev : {:?}",w_rev);
                
        let vowels = "aeiou".to_string();        
        let mut w_pig2 = String::new();

        if vowels.contains(&first_ch) {
            println!("is vowel");

            w_pig2 = word
                .to_string();
            println!("w_pig2 : {:?}",w_pig2);

            w_pig2.push_str("hai");      

        } else {
            
            println!("is consonant");
            println!("second v------------");

            w_pig2 = w_rev
                .chars()
                .rev()
                .collect::<String>();
            println!("w_pig2 : {:?}",w_pig2);
            println!("{:?}", first_ch);

            //let str_to_add = "-".to_owned()+&first_ch+"ai";
            let str_to_add = format!("-{}ai", first_ch);

            w_pig2.push_str(&str_to_add);

        }
        println!("w_pig2 before result : {:?}",w_pig2);

        pig_latin_result= format!("{} {}", pig_latin_result,w_pig2);
        println!("pig latin result partial {} ",pig_latin_result);
        
    }
    
    println!("");
    println!("pangram   : {}", pangram);
    println!("pig latin : {}",pig_latin_result);

}
