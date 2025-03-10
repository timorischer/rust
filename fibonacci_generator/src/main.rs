use std::io;

fn main() {
    println!("Input desired fibonacci index");
    let index: u32 = read_input();
    let result: u32 = fibonacci(index);
    println!("Fibonacci number {index} is {result}");
}

fn read_input() -> u32 {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Not a number");

    input
}

fn fibonacci(index: u32) -> u32 {
    let mut first: u32 = 0;
    let mut second: u32 = 1;
    let mut current;
    let mut i: u32 = 3;

    if index == 0 {
        0
    } else if index == 1 {
        first
    } else if index == 2 {
        second
    } else {
        loop {
            current = first + second;
            if i == index {
                break
            }
            first = second;
            second = current;
            i = i+1;
        }
        current
    }
}