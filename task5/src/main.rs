use std::io;

fn main() {

    let mut a = String::new();
    let mut b = String::new();

    println!("Enter number a");
    io::stdin()
        .read_line(&mut a)
        .expect("Enter a value");
    println!("Enter number b");
    io::stdin()
        .read_line(&mut b)
        .expect("Enter a value");
    let a = a.trim().parse::<f64>().expect("Enter a number").abs();
    let b = b.trim().parse::<f64>().expect("Enter a number").abs();

    let average_ariphmetic = (a + b)/2.0;
    let average_geometry = (a * b).sqrt();

    println!("Average (ariphmetic): {}", average_ariphmetic);
    println!("Average (geometric): {}", average_geometry);

}
