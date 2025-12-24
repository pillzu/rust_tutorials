fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // can also use `for n in 1...=100` for inclusive loop
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    println!("**** FOR AND ITERATORS  ****");

    // **** FOR AND ITERATORS
    let mut names = vec!["Bob", "Frank", "Ferris"];

    println!("* iter");
    // `iter` borrows each element of the collection through each iteration. Thus leaving the
    // collection untouched and available for reuse after the loop.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    println!("* into_iter");
    // into_iter consumes the collection so that on each iteration the exact data is provided.
    // Once the collection has been consumed it is no longer available for reuse as it has been
    // 'moved' within the loop.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line``

    // needt o re-init since into_iter consumed the slice/array already
    names = vec!["Bob", "Frank", "Ferris"];

    println!("* iter_mut");
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
