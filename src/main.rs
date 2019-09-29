use std::io;

fn main() {
    println!("Enter a number");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to load");
    
    println!("Your guess is {}", number);
}
