use std::io;

fn main() {
    loop {
        println!("1. Trapezium");
        println!("2. Rhombus");
        println!("3. Parallelogram");
        println!("4. Cube");
        println!("5. Exit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Invalid input");

        let choice = choice.trim();

        if choice == "1" {
            let mut h = String::new();
            let mut b1 = String::new();
            let mut b2 = String::new();

            println!("Enter height:");
            io::stdin().read_line(&mut h).expect("Invalid input");

            println!("Enter first base:");
            io::stdin().read_line(&mut b1).expect("Invalid input");

            println!("Enter second base:");
            io::stdin().read_line(&mut b2).expect("Invalid input");

            let h: f64 = h.trim().parse().expect("Enter a number");
            let b1: f64 = b1.trim().parse().expect("Enter a number");
            let b2: f64 = b2.trim().parse().expect("Enter a number");

            println!("Area = {}", 0.5 * h * (b1 + b2));
        }

        else if choice == "2" {
            let mut d1 = String::new();
            let mut d2 = String::new();

            println!("Enter first diagonal:");
            io::stdin().read_line(&mut d1).expect("Invalid input");

            println!("Enter second diagonal:");
            io::stdin().read_line(&mut d2).expect("Invalid input");

            let d1: f64 = d1.trim().parse().expect("Enter a number");
            let d2: f64 = d2.trim().parse().expect("Enter a number");

            println!("Area = {}", 0.5 * d1 * d2);
        }

        else if choice == "3" {
            let mut b = String::new();
            let mut h = String::new();

            println!("Enter base:");
            io::stdin().read_line(&mut b).expect("Invalid input");

            println!("Enter height:");
            io::stdin().read_line(&mut h).expect("Invalid input");

            let b: f64 = b.trim().parse().expect("Enter a number");
            let h: f64 = h.trim().parse().expect("Enter a number");

            println!("Area = {}", b * h);
        }

        else if choice == "4" {
            let mut s = String::new();

            println!("Enter side:");
            io::stdin().read_line(&mut s).expect("Invalid input");

            let s: f64 = s.trim().parse().expect("Enter a number");

            println!("Surface area = {}", 6.0 * s * s);
        }

        else if choice == "5" {
            println!("Goodbye!");
            break;
        }

        else {
            println!("Wrong choice");
        }
    }
}