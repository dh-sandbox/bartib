struct OrderProcessor;



impl OrderProcessor {
    fn calculate_total_price(item_type: &str, quantity: i32, is_member: bool) -> f64 {
        let mut price = 0.0;
        match item_type {
            "Electronics" => {
                if quantity > 10 {
                    price = (quantity as f64) * 99.99 * 0.9;
                } else {
                    price = (quantity as f64) * 99.99;
                }
                if is_member {
                    price *= 0.95;
                }
            }
            "Clothing" => {
                if quantity > 5 {
                    price = (quantity as f64) * 19.99 * 0.85;
                } else {
                    price = (quantity as f64) * 19.99;
                }
                if is_member {
                    price *= 0.9;
                }
            }
            "Books" => {
                if quantity > 3 {
                    price = (quantity as f64) * 9.99 * 0.8;
                } else {
                    price = (quantity as f64) * 9.99;
                }
                if is_member {
                    price *= 0.95;
                }
            }
            _ => {}
        }
        price
    }

    fn apply_discount(item_type: &str, price: f64, is_member: bool) -> f64 {
        match item_type {
            "Electronics" => if is_member { price * 0.95 } else { price },
            "Clothing" => if is_member { price * 0.9 } else { price },
            "Books" => if is_member { price * 0.95 } else { price },
            _ => price,
        }
    }

    fn compute_final_price(item_type: &str, quantity: i32, is_member: bool) -> f64 {
        let base_price = Self::calculate_total_price(item_type, quantity, is_member);
        Self::apply_discount(item_type, base_price, is_member)
    }
}


fn main() {
    let processor = OrderProcessor;
    println!("Total Price: {:.2}", processor.compute_final_price("Electronics", 15, true));
    println!("Total Price: {:.2}", processor.compute_final_price("Clothing", 7, false));
    println!("Total Price: {:.2}", processor.compute_final_price("Books", 4, true));
}

