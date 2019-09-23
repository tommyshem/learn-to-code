// pub is for public access from outside the module
// private function can be used inside the module

pub mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {
            println!("add to wait list");
        }

        fn seat_at_table() {
            println!("seat at table");
        }
    }

    pub mod serving {
        fn take_order() {
            println!("take order");
        }

        fn serve_order() {
            println!("serve order");
        }

        fn take_payment() {
            println!("take payment");
        }
        pub fn serve() {
            take_order();
            serve_order();
            take_payment();
        }
    }
}

#[test]
fn public_mod_test() {
    front_of_house::serving::serve();
}
