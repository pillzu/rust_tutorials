// Compile with `rustc executable.rs --extern rary=library.rlib`
// TODO : Comment me when compiling with ^ command
mod rary;

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
