#[derive(Clone, Copy)]
pub struct Point3 {
    pub x: u16,
    pub y: u16
}

impl Point3 {
    pub fn new(x: u16, y:u16) -> Self {
        Point3{x,y}
    }
}

impl PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}