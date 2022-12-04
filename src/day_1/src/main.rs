use std::fs;

fn get_calories_from_backpack(raw_string: &str) -> i32 {
    return raw_string
        .lines()
        .map(|s| {
            let c: i32 = s.parse().unwrap();
            return c;
        })
        .sum();
}

fn get_elf_with_most_calories(file: &str) -> (usize, i32) {
    let mut elf_with_most_calories = (0, 0);
    let mut top_three_calories = [0, 0, 0];
    fs::read_to_string(file)
        .expect("Could not find input file")
        .split("\n\n")
        .enumerate()
        .for_each(|(index, raw_string)| {
            let carlories = get_calories_from_backpack(raw_string);
            println!("calories: {} index: {}", carlories, index);
            if carlories > elf_with_most_calories.1 {
                elf_with_most_calories = (index + 1, carlories)
            }
            top_three_calories.sort();
            for (i, value) in top_three_calories.iter().enumerate() {
                if carlories > *value {
                    top_three_calories[i] = carlories;
                    break;
                }
            }
        });
    println!("\ntop three: {} {} {}", top_three_calories[0],  top_three_calories[1],  top_three_calories[2]);
    let top_sum: i32 = top_three_calories.iter().sum();
    println!("sun: {} ", top_sum);
    return elf_with_most_calories;
}

//Part A + B: input1.txt test1.txt

fn main() {
    let elf_with_most_calories = get_elf_with_most_calories("input1.txt");
    println!(
        "Elf with most calories is elf number {} with {} calories",
        elf_with_most_calories.0, elf_with_most_calories.1
    );
}
