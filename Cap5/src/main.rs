#![allow(unused)]
fn main() {
#[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.length * self.width 
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }
}

