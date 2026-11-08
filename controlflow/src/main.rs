fn main() {
    let x = 5;

    if x > 0 {
        println!("x is positive");
    } else {
        println!("x is negative");
    }
    
    let y = ["apple", "banana", "cherry"];
    for i  in y.iter() {
        println!("{}", i);
    }

    let mut z = 15;

    while z < 50 {
        println!("z is {}", z);
        z += 5;
    }
}
