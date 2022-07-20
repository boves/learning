use std::collections::HashMap;
use std:io:stdin;

fn main() { 
    // 1. Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode (the value
    // that occurs most often; a hash map will be helpful here) of the list.

    
    let list = vec![60, 60, 80, 90, 10, 20, 30, 40, 50];

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
    let t1 = String::from("first");
    let p1 = to_piglatin(&t1);

    assert_eq!(String::from("irst-fay"), p1);
    let t2 = String::from("apple");
    let p2 = to_piglatin(&t2);
    assert_eq!(String::from("apple-hay"), p2);
    let t3 = String::from("first apple");
    let p3 = to_piglatin(&t3);
    assert_eq!(String::from("irst-fay apple-hay"), p3);

    println!("{}", p1);
    println!("{}", p2);
    println!("{}", p3);

    let tx = String::from("Pig Latin");
    let px = to_piglatin(&tx);

    println!("The pig-latin for {:?} is {:?}",tx, px);

   println!("\n\n"); 

    // 3. Using a hash map and vectors, create a text interface to allow a
    // user to add employee names to a department in a company. For
    // example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
    // the user retrieve a list of all people in a department or all people
    // in the company by department, sorted alphabetically.

    println!("EMPLOYEE ROSTER INTERFACE");
    println!("What do you want to do? ")
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line.");

    


}

// 1. Calculating average, median, and mode
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

// 2. Pig Latin
fn to_piglatin(text: &String) -> String {
    let vowels = vec!['a', 'o', 'u', 'e', 'i', 'y'];

    let words = text.split_whitespace();
    let mut output = String::new();
    for (i, word) in words.enumerate() {
        let first_char = word.chars().next().unwrap();
        if vowels.contains(&first_char){
            let pig_word = format!("{}-{}", word, "hay");
            if i == 0 {
                output = pig_word;
            } else {
                output = format!("{} {}", output, pig_word);
            } 
        } else {
            let pig_word = format!("{}-{}", &word[1..], format!("{}{}", &word[..1], "ay"));
            if i == 0 {
                output = pig_word;
            } else {
                output = format!("{} {}", output, pig_word);
            }
        }
    }
    output
}

// 3. Employee
