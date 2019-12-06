use std::collections::HashMap;

pub fn day_six(input: String) -> () {
    let orbits = input.split("\n");
    let mut orbit_map: HashMap<&str, &str> = HashMap::new();
    for orbit in orbits {
        let parts: Vec<&str> = orbit.split(")").collect();
        if parts.len() == 2 {
            orbit_map.insert(parts[1], parts[0]);
        }
    }
    let mut checksum = 0;
    for orbit in orbit_map.iter() {
        let mut step = orbit.1;
        loop {
            checksum += 1;
            match step {
                &"COM" => break,
                _ => (),
            }
            step = &orbit_map[step];
        }
    }
    println!("Checksum:        {}", checksum);
    let mut you_path: Vec<&str> = Vec::new();
    let mut san_path: Vec<&str> = Vec::new();
    let mut step = orbit_map["YOU"];
    loop {
        match step {
            "COM" => break,
            _ => (),
        }
        you_path.push(orbit_map[step]);
        step = &orbit_map[step];
    }
    step = orbit_map["SAN"];
    let target: &str;
    let mut transfers = 0;
    loop {
        if you_path.contains(&step) {
            target = step;
            break;
        }
        transfers += 1;
        san_path.push(orbit_map[step]);
        step = &orbit_map[step];
    }
    for you in you_path {
        transfers += 1;
        if you == target {
            break;
        }
    }
    println!("Total Transfers: {}", transfers);
}
