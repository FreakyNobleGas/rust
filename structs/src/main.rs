#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Note area function is borrowing rect1 struct
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
