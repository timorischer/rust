use std::collections::HashMap;

use actions::{Employee, read_string};
mod actions;

fn main() {
    let mut list: HashMap<String, Vec<Employee>> = HashMap::new();

    loop {
        println!("Please choose your option: ");
        println!("1: Create new department");
        println!("2: Add new employee to department");
        println!("3: Move employee to another department");
        println!("4: Retreive all employees");
        println!("5: Retreive all departments");
        println!("6: Retreive all employees in department");
        println!("0: Exit application");

        let mut input: String = String::new();
        read_string(&mut input);

        let cmd = input.trim();
        match cmd.parse::<u8>() {
            Ok(1) => actions::create_department_prompt(&mut list),
            Ok(2) => actions::add_employee_prompt(&mut list),
            //Ok(3) => actions::move_employee(&mut list),
            Ok(4) => actions::get_employees(&mut list),
            Ok(5) => actions::get_departments(&mut list),
            Ok(6) => actions::get_employees_in_department(&mut list),
            Ok(0) => break,
            Ok(i) => println!("{i} is not a valid option"),
            Err(_) => println!("Please enter valid input"),
        }
    }
}
