#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    // We do not want to take ownership
    fn area(&self) -> u16 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn square(size: u16) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square {:#?}", sq);
}
