trait Animal {
    fn greet(&self) -> String;
}

struct Cat {
    name: String,
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn greet(&self) -> String {
        String::from("rof rof rof")
    }
}

impl Animal for Cat {
    fn greet(&self) -> String {
        String::from("meow meow")
    }
}

fn main() {
    let max = Dog { name: String::from("Max") };
    println!("Max (Hund) sagt {}", max.greet());

    let luna = Cat { name: String::from("Luna") };
    println!("Luna (Katze) sagt {}", luna.greet());
}
