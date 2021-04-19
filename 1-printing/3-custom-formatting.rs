use std::fmt;

#[derive(Debug)] // For debug formatting, unnecessary for custom formatting
struct Point2D {
    x: u8,
    y: u8
}

impl fmt::Display for Point2D {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point ({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point2D { x: 1, y: 5 };
    println!("{}", point); // custom formatting
    println!("{:?}", point); // debug formatting
}
