use std::collections::HashMap;

fn main() { 
    // 1. Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode (the value
    // that occurs most often; a hash map will be helpful here) of the list.

    
    let mut list = vec![60, 60, 80, 90, 10, 20, 30, 40, 50];

    let avg = calculate_average(&list);
    let median = calculate_median(&list);
    let mode = calculate_mode(&list);


    println!("Fam's in from Ireland and I'm lazy, so this is all I get today.");

    println!("Solution to problem 1");
    
    println!("avg is: {:?}", avg); 
    println!("median is: {:?}", median); 
    println!("mode is: {:?}", mode); 


    // 2. Convert strings to pig latin. The first consonant of each word is
    // moved to the end of the word and “ay” is added, so “first”
    // becomes “irst-fay.” Words that start with a vowel have “hay” added to
    // the end instead (“apple” becomes “apple-hay”). Keep in mind the
    // details about UTF-8 encoding!

    

    // 3. Using a hash map and vectors, create a text interface to allow a
    // user to add employee names to a department in a company. For
    // example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
    // the user retrieve a list of all people in a department or all people
    // in the company by department, sorted alphabetically.

}


fn calculate_average(nums: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for x in nums {
        sum += x;
    }

    sum as f32 / nums.len() as f32
}

fn calculate_median(nums: &Vec<i32>) -> f32 {
    let mut sorted = nums.clone();
    sorted.sort();

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        let x = sorted[mid] as f32;
        let y = sorted[mid-1] as f32;
        return (x + y) * 0.5;
    }

    sorted[mid] as f32
}

fn calculate_mode(nums: &Vec<i32>) -> i32 {
    let mut times = HashMap::new();
    for x in nums {
        let count = times.entry(x).or_insert(0);
        *count += 1
    }

    let mut best_key = nums[0]; {

    }
    let mut best_val = times.get(&best_key).unwrap();

    for (key, value) in &times {
        if value > best_val {
            best_key = **key;
            best_val = value;
        }
    }
    best_key
}