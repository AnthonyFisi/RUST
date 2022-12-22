#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:?}",self);
    }
}


enum Option<T> {
    None,
    Some(T),
}
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    /*struct IpAddr{
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr{
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr{
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
    */



    let m = Message::Write(String::from("hello"));
    m.call();


    // Null value
    /*let x: i8 = 5;
    let mut y: Option<i8> = None;

    y = 10;

    let sum = x + y;*/

    let x = Some("air");
    println!("{}",x.unwrap());


 


}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}