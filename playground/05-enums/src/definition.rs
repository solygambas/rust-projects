enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// with structs
// struct Ipv4Addr;
// struct Ipv6Addr;

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// with various types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// method
impl Message {
    fn call(&self) {}
}

// option enum (handling null)
enum Option<T> {
    None,
    Some(T),
}

pub fn run() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // route(IpAddrKind::V4);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = Option::None;
}

fn route(ip_kind: IpAddrKind) {}
