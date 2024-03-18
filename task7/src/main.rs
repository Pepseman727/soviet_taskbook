use std::io;

fn main() {
    let mut v_1 = String::new();
    let mut v_2 = String::new();

    let mut t_1 = String::new();
    let mut t_2 = String::new();

    println!("Введите первый объём воды");
    io::stdin()
        .read_line(&mut v_1)
        .expect("Enter a value");

    println!("Введите первую температуру воды");
    io::stdin()
        .read_line(&mut t_1)
        .expect("Enter a value");

    println!("Введите второй объём воды");
    io::stdin()
        .read_line(&mut v_2)
        .expect("Enter a value");


    println!("Введите вторую температуру воды");

    io::stdin()
        .read_line(&mut t_2)
        .expect("Enter a value");

    let v_1 = v_1.trim().parse::<f64>().expect("Enter a number");
    let v_2 = v_2.trim().parse::<f64>().expect("Enter a number");
    if v_1 <=0.0 || v_2 <= 0.0 {
        println!("Введены некорректные данные\n{} {}",v_1, v_2);
    }

    let t_1 = t_1.trim().parse::<f64>().expect("Enter a number");
    let t_2 = t_2.trim().parse::<f64>().expect("Enter a numbre");


    println!("Объём смеси: {}",v_1+v_2);
    println!("Температура смеси: {}",(v_1*t_1+v_2*t_2)/(v_1+v_2));



}
