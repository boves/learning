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

println!("Time for bed bitches.");
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
