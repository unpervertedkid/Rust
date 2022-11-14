pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn is_square(&self) -> bool{
        return self.height == self.width;
    }
}
pub fn create_square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
}

pub fn create_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}
