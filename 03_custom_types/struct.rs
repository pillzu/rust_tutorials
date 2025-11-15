// Learning: Prepend _ to ignore variable  unused

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// activity is to add a function to calcuclate area of rectangle
// - bonus if you use nested destructuring
fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: tl_x, y: tl_y },
        bottom_right: Point { x: br_x, y: br_y },
    } = rect;

    f32::abs((tl_x - br_x) * (tl_y - br_y))
}

// activity is to build a square given point and length
fn square(tp: Point, length: f32) -> Rectangle {
    Rectangle {
        top_left: Point { ..tp },
        bottom_right: Point {
            x: tp.x + length,
            y: tp.y + length,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    // activity
    let area = rect_area(&rectangle);
    println!("area of rectangle {:?} is: {}", rectangle, area);

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // activity for square
    let p2 = Point { x: 5.0, y: 5.0 };
    let sq = square(p2, 5.0);
    println!("Square is {}", rect_area(&sq));
}
