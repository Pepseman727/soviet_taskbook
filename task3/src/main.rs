use std::io;


fn main() {
    let mut edge = String::new();

    println!("Enter the value of edge");
    io::stdin()
        .read_line(&mut edge)
        .expect("Enter a value");

    let edge: f64 = edge.trim().parse().expect("Enter a number");

    println!("Sqaure of plane {}", edge * edge);
    println!("Volume of cube {}", edge * edge * edge);
}
