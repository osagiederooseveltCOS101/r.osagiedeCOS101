use std::io;

fn main(){
    let mut input = String::new();

    println!("Enter a:");
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter b:");
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter c:");
    io::stdin().read_line(&mut input).unwrap();
    let c: f64 = input.trim().parse().unwrap();

    let d= b * b - 4.0 * a * c;

    if d > 0.0 {
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);

        println!("Two distinct roots: {} and {}", x1, x2);
    } else if d == 0.0 {
        let x = -b / (2.0 * a);

        println!("Exactly one root: {}",x);
    } else {
        println!("No real roots");
    }

}