fn main() {
    println!("Hello, world!");
}

#[test]
fn new_vector_test() {
    let v: Vec<i32> = Vec::new();
    println!("new vector = {:?}", v);
}

#[test]
fn init_vector_test() {
    let v = vec![1, 2, 3];
    println!("init vector = {:?}", v);
}

#[test]
fn update_vector_test() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("updating vector = {:?}", v);
}

#[test]
fn remove_element_vector_test() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("vector {:?}", v);
    v.pop();
    println!("remove element {:?}", v);
}

#[test]
fn accessing_vector_value_test() {
    //two ways to access the vector values
    let v = vec![1, 2, 3, 4, 5];
    //use index to access yje value in the vector
    //returns the value note will panic if out of range
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // using get to access the value in the vector
    // returns option note will not panic just returns none
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.",),
    }
}

#[test]
fn iterate_all_items_vector_test() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

#[test]
fn iterate_all_mut_items_vector_test() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

#[test]
fn add_enum_to_vector_test() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("spreadsheetcell {:?}", row);
}
