fn main() {
    println!("**************************************************");
    println!("\n\t\tdata_types/main.rs\n\n\n");

    println!("\t\tFloating-Point Numbers");
    let x = 2.0; // f64
    println!("\tx is: {}",x);

    let y: f32 = 3.0; // f32
    println!("\ty is: {}", y);
    println!("\n");

    println!("\t\tNumeric Operations");
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // results in 0

    // remainder
    let remainder = 43 % 5;

    println!("\tdifference is: {}", difference);
    println!("\tproduct is: {}", product);
    println!("\tquotient is: {}", quotient);
    println!("\tfloored is: {}", floored);
    println!("\tremainder is: {}", remainder);

    println!("\n\n");
    println!("\t\tThe Boolean Type\n");
    
    let t = true; // implicit type annotation
    let f: bool = false; // with explicit type annotation
    println!("Implicitly, 't' is: {}", t);
    println!("Explicitly, 'f' is: {}", f);
    println!("\n");

    println!("\t\tThe Character Type");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("\tc is: {}", c); 
    println!("\tz is: {}", z); 
    println!("\theart_eyed_cat is: {}", heart_eyed_cat); 

    // End of output
    println!("\n\n");
    println!("**************************************************");
}