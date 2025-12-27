// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function(i: i32) {
            println!("called `deeply::nested::function() with {}`", i);
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function(1);

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function(2);

        println!("Leaving block");
    }

    function();
}
