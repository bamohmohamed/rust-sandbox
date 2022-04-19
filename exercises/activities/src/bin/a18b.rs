// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum JobFunction {
    Maintenance,
    Marketing,
    Managers,
    Line,
    Kitchen,
    Assembly,
}

struct Employee {
    job: JobFunction,
    active: bool,
}

fn can_access(emp: &Employee) -> Result<(), String> {
    match emp.active {
        false => return Err("Not Active".to_owned()),
        _ => (),
    }
    match emp.job {
        JobFunction::Maintenance | JobFunction::Marketing | JobFunction::Managers => Ok(()),
        _ => Err("Not Allowed to access".to_owned()),
    }
    // Ok(())
}

fn print_access(emp: &Employee) -> Result<(), String> {
    let try_access = can_access(emp)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let emp = Employee {
        job: JobFunction::Marketing,
        active: true,
    };
    match print_access(&emp) {
        Err(e) => println!("Access denied {:?}", e),
        _ => (),
    }
}
