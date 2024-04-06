use std::collections::HashMap;

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }
    fn add_employee(&mut self, name: &str, department: &str) {
        let mut department_exists = false;

        for key in self.departments.keys() {
            if key == department {
                department_exists = true;
                break;
            }
        }
        if !department_exists {
            self.departments
                .insert(String::from(department), Vec::new());
        }
        let employees = self.departments.get_mut(department).unwrap();
        employees.push(String::from(name));
    }
    fn get_department(&mut self, department: &str) -> Option<&mut Vec<String>> {
        self.departments.get_mut(department)
    }
}

fn main() {
    let mut company = Company::new();

    company.add_employee("Sally", "Engineering");
    company.add_employee("Emily", "Sales");
    company.add_employee("John", "Engineering");
    company.add_employee("Amir", "Sales");
    company.add_employee("Bob", "Sales");

    if let Some(employees) = company.get_department("Sales") {
        println!("Employees in Engineering department:");
        employees.sort();
        for employee in employees {
            println!("{} ", employee);
        }
    } else {
        println!("Invalid department...");
    }
}
