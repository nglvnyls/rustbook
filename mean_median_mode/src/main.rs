use std::collections::HashMap;

// A struct with two fields
#[derive(Debug)]
struct Max {
    integer: u32,
    number_of_repetions: u32,
}

fn main() {
    println!("82.1 -mean_median_mode");
    println!("Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list."); 
    println!("");
    println!("list of integers given");
    println!("");
    //let int_list: Vec<i32> = Vec::new();
    let int_list = vec![9,1,2,9,5,3,2,8,8,8,2,2,3,3,4,4,5,5,5,5,5,5,6,7,8,8,9];
    println!("int_list = {:?}", int_list);
    println!("");

    //the mean (the average value)
    let mut sum: u32 = 0;
    let mut average: f32 = 0.00;
    for (i,value) in int_list.iter().enumerate() {
        sum += value;
        println!("i = {} and value = {}, total= {}", i, value, sum);
        average = sum as f32 / (i as f32 + 1.00);
    }
    println!("");
    println!("the mean of integer list is = {}", average);
    println!("");

    //median (when sorted, the value in the middle position), 
    println!("MEDIAN. Calculating -----------------------");
    println!("");
    let mut sorted_int_list = int_list.clone();
    sorted_int_list.sort();
    println!("sorted_int_list : {:?}", sorted_int_list);
    //sorted_int_list = sorted_int_list.iter().sort();

    
    let half_position =  int_list.clone().iter().count() / 2 + 1;
    println!("half index is : {:?}", half_position);

    println!("");
    println!("the median of integer list is = {}", sorted_int_list[half_position]);
    println!("");

    //mode  (the value that occurs most often)
    println!("MODE. Calculating -----------------------");
    let mut map = HashMap::new();    

    for value in int_list.iter() {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    println!("map now is: {:?} ", &map);

    let mut max = Max { integer: 0, number_of_repetions: 0 };

    for (index , v) in map {
        println!("{:?}: {:?}", index, v);
        if v > max.number_of_repetions {
            max.integer  = index.clone();
            max.number_of_repetions  = v;
            println!("max now is: {:#?} ", max);
        }
    }

    println!("mode is integer number: {}",max.integer);
    

}
