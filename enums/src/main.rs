enum IpAddressKind {
    V4,
    V6,
}

// We can organise and use enums with structs to associated data with the new variants fo the type.
struct IpAddress {
    ip_addr_kind: IpAddressKind,
    address: String,
}

// however it is easier to just specify the data associated with the enums, basically case class notation for the types ADT.
// There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
// Version four IP addresses will always have four numeric components that will have values between 0 and 255.
// If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:

enum IpAddressKindV2 {
    V4(u8, u8, u8, u8),  // instead of a String we can restrict it to 4 u8 values
    V6(String),
}

fn using_enums() {
    let ipv4 = IpAddressKind::V4;
    let ipv6 = IpAddressKind::V6;
}

fn route(ip_kind: IpAddressKind) {}

// Multi data type enums
// we define an enum here since the alternative is to use multiple structs, however that would prevent us from defining a function that takes all of that data in a convenient way.
// this way all the data is grouped under a single Messages type.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can also use impl when working with enums

impl Message {
    fn call(&self) {
        // some implementation
    }
}

fn impl_enums() {
    let m = Message::Write(String::from("hello"));
    m.call()
}


fn main() {
    let ipv4 = IpAddressKind::V4;
    let ipv6 = IpAddressKind::V6;

    route(ipv4);
    route(ipv6);

    let home =
        IpAddress {
            ip_addr_kind: IpAddressKind::V4,
            address: String::from("127.0.0.1"),
        };

    let loopback =
        IpAddress {
            ip_addr_kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

    let home_v2 =
        IpAddressKindV2::V4(127, 0, 0, 1);

    let loopback_v2 =
        IpAddressKindV2::V6(String::from("::1"));


    println!("Hello, world!");
}
