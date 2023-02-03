use std::assert_eq;
use std::fs;

fn calculate_sum_of_priorities(file_name: &str) -> i32 {
    return fs::read_to_string(file_name)
        .expect("Could not find input file")
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
    println!("Sum: {:?}", calculate_sum_of_priorities("input.txt"));
}
