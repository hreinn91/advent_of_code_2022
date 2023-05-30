use std::collections::HashMap;
use crate::common;

pub fn solve_part_1() {
    let mut cargo_plan = common::read_input_to_cargo_plan("day_5/input.txt");
    execute_cargo_plan(&mut cargo_plan.stacks, &cargo_plan.instructions);
    println!("Solution to part 1 {}", common::get_stack_order(&cargo_plan.stacks));
}

fn execute_cargo_plan(stacks: &mut HashMap<i32, Vec<char>>, instructions: &Vec<String>) {
    for instruction in instructions {
        execute_instruction(stacks, instruction);
    }
}

fn execute_instruction(stacks: &mut HashMap<i32, Vec<char>>, instruction: &String) {
    let mut instruction_numbers = instruction
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .flatten();
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