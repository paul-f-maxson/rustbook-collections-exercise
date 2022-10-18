// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use regex::{Captures, Regex, RegexSet};
use std::collections::HashMap;

#[derive(Debug)]
struct EmployeeDB {
    db: HashMap<String, Vec<String>>,
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
    Nothing,
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
                let insertion_index = department.partition_point(|existing_employee_name| {
                    existing_employee_name < &new_employee_name
                });

                // Insert name in order
                department.insert(insertion_index, new_employee_name)
            }
            // Department does not exist
            None => {
                // Create new department and add employee to it
                self.db.insert(department_name, [new_employee_name].into());
            }
        };
    }

    // Retrieval functions

    fn send(&mut self, action: EmployeeDBAction) {
        use EmployeeDBAction::{InsertEmployee, ListAll, ListDepartment, Nothing};

        match action {
            InsertEmployee {
                new_employee_name,
                department_name,
            } => self.insert_employee(department_name, new_employee_name),

            ListAll => {
                println!("{:#?}", self.db)
            }

            ListDepartment { department_name } => {
                println!("{:#?}", self.db[&department_name])
            }

            Nothing => {}
        }
    }
}

struct InputParser {
    set: RegexSet,

    regexes: Vec<Regex>,
}

impl InputParser {
    fn new() -> Self {
        let set = RegexSet::new(&[
            r"^(?P<add_employee>add (?P<employee_name>\w+) to (?P<department_name>\w+))$",
            r"^(?P<show_department>show (?P<department_name>\w+))$",
            r"^(?P<show_all>show)$",
        ])
        .unwrap();

        // Compile each pattern independently.
        let regexes = set
            .patterns()
            .iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect();

        InputParser { set, regexes }
    }

    // Text processing function
    fn parse(&self, input: &str) -> EmployeeDBAction {
        let captures = &(self
            // Match against the whole set first and identify the individual matching patterns.
            .set
            .matches(input)
            .into_iter()
            // Dereference the match index to get the corresponding
            // compiled pattern.
            .map(|match_index| &self.regexes[match_index])
            // To get match locations or any other info, we then have to search the exact same text again, using our separately-compiled pattern.
            .map(|pattern| pattern.captures(input).unwrap())
            .collect::<Vec<Captures>>())[0];

        let actions = vec!["add_employee", "show_department", "show_all"];

        let action = actions.iter().find(|x| captures.name(x).is_some()).unwrap();

        match *action {
            "add_employee" => EmployeeDBAction::InsertEmployee {
                new_employee_name: captures["employee_name"].to_owned(),
                department_name: captures["department_name"].to_owned(),
            },
            "show_department" => EmployeeDBAction::ListDepartment {
                department_name: captures["department_name"].to_owned(),
            },
            "show_all" => EmployeeDBAction::ListAll,
            _ => EmployeeDBAction::Nothing,
        }
    }
}

fn main() {
    // Database hashmap
    let mut employee_db = EmployeeDB::new();

    let input_parser = InputParser::new();

    let commands = vec![
        "add jason to sales",
        // "show sales",
        "show"
    ];

    for command in commands {
        employee_db.send(input_parser.parse(command))
    }

    println!("{employee_db:#?}");
    // loop {
    //     // take input
    //     // process input
    //     // determine intent
    //     // extract meaning
    //     // perform operations
    //     // return result
    // }
}
