use std::io;

fn main() {
    loop {
        println!("Enter temperature in Celsius to convert to Fehrenheit:");
        let input: i32 = read_input();
        println!("You entered: {input}");
        let result: i32 = input * 9 / 5 + 32;
        println!("{input} degrees Celsius is {result} degrees in Fahrenheit")
    }
}

fn read_input() -> i32 {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Not a number");

    input
}
