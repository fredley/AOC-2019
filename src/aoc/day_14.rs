use std::collections::HashMap;

use crate::aoc::utils;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reagent {
    quantity: i64,
    name: String,
}

impl Reagent {
    pub fn produced_by(&self, reaction_chain: &mut HashMap<String, Vec<String>>, others: Vec<Reagent>) -> bool {
        for other in others.clone() {
            let mut to_check = vec![other.name];
            while !to_check.is_empty() {
                let next = to_check.remove(0);
                if next == self.name {
                    return true;
                }
                for to_add in reaction_chain.entry(next).or_insert(vec![]) {
                    to_check.push(to_add.to_string());
                }
            }
        }
        return false;
    }
}

pub fn parse_reagent (input: String) -> Reagent {
    let parts: Vec<&str> = input.trim().split(" ").collect();
    let quantity = utils::parse_int_64(parts[0]);
    let name = parts[1].trim().to_string();
    return Reagent{
        quantity: quantity,
        name: name,
    }
}

pub fn day_fourteen(input: String) -> () {
    println!("Ore quantity: {}", calculate_ore(input.clone(), 1));
    let target: i64 = 1000000000000;
    let mut test_amount: i64;
    let mut min: i64 = 0;
    let mut max = target;
    let mut test = target / 2;
    loop {
        test_amount = calculate_ore(input.clone(), test);
        if test_amount > target {
            max = test;
            test = (min + max) / 2;
        } else if test_amount < target {
            min = test;
            test = (min + max) / 2;
        }
        if max - min < 2 {
            break;
        }
    }
    println!("Fuel produced: {}", test);
}

fn calculate_ore(input: String, fuel_quantity: i64) -> i64 {
    let reaction_inputs: Vec<&str> = input.trim().split("\n").collect();
    let mut reaction_lookup: HashMap<String, Reagent> = HashMap::new();
    let mut reaction_chain: HashMap<String, Vec<String>> = HashMap::new();
    let mut reactions: HashMap<Reagent, Vec<Reagent>> = HashMap::new();
    for reaction in reaction_inputs {
        let components: Vec<&str> = reaction.split("=>").collect();
        let output = parse_reagent(components[1].to_string());
        let input_strs: Vec<&str> = components[0].split(",").collect();
        let mut inputs: Vec<Reagent> = Vec::new();
        let mut input_names: Vec<String> = Vec::new();
        for input_str in input_strs {
            let input_reagent = parse_reagent(input_str.to_string());
            inputs.push(input_reagent.clone());
            input_names.push(input_reagent.name);
        }
        reactions.insert(output.clone(), inputs);
        reaction_chain.insert(output.clone().name, input_names);
        reaction_lookup.insert(output.clone().name, output);
    }
    // starting with FUEL, calculate the total cost to get to ORE
    let mut to_process = vec![Reagent{name: "FUEL".to_string(), quantity: fuel_quantity}];
    let mut ore_quantity = 0;
    while to_process.len() > 0 {
        let next = to_process.remove(0);
        if next.produced_by(&mut reaction_chain, to_process.clone()) {
            // not ready to process this reaction yet
            to_process.push(next);
            continue;
        }
        let next_name = next.clone().name;
        let lookup = reaction_lookup.entry(next_name).or_insert(Reagent{name: "ERR".to_string(), quantity: 0});
        let components = reactions.entry(lookup.clone()).or_insert(vec![]);
        let mut multiple = (next.quantity / lookup.quantity) + 1;
        if next.quantity % lookup.quantity == 0 {
            multiple -= 1;
        }
        for component in components {
            if component.name == "ORE" {
                ore_quantity += component.quantity * multiple;
            } else {
                let mut found = false;
                let mut i = 0;
                while i < to_process.clone().len() {
                    let existing = to_process.get(i).unwrap();
                    if existing.clone().name == component.name {
                        to_process[i] = Reagent{name: existing.clone().name, quantity: existing.quantity + component.quantity * multiple};
                        found = true;
                        break;
                    }
                    i+=1;
                }
                if !found {
                    to_process.push(Reagent{name: component.clone().name, quantity: component.quantity * multiple});
                }
            }
        }
    }
    return ore_quantity;
}