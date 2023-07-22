use std::io;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_input_i32(prompt: &str) -> i32 {
    loop {
        println!("+ - {} - +", prompt);
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse() {
                Ok(num) => break num,
                Err(_) => println!("Please enter a valid number!"),
            },
            Err(_) => println!("Error reading input!"),
        }
    }
}
