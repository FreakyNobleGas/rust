
// Define Rectangle Struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Define Methods for Rectangle struct. &self is always first parameter, unless you are
// defining an associated function, such as a constructor.
// impl stands for 'implementation' block
impl Rectangle {
    // Square Constructor (Note: Since this is associated function, there is no need for self parameter)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    let sq = Rectangle::square(3);

    println!(
        "Can Rect2 fit in Rect1? {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can Rect2 fit in Sq? {}",
        sq.can_hold(&rect2)
    );

    // Note area function is borrowing rect1 struct
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Derived Debug Statement
    //dbg!(&rect1);

    if rect1.width() {
        println!("Rectangle has a non-zero width of {}", rect1.width);
    }
}

// No longer needed after defining struct impl block
//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}
