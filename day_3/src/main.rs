use std::assert_eq;
use std::fs;

fn calculate_group_priorities_sum(file_name: &str) -> i32 {
    let rucksacks: Vec<String> = fs::read_to_string(file_name)
        .expect("Could not find input file")
        .lines()
        .map(|s| String::from(s))
        .collect();

        let mut i = 0;
        let mut sum = 0;
        while i < rucksacks.len() {
            sum = sum + get_group_badge_priority(&rucksacks[i], &rucksacks[i + 1], &rucksacks[i + 2]).expect("Badge not found");
            i = i + 3;
        }
        return sum;
}

fn get_group_badge_priority(first: &String, second: &String , third: &String) -> Result<i32, ()> {
    for item in first.chars(){
        if second.contains(item) && third.contains(item) {
            return Ok(get_priority_of_item(item));
        }
    }
    return Err(());
}

fn calculate_priorities_sum(file_name: &str) -> i32 {
    return fs::read_to_string(file_name).expect("Could not find input file")
        .lines()
        .map(get_priority_from_items)
        .sum();
}

fn get_priority_from_items(raw_string: &str) -> i32 {
    let duplicate = get_duplicate_item(raw_string).expect("Failed to find duplicate item");
    return get_priority_of_item(duplicate);
}

fn get_duplicate_item(raw_string: &str) -> Result<char, &str> {
    let items: Vec<char> = raw_string.chars().collect();
    let num_items = items.len();
    assert_eq!(0, num_items % 2);
    let compartment_size = num_items / 2;
    let (first, second) = items.split_at(compartment_size);
    for item in first {
        if second.contains(item) {
            return Ok(*item);
        }
    }
    return Err("No duplicate found");
}

fn get_priority_of_item(item: char) -> i32 {
    if item.is_uppercase() {
        return item as i32 - 38;
    }
    return item as i32 - 96;
}

fn main() {
    println!("Part One Sum: {:?}", calculate_priorities_sum("day_3/input.txt"));
    println!("Part Two Sum: {:?}", calculate_group_priorities_sum("day_3/input.txt"));
}
