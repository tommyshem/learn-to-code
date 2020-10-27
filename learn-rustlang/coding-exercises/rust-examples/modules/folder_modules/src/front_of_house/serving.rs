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