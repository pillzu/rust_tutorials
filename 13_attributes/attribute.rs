// outer_attribute can be applied to the item following it
#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

fn main() {
    // innter attribute applies to the scope in which it's placed
    #![allow(unused_variables)]

    let x = 3;
    let rect = Rectangle {
        width: 10.0,
        height: 10.0,
    };
}
