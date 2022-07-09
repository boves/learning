fn main() { 
    // 1. Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode (the value
    // that occurs most often; a hash map will be helpful here) of the list.

    use std::collections::HashMap;
    let mut list = vec![60, 70, 80, 90, 10, 20, 30, 40, 50];

    // sort the vector
    list.sort();
    assert_eq!(list, vec![10, 20, 30, 40, 50, 60, 70, 80, 90]);

    // count the vector
    let count = list.len();


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
