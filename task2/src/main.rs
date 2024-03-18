use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    println!("Enter number 'x'");
    io::stdin()
        .read_line(&mut x)
        .expect("Error of input");

    println!("Enter number 'y'");
    io::stdin()
        .read_line(&mut y)
        .expect("Error of input");

    let x: f64 = x.trim().parse().expect("Enter a number");
    let y: f64 = y.trim().parse().expect("Enter a number");
    let top_component = x.abs() - y.abs();
    let bot_component = 1.0 + (x*y).abs();

    println!("Result of formula: {}", top_component/bot_component);

}
