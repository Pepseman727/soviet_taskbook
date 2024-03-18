use std::{io, process};

fn main() {
    let mut height = String::new();
    let g = 9.8;
    //Простая физическая задача
    // h = (g*t^2)/2, g = 9.8
    // t = sqrt((2*h)/g)
    println!("Enter a height");
    io::stdin().read_line(&mut height).expect("Enter a value");

    let height = height.trim().parse::<f64>().expect("Enter a number");

    if height < 0.0 {
        println!("Bad input");
        process::exit(0);
    }

    println!("Fall time: {:.4} sec", ((2.0 * height) / g).sqrt());
}