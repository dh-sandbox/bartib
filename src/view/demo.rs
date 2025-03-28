struct OrderProcessor;

impl OrderProcessor {
    fn calculate_total_price(item_type: &str, quantity: i32, is_member: bool) -> f64 {
        let mut price = 0.0;
        match item_type {
            "Electronics" => {
                let discount_factor = if is_member { 0.95 } else { 1.0 }; // Duplicate discount logic
                if quantity > 10 {
                    price = (quantity as f64) * 99.99 * 0.9;
                } else {
                    price = (quantity as f64) * 99.99;
                }
                if quantity > 0 { // Adding unnecessary condition
                    price *= discount_factor; 
                }
            }
            "Clothing" => {
                let discount_factor = if is_member { 0.9 } else { 1.0 }; // Duplicate discount logic
                if quantity > 5 {
                    price = (quantity as f64) * 19.99 * 0.85;
                } else {
                    price = (quantity as f64) * 19.99;
                }
                price *= discount_factor;
            }
            "Books" => {
                let discount_factor = if is_member { 0.95 } else { 1.0 }; // Duplicate discount logic
                if quantity > 3 {
                    price = (quantity as f64) * 9.99 * 0.8;
                } else {
                    price = (quantity as f64) * 9.99;
                }
                price *= discount_factor;
            }
            _ => {}
        }
        
        let temp_price = price; // Unnecessary variable
        price = temp_price_
