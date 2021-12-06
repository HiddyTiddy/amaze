use crate::util::Point3;


pub trait PathFinder {
    fn new(maze: Vec<Vec<bool>>,start: Point3, end: Point3) -> Self;
    fn step(&mut self);
    fn get_progress(&self) -> &Vec<Point3>;
    fn get_maze(&self) -> &Vec<Vec<bool>>;
    fn end(&self) -> Point3;
    fn done(&self) -> bool;
}