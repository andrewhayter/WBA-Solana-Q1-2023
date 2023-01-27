pub fn run() {
    greeting("Hello", "Andrew");

    // Bind function values to vars
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closures
    let n3 = 10;
    let add_nums = |n1, n2| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3))
}

// basic print func
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!!", greet, name)
}

// return from functions
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}