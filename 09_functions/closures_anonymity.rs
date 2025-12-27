// * Learning:
// - Every closure = unique anonymous struct + trait impl(s)
// - The struct holds the captured environment (by ref / mut ref / value)
// - The variable gets that unique type
// - Functions that want to accept any closure must use generics + trait bound
// - The trait bound replaces the need to know the concrete (unknown) type

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    // me breaking the compilers' head
    let mut y = 0;
    let mut test = || {
        y += 1;
        println!("{y}");
    };
    test();
    test();

    // apply(test);
    // TODO :Uncomment me and modify the type trait (guess which one)

    apply(print);
}
