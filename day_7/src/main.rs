
mod common;

fn solve_part_1() -> i32 {
    let directory_max_size = 100000;
    let filesystem = common::read_input_to_filesystem("day_7/input.txt");
    filesystem.get_directories_sum(directory_max_size)
}

fn solve_part_2() -> i32 {
    let filesystem = common::read_input_to_filesystem("day_7/input.txt");
    let needed_disk_space = 30000000;
    let total_disk_space = 70000000;
    let used_disk_space = filesystem.get_directory_size(&"/".to_string());
    let free_disk_space = total_disk_space - used_disk_space;
    let threshold_disk_space = needed_disk_space - free_disk_space;
    println!("{} ", filesystem.get_smallest_bigger_than(threshold_disk_space));

    0
}


fn main() {
    println!("Part 1 {}", solve_part_1());
    println!("Part 2 {}", solve_part_2());
}
