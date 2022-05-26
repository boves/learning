fn main() {


    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // this will print 'hello, world!' 

    let x = 5;
    let y = x;

    println!("x is {}, and y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 is {}, s2 is {}", s1, s2);

    takes_ownership(s); // s's values move into function
                        // ... and is no longer valid here

    let x = 5;          // x comes into scope

    makes_copy(x);      // x would move into the function
                        // but i32 is a copy, so it's ok to still
                        // use x afterwards


    // Return Values and Scope
    println!("Return values and scope");

    let s1 = gives_ownership();

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // Returning ownership of parameters
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    
    let len = calculate_length_reference(&s2);
    println!("The length of '{}' is {}.", s2, len);
    
    let mut s_change = String::from("hello");
    change(&mut s_change);

    println!("Time for bed bitches.");

    println!("S_change is: {}.", s_change);

    let mut s_multiple_refs = String::from("hello");
   
    // Multiple but non-simultaneous references to same value
    {
        let r1 = &mut s_multiple_refs;
    }   // r1 goes out of scope here, so we can make a new ref
    
    let r2 = &mut s_multiple_refs;

    //  println!("{}, {}", r1, r2);

    let mut big_problem = String::from("hello");

    let r10 = &big_problem;
    let r20 = &big_problem;

    println!("{} and {}", r10, r20);
    let r30 = &mut big_problem;

    println!("{}", r30); // no problem!
    
    // let reference_to_nothing = dangle();

    // The slice type
     
    println!("THE SLICE TYPE");
    
    let mut stronk = String::from("hello world");

    let word = first_words(&stronk); // word will get the value 5
    println!("Word is: {},", word);
    stronk.clear();

    println!("Word is: {}", word); 

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Spliced back together: {} {}", hello, world);

    // I can't be bothered to code today
    let day = "today";
    println!("I'm too tired to code, but I must commit {}.", day);

    // Had a procedure today
    let excuse = "procedure";

    println!("I didn't code today because I had a {}, but I'm still making a commit.", excuse);
    

} // Here, x goes out of scope, then s. But because s's value 
  // was moved, nothing special happens
 
 fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
 } // Here, some_string goes out of scope and `drop` is called.
   // The backing memory is freed.
  
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {

    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) ->
String {
    a_string
}

fn calculate_length(s: String) ->(String, usize){
    let length = s.len();
    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
     some_string.push_str(", world"); 
}

// fn dangle() -> String {
    // let s = String::from("hello");

    // &s // this won't work; it's a dangling reference

    // s // this works!
// }

fn first_words(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
