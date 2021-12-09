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



pub fn get_neighbors(x: u16, y: u16, w: u16, h: u16) -> Vec<Point3> {
    let mut neighbors = vec![];

    if x != 0 {
        neighbors.push(Point3::new(x - 1, y));
    }
    if y != 0 {
        neighbors.push(Point3::new(x, y - 1));
    }
    if x < w - 1 {
        neighbors.push(Point3::new(x + 1, y));
    }
    if y < h - 1 {
        neighbors.push(Point3::new(x, y + 1));
    }

    neighbors
}