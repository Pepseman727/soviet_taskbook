use std::io;
use std::f64::consts::*;
fn main() {
    //Рассчётная формула R = a/(2sin(180/n))
    let mut radius = String::new();
    let mut n = String::new();

    println!("Enter the number of angles of the polygon");
    io::stdin()
        .read_line(&mut n)
        .expect("Enter value");

    println!("Enter circumradius");
    io::stdin()
        .read_line(&mut radius)
        .expect("Enter value");

    let radius = radius.trim().parse::<f64>().expect("Enter a number");
    let n = n.trim().parse::<f64>().expect("Enter a number");

    if radius < 0.0 || n < 3.0 {
        println!("Enter valid numbers\n You enter number of angles {n}, circumradius {radius}");
    }

    let side_length = 2.0 * radius *  f64::sin(PI/n);

    println!("Perimeter equal: {}", side_length * n);

}
