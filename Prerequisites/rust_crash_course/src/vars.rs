// Variables hold primative data or references to data
// Variables are immutable by default
// Rust is a block-scoped langauage. 

pub fn run() {
    let name = "Andrew";
    let mut age = 38;

    println!("My name is {} and I am {}", name, age);


    age = 39;

    println!("My name is {} and I am {}", name, age);


    // Define Constant

    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple vars 

    let (my_name, my_age) = ("Andrew", "38");

    println!("{} is {}", my_name, my_age);
}