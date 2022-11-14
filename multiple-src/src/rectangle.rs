pub struct Rectangle{
    width:u32,
    height:u32
}
impl Rectangle{
    pub fn is_square(&self) -> bool{
        return self.width == self.height;
    }
    pub fn area(&self) -> u32{
        return self.width * self.height;
    }
    pub fn can_hold(&self, rectangle:&Rectangle) -> bool{
        return self.height > rectangle.height && self.width > rectangle.width;
    }
}

pub fn create_rectangle(height:u32,width:u32) -> Rectangle{
    return Rectangle{height: height, width: width};
}

pub fn create_square(height:u32) -> Rectangle{
    return Rectangle{height:height, width:height};
}
