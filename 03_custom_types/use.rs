// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
enum Stage {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug)]
enum Role {
    Student,
    Teacher,
    Manager,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Stage::{Advanced, Beginner, Intermediate};
    // Automatically `use` each name inside `Role`.
    use crate::Role::*;

    for stage in [Beginner, Intermediate, Advanced] {
        for role in [Student, Teacher, Manager] {
            print!("**** {:?} at {:?} ****\n", stage, role);
            match stage {
                // Note the lack of scoping because of the explicit `use` above.
                Beginner => println!("Beginners are starting their learning journey!"),
                Intermediate => println!("Intermediate learners are expanding their horizons!"),
                Advanced => println!("Advanced learners are mastering their subjects..."),
            }

            match role {
                // Note again the lack of scoping.
                Student => println!("Students are acquiring knowledge!"),
                Teacher => println!("Teachers are spreading knowledge!"),
                Manager => println!("Managers are directing learning!"),
            }
        }
    }
}
