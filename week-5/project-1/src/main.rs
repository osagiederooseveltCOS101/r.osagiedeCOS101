use std::io;

fn main() {
    println!("===== RESTAURANT MENU =====");
    println!("P - Pounded Yam / Edikangiko Soup - N3,200");
    println!("F - Fried Rice & Chicken - N3,000");
    println!("A - Amala & Ewedu Soup - N2,500");
    println!("E - Eba & Egusi Soup - N2,000");
    println!("W - White Rice & Stew - N2,500");

    println!("Enter the food type:");

    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Invalid value");

    let food = food.trim().to_uppercase();

    println!("Enter quantity:");

    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Invalid value");

    let quantity: i32 = quantity.trim().parse().expect("Invalid value");

    let price: i32;
    let food_name: &str;

    match food.as_str() {
        "P" => {
            price = 3200;
            food_name = "Pounded Yam / Edikangiko Soup";
        }

        "F" => {
            price = 3000;
            food_name = "Fried Rice & Chicken";
        }

        "A" => {
            price = 2500;
            food_name = "Amala & Ewedu Soup";
        }

        "E" => {
            price = 2000;
            food_name = "Eba & Egusi Soup";
        }

        "W" => {
            price = 2500;
            food_name = "White Rice & Stew";
        }

        _ => {
            println!("Invalid food type.");
            return;
        }
    }

    let total = price * quantity;

    println!("Food: {}", food_name);
    println!("Quantity: {}", quantity);
    println!("Total before discount: N{}", total);

    if total > 10000 {
        let discount = total * 5 / 100;
        let final_total = total - discount;

        println!("Discount: N{}", discount);
        println!("Final total: N{}", final_total);
    } else {
        println!("No discount.");
        println!("Final total: N{}", total);
    }
}