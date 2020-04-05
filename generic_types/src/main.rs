fn main() {
    println!("Generic Data Types in function definitions");
    println!("...contains a copy of Generic types and adds more code");
    println!("");

    /*
    Remove duplication that doesn’t involve generic types 
    by extracting a function

    Consider a short program that finds the largest number in a list
    */

    let number_list = vec![34, 50, 25, 100, 65]; //stores a list of integers in the variable number_list

    let mut largest = number_list[0];// places the first number in the list in a variable named largest

    for number in number_list { // it iterates through all the numbers in the list
        if number > largest { // if the current number is greater than the number stored in largest, 
                              //it replaces the number in that variable
            largest = number;
        }
    }

    /*
    After considering all the numbers in the list, largest should hold the largest number, 
    which in this case is 100.
    */

    println!("The largest number is {}", largest);

    /*
    To find the largest number in two different lists of numbers, 
    we can duplicate the code before and use the same logic at 
    two different places in the program.
    */

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    /*
    Although this code works, duplicating code is tedious and error prone. 
    We also have to update the code in multiple places when we want to change it.
    */

    /*
    Unlike the code before, which can find the largest number in only one particular list, 
    this program can find the largest number in two different lists.
    */
    
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);

    /*
    In sum, here are the steps we took to change the code:
    - Identify duplicate code.
    - Extract the duplicate code into the body of the function and 
    - specify the inputs and return values of that code in the function signature.
    - Update the two instances of duplicated code to call the function instead.
    
    It can be done for another type like char with another function "largest_char"
    */
    
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    /*
    Next, we’ll use these same steps with generics to reduce code duplication in different ways.
    Neither only with different i32 values list,nor with different types of values. 
    */



}

/*
To eliminate this duplication, we can create an abstraction by defining a function 
that operates on any list of integers given to it in a parameter.
*/

fn largest_i32(list: &[i32]) -> i32 { //function has a parameter called list, 
                                //which represents any concrete slice of i32 values 
                                //that we might pass into the function.
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
/*
The largest_char function finds the largest char in a slice
*/

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
The largest_i32 function and the largest_char have the same code, 
so let’s eliminate the duplication by introducing a 
generic type parameter in a single function.
*/

fn largest<T>(list: &[T]) -> T { //We read this definition as: 
            //the function largest is generic over some type T. 
            //This function has one parameter named list, 
            //which is a slice of values of type T. The largest 
            //function will return a value of the same type T.
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
 the body of largest won’t work for all possible types that T could be. 
 Because we want to compare values of type T in the body, we can 
 only use types whose values can be ordered. To enable comparisons, 
 the standard library has the std::cmp::PartialOrd trait 
 that you can implement on types.
*/




