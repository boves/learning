use std::collections::HashMap;

fn main() { 
    // 1. Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode (the value
    // that occurs most often; a hash map will be helpful here) of the list.


    let numbers = vec![10, 12, 24, 36, 42, 35, 46, 60, 19, 10, 12];
    let avg = calculate_average(&numbers);
    let median = calculate_median(&numbers);
    let mode = calculate_mode(&numbers);

    println!("median: {}", median); 
    println!("average: {}", avg);
    println!("mode: {}", mode);
    


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

fn calculate_median(numbers: &Vec<i32>) -> f32 {
    let mut sorted = numbers.clone();
    sorted.sort();

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        let x = sorted[mid] as f32;
        let y = sorted[mid -1] as f32;
        return (x + y) * 0.5;
    }
    sorted[mid] as f32

}

fn calculate_mode(numbers: &Vec<i32>) -> i32 {
    let mut times = HashMap::new();
    for x in numbers {
        let count = times.entry(x).or_insert(0);
        *count += 1;
    }

    let mut best_key = numbers[0]; 
    let mut best_val = times.get(&best_key).unwrap();

    for (key, value) in &times {
        if value > best_val {
            best_key = **key;
            best_val = value;
        }
    }
    best_key
}

fn calculate_average(numbers: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for x in numbers {
        sum += x;
    }

    sum as f32 / numbers.len() as f32
}

