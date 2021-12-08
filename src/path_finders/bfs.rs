use crate::util::Point3;
use std::collections::{HashSet, VecDeque};

use super::path_finder::PathFinder;

#[derive(Clone)]
pub struct Bfs {
    maze: Vec<Vec<bool>>,
    queue: VecDeque<Point3>,
    end: Point3,
    progress: Vec<Point3>,
    seen: HashSet<u32>,
    done: bool,
}

impl PathFinder for Bfs {
    fn step(&mut self) {
        if self.done {
            return;
        }
        if let Some(current) = self.queue.pop_front() {
            if current == self.end {
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
                let key = (new.x as u32) << 16 | new.y as u32;
                if !self.maze[new.y as usize][new.x as usize] && !self.seen.contains(&key) {
                    self.queue.push_back(new);
                    self.seen.insert(key);
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
            seen: HashSet::new(),
            done: false,
        }
    }

    fn end(&self) -> Point3 {
        self.end
    }
    fn done(&self) -> bool {
        self.done
    }
    fn get_estimated_path(&self) -> Vec<Point3> {
        todo!()
    }
}
