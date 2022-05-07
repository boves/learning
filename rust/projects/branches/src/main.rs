fn main() {
    let number = 7;

    if number  < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = 3;

    if number2 != 0{
        println!("number2 was something other than zero");
    }
    
    // multiple if statements in a row using else
    let number3 = 6;
    
    if number3 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number3 % 3 == 0 {
        println!("number3 is divisible by 3");
    } else if number3 % 2 == 0{
        println!("number3 is divisible by 2");
    } else {
        println!("number3 is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition { 5 }  else { 6 };

    println!("The value of number is: {}", number);  
}
