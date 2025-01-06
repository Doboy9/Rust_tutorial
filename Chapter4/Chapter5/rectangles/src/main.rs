#[derive(Debug)]
struct Rectangle {
    width: u16,
    length: u16,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    println!("rect1 is {rect1:?}");
    println!("The area of the rectangle is {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u16 {
    rectangle.width * rectangle.length
}
