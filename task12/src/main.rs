use std::io;

fn main() {
    let mut triangle_side = String::new();

    println!("Введите сторону равностороннего треугольника");
    io::stdin()
        .read_line(&mut triangle_side)
        .expect("Enter a value");

    let tri_side = triangle_side.trim().parse::<f64>().expect("Enter a number");

    println!("Площадь треугольника = {}", tri_side * tri_side * 3_f64.sqrt()/4.0);
}
