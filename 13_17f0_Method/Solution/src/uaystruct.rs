pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn perimeter(&self) -> u32 {
        2*self.width + 2*self.height
    }
}