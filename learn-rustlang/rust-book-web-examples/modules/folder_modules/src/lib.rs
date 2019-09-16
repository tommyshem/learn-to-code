mod front_of_house;

pub use crate::front_of_house::serving;


#[test]
fn eat_test() {
    serving::serve();
}