fn main() {
    println!("Generic Types");
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

    let result = largest2(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest2(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);

    /*
    In sum, here are the steps we took to change the code:
    - Identify duplicate code.
    - Extract the duplicate code into the body of the function and 
    - specify the inputs and return values of that code in the function signature.
    - Update the two instances of duplicated code to call the function instead.

    Next, we’ll use these same steps with generics to reduce code duplication in different ways.
    Neither only with different i32 values list,nor with different types of values. 
    */


}

/*
To eliminate this duplication, we can create an abstraction by defining a function 
that operates on any list of integers given to it in a parameter.
*/

fn largest2(list: &[i32]) -> i32 { //function has a parameter called list, 
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

