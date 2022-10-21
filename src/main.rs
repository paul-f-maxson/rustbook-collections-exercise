// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use regex::{Regex, RegexSet};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, Write};

struct Department {
    employees: Vec<String>,
}

impl Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for employee_name in self.employees.iter() {
            writeln!(f, "{employee_name}")?;
        }
        Ok(())
    }
}

struct EmployeeDB {
    db: HashMap<String, Department>,
}

impl Display for EmployeeDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (department_name, department) in self.db.iter() {
            writeln!(f, "{department_name}:")?;
            write!(f, "{department}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum EmployeeDBAction {
    InsertEmployee {
        new_employee_name: String,
        department_name: String,
    },
    ListDepartment {
        department_name: String,
    },
    ListAll,
}

impl EmployeeDB {
    fn new() -> Self {
        Self { db: HashMap::new() }
    }

    // Add the employee to the department in the correct order
    fn insert_employee(&mut self, department_name: String, new_employee_name: String) {
        match self.db.get_mut(&department_name) {
            // The department already exists
            Some(department) => {
                // Find insertion point
                let insertion_index =
                    department
                        .employees
                        .partition_point(|existing_employee_name| {
                            existing_employee_name < &new_employee_name
                        });

                // Insert name in order
                department
                    .employees
                    .insert(insertion_index, new_employee_name)
            }
            // Department does not exist
            None => {
                // Create new department and add employee to it
                self.db.insert(
                    department_name,
                    Department {
                        employees: [new_employee_name].into(),
                    },
                );
            }
        };
    }

    // Retrieval functions

    fn send(&mut self, action: EmployeeDBAction) {
        use EmployeeDBAction::{InsertEmployee, ListAll, ListDepartment};

        match action {
            InsertEmployee {
                new_employee_name,
                department_name,
            } => self.insert_employee(department_name, new_employee_name),

            ListAll => println!("{}", self),

            ListDepartment { department_name } => {
                if let Some(department) = self.db.get(&department_name) {
                    println!("{}", department);
                } else {
                    println!("no department by that name");
                }
            }
        }
    }
}

struct InputParser<'a> {
    capture_names: Vec<&'a str>,
    set: RegexSet,
    regexes: Vec<Regex>,
}

impl<'a> InputParser<'a> {
    fn new() -> Self {
        let capture_names = vec!["add_employee", "show_department", "show_all"];

        let set = RegexSet::new(&[
            r"(?P<add_employee>add (?P<employee_name>\w+) to (?P<department_name>\w+))",
            r"^(?P<show_department>show (?P<department_name>\w+))",
            r"^(?P<show_all>show)",
        ])
        .expect("Hardcoded regex should always compile");

        // Compile each pattern independently.
        let regexes = set
            .patterns()
            .iter()
            .map(|pat| Regex::new(pat).expect("Hardcoded regex that has already been compiled once should /definitely/ always compile"))
            .collect();

        InputParser {
            set,
            regexes,
            capture_names,
        }
    }

    // Text processing function
    fn parse(&self, input: &str) -> Option<EmployeeDBAction> {
        let captures = self
            // Match against the whole set first and identify the individual matching patterns.
            .set
            .matches(input)
            .into_iter()
            // Dereference the match index to get the corresponding compiled pattern.
            .map(|match_index| &self.regexes[match_index])
            // To get match locations or any other info, we then have to search the exact same text again, using our separately-compiled pattern.
            .map(|pattern| {
                pattern
                    .captures(input)
                    .expect("If a pattern has matched already, it must also capture")
            })
            // pull out the first match
            // there really shouldn't be more than one anyway, but we still have to do this
            // Question mark because we want to abort if nothing matched
            .next()?;

        let action = self
            .capture_names
            .iter()
            .find(|x| captures.name(x).is_some())
            .unwrap();

        match *action {
            "add_employee" => Some(EmployeeDBAction::InsertEmployee {
                new_employee_name: captures["employee_name"].to_owned(),
                department_name: captures["department_name"].to_owned(),
            }),
            "show_department" => Some(EmployeeDBAction::ListDepartment {
                department_name: captures["department_name"].to_owned(),
            }),
            "show_all" => Some(EmployeeDBAction::ListAll),
            _ => None,
        }
    }
}

fn main() -> io::Result<()> {
    // Database hashmap
    let mut employee_db = EmployeeDB::new();

    let input_parser = InputParser::new();

    let stdin = io::stdin();

    loop {
        // prompt
        print!("EDB:> ");

        io::stdout().flush()?;

        // take input
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                if let Some(command) = input_parser.parse(&input) {
                    employee_db.send(command);
                } else {
                    println!("invalid command!")
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}
