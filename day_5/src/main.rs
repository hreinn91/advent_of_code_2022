use std::collections::HashMap;
use std::fs::read_to_string;

struct CargoPlan {
    stacks: HashMap<i32, Vec<char>>,
    instructions: Vec<String>,
}

fn execute_cargo_plan(mut stacks: HashMap<i32, Vec<char>>, instructions: Vec<String>){
    for instruction in instructions {
        execute_instruction(&mut stacks, &instruction);
    }
    print_map(&stacks);
}

fn print_map(stacks: &HashMap<i32, Vec<char>>) {
    for (key, value) in stacks {
        println!("{} {:?}", key, value);
    }
    println!();
}

fn execute_instruction(stacks: &mut HashMap<i32, Vec<char>>, instruction: &String) {
    let mut instruction_numbers = instruction.split_whitespace().map(|s| s.parse::<i32>()).flatten();
    let number_move = instruction_numbers.next().unwrap();
    let from_stack_key = instruction_numbers.next().unwrap();
    let to_stack_key = instruction_numbers.next().unwrap();

    let mut from_stack = stacks.get_mut(&from_stack_key).cloned().unwrap();
    let mut to_stack = stacks.get_mut(&to_stack_key).cloned().unwrap();
    for _ in 0..number_move {
        let cargo = from_stack.remove(0);
        to_stack.insert(0, cargo);
    }
    stacks.insert(from_stack_key, from_stack);
    stacks.insert(to_stack_key, to_stack);
}

fn read_input_to_cargo_plan(file_name: &str) -> CargoPlan {
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

fn main() {
    let cargo_plan = read_input_to_cargo_plan("day_5/input.txt");
    execute_cargo_plan(cargo_plan.stacks, cargo_plan.instructions);
}
