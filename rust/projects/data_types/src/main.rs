use std::io;

fn main() {
    // let asterisks =  "**************************************************";
    let asterisks =  "********************************************";
                                                    // ************
    println!("{}{}", asterisks, asterisks);
    println!("\n\t\t\tdata_types/main.rs\n");

    println!("\t\t\tFloating-Point Numbers");
    let x = 2.0; // f64
    println!("\t\tx is: {}",x);

    let y: f32 = 3.0; // f32
    println!("\t\ty is: {}", y);
    println!("\n");

    println!("\t\t\tNumeric Operations");
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // results in 0

    // remainder
    let remainder = 43 % 5;

    println!("\t\tdifference is: {}", difference);
    println!("\t\tproduct is: {}", product);
    println!("\t\tquotient is: {}", quotient);
    println!("\t\tfloored is: {}", floored);
    println!("\t\tremainder is: {}", remainder);

    println!("\n");
    println!("\t\t\tThe Boolean Type\n");
    
    let t = true; // implicit type annotation
    let f: bool = false; // with explicit type annotation
    println!("\t\tImplicitly, 't' is: {}", t);
    println!("\t\tExplicitly, 'f' is: {}", f);
    println!("\n");

    println!("\t\t\tThe Character Type");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("\t\tc is: {}", c); 
    println!("\t\tz is: {}", z); 
    println!("\t\theart_eyed_cat is: {}\n", heart_eyed_cat); 
    
    println!("\t\t\tCOMPOUND TYPES");
    println!("\t\t\t    tuples");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("\t\ttup is: {:?}", tup);
    let (x, y, z) = tup;
    
    println!("\t\t - The value of x is: {}", x);
    println!("\t\t - The value of y is: {}", y);
    println!("\t\t - The value of z is: {}", z);


    println!("\n\t\ttuples direct assignment");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("\t\t - Five hundred is: {}", five_hundred);
    println!("\t\t - Six point four is: {}", six_point_four);
    println!("\t\t - One is: {}", one);
    
    println!("\n\t\t\tARRAYS");
    let a = [3; 5];
    println!("\t\t - The array \"a\" is: {:?}", a);
    let month = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("\t\tThe months are: {:?}", month);

    // accessing arrays [in progress]
    println!("\n\t\t\t accessing items of an array");

    let a2 = [1, 2, 3, 4, 5];
    let first = a2[0];
    let second = a2[1];
    println!("\t\tFirst is: {} and second is {}", first, second);
    println!("\t\tPlease enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("\t\tFailed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("\t\tIndex entered was not a numbrer");

    let element = a2[index];

    println!(
        "\t\tThe value of an element at index {} is: {}",
        index, element
    );


    // End of output
    println!("\n\n");
    println!("{}", asterisks);
}
