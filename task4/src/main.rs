use std::io;
use std::process;

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
    let a: f64 = a.trim().parse().expect("Enter a number");
    let b: f64 = b.trim().parse().expect("Enter a number");

    if a < 0.0 || b < 0.0{
        println!("Enter a number higher than zero\n You entered a:{a} b:{b}", );
        process::exit(0);
    }

    let average_ariphmetic = (a + b)/2.0;
    let average_geometry = (a * b).sqrt();

    println!("Average (ariphmetic): {}", average_ariphmetic);
    println!("Average (geometric): {}", average_geometry);

}
