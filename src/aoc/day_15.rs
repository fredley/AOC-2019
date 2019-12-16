use crate::aoc::computer;
use crate::aoc::utils;

use pathfinding::directed::dijkstra;

use std::collections::HashMap;
use std::cmp;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

fn turn_left(dir: i64) -> i64 {
    return match dir {
        1 => 3,
        2 => 4,
        3 => 2,
        4 => 1,
        _ => -1,
    }
}

fn turn_right(dir: i64) -> i64 {
    return match dir {
        1 => 4,
        2 => 3,
        3 => 1,
        4 => 2,
        _ => -1,
    }
}

fn add_edge(graph: &mut HashMap<Coord, Vec<Coord>>, from: Coord, to: Coord) -> () {
    (*graph.entry(from).or_insert(vec![])).push(to);
    (*graph.entry(to).or_insert(vec![])).push(from);
}

pub fn day_fifteen(input: String) -> () {
    let mut x = 0;
    let mut y = 0;
    let mut memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    let mut computer = computer::Computer::new(vec![]);
    computer.set_memory(memory);
    // 1 north, 2 south, 3 west 4 east
    let mut dir = 2;
    let mut output = 0;
    let mut map: HashMap<Coord, char> = HashMap::new();
    let mut graph: HashMap<Coord, Vec<Coord>> = HashMap::new();
    let mut top_left = Coord{x: 0, y: 0};
    let mut bottom_right = Coord{x: 0, y: 0};
    let mut oxygen_pos = Coord{x: 1000000, y: 1000000};
    map.insert(Coord{x: x, y: y}, '.');
    let mut steps = 0;
    loop {
        let start_pos = Coord{x: x, y: y};
        computer.set_input(vec![dir]);
        let output = computer.run();
        if output == 0 {
            let wall_x = match dir {
                3 => x - 1,
                4 => x + 1,
                _ => x,
            };
            let wall_y = match dir {
                1 => y - 1,
                2 => y + 1,
                _ => y,
            };
            map.insert(Coord{x: wall_x, y: wall_y}, '#');
            top_left.x = cmp::min(top_left.x, wall_x);
            top_left.y = cmp::min(top_left.y, wall_y);
            bottom_right.x = cmp::max(bottom_right.x, wall_x);
            bottom_right.y = cmp::max(bottom_right.y, wall_y);
            dir = turn_right(dir);
        } else {
            // update pos
            x = match dir {
                3 => x - 1,
                4 => x + 1,
                _ => x,
            };
            y = match dir {
                1 => y - 1,
                2 => y + 1,
                _ => y,
            };
            let new_pos = Coord{x: x, y: y};
            add_edge(&mut graph, start_pos, new_pos);
            map.insert(new_pos, '.');
            top_left.x = cmp::min(top_left.x, x);
            top_left.y = cmp::min(top_left.y, y);
            bottom_right.x = cmp::max(bottom_right.x, x);
            bottom_right.y = cmp::max(bottom_right.y, y);
            steps += 1;
            if output == 2 {
                oxygen_pos = Coord{x: x, y: y};
            }
            //turn left and check wall
            dir = turn_left(dir);
            computer.set_input(vec![dir]);
            let output = computer.run();
            if output == 2 {
                x = match dir {
                    3 => x - 1,
                    4 => x + 1,
                    _ => x,
                };
                y = match dir {
                    1 => y - 1,
                    2 => y + 1,
                    _ => y,
                };
                oxygen_pos = Coord{x: x, y: y};
                break;
            } else if output == 0 {
                // turn right again
                let wall_x = match dir {
                    3 => x - 1,
                    4 => x + 1,
                    _ => x,
                };
                let wall_y = match dir {
                    1 => y - 1,
                    2 => y + 1,
                    _ => y,
                };
                map.insert(Coord{x: wall_x, y: wall_y}, '#');
                top_left.x = cmp::min(top_left.x, wall_x);
                top_left.y = cmp::min(top_left.y, wall_y);
                bottom_right.x = cmp::max(bottom_right.x, wall_x);
                bottom_right.y = cmp::max(bottom_right.y, wall_y);
                dir = turn_right(dir);
            } else {
                let new_pos = Coord{x: x, y: y};
                add_edge(&mut graph, start_pos, new_pos);
                // we moved round a corner
                // update pos
                x = match dir {
                    3 => x - 1,
                    4 => x + 1,
                    _ => x,
                };
                y = match dir {
                    1 => y - 1,
                    2 => y + 1,
                    _ => y,
                };
                let corner_pos = Coord{x: x, y: y};
                add_edge(&mut graph, new_pos, corner_pos);
                map.insert(corner_pos, '.');
                top_left.x = cmp::min(top_left.x, x);
                top_left.y = cmp::min(top_left.y, y);
                bottom_right.x = cmp::max(bottom_right.x, x);
                bottom_right.y = cmp::max(bottom_right.y, y);
                steps += 1;
            }
        }
        if x == 0 && y == 0 {
            break;
        }
    }
    //lets do some fucking djikstra
    println!("Oxygen pos: {:?} in steps {}", oxygen_pos, steps);
    let pathfinding_result = dijkstra::dijkstra_partial(&Coord{x: 0, y: 0}, |n| get_neighbours(&mut graph, *n).into_iter().map(|n| (n,1)), |n| n == &oxygen_pos);
    let path = dijkstra::build_path(&oxygen_pos, &pathfinding_result.0);

    // render world
    let mut j = top_left.y;
    let mut i: i64;
    while j <= bottom_right.y {
        i = top_left.x;
        while i <= bottom_right.x {
            let mut pixel: char = ' ';
            if i == 0 && j == 0 {
                pixel = 'S';
            } else if i == x && j == y {
                pixel = 'D';
            } else if i == oxygen_pos.x && j == oxygen_pos.y {
                pixel = 'O';
            } else if path.contains(&Coord{x: i, y: j}) {
                pixel = 'X';
            } else if map.contains_key(&Coord{x: i, y: j}) {
                pixel = *map.entry(Coord{x: i, y: j}).or_insert('.');
            }
            print!("{}", pixel);
            i += 1;
        }
        print!("\n");
        j += 1;
    }
    println!("Min cost: {:?}", pathfinding_result.0.get(&oxygen_pos));

    let oxygen_result = dijkstra::dijkstra_all(&oxygen_pos, |n| get_neighbours(&mut graph, *n).into_iter().map(|n| (n,1)));
    let values = oxygen_result.values().map(|v| v.1);
    let mut max_value = 0;
    for value in values {
        max_value = cmp::max(max_value, value);
    }
    println!("Time to flood with oxygen: {} minutes", max_value);
}

fn get_neighbours(graph: &mut HashMap<Coord, Vec<Coord>>, node: Coord) -> Vec<Coord> {
    return graph.entry(node).or_insert(vec![]).to_vec()
}