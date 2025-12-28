// * Learning
// - cfg attribute: #[cfg(...)] - removes code based on conditional
// - cfg! macro: cfg!(...) - only evaluates to true or false, i.e. code is kept regardless
// - You can also pass custom conditions via --cfg flag in rustc
//     e.g. `rustc --cfg custom_condition main.rs` with #![cfg(custom_condition)]

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
