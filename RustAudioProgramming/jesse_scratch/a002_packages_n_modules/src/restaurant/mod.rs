// everything is private by default

mod pizza_order {
    pub struct Pizza{
        pub dough: String,
        pub sauce: String,
        pub cheese: String,
        pub toppings: Vec<String>
    }

    // functions to interact with the pizza struct "object"
    impl Pizza {
        pub fn new(toppings: Vec<String>) -> Pizza {
            Pizza {
                dough: String::from("thin"),
                sauce: String::from("tomato"),
                cheese: String::from("mozzarella"),
                toppings: toppings
            }
        }

        pub fn add_topping(&mut self, topping: String) {
            self.toppings.push(topping);
        }

        pub fn remove_topping(&mut self, topping: String) {
            let mut index = 0;
            for t in &self.toppings {
                if t == &topping {
                    break;
                }
                index += 1;
            }
            self.toppings.remove(index);
        }
    }

    pub mod help_customer{
        fn seat_at_table() {
            println!("Seating customer at table");
        }

        pub fn take_order(){
            seat_at_table();
            println!("Taking order");
            let cust_pizza: super::Pizza = 
                super::Pizza::new(vec![String::from("pepperoni"), String::from("mushrooms"), String::from("onions")]);
            serve_customer(cust_pizza);
        }

        fn serve_customer(cust_pizza: super::Pizza){
            println!("Serving customer");
            for i in cust_pizza.toppings {
                println!("Topping: {}", i);
            }
        }
    }

}


// out in main we can call order_food
pub fn order_food(){
    // crate::directory::module::function
    crate::restaurant::pizza_order::help_customer::take_order();
}