#[allow(dead_code)]
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
fn route(ip_kind: IpAddrKind) {
    println!("route = {:#?}", ip_kind);
}

#[test]
fn use_enum_v4_test() {
    let ip_type = IpAddrKind::V4(127, 0, 0, 1);
    route(ip_type);
}
#[test]
fn use_enum_v6_test() {
    let ip_type1 = IpAddrKind::V6(String::from("::1"));
    route(ip_type1);
}
