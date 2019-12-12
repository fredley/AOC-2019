use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    pub fn energy(&self) -> i64 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Moon {
    pos: Coord,
    vel: Coord,
}

impl Moon {
    pub fn new(pos: Coord) -> Moon {
        return Moon{
            pos: pos,
            vel: Coord{x:0, y:0, z:0},
        }
    }

    pub fn energy(&self) -> i64 {
        return self.pos.energy() * self.vel.energy();
    }
}

pub fn day_twelve() -> () {
    let moons = vec![
        Moon::new(Coord{x:-8, y:-9, z:-7}),
        Moon::new(Coord{x:-5, y:2, z:-1}),
        Moon::new(Coord{x:11, y:8, z:-14}),
        Moon::new(Coord{x:1, y:-4, z:-11}),
    ];
    cycle_time(&mut moons.clone());
}

fn x_state(moons: Vec<Moon>) -> Vec<i64> {
    return vec![
        moons[0].pos.x,
        moons[1].pos.x,
        moons[2].pos.x,
        moons[3].pos.x,
        moons[0].vel.x,
        moons[1].vel.x,
        moons[2].vel.x,
        moons[3].vel.x,
    ]
}

fn y_state(moons: Vec<Moon>) -> Vec<i64> {
    return vec![
        moons[0].pos.y,
        moons[1].pos.y,
        moons[2].pos.y,
        moons[3].pos.y,
        moons[0].vel.y,
        moons[1].vel.y,
        moons[2].vel.y,
        moons[3].vel.y,
    ]
}

fn z_state(moons: Vec<Moon>) -> Vec<i64> {
    return vec![
        moons[0].pos.z,
        moons[1].pos.z,
        moons[2].pos.z,
        moons[3].pos.z,
        moons[0].vel.z,
        moons[1].vel.z,
        moons[2].vel.z,
        moons[3].vel.z,
    ]
}

fn cycle_time(moons: &mut Vec<Moon>) -> () {
    // find cycle x
    let len = moons.len();

    let mut seen_states_x: HashSet<Vec<i64>> = HashSet::new();
    let mut seen_states_y: HashSet<Vec<i64>> = HashSet::new();
    let mut seen_states_z: HashSet<Vec<i64>> = HashSet::new();
    let mut found_x = false;
    let mut found_y = false;
    let mut found_z = false;
    let mut i = 0;
    while !found_x || !found_y || !found_z {
        if i == 1000 {
            let mut total_energy = 0;
            let mut j = 0;
            while j < len {
                total_energy += moons[j].energy();
                j += 1;
            }
            println!("Total Energy: {}", total_energy);
        }
        let mut moon_a_idx = 0;
        while moon_a_idx < len {
            let moon_a = moons[moon_a_idx];
            if moon_a_idx < len - 1 {
                let mut moon_b_idx = moon_a_idx + 1;
                while moon_b_idx < len {
                    let moon_b = moons[moon_b_idx];
                    if moon_a.pos.x > moon_b.pos.x {
                        moons[moon_a_idx].vel.x -= 1;
                        moons[moon_b_idx].vel.x += 1;
                    } else if moon_a.pos.x < moon_b.pos.x {
                        moons[moon_a_idx].vel.x += 1;
                        moons[moon_b_idx].vel.x -= 1;
                    }
                    if moon_a.pos.y > moon_b.pos.y {
                        moons[moon_a_idx].vel.y -= 1;
                        moons[moon_b_idx].vel.y += 1;
                    } else if moon_a.pos.y < moon_b.pos.y {
                        moons[moon_a_idx].vel.y += 1;
                        moons[moon_b_idx].vel.y -= 1;
                    }
                    if moon_a.pos.z > moon_b.pos.z {
                        moons[moon_a_idx].vel.z -= 1;
                        moons[moon_b_idx].vel.z += 1;
                    } else if moon_a.pos.z < moon_b.pos.z {
                        moons[moon_a_idx].vel.z += 1;
                        moons[moon_b_idx].vel.z -= 1;
                    }
                    moon_b_idx += 1;
                }
            }
            moons[moon_a_idx].pos.x += moons[moon_a_idx].vel.x;
            moons[moon_a_idx].pos.y += moons[moon_a_idx].vel.y;
            moons[moon_a_idx].pos.z += moons[moon_a_idx].vel.z;
            moon_a_idx += 1;
        }
        if !found_x {
            let state = x_state(moons.to_vec());
            if seen_states_x.contains(&state) {
                found_x = true
            }
            seen_states_x.insert(state);
        }
        if !found_y {
            let state = y_state(moons.to_vec());
            if seen_states_y.contains(&state) {
                found_y = true
            }
            seen_states_y.insert(state);
        }
        if !found_z {
            let state = z_state(moons.to_vec());
            if seen_states_z.contains(&state) {
                found_z = true
            }
            seen_states_z.insert(state);
        }
        i += 1;
    }
    // find lcm of x,y,z
    let lcm_x_y = lcm(seen_states_x.len(), seen_states_y.len());
    println!("Loop Period:  {}", lcm(lcm_x_y, seen_states_z.len()));
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}
 
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}