// Verbose way to represent multiple_bounds
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug` or
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

trait PrintInOption2 {
    fn print_in_option2(self);
}

impl<T: Debug> PrintInOption2 for T {
    fn print_in_option2(self) {
        println!("2 => {:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.clone().print_in_option();

    // NOTE: I made this to show how both methods are equal
    vec.print_in_option2();
}
