#[test]
fn new_string_test() {
    let s = String::new();
    println!("New string {:?}", s);
}

// In this case, String::from and to_string do the same thing, so which you choose is a matter of style.
#[test]
fn to_string_test() {
    let data = "initial contents";
    let s = data.to_string();
    println!("string {:?}", s);

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();
}

#[test]
fn from_string_test() {
    let data = "initial contents";
    let _s = String::from(data);
    let _s = String::from("initial contents");
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
}
