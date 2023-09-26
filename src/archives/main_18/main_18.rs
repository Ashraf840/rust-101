// Tut-18 (Modules)
// Module: "restaurant/mod.rs"

// To use the function "order_food", requires to define the module "restaurant" explicitly
mod restaurant;
use restaurant::order_food;

fn main() {
    order_food()
}