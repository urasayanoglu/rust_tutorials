fn main() {
    
    // Scalar types - integer, float, boolean, character

    let int_value: i16 = 1213;
    let float_value: f32 = 12.13;

    println!("The int_value variable has a value of {}.", int_value);
    println!("The float_value variable has a value of {}.", float_value);

    // Boolen types

    let bool_value: bool = true;
    println!("The bool_value variable has a value of {}.", bool_value);

    let another_bool_value: bool = false;
    println!("The another_bool_value variable has a value of {}.", another_bool_value);

    // Character types
    let char_value: char = 'A'; // char values are enclosed in single quotes.
    println!("The char_value variable has a value of {}.", char_value);

    // Compound types - tuple, array
    let tuple_value: (i32, f64, char) = (42, 3.14, 'Z');
    println!("The tuple_value variable has a value of {:?}.", tuple_value);
    let (x, y, z) = tuple_value;
    println!("The tuple_value variable has a value of {}.", x);
    println!("The tuple_value variable has a value of {}.", y);
    println!("The tuple_value variable has a value of {}.", z);

    // Accessing tuple values can also be done using the dot notation
    println!("The tuple_value variable has a value of {}.", tuple_value.0);
    println!("The tuple_value variable has a value of {}.", tuple_value.1);
    println!("The tuple_value variable has a value of {}.", tuple_value.2);

    // Array Types
    let array_value: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array_value variable has a value of {:?}.", array_value);
    println!("The array_value variable has a value of {}.", array_value[0]);

    let months: [&str; 12] = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    println!("The months variable has a value of {:?}.", months);
    println!("The months variable has a value of {}.", months[0]);

    

}
