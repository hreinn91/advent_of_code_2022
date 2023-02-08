use std::fs;

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn contains_point(&self, point: i32) -> bool {
        return self.start <= point && self.end >= point;
    }

    fn overlap(&self, other: &Range) -> bool {
        return self.contains_point(other.start) || self.contains_point(other.end);
    }

}

fn count_pairs(file_name: &str, predicate: impl Fn(&(Range, Range)) -> bool) -> usize {
    return fs::read_to_string(file_name).expect("Could not find input file")
        .lines()
        .filter(|line| {
            let pair = parse_pairs(line);
            return predicate(&pair);
        }).count();
}

fn parse_pairs(line: &str) -> (Range, Range) {
    let mut ranges = line
        .split(",")
        .map(|s| s.split("-"))
        .map(|iter| {
            let mut itty = iter.map(|s| s.parse::<i32>().unwrap()).into_iter();
            Range { start: itty.next().unwrap(), end: itty.next().unwrap() }
        })
        .into_iter();
    return (ranges.next().unwrap(), ranges.next().unwrap());
}

fn contains(pair: &(Range, Range)) -> bool {
    pair.0.contains(&pair.1) || pair.1.contains(&pair.0)
}

fn overlaps(pair: &(Range, Range)) -> bool {
    return pair.0.overlap(&pair.1) || pair.1.overlap(&pair.0);
}


fn main() {
    println!("Number of pair of ranges that fully contain {}", count_pairs("day_4/input.txt", contains));
    println!("Number of pair of ranges that overlap {}", count_pairs("day_4/input.txt", overlaps));
}
