fn main() {
    // full notation for Vec<T> (vector) collection
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3,];

    let mut v3 = Vec::new(); 

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("{:?}", v3);

    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v5 = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v5[100];
    let does_not_exist = v5.get(100);

    let mut v6 = vec![1, 2, 3, 4, 5];
    let first = &v6[0];

    v6.push(6);

    // println!("The first element is: {}", first);
    println!("The first element is: {:?}", &v6[0]);

    // iterating over the values in a Vector
    println!("\nIterating Over a Vector");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("-{}", i);

    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("--{:?}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }  

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Loading data onto empty strings
    let mut s = String::new(); // empty string
    let data = "initial contents"; // string slice

    let s = data.to_string(); // 

    // this method also works on literals directly
    let s = "initial contents".to_string();

    // Strings are UTF-8 encoded, so can include the full character set
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // appending to strings
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{:?}", s);

    // do stuff with  v2
} // <- v2 goes out of scope and is freed here