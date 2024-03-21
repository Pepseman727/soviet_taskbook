use std::io;
use std::f64::consts::*;

fn main() {
    let mut l = String::new();

    println!("Введите длину маятника");
    io::stdin()
        .read_line(&mut l)
        .expect("Enter a value");
    let l = l.trim().parse::<f64>().expect("Enter a number");
    let g = 9.80665;
    println!("Период колебания маятника {}", 2.0*PI*(l/g).sqrt())
}
