use std::collections::HashMap;

use crate::util::{Point3, PointHash};

use super::path_finder::PathFinder;

#[derive(Clone)]
pub struct Dfs {
    maze: Vec<Vec<bool>>,
    stack: Vec<Point3>,
    end: Point3,
    progress: Vec<Point3>,
    prev: HashMap<PointHash, PointHash>,
    done: bool,
    start: Point3,
}

impl PathFinder for Dfs {
    fn step(&mut self) {
        if self.done {
            return;
        }
        if let Some(current) = self.stack.pop() {
            if current.hash() == self.end.hash() {
                self.done = true;
                println!("done!");
            }
            let mut neighbors = vec![];
            if current.x > 0 {
                neighbors.push(Point3::new(current.x - 1, current.y));
            }
            if current.y < self.maze.len() as u16 - 1 {
                neighbors.push(Point3::new(current.x, current.y + 1));
            }
            if current.x < self.maze[0].len() as u16 - 1 {
                neighbors.push(Point3::new(current.x + 1, current.y));
            }
            if current.y > 0 {
                neighbors.push(Point3::new(current.x, current.y - 1));
            }
            for neighbor in neighbors {
                if !self.maze[neighbor.y as usize][neighbor.x as usize]
                    && !self.prev.contains_key(&neighbor.hash())
                {
                    self.stack.push(neighbor);
                    self.prev.insert(neighbor.hash(), current.hash());
                }
            }
            self.progress.push(current);
        }
    }

    fn get_progress(&self) -> &Vec<Point3> {
        &self.progress
    }

    fn get_maze(&self) -> &Vec<Vec<bool>> {
        &self.maze
    }

    fn new(maze: Vec<Vec<bool>>, start: Point3, end: Point3) -> Self {
        Dfs {
            maze,
            stack: vec![start],
            end,
            progress: vec![start],
            prev: HashMap::new(),
            done: false,
            start,
        }
    }

    fn end(&self) -> Point3 {
        self.end
    }
    fn done(&self) -> bool {
        self.done
    }

    fn get_estimated_path(&self) -> Vec<Point3> {
        if let Some(current) = self.progress.last() {
            let mut current = *current;
            let mut out = vec![current];

            while current.hash() != self.start.hash() {
                current = Point3::from(*self.prev.get(&current.hash()).unwrap());
                out.push(current);
            }

            out.reverse();
            out
        } else {
            vec![]
        }
    }
}


impl Dfs {
    pub fn get_considered(&self) -> &Vec<Point3> {
        &self.stack
    }
}