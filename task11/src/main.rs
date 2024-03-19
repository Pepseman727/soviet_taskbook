use std::f64::consts::*;
use std::io;

fn fact(n: u64) -> f64 {
    (1..=n).product::<u64>() as f64
}

fn main() {
    println!("Enter x, y, z separated by spaces");
    let mut values = String::new();
    io::stdin().read_line(&mut values).expect("Enter a value");

    let values: Vec<f64> = values
        .trim()
        .split(' ')
        .map(|value| value.to_string().parse::<f64>().expect("Enter a number"))
        .collect();

    if let [x, y, z] = values.clone().as_slice() {
        //пункт а
        let a = (((x - 1.0).abs()).sqrt() - y.abs().cbrt()) / (1.0 + x * x / 2.0 + y * y / 4.0);
        let b = x * (z.atan() + (-(x + 3.0)).exp());
        println!("Пункт а\na = {}\nb = {}", a, b);

        //пункт б
        let a = (3.0 + (y - 1.0).exp()) / (1.0 + x * x * (y - z.tan()).abs());
        let b = 1.0 + (y - x).abs() + (y - x) * (y - x) / 2.0 + (y - x).powf(3.0).abs() / 3.0;
        println!("Пункт б\na = {}\nb = {}", a, b);

        //пункт в
        let a = (1.0 + y) * (x + y / (x * x + 4.0)) / ((-x - 2.0).exp() + 1.0 / (x * x + 4.0));
        let b = (1.0 + (y - 2.0).cos()) / (x * x * x * x / 2.0 + z.sin().powf(2.0));
        println!("Пункт в\na = {}\nb = {}", a, b);

        //пункт г
        let a = y + (x / (y * y + (x * x / (y + (x * x * x) / 3.0)).abs()));
        let b = 1.0 + (z / 2.0).tan().powi(2);
        println!("Пункт г\na = {}\nb = {}", a, b);

        //пункт д
        let a = 2.0 * (x - FRAC_PI_6).cos() / (1.0 / 2.0 + z.sin().powf(2.0));
        let b = 1.0 + (z * z) / (3.0 + z * z / 5.0);
        println!("Пункт д\na = {}\nb = {}", a, b);

        //пункт е
        let a =
            (1.0 + (x + y).sin().powf(2.0)) / (2.0 + (x - 2.0 * x / (1.0 + x * x * y * y)).abs());
        let b = (1.0 / z).atan().cos().powf(2.0);
        println!("Пункт е\na = {}\nb = {}", a, b);

        //пункт ж
        let a = ((y - x.abs().sqrt()) * (x - y / (z + x * x / 4.0))).ln();
        let b = x - x * x / fact(3) + x.powf(5.0) / fact(5); //Похож на ряд Тейлора для ln(1+x)
        println!("Пункт ж\na = {}\nb = {}", a, b);
    } else {
        println!("Bad input");
    }
}
