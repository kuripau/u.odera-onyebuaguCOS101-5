use std::io;

fn main() {
    println!("Welcome to PAU Cafe, what would you like to order:");

    let mut orders: Vec<(String, f32, f32)> = Vec::new();

    loop {
        println!("\n[P] Poundo Yam / Edinkaiko Soup  - N3,200");
        println!("[F] Fried Rice & Chicken         - N3,000");
        println!("[A] Amala & Ewedu Soup           - N2,500");
        println!("[E] Eba & Egusi Soup             - N2,000");
        println!("[W] White Rice & Stew            - N2,500");
        println!("Enter the letter corresponding to your food choice (P, F, A, E, W): ");

        let mut order = String::new();
        io::stdin().read_line(&mut order).expect("Failed to read input");
        let choice = order.trim().to_uppercase();

        let (name, unit_price) = match choice.as_str() {
            "P" => ("Poundo Yam / Edinkaiko Soup", 3200.0),
            "F" => ("Fried Rice & Chicken", 3000.0),
            "A" => ("Amala & Ewedu Soup", 2500.0),
            "E" => ("Eba & Egusi Soup", 2000.0),
            "W" => ("White Rice & Stew", 2500.0),
            _ => {
                println!("Invalid food choice code selected!");
                continue; 
            }
        };

        println!("How many do you want to order: ");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Failed to read input");
        let quantity_ordered: f32 = match quantity.trim().parse() {
            Ok(q) if q > 0.0 => q,
            _ => {
                println!("Please enter a valid number greater than 0.");
                continue;
            }
        };

        orders.push((name.to_string(), unit_price, quantity_ordered));
        println!("Added {} x{} to your order.", name, quantity_ordered);

        println!("Order another item? (Y/N): ");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read input");
        if again.trim().to_uppercase() != "Y" {
            break; 
        }
    }

    let mut total_charge: f32 = 0.0;

    println!("\n--- Order Summary ---");
    for (name, unit_price, quantity) in &orders {
        let line_total = unit_price * quantity;
        total_charge += line_total;
        println!("{} x{} - N{:.2}", name, quantity, line_total);
    }

    let mut discount_applied = false;
    if total_charge > 10000.0 {
        total_charge *= 0.95;
        discount_applied = true;
    }

    if discount_applied {
        println!("5% discount applied!");
    }
    println!("Total: N{:.2}", total_charge);
}