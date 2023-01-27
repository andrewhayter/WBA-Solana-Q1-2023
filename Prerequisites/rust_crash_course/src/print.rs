pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting and interpolation
    println!("{} is from {}", "Andrew", "Vancouver");

    // POsitional Arguments
    println!("{0} is from {1} and {0} likes {2}", "Andrew", "Vancouver", "code");

    // Named Arguments 
    println!("{name} likes to play {activity}", name = "Andrew", activity = "golf");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traint
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
