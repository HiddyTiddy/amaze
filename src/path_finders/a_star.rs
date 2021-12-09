use std::collections::HashMap;

use priority_queue::DoublePriorityQueue;

use crate::util::{get_neighbors, Point3, PointHash};

use super::path_finder::PathFinder;

#[derive(Clone)]
pub struct Astar {
    maze: Vec<Vec<bool>>,
    end: Point3,
    progress: Vec<Point3>,
    prev: HashMap<PointHash, PointHash>,
    done: bool,
    start: Point3,
    vertex_set: DoublePriorityQueue<PointHash, u32>,
}

fn h(node: Point3, end: Point3) -> u32 {
    // (((node.x as i32 - end.x as i32) * (node.x as i32 - end.x as i32)
    //     + (node.y as i32 - end.y as i32) * (node.y as i32 * end.y as i32)) as f32)
    //     .sqrt() as u32
    ((node.x as i32 - end.x as i32).abs() + (node.y as i32 - end.y as i32).abs()) as u32
}

impl PathFinder for Astar {
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

        Astar {
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
            let neighbors = get_neighbors(
                pt.x,
                pt.y,
                self.maze[0].len() as u16,
                self.maze.len() as u16,
            );

            for neighbor in neighbors {
                if let Some((neigh, old_dist)) = self.vertex_set.remove(&neighbor.hash()) {
                    if old_dist == u32::MAX || dist < old_dist - 1 {
                        let alt = dist + 1 + h(Point3::from(neigh), self.end);
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
