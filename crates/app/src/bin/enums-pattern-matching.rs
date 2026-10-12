#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({x}, {y})"),
            Message::Write(text) => println!("Write: {text}"),
            Message::ChangeColor(r, g, b) => println!("Change color to rgb({r}, {g}, {b})"),
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    
    describe(&home);
    describe(&loopback);

    let q = Message::Quit;
    let m = Message::Move {
        x: 10,
        y: 20,
    };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(255, 0, 0);
    
    q.call();
    m.call();
    w.call();
    c.call();
}


fn describe(ip: &IpAddrKind) {
    match ip {
        IpAddrKind::V4(a, b, c, d) => println!("IPv4: {a}.{b}.{c}.{d}"),
        IpAddrKind::V6(addr) => println!("IPv6: {addr}"),
    }
}
