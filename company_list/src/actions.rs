use std::{collections::HashMap, io};

#[derive(Debug)]
pub struct Employee {
    first_name: String,
    last_name: String,
    department: String,
}

impl std::fmt::Display for Employee {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "Employee name is {} {}, in the {} department",
            self.first_name.trim(), self.last_name.trim(), self.department.trim()
        )
    }
}

// Reads input from command line and writes it to a given mutable string variable
pub fn read_string(name: &mut String) -> String {
    io::stdin().read_line(name).expect("Failed to read line");
    let name = name.trim().to_string();
    name
}

pub fn create_department_prompt(list: &mut HashMap<String, Vec<Employee>>) {
    let dept_employees: Vec<Employee> = Vec::new();
    println!("Input department name: ");
    let mut dept_name: String = String::new();
    read_string(&mut dept_name);

    create_department(list, dept_employees, dept_name);
}

fn create_department(
    list: &mut HashMap<String, Vec<Employee>>,
    employees: Vec<Employee>,
    dept_name: String,
) {
    list.insert(dept_name, employees);
}

pub fn add_employee_prompt(list: &mut HashMap<String, Vec<Employee>>) {
    let mut first_name = String::new();
    println!("Input employee's first name: ");
    read_string(&mut first_name);

    let mut last_name = String::new();
    println!("Input employee's last name: ");
    read_string(&mut last_name);

    let mut employee_dept = String::new();
    println!("Input employee's department: ");
    read_string(&mut employee_dept);

    match list.contains_key(&employee_dept) {
        true => {
            list
                .get_mut(&employee_dept)
                .unwrap()
                .push(add_employee(first_name, last_name, employee_dept));
        },
        false => {
            println!("Department does not exist");
        }, 
    }
}

fn add_employee(first_name: String, last_name: String, department: String) -> Employee {
    Employee {
        first_name,
        last_name,
        department,
    }
}

pub fn get_employees(list: &mut HashMap<String, Vec<Employee>>) {
    for department in list {
        for i in 0..department.1.len() {
            print!("{}\n", department.1[i]);
        }
    }
}

pub fn get_departments(list: &mut HashMap<String, Vec<Employee>>) {
    for i in list.keys() {
        println!("{}", i.trim());
    }
}

pub fn get_employees_in_department(list: &mut HashMap<String, Vec<Employee>>) {
    let mut dept = String::new();
    println!("Which department would you like to list: "); 
    read_string(&mut dept); 

    if list.contains_key(&dept) {
        for t in list.get(&dept).unwrap() {
            let first_name = &t.first_name.trim();
            let last_name = &t.last_name.trim();
            println!("{first_name} {last_name}");
        }
    }
}
