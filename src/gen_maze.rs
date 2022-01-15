use rand::Rng;

use crate::util::Point3;

// macro_rules! to_index {
//     ($a:expr, $b:expr) => {
//         ((($a) << 16) | ($b)).into()
//     };
// }

// macro_rules! from_index {
//     ($ind:expr) => {{
//         let tmp = $ind;
//         ((((tmp) >> 16) & 0xffff, (tmp) & 0xffff))
//     }};
// }

fn insert(walls: &mut Vec<u32>, element: u32) {
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
    let mut cur = Point3::new(0, 1);
    out[cur.y as usize][cur.x as usize] = false;

    let mut walls = vec![Point3::new(1, 1).hash()];
    walls.sort_unstable(); // lol clippy
    let mut rng = rand::thread_rng();

    while !walls.is_empty() {
        let index = rng.gen_range(0..walls.len());
        let element = Point3::from(walls.remove(index));

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

        {
            if element.x > 0 {
                if element.y > 0 && !out[element.y as usize - 1][element.x as usize - 1] {
                    unopen += 1;
                }
                if !out[element.y as usize][element.x as usize - 1] {
                    unopen += 1;
                }
                if (element.y as usize) < h - 1
                    && !out[element.y as usize + 1][element.x as usize - 1]
                {
                    unopen += 1;
                }
            }
            if (element.x as usize) < w - 1 {
                if element.y > 0 && !out[element.y as usize - 1][element.x as usize + 1] {
                    unopen += 1;
                }
                if !out[element.y as usize][element.x as usize + 1] {
                    unopen += 1;
                }
                if (element.y as usize) < h - 1
                    && !out[element.y as usize + 1][element.x as usize + 1]
                {
                    unopen += 1;
                }
            }
        }

        if element.y > 0 && !out[element.y as usize - 1][element.x as usize] {
            unopen += 1;
        }
        if (element.y as usize) < h - 1 && !out[(element.y as usize) + 1][element.x as usize] {
            unopen += 1;
        }

        if (1..=2).contains(&unopen)
            && !(out[element.y as usize + 1][element.x as usize]
                && out[element.y as usize - 1][element.x as usize]
                && out[element.y as usize][element.x as usize + 1]
                && out[element.y as usize][element.x as usize - 1])
        {
            cur = element;
            out[element.y as usize][element.x as usize] = false;

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

            if element.x > 1 && out[element.y as usize][element.x as usize - 1] {
                insert(&mut walls, Point3::new(element.x - 1, element.y).hash());
            }
            if (element.x as usize) < w - 2 && out[element.y as usize][element.x as usize + 1] {
                insert(&mut walls, Point3::new(element.x + 1, element.y).hash());
            }
            if element.y > 1 && out[element.y as usize - 1][element.x as usize] {
                insert(&mut walls, Point3::new(element.x, element.y - 1).hash());
            }
            if (element.y as usize) < h - 2 && out[element.y as usize + 1][element.x as usize] {
                insert(&mut walls, Point3::new(element.x, element.y + 1).hash());
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


    (out, cur)
}
