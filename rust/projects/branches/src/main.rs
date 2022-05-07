fn main() {
    println!("If/Else Statements");
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
    println!("\nHandling Multiple Conditions with Else/If");
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
    println!("\nUsing If in a Let Statement");
    let condition = true;
    let number = if condition { 5 }  else { 6 };

    println!("The value of number is: {}", number);  

    // Repeating Code with loop
    println!("\nRepeating Code with Loops");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    
    println!("\nReturning Values from Loops"); 
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);
    
    println!("Repetition in Loops");    
    println!("\n COUNTDOWN");
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -=1;
    }

    println!("LIFTOFF!!!");

    println!("\nLooping Through a Collection with For");
    println!("First: looping through a collection with while");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!(" - the value is: {}", a[index]);

        index += 1;
    }

    println!("\nSecond: looping through a collection with for");

    for element in a {
        println!(" - the value is: {}", element);
    }

    println!("\nThird: looping through a range using .rev()");
    for number in (1..4).rev() {
        println!(" - {}!", number);
    }
    println!(" - LIFTOFF!!!");
}
