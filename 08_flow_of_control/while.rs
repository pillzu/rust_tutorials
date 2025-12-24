fn my_fizz_buzz(n: i32) -> String {
    let mut found = false;
    let mut ret = "".to_string();

    if n % 3 == 0 {
        ret = "Fizz".to_string();
        found = true;
    }
    if n % 5 == 0 {
        ret += "Buzz";
        found = true;
    }

    if !found {
        ret = format!("{}", n)
    }

    return ret;
}

fn their_fizz_buzz(n: i32) -> String {
    let ret = if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        format!("{}", n)
    };
    return ret;
}

fn main() {
    // counter
    let mut n = 1;

    while n < 101 {
        let their = their_fizz_buzz(n);
        let my = my_fizz_buzz(n);

        assert_eq!(their, my);

        println!("Their => {their} ; My => {my}");
        n += 1;
    }
}
