#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 4,
        height: 5,
    };

    let rect_kwegezamu = Rectangle {
        width: 6,
        height: 7,
    };

    println!("rect:{:?}", rect);
    println!("rect2: {:?}", rect_kwegezamu);

    println!("area rect: {}", rect.area());
    println!("area rect_kwegezamu: {}", rect_kwegezamu.area());

    println!("is rect bigger?: {}", rect.can_hold(&rect_kwegezamu));
}
