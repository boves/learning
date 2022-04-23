fn main() {
    
    // variables are immutable by default 
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // constants can only be computed using constant 
    // expresssions
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing variables
    println!("Shadowing variables\n");

    let y = 5;
    
    println!("The initial value of Y is: {}", y);

    let y = y +1;

    println!("The updated value of Y is: {}", y);

    {
        // The inner scope is a new shadow that doesn't
        // extend outside of here

        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    // The difference between mutable and immutable
    // let spaces = "   ";
    // let spaces = spaces.len();
}
