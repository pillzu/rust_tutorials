// The newtype idiom gives compile time guarantees that the right type of value is supplied to a program.

#[derive(Debug)]
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(25);
    let age_days = age.to_days();
    println!("Is an adult? {}", is_adult(&age));
    println!("Is an adult? {}", is_adult(&age_days.to_years()));
    // println!("Is an adult? {}", is_adult(&age_days));

    // Get newtype value as base_type
    let years = Years(42);
    let years_as_primitive_1: i64 = years.0; // Tuple
    let Years(years_as_primitive_2) = years; // Destructuring
    println!(
        "Years: {:?} ({})",
        years,
        std::any::type_name_of_val(&years)
    );
    println!(
        "Years as primitive 1: {:?} ({})",
        years_as_primitive_1,
        std::any::type_name_of_val(&years_as_primitive_1)
    );
    println!(
        "Years as primitive 2: {:?} ({})",
        years_as_primitive_2,
        std::any::type_name_of_val(&years_as_primitive_2)
    );
}
