use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to the employee database text interface.\nHow can I help you?");
    
    let mut employees = HashMap::new();
    employees.insert(
        "name".to_string(),
        "department".to_string(),
    );

    loop{ 
        let mut command = String::new();
        println!("");
        print!("--------------------------------- ");
        println!("add/remove (name) to (department)");

        io::stdin().read_line(&mut command)
            .expect("Failed to read line.");

        println!("command: {}", &command);

        command = command.to_lowercase();
        println!("command minuscules: {}", command);
        
        let command_words = command.split_whitespace().collect::<Vec<&str>>();
        println!("words in command: {:?}", command_words);
        
        if command_words[0] == "add" || command_words[0] == "remove"{
            println!("add/remove: {:?}", command_words[0]);
        } else {
            println!("you don't say 'add' or 'remove'. Please try again!!");    
            continue
        }

        if command_words[0] == "add" {
            if command_words.len() == 1 {
                println!("you have forget the 'name'. Please try again!!");
                continue
            } else if command_words.len() == 2 {
                println!("you have forget the DEPARTMENT to ADD. Please try again!!");
                continue
            } else if command_words.len() == 3 {
                println!("you have forget the DEPARTMENT. Please try again!!");
                continue
            } else {
                println!("ADD --- you want to ADD this name: {:?} to department: {}", command_words[1],command_words[3]);
                println!("");
            }
        } else if command_words[0] == "remove" { 
            if command_words.len() == 1 {
                println!("you have forget the 'name'. Please try again!!");
                continue
            } else {
                println!("REMOVE --- you want to REMOVE this name: {:?}", command_words[1]);
                println!("");
            }
           
        }

        if command_words[0] == "add" {
            println!("you want to ADD : {} in departement: {}",command_words[1],command_words[3]);
            
            employees.insert(
                command_words[1].to_string(),
                command_words[3].to_string(),
            );
            
        } else if command_words[0] == "remove" {
            println!("REMOVE- you want to: {}",command_words[0]);
            // Look up the values associated with some keys.
            // Check for a specific one.
            // When collections store owned values (String), they can still be
            // queried using references (&str).
            
            if employees.contains_key(command_words[1]) {
                println!("We've got {} in employees, and it will be removed",command_words[1] );
                employees.remove(command_words[1]);        
            } else {
                println!("????????? Sorry this employee does not exist!!")
            }
            
        } 
        println!("");
        println!("employees: {:#?}", employees);

    }
}