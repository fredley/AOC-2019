use std::collections::HashMap;
use std::collections::HashSet;
use std::f64;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    pub fn angle_to(&self, other: Asteroid) -> i64 {
        let denominator = (other.y - self.y) as f64;
        let angle = ((other.x - self.x) as f64).atan2(denominator) + f64::consts::PI;
        return (angle * 10000000.0) as i64;
    }

    pub fn distance(&self) -> i64 {
        return ((((20 - self.y).pow(2) + (31 - self.x).pow(2)) as f64).sqrt() * 1000000.0) as i64;
    }

    pub fn equals(&self, other: Asteroid) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

pub fn day_ten(input: String) -> () {
    let mut asteroids: Vec<Asteroid> = Vec::new();
    // load map
    let mut y: i32 = 0;
    for row in input.split("\n") {
        let mut x = 0;
        while x < row.len() {
            if row.chars().nth(x).unwrap() == '#' {
                asteroids.push(Asteroid{x: x as i32, y: y});
            }
            x += 1;
        }
        y += 1;
    }
    // For each asteroid, calculate the angle to every other asteroid
    // The number it can see are the number of unique angles
    let mut best_asteroid: Asteroid = Asteroid{x: 0, y: 0};
    let mut most_seen = 0;
    let mut angle_map: HashMap<i64, Vec<Asteroid>> = HashMap::new();
    for asteroid in &asteroids {
        let mut angle_set: HashSet<i64> = HashSet::new();
        for other_asteroid in &asteroids {
            if other_asteroid.equals(*asteroid) {
                continue
            }
            let angle = asteroid.angle_to(*other_asteroid);
            angle_set.insert(angle);
            if asteroid.x == 31 && asteroid.y == 20 {
                angle_map.entry(angle).or_insert(Vec::new()).push(*other_asteroid);
            }
        }
        if angle_set.len() > most_seen {
            most_seen = angle_set.len();
            best_asteroid = *asteroid;
        }
    }
    println!("Best Asteroid {},{} can see {} others.", best_asteroid.x, best_asteroid.y, most_seen);

    for entry in angle_map.iter_mut() {
        entry.1.sort_by(|a, b| b.distance().cmp(&a.distance()));
    }

    let mut num_destroyed = 0;
    loop {
        if angle_map.is_empty() {
            break;
        }
        let mut keys: Vec<i64> = Vec::new();
        for key in angle_map.keys() {
            keys.push(*key);
        }
        keys.sort();
        keys.reverse();
        for key in keys {
            let entry = angle_map.entry(key).or_default();
            let destroyed = entry.pop().expect("Empty"); // destroy asteroid
            num_destroyed += 1;
            if num_destroyed == 200 {
                println!("200th Destroyed: {}", destroyed.x * 100 + destroyed.y);
                return;
            }
            if entry.is_empty() {
                angle_map.remove(&key);
            }
        }
    }
}
