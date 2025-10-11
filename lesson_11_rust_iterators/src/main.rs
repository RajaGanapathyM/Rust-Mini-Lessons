use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Employee {
    name: String,
    department: String,
    salary: u32,
}

fn parse_csv(file_path: &str) -> Result<Vec<Employee>, io::Error> {
    let csv_file = File::open(file_path)?;
    let reader = io::BufReader::new(csv_file);

    let empoyees: Vec<Employee> = reader
        .lines()
        .filter_map(|line| match line {
            Ok(line_str) => {
                let line_str = line_str.trim();
                if line_str.is_empty() || line_str.starts_with("name,department,salary") {
                    None
                } else {
                    Some(line_str.to_string())
                }
            }
            Err(_) => None,
        })
        .filter_map(|line_str| {
            let parts: Vec<_>= line_str.split(',').collect();

            if parts.len() != 3 {
                None
            } else {
                let name = parts[0].to_string();
                let department = parts[1].to_string();
                let salary = parts[2].parse::<u32>().ok()?;
                Some(Employee {
                    name,
                    department,
                    salary,
                })
            }
        })
        .collect();

    Ok(empoyees)
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Command not in format: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let path = args.get(1).expect("No file path provided");
    let employees = parse_csv(path)?;

    let total_salary: u32 = employees.iter().map(|e| e.salary).sum();
    let employee_count = employees.len() as u32;
    let avg_salary = if employee_count > 0 {
        total_salary / employee_count
    } else {
        0
    };
    println!(
        "Total employees: {}, Average salary: {:.2}",
        employee_count, avg_salary
    );

    let department_sum: HashMap<String, (u32, u32)> =
        employees.iter().fold(HashMap::new(), |mut acc, emp| {
            let entry = acc.entry(emp.department.clone()).or_insert((0, 0));
            entry.0 += emp.salary;
            entry.1 += 1;
            acc
        });

    // Alternative using for loop for aggregation
    // let mut department_sum: HashMap<String, (u32, u32)> = HashMap::new();
    // for emp in &Employees{
    //     let entry=department_sum.entry(emp.department.clone()).or_insert((0,0));
    //     entry.0 += emp.salary;
    //     entry.1 += 1;
    // }

    for (dept, (total, count)) in &department_sum {
        println!("Department: {}, Total Salary: {}, Department Count: {}", dept, total,count);
    }
    Ok(())
}
