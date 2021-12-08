use std::collections::HashMap;

use priority_queue::DoublePriorityQueue;

use crate::util::{Point3, PointHash};

use super::path_finder::PathFinder;

#[derive(Clone)]
pub struct Dijkstra {
    maze: Vec<Vec<bool>>,
    end: Point3,
    progress: Vec<Point3>,
    prev: HashMap<PointHash, PointHash>,
    done: bool,
    vertex_set: DoublePriorityQueue<PointHash, u32>,
    start: Point3,
}

impl PathFinder for Dijkstra {
    fn new(maze: Vec<Vec<bool>>, start: Point3, end: Point3) -> Self {
        let prev = HashMap::new();
        let mut vertex_set = DoublePriorityQueue::new();
        let start_hash = start.hash();

        for (i, row) in maze.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if !*val {
                    let point = Point3::new(j as u16, i as u16).hash();
                    vertex_set.push(point, if point == start_hash { 0 } else { u32::MAX });
                }
            }
        }

        Dijkstra {
            maze,
            end,
            progress: vec![start],
            prev,
            done: false,
            vertex_set,
            start,
        }
    }

    fn step(&mut self) {
        if self.done {
            return;
        }
        if let Some((pt, dist)) = self.vertex_set.pop_min() {
            /*
            for neighbors:
                alt <- dist + 1;
                if alt < dist[v]:
                    dist[v] = alt
                    prev[v] = u
            */
            let pt = Point3::from(pt);
            let mut neighbors = vec![];
            {
                if pt.x > 0 {
                    neighbors.push(Point3::new(pt.x - 1, pt.y).hash());
                }
                if pt.y > 0 {
                    neighbors.push(Point3::new(pt.x, pt.y - 1).hash());
                }
                if pt.x < self.maze[0].len() as u16 - 1 {
                    neighbors.push(Point3::new(pt.x + 1, pt.y).hash());
                }
                if pt.x < self.maze[0].len() as u16 - 1 {
                    neighbors.push(Point3::new(pt.x + 1, pt.y).hash());
                }
                if pt.y < self.maze.len() as u16 - 1 {
                    neighbors.push(Point3::new(pt.x, pt.y + 1).hash());
                }
            }

            for neighbor in neighbors {
                if let Some((neigh, old_dist)) = self.vertex_set.remove(&neighbor) {
                    if old_dist == u32::MAX || dist < old_dist - 1 {
                        let alt = dist + 1;
                        self.vertex_set.push(neigh, alt);
                        self.prev.insert(neigh, pt.hash());
                    } else {
                        self.vertex_set.push(neigh, old_dist);
                    }
                }
            }
            self.progress.push(pt);
            if pt.hash() == self.end.hash() {
                self.done = true;
            }
        }
    }

    fn get_progress(&self) -> &Vec<Point3> {
        &self.progress
    }

    fn get_maze(&self) -> &Vec<Vec<bool>> {
        &self.maze
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
