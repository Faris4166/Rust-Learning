fn main() {
    let s = String::from("Hello, ownership!");
    take_ownership(s);
}

fn take_ownership(s: String) {
    println!("Taking ownership of: {}", s);
}