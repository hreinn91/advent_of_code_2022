use std::collections::HashSet;
use std::fs::read_to_string;


fn read_to_signal(filename: &str) -> String {
    return read_to_string(filename).expect("Read file to string error.");
}

fn find_marker(signal: &String, marker_size: usize) -> i32 {
    let chars: Vec<char> = signal.chars().into_iter().collect();
    for i in 0..chars.len() {
        let mut marker_set: HashSet<char>= HashSet::new();
        for j in 0..marker_size {
            marker_set.insert(chars[i + j]);
        }
        if marker_set.len() == marker_size {
            return (i + marker_size) as i32;
        }
    }
    return -1;
}

fn main() {
    let signal = read_to_signal("day_6/input.txt");
    println!("Packet marker: {} Message marker: {} ", find_marker(&signal, 4), find_marker(&signal, 14));
}
