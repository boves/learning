fn main() {
    // Formatting
    
    println!("**************************************************");
    println!("\n\t\tvariables/main.rs\n\n\n");
    
    // variables are immutable by default 
    println!("\t\tTypes of numbers in rust\n");
    let mut x = 5;
    println!("\tThe value of x is: {}", x);
    x = 6;
    println!("\tThe value of x is: {}", x);
    
    // constants can only be computed using constant 
    // expresssions
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("\n\n");
    // shadowing variables
    println!("\t\tShadowing variables\n");

    let y = 5;
    
    println!("\tThe initial value of Y is: {}", y);

    let y = y +1;

    println!("\tThe updated value of Y is: {}", y);

    {
        // The inner scope is a new shadow that doesn't
        // extend outside of here

        let y = y * 2;
        println!("\tThe value of y in the inner scope is: {}", y);
    }

    println!("\tThe value of y is: {}", y);
    println!("\n\n");
    println!("\t\tData Types");    



    // The difference between mutable and immutable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("\tthe length of spaces is: {}", spaces);




    // End of output
    println!("\n\n\n");
    println!("**************************************************");
}