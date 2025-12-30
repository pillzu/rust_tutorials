trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct SmartDude {
    name: String,
    university: String,
    favourite_language: String,
    git_username: String,
}

impl Person for SmartDude {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for SmartDude {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for SmartDude {
    fn fav_language(&self) -> String {
        self.favourite_language.clone()
    }
}

impl CompSciStudent for SmartDude {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn smart_dude() -> impl CompSciStudent {
    SmartDude {
        name: "Piyush".to_string(),
        university: "University of Waterloo".to_string(),
        favourite_language: "Rust".to_string(),
        git_username: "pillzu".to_string(),
    }
}

fn main() {
    let sd = smart_dude();
    println!("{}", comp_sci_student_greeting(&sd));
}
