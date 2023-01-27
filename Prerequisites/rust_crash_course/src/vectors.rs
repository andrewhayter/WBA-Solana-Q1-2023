// Vector - resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Reassign Value
    numbers[2] = 20;

    // Add on to Vector
    numbers.push(5);
    numbers.push(6);

    numbers.pop();

    println!("{:?}", numbers);

    // Get Single Val
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slices from Array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    // Loop & Mutate Values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers)
}