// pub has to be in front of the mod to be used out side of the module
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {println!("waitlist");}

        pub fn seat_at_table() {println!("seat at table");}
    }

    pub mod serving {
        fn take_order() {println!("take order");}

        fn serve_order() {println!("serve order"); {}

        pub fn take_order_and_serve(){
            take_order();
            serve_order();
        }
    }
}}

#[test]
fn mod_hosting_test() {
    println!("Hosting Testing");
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
}
// this test fails as the functions are private by default and to make the public 
// you need to put pub keyword in front of the functrion
//#[test]
//fn mod_serving_test() {
//    println!("Serving Testing");
//    front_of_house::serving::take_order();
//    front_of_house::serving::serve_order();
//}

#[test]
fn mod_all_test() {
    front_of_house::serving::take_order_and_serve();
}