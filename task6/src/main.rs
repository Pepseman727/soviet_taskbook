use std::io;

fn main() {
    let mut first_leg = String::new();
    let mut second_leg = String::new();

    println!("Введите длину первого катета");
    io::stdin()
        .read_line(&mut first_leg)
        .expect("Enter a number");

    println!("Введите длину второго катета");
    io::stdin()
        .read_line(&mut second_leg)
        .expect("Enter a number");

    let first_leg = first_leg.trim().parse::<f64>().expect("Введите число");
    let second_leg = second_leg.trim().parse::<f64>().expect("Введите число");

    let hypotenuse = (first_leg * first_leg +second_leg * second_leg).sqrt();
    println!("Гипотенуза треугольника: {}", hypotenuse);
    println!("Площадь треугольника {}", (first_leg* second_leg)/2.0);
}
