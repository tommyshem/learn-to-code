#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str, // needed for reference to objects not owned by the struct eg string slice
}
#[allow(dead_code)]
fn createstruct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}

#[test]
fn createstruct_test() {
    createstruct();
}
