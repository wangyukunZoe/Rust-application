// 枚举允许我们列举所有可能的值来定义一个类型

enum IpAddressKind {
    V4,
    V6,
}

struct IpAddress{
    kind: IpAddressKind,
    address: String,
}

fn main() {

    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };

    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    route(four);
    route(six);
    route(IpAddressKind::V6);

}

fn route(ip_kind: IpAddressKind)
{

}