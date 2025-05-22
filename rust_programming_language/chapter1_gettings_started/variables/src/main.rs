fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    // x = 6; cannot assign twice to immutable variable
    // println!("The value of x is {}", x);
    let x = 6; // shadowing
    println!("The value of x is {}", x);

    let mut y = 5;
    println!("The value of y is {}", y);
    y = 7; // mutable variable
    println!("The value of y is now {}", y);

    let z: i32 = 14;
    println!("The value of z is {z}"); // earlier rust version does not support this syntax

    const SECONDS_IN_HOUR: u32 = 60 * 60; // cannot be mutable by using mut, also needs to have type annotation
    println!("There are {} seconds in an hour", SECONDS_IN_HOUR);

    // Shadowing can be done in the same scope or in a different scope

    let q: u8 = 13;
    println!("The value of q is {}", q);
    {
        let q: u8 = 14;
        println!("The value of q in the inner scope is {}", q);
    }
    let q: u8 = 15;
    println!("The value of q after the inner scope is {}", q);

    let t: u8 = 16;
    let t: &str = "17"; // shadowing with different type
    println!("The value of t is {}", t); 

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is {}", spaces);

    let mut spaces = "   ";
    // spaces = spaces.len(); cannot change the type of the mutable variable 
    println!("The length of spaces is {}", spaces.len());
}
