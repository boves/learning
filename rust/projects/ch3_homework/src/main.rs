// 1. Convert temperatures between Fahrenheit and Celsius.
// 2. Generate the nth Fibonacci number.
// 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


fn main() {
    
    // TODO: collect input for scale and temp
    let scale = convert_temp('c'); //, 20);
}

fn convert_temp (scale: char){//, temp: f64) -> char { // &str {
   

    if scale == 'c'{
        println!("Scale is Celsius");
    }

     else{
         println!("The scale is Fahrenheit.");
     }

}
