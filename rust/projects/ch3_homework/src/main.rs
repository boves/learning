// 1. Convert temperatures between Fahrenheit and Celsius.
// 2. Generate the nth Fibonacci number.
// 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


fn main() {

    println!("");

    println!("CHAPTER 3 HOMEWORK"); 
    println!("1. Convert temperatures between F and C"); 
    convert_temp('c', 20); //, 20);
    convert_temp('f', 212);
    
}

fn convert_temp (scale: char, temp: i32) {// -> char { // &str {
   

    if scale == 'c'{
        println!(" Scale is Celsius");
        // println!("The temp is: {}°C", temp);
        let temperature = (temp * 9/5) + 32;
        println!("  -{}°C is: {}°F\n", temp,  temperature);
    }

     else{
         println!(" Scale is Fahrenheit.");
         let temperature = (temp - 32) * 5/9;
         println!("  -{}°F is: {}°C\n", temp, temperature); 
     }

}
