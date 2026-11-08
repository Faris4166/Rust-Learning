enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c: Color = Color::Red;
    let e: Color = Color::Green;
    let k: Color = Color::Blue;

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
    match e {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
    match k {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
