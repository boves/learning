// 1. Convert temperatures between Fahrenheit and Celsius.
// 2. Generate the nth Fibonacci number.
// 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


fn main() {

    let temp_scale = convert_temp('c'); //, 20);
    let scale = 'c';

    if scale == 'c' {
        println!("Celsius");
    }
}

fn convert_temp (scale: char){//, temp: f64) -> char { // &str {
    
    if scale == 'c'{
        println!("Scale is Celsius");
        // let celsius =
        // return scale;
    }
    else{
         println!("The scale is Fahrenheit.");
         return scale;
    }
}
