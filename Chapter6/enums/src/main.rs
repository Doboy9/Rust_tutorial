enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// No need now
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,                       // Quit has no data associated with it at all.
    Move { x: i32, y: i32 },    //Move has named fields, like a struct does.
    Write(String),              //Write includes a single String.
    ChangeColor(i32, i32, i32), //ChangeColor includes three i32 values.
}

impl Message {
    fn call(&self) {
        //Code
    }
}

fn main() {
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);

    println!("{}", sum);
}
