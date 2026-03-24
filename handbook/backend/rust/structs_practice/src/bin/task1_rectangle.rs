/// Task 1: Rectangle — area and perimeter.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Returns the rectangle area.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Returns the rectangle perimeter.
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    /// Constructs a square with the given side length.
    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "rect: {:?}, area: {}, perimeter: {}",
        rect,
        rect.area(),
        rect.perimeter()
    );

    let sq = Rectangle::square(5);
    println!("square: {:?}, area: {}", sq, sq.area());
}

