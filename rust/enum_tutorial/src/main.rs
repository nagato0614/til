#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message
{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
}
