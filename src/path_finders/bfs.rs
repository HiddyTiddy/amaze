use crate::util::{Point3, PointHash};
use std::collections::{HashMap, VecDeque};

use super::path_finder::PathFinder;

#[derive(Clone)]
pub struct Bfs {
    maze: Vec<Vec<bool>>,
    queue: VecDeque<Point3>,
    end: Point3,
    progress: Vec<Point3>,
    prev: HashMap<PointHash, PointHash>,
    done: bool,
    start: Point3,
}

impl PathFinder for Bfs {
    fn step(&mut self) {
        if self.done {
            return;
        }
        if let Some(current) = self.queue.pop_front() {
            if current.hash() == self.end.hash() {
                self.done = true;
            }
            let mut neighbors = vec![];
            if current.x > 0 {
                neighbors.push((-1, 0));
            }
            if current.x < self.maze[0].len() as u16 - 1 {
                neighbors.push((1, 0));
            }
            if current.y > 0 {
                neighbors.push((0, -1));
            }
            if current.y < self.maze.len() as u16 - 1 {
                neighbors.push((0, 1));
            }
            for neighbor in neighbors {
                let new = Point3::new(
                    (current.x as i32 + neighbor.0) as u16,
                    (current.y as i32 + neighbor.1) as u16,
                );
                let key = new.hash();
                if !self.maze[new.y as usize][new.x as usize] && !self.prev.contains_key(&key) {
                    self.queue.push_back(new);
                    self.prev.insert(key, current.hash());
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
        Bfs {
            maze,
            queue: VecDeque::from([start]),
            end,
            progress: vec![start],
            prev: HashMap::new(),
            done: false,
            start
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
