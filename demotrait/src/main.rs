struct Person {
    name: String,
}

struct Cat {
    name: String,
}

trait Eat {
    fn eat_dinner(&self);
}

impl Eat for Person {
    fn eat_dinner(&self) {
        println!("{} is eating dinner", self.name);
    }
}

impl Eat for Cat {
    fn eat_dinner(&self) {
        println!("{} is eating dinner", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };
    person.eat_dinner();
    cat.eat_dinner();
}