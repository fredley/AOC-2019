use std::collections::HashMap;

use pathfinding::directed::dijkstra;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
    depth: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        return Coord{x: x, y: y, depth:1}
    }
    pub fn newd(x: usize, y: usize, depth: usize) -> Coord {
        return Coord{x: x, y: y, depth:depth}
    }
}

pub fn day_twenty(input: String) -> () {
    let lines = input.split("\n");
    let mut x: usize;
    let mut y = 0;
    let mut maze: HashMap<Coord, char> = HashMap::new();
    let mut map: HashMap<Coord, Vec<Coord>> = HashMap::new();
    let mut portals: HashMap<String, Coord> = HashMap::new();
    for line in lines.clone() {
        x = 0;
        for letter in line.chars() {
            maze.insert(Coord::new(x, y), letter);
            x += 1;
        }
        y += 1;
    }
    y = 0;
    let mut aa: Coord = Coord::new(1, 1);
    let mut zz: Coord = Coord::new(1, 1);
    for line in lines {
        x = 0;
        for letter in line.chars() {
            if letter == '.' {
                // path
                let mut neighbours: Vec<Coord> = Vec::new();
                let coord = Coord::new(x, y);
                let east_coord = Coord::new(x+1, y);
                let east = *maze.get(&east_coord).unwrap();
                if east == '.' {
                    neighbours.push(east_coord);
                } else if east != '#' {
                    // portal
                    let next = *maze.get(&Coord::new(x+2, y)).unwrap();
                    let key = vec![east, next].into_iter().collect();
                    if key == "AA" {
                        aa = coord;
                    } else if key == "ZZ" {
                        zz = coord;
                    }
                    if portals.contains_key(&key) {
                        let target = *portals.get(&key).unwrap();
                        neighbours.push(target);
                        let mut other = map.get(&target).unwrap().to_vec();
                        other.push(coord);
                        map.insert(target, other);
                    } else {
                        portals.insert(key, coord);
                    }
                }
                let west_coord = Coord::new(x-1, y);
                let west = *maze.get(&west_coord).unwrap();
                if west == '.' {
                    neighbours.push(west_coord);
                } else if west != '#' {
                    // portal
                    let prev = *maze.get(&Coord::new( x-2,  y)).unwrap();
                    let key = vec![prev, west].into_iter().collect();
                    if key == "AA" {
                        aa = coord;
                    } else if key == "ZZ" {
                        zz = coord;
                    }
                    if portals.contains_key(&key) {
                        let target = *portals.get(&key).unwrap();
                        neighbours.push(target);
                        let mut other = map.get(&target).unwrap().to_vec();
                        other.push(coord);
                        map.insert(target, other);
                    } else {
                        portals.insert(key, coord);
                    }
                }
                let south_coord = Coord::new(x, y+1);
                let south = *maze.get(&south_coord).unwrap();
                if south == '.' {
                    neighbours.push(south_coord);
                } else if south != '#' {
                    // portal
                    let next = *maze.get(&Coord::new(x, y+2)).unwrap();
                    let key = vec![south, next].into_iter().collect();
                    if key == "AA" {
                        aa = coord;
                    } else if key == "ZZ" {
                        zz = coord;
                    }
                    if portals.contains_key(&key) {
                        let target = *portals.get(&key).unwrap();
                        neighbours.push(target);
                        let mut other = map.get(&target).unwrap().to_vec();
                        other.push(coord);
                        map.insert(target, other);
                    } else {
                        portals.insert(key, coord);
                    }
                }
                let north_coord = Coord::new(x, y-1);
                let north = *maze.get(&north_coord).unwrap();
                if north == '.' {
                    neighbours.push(north_coord);
                } else if north != '#' {
                    // portal
                    let prev = *maze.get(&Coord::new( x,  y-2)).unwrap();
                    let key = vec![prev, north].into_iter().collect();
                    if key == "AA" {
                        aa = coord;
                    } else if key == "ZZ" {
                        zz = coord;
                    }
                    if portals.contains_key(&key) {
                        let target = *portals.get(&key).unwrap();
                        neighbours.push(target);
                        let mut other = map.get(&target).unwrap().to_vec();
                        other.push(coord);
                        map.insert(target, other);
                    } else {
                        portals.insert(key, coord);
                    }
                }
                map.insert(coord, neighbours);
            }
            x += 1;
        }
        y += 1;
    }
    // Now do the djikstra
    let pathfinding_result = dijkstra::dijkstra_partial(&aa, |n| get_neighbours(&mut map, *n).into_iter().map(|n| (n,1)), |n| n == &zz);
    println!("Path length: {:?}", pathfinding_result.0.get(&zz).unwrap().1);

    let recursive_result = dijkstra::dijkstra_partial(&aa, |n| get_neighbours_recursive(&mut map, &aa, &zz, *n).into_iter().map(|n| (n,1)), |n| n == &zz);
    println!("Recursive Path length: {:?}", recursive_result.0.get(&zz).unwrap().1);
}

fn get_neighbours(graph: &mut HashMap<Coord, Vec<Coord>>, node: Coord) -> Vec<Coord> {
    return graph.entry(node).or_insert(vec![]).to_vec();
}

fn get_neighbours_recursive(graph: &mut HashMap<Coord, Vec<Coord>>, aa: &Coord, zz: &Coord, node: Coord) -> Vec<Coord> {
    let lookup_node = Coord::new(node.x, node.y);
    let neighbours = graph.entry(lookup_node).or_insert(vec![]).to_vec();
    if node.depth == 1 {
        // we're on the outermost level
        let mut filtered: Vec<Coord> = Vec::new();
        // outer-edge portals are walls
        for neighbour in neighbours {
            if is_outer_portal(&node, &neighbour){
                continue
            }
            if is_inner_portal(&node, &neighbour){
                // add one to depth
                // println!("Sinking to depth {} at {:?} {:?}", node.depth + 1, node, neighbour);
                filtered.push(Coord::newd(neighbour.x, neighbour.y, node.depth + 1));
            } else {
                filtered.push(neighbour);
            }
        }
        return filtered;
    } else {
        // we're in an inner level
        let mut filtered: Vec<Coord> = Vec::new();
        // remove aa and zz
        for neighbour in neighbours {
            if neighbour != *aa && neighbour != *zz {
                if is_outer_portal(&node, &neighbour){
                    // subtract one from depth
                    filtered.push(Coord::newd(neighbour.x, neighbour.y, node.depth - 1));
                } else if is_inner_portal(&node, &neighbour){
                    // add one to depth
                    // println!("Sinking to depth {} at {:?} {:?}", node.depth + 1, node, neighbour);
                    filtered.push(Coord::newd(neighbour.x, neighbour.y, node.depth + 1));
                } else {
                    filtered.push(Coord::newd(neighbour.x, neighbour.y, node.depth));
                }
            }
        }
        return filtered;
    }
}

fn is_inner_portal(node: &Coord, neighbour: &Coord) -> bool {
    let x1 = node.x as i32;
    let x2 = neighbour.x as i32;
    let y1 = node.y as i32;
    let y2 = neighbour.y as i32;
    return (node.x == 32 ||
    node.x == 88 ||
    node.y == 32 ||
    node.y == 86) && ((x1 - x2).abs() > 1 || (y1 - y2).abs() > 1);
}

fn is_outer_portal(node: &Coord, neighbour: &Coord) -> bool {
    return node.x == 2 && neighbour.x > 3 ||
    node.x == 118 && neighbour.x < 117 ||
    node.y == 2 && neighbour.y > 3 ||
    node.y == 116 && neighbour.y < 115
}

// fn is_inner_portal(node: &Coord, neighbour: &Coord) -> bool {
//     let x1 = node.x as i32;
//     let x2 = neighbour.x as i32;
//     let y1 = node.y as i32;
//     let y2 = neighbour.y as i32;
//     return (x1 == 8 ||
//     x1 == 36 ||
//     y1 == 8 ||
//     y1 == 28) && ((x1 - x2).abs() > 1 || (y1 - y2).abs() > 1);
// }

// fn is_outer_portal(node: &Coord, neighbour: &Coord) -> bool {
//     return node.x == 2 && neighbour.x > 3 ||
//     node.x == 42 && neighbour.x < 41 ||
//     node.y == 2 && neighbour.y > 3 ||
//     node.y == 34 && neighbour.y < 33
// }
