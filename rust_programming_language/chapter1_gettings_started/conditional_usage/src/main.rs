fn main() {

    let number: u8 = 3;
    if number < 5 {
        println!("The number is less than 5.");
    } else {
        println!("The number is 5 or greater.");
    }

    let another_number: u8 = 16;

    if another_number % 4 == 0 {
        println!("The number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3.");
    } else {
        println!("The number is not divisible by 4 or 3.");
    }

    // if statements can be used in a let statement
    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    /*   
    let another_condition: bool = true;
    let another_number = if another_condition { 10 } else { "Another number!" };
    println!("The value of another_number is: {}", another_number);

    It panics because the types of the branches are different.
    */


}
