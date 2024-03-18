use::std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    
    println!("Enter number a: ");
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    println!("Enter number b: ");
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");
    
    let a: f64 = a.trim().parse().expect("Please write a number");
    let b: f64 = b.trim().parse().expect("Please write a number");

    println!("Sum of two numbers: {}", a+b);
    println!("Sub of two numbers: {}", a-b);
    println!("Mul of two numbers: {}", a*b);
}
