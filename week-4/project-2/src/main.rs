use std::io;

fn main(){
    let mut input = String::new();

    println!("Are you experienced ? (yes/no):");
    io::stdin().read_line(&mut input).unwrap();
    let experienced = input.trim().to_lowercase();

    input.clear();
    println!("Enter age:");
    io::stdin().read_line(&mut input).unwrap();
    let age: i32 = input.trim().parse().unwrap();

    let incentive;

    if experienced == "yes" && age >40 {
        incentive = 1_560_000;
    } else if experienced == "yes" && age >=30{
        incentive = 1_480_00;
    } else if experienced == "yes" && age < 28{
        incentive = 1_300_000;
    } else {
        incentive = 100_000;
    }

    println!("Annual incentive: N{}", incentive);
}