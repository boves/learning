use std::collections::HashMap;

fn main() { 
    { // Part 1: Calculations

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
    }
    
    { // Part 2: Pig-Latin 

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
    }

    { // Part 3: Employee list
        // 3. Using a hash map and vectors, create a text interface to allow a
        // user to add employee names to a department in a company. For
        // example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
        // the user retrieve a list of all people in a department or all people
        // in the company by department, sorted alphabetically.

        let mut company: HashMap<String, Vec<String>> = HashMap::new();
        // handle_command(&command, &company);
        let (name, dep) = get_name_and_department("Add Sally to Engineering".to_string());
        company.entry(dep).or_insert(Vec::new()).push(name);

        let (name, dep) = get_name_and_department("Add Amir to Sales".to_string());
        company.entry(dep).or_insert(Vec::new()).push(name);

        let (name, dep) = get_name_and_department("Add Beate to Sales".to_string());
        company.entry(dep).or_insert(Vec::new()).push(name);

        let (name, dep) = get_name_and_department("Add Cecil to Marketing".to_string());
        company.entry(dep).or_insert(Vec::new()).push(name);

        let sorted = get_all_employees(&company);
        println!("SORTED: {:?}", sorted);

        println!("{:?}", company);
    }
}

// Part 1: calculations functions
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


// Part 2. Pig Latin functions
fn to_piglatin(text: &String) -> String {
    let vowels = vec!['a', 'o', 'u', 'e', 'i', 'y'];

    let words = text.split_whitespace();
    let mut output = String::new();
    for (i, word) in words.enumerate() {
        let first_char = word.chars().next().unwrap();
        if vowels.contains(&first_char) {
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


// Part 3. Employee list functions
fn get_all_employees(company: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut employees: Vec<String> = Vec::new();

    for (_, value) in company {
        employees = [employees, value.clone()].concat();
    }

    employees.sort();
    employees
}

fn get_name_and_department(s: String) -> (String, String) {
    let words = text_to_words(&s);
    return (String::from(words[1]), String::from(words[words.len() - 1]));
}

fn text_to_words(s: &String) -> Vec<&str> {
    let bytes = s.as_bytes();
    let num_bytes = bytes.len();
    let mut words = Vec::new();
    let mut last_whitespace = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            words.push(&s[last_whitespace..i]);
            last_whitespace = i + 1;
        } else if i == num_bytes - 1 {
            words.push(&s[last_whitespace..]);
        }
    }
    return words;
}
