use simple_add_crate::simple_add::add_two;

fn run_simple_add_crate_example() {
    println!("Attempting to use the homemade simple_add_crate from it's own repository.");
    let a = 5;
    let b = 10;
    let result = add_two(a, b);
    println!("The result of adding {} and {} is: {}", a, b, result);
}

fn main() {
    println!("START");
    println!();

    run_simple_add_crate_example();

    println!();
    println!("END");
}
