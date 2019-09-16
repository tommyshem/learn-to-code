// Information taken from the website http://bryce.fisher-fleig.org/blog/strategies-for-returning-references-in-rust/index.html
mod return_owned_structure;
mod return_boxed_structure;
mod return_higher_scope;

fn main() {
    let conn_owned = return_owned_structure::connect_owned(&String::from("testing again"));
    println!("{:?}", conn_owned);
    let conn_box = return_boxed_structure::connect_box(&String::from("testing again"));
    println!("{:?}", conn_box);

    let mut conn_scope = Structure {
        id: 1,
        url: String::from("test"),
    };
    let conn_higher_scope = return_higher_scope::connect_higher_scope(&mut conn_scope);
    println!("{:?}", conn_higher_scope);
}
// Structure to pass around for examples
#[derive(Debug)]
pub struct Structure {
    pub id: u8,
    pub url: String,
}


