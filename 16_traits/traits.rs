struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Associated function signature; `Self` refers to the implementer type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementer methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementer type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();

    let bunga: Cow = Animal::new("Bunga");
    bunga.talk();
}

// NOTE: Piyush having fun..
struct Cow {
    name: &'static str,
}

impl Animal for Cow {
    // `Self` is the implementer type: `Cow`.
    fn new(name: &'static str) -> Cow {
        Cow { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "moooooo..."
    }

    // Default trait for talk
}
