use std::io;

fn main() {
    let mut resist = String::new();


    //Рассчётная формула 1/R = 1/R1 + 1/R2 + 1/R3
    // => R = R1R2R3/R2R3+R1R3+R1R2
    println!("Enter three resistance values separated by a space");
    io::stdin()
        .read_line(&mut resist)
        .expect("Enter value");

    let resist: Vec<&str> = resist.trim().split(' ').collect();
    let resist: Vec<f64> = resist
                            .iter()
                            .map(|value| value.to_string().parse::<f64>().expect("Enter a number"))
                            .collect();
    if let [r1, r2, r3] = resist.as_slice() {
        if *r1 < 0.0 || *r2 < 0.0 || *r3 < 0.0 {
            println!("Bad input");
        } else {
            let numerator: f64 = r1 * r2 * r3;
            let denominator = r2 * r3 + r1 * r3 + r1 * r2;
            println!("Total resistance {}", numerator/denominator);
        }
    } else {
        println!("Bad input");
    }




}
