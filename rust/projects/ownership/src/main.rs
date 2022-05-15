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
}                   
