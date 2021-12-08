#[derive(Clone, Copy, Debug, Default)]
pub struct Point3 {
    pub x: u16,
    pub y: u16,
}

pub type PointHash = u32;

impl Point3 {
    pub fn new(x: u16, y: u16) -> Self {
        Point3 { x, y }
    }

    pub fn hash(&self) -> u32 {
        (self.x as u32) << 16 | self.y as u32
    }

    pub fn from<T>(t: T) -> Point3
    where
        T: Into<u32>,
    {
        let t: u32 = t.into();
        Point3 {
            x: (t >> 16 & 0xffff) as u16,
            y: (t & 0xffff) as u16,
        }
    }
}

impl PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
