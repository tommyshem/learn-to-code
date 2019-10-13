#[allow(dead_code)]
#[derive(Debug)]
pub enum IpAddrKind {
    V4,
    V6,
}
#[allow(dead_code)]
fn route(ip_kind: IpAddrKind) {
    println!("route = {:#?}", ip_kind);
}

#[test]
fn use_enum_v4_test() {
    let ip_type = IpAddrKind::V4;
    route(ip_type);
}
#[test]
fn use_enum_v6_test() {
    let ip_type1 = IpAddrKind::V6;
    route(ip_type1);
}
