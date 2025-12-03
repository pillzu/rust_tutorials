use std::convert::{From, Into};

#[derive(Debug)]
struct Number {
    value: i32,
}

// added for fun
// impl fmt::Display for Number {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Number: {}", self.value)
//     }
// }

// From and into are interchangeable
// But, Into does not mean you will get a "From" as well (other way is true)
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type annotation
    let num2: Number = int.into();
    println!("My number 2 is {:?}", num2);
}
