use std::io::stdin;
fn main() {
    loop {
        println!("Do  you wish to continue running the program? (y/n:): ");
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.trim().to_lowercase() == "y" {
            //place more shitty code here
            println!("The loop is functional");
        } else if user_input.trim().to_lowercase() == "n" {
            break;
        }
    }
}
