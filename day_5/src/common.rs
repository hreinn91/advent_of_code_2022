use std::fs::read_to_string;
use std::collections::HashMap;

pub struct CargoPlan {
    pub stacks: HashMap<i32, Vec<char>>,
    pub instructions: Vec<String>,
}

pub fn read_input_to_cargo_plan(file_name: &str) -> CargoPlan {
    let raw_input_spit_by_lines: Vec<String> = read_to_string(file_name)
        .expect("Could not find input file")
        .lines()
        .map(|s| s.to_owned())
        .collect();
    let stacks = get_stack_map(&raw_input_spit_by_lines);
    let instructions = raw_input_spit_by_lines.split(|line| line.eq("")).skip(1).next().unwrap().to_vec();
    return CargoPlan { instructions, stacks };
}

fn get_stack_map(raw_input_spit_by_lines: &Vec<String>) -> HashMap<i32, Vec<char>> {
    let stack_map_size = raw_input_spit_by_lines.first().unwrap().len() / 4 + 1;
    let mut stack_map = HashMap::new();
    for stack_index in 1..=stack_map_size {
        let raw_input_stacks = raw_input_spit_by_lines.split(|line| line.contains(" 1")).next().unwrap();
        let stack_vector: Vec<char> = raw_input_stacks.iter()
            .map(|line| line.chars().nth(1 + (stack_index - 1) * 4).unwrap())
            .filter(|stack_character| *stack_character != ' ')
            .collect();
        stack_map.insert(stack_index as i32, stack_vector);
    }
    return stack_map;
}

pub fn get_stack_order(stacks: &HashMap<i32, Vec<char>>) -> String {
    let mut order = String::new();
    for i in 1..=stacks.keys().len() {
        let bottom_crate = stacks
            .get(&(i as i32)).unwrap()[0]
            .to_string();
        order = format!("{}{}", order, bottom_crate);
    }
    order
}