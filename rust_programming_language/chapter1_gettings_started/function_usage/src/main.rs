fn main() {
    println!("println! is a macro, not a function");

    my_useless_function();

    funtion_call_in_another_function();

    parameter_funtion(42);

    print_labeled_measurement(5, 'h');

    println!("Are of a circle with radius 5.0 is {:.2}", area_circle(5.0));

}

// Functions can be defined before or after the main function

fn my_useless_function() {
    println!("This function does nothing");
}

//  Functions can be called inside other functions
fn funtion_call_in_another_function() {
    println!("This function calls another function");
    my_useless_function();
}

// Paramaters can be passed to functions
// Multiple parameters are separated by commas
fn parameter_funtion(param: i32) {
    println!("This function takes a parameter: {}", param);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions with return values

fn area_circle(radius: f64) -> f64 {
    std::f64::consts::PI * radius *radius
}