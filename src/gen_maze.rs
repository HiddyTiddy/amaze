use crate::Point3;
use rand::Rng;

macro_rules! to_index {
    ($a:expr, $b:expr) => {
        ((($a) << 16) | ($b))
    };
}

macro_rules! from_index {
    ($ind:expr) => {{
        let tmp = $ind;
        ((((tmp) >> 16) & 0xff, (tmp) & 0xff))
    }};
}

fn insert(walls: &mut Vec<usize>, element: usize) {
    for (i, j) in walls.iter().enumerate() {
        if *j == element {
            return;
        }
        if *j > element {
            walls.insert(i, element);
            return;
        }
    }
    walls.push(element);
}

pub fn gen_maze(h: u16, w: u16) -> (Vec<Vec<bool>>, Point3) {
    /*
    Start with a grid full of walls.
    Pick a cell, mark it as part of the maze. Add the walls of the cell to the wall list.
    While there are walls in the list:
        Pick a random wall from the list. If only one of the cells that the wall divides is visited, then:
            Make the wall a passage and mark the unvisited cell as part of the maze.
            Add the neighboring walls of the cell to the wall list.
        Remove the wall from the list.
    */
    let w = w as usize;
    let h = h as usize;
    let mut out = vec![vec![true; w]; h];
    let mut cur = (0, 1);
    out[cur.1][cur.0] = false;

    let mut walls = vec![to_index!(1, 1)];
    walls.sort_unstable(); // lol clippy
    let mut rng = rand::thread_rng();

    while !walls.is_empty() {
        let index = rng.gen_range(0..walls.len());
        // println!("{} / {}", index, walls.len());
        let element = from_index!(walls.remove(index));

        let mut unopen = 0;
        { // if element.0 > 0 && !out[element.1][element.0 - 1] {
             //     unopen += 1;
             // }
             // if element.0 < w - 1 && !out[element.1][element.0 + 1] {
             //     unopen += 1;
             // }
             // if element.1 > 0 && !out[element.1 - 1][element.0] {
             //     unopen += 1;
             // }
             // if element.1 < h - 1 && !out[element.1 + 1][element.0] {
             //     unopen += 1;
             // }
        }

        // println!("{:?}", walls);
        {
            if element.0 > 0 {
                if element.1 > 0 && !out[element.1 - 1][element.0 - 1] {
                    unopen += 1;
                }
                if !out[element.1][element.0 - 1] {
                    unopen += 1;
                }
                if element.1 < h - 1 && !out[element.1 + 1][element.0 - 1] {
                    unopen += 1;
                }
            }
            if element.0 < w - 1 {
                if element.1 > 0 && !out[element.1 - 1][element.0 + 1] {
                    unopen += 1;
                }
                if !out[element.1][element.0 + 1] {
                    unopen += 1;
                }
                if element.1 < h - 1 && !out[element.1 + 1][element.0 + 1] {
                    unopen += 1;
                }
            }
        }

        if element.1 > 0 && !out[element.1 - 1][element.0] {
            unopen += 1;
        }
        if element.1 < h - 1 && !out[element.1 + 1][element.0] {
            unopen += 1;
        }

        if (1..=2).contains(&unopen)
            && !(out[element.1 + 1][element.0]
                && out[element.1 - 1][element.0]
                && out[element.1][element.0 + 1]
                && out[element.1][element.0 - 1])
        {
            cur = (element.0, element.1);
            out[element.1][element.0] = false;

            // if element.0 > 1 {
            //     if element.1 > 1 && out[element.1 - 1][element.0 - 1] {
            //         insert(&mut walls, to_index!(element.0-1, element.1-1));
            //     }
            //     if out[element.1][element.0 - 1] {
            //         insert(&mut walls, to_index!(element.0-1, element.1));
            //     }
            //     if element.1 < h - 2 && out[element.1 + 1][element.0 - 1] {
            //         insert(&mut walls, to_index!(element.0-1, element.1+1));
            //     }
            // }
            // if element.0 < w - 2 {
            //     if element.1 > 1 && out[element.1 - 1][element.0 + 1] {
            //         insert(&mut walls, to_index!(element.0+1, element.1-1));
            //     }
            //     if out[element.1][element.0 + 1] {
            //         insert(&mut walls, to_index!(element.0+1, element.1));
            //     }
            //     if element.1 < h - 2 && out[element.1 + 1][element.0 + 1] {
            //         insert(&mut walls, to_index!(element.0+1, element.1+1));
            //     }
            // }

            // if element.1 > 1 && out[element.1 - 1][element.0] {
            //     insert(&mut walls, to_index!(element.0, element.1-1));
            // }
            // if element.1 < h - 2 && out[element.1 + 1][element.0] {
            //     insert(&mut walls, to_index!(element.0, element.1+1));
            // }

            if element.0 > 1 && out[element.1][element.0 - 1] {
                insert(&mut walls, to_index!(element.0 - 1, element.1));
            }
            if element.0 < w - 2 && out[element.1][element.0 + 1] {
                insert(&mut walls, to_index!(element.0 + 1, element.1));
            }
            if element.1 > 1 && out[element.1 - 1][element.0] {
                insert(&mut walls, to_index!(element.0, element.1 - 1));
            }
            if element.1 < h - 2 && out[element.1 + 1][element.0] {
                insert(&mut walls, to_index!(element.0, element.1 + 1));
            }
        }

        // if element.0 > 0 && out[element.1][element.0 - 1] {
        //     add!(walls, w, h, out, element.0 - 1, element.1);
        // }
        // if element.1 > 0 {
        //     add!(walls, w, h, out, element.0, element.1 - 1);
        //     // insert(&mut walls, to_index!(element.0, element.1 - 1));
        //     // out[element.1 - 1][element.0] = false;
        // }
        // if element.0 < w - 1 {
        //     add!(walls, w, h, out, element.0 + 1, element.1);
        //     // insert(&mut walls, to_index!(element.0 + 1, element.1));
        //     // out[element.1][element.0 + 1] = false;
        // }
        // if element.1 < h - 1 {
        //     add!(walls, w, h, out, element.0, element.1 + 1);
        //     // insert(&mut walls, to_index!(element.0, element.1 + 1));
        //     // out[element.1 + 1][element.0] = false;
        // }
    }

    println!("{}", out[cur.1][cur.0]);

    (out, Point3::new(cur.1 as u16, cur.0 as u16))
}
