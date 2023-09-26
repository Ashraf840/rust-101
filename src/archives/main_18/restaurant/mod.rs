#![allow(dead_code)]

// Module (Parent module)
mod pizza_order {
    // Pizza type struct
    pub struct Pizza {
        dough: String,
        cheese: String,
        topping: String,
    }

    // Implement functionality: return "Pizza" type w/ initial values
    impl Pizza {
        pub fn lunch(topping: &str) -> Self {
            Self {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }

    // Nested module (help_customer): taking pizza order
    pub mod help_customer {
        // Initial state of customer
        fn seat_at_table() {
            println!("Customer seated at table!")
        }

        // take pizza order
        pub fn take_order() {
            // Define the current status of the customer
            seat_at_table();
            // Invoke & store the "lunch" function of the Pizza type
            let order: super::Pizza = super::Pizza::lunch("veggie");
            // print out the pizza ordered by the customer
            println!(r##"The customer has ordered a regular pizza with "{}" topping."##, order.topping);
        }
    }
}

// This module will be accessed with a public function
pub fn order_food() {
    // Define the module path explicitly, in order to invoke the "take_order" function
    crate::restaurant::pizza_order::help_customer::take_order()
}

