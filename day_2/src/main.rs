use std::fs;

#[derive(PartialEq, Eq, Debug)]
struct Play(i32);
const ROCK: Play = Play(1);
const PAPER: Play = Play(2);
const SCISSOR: Play = Play(3);

fn evaluate_strategy_guide(file: &str, calc_function: impl Fn(&str) -> i32) -> i32 {
    return fs::read_to_string(file)
        .expect("Could not find input file")
        .lines()
        .map(|raw| -> i32  {
            return calc_function(raw);
        })
        .sum();
}

fn parse_and_calculate_score_a(raw: &str) -> i32 {
    let play_split: Vec<&str> = raw.split(" ").collect();
    let o_play = parse_a(play_split[0].chars().next().unwrap()).expect("FAILED TO GET OPPONENT");
    let m_play = parse_a(play_split[1].chars().next().unwrap()).expect("FAILED TO GET MYPLAY");
    return calculate_score(o_play, m_play);
}

fn parse_and_calculate_score_b(raw: &str) -> i32 {
    let play_split: Vec<&str> = raw.split(" ").collect();
    let o_play = parse_a(play_split[0].chars().next().unwrap()).expect("FAILED TO GET OPPONENT");
    let m_play = parse_b(&o_play, play_split[1].chars().next().unwrap()).expect("FAILED TO GET MYPLAY");
    return calculate_score(o_play, m_play);
}

fn parse_b(o_play: &Play, unwrap: char) -> Result<Play, String> {
    return match unwrap {
        'X' => Ok(get_loss(o_play)),
        'Y' => Ok(get_same(o_play)),
        'Z' => Ok(get_win(o_play)),
        _ => Err(format!("Failed to parse string: {}", unwrap)),
    }
}

fn get_loss(play: &Play) -> Play{
    return match play {
        &ROCK => SCISSOR,
        &PAPER => ROCK,
        _ => PAPER
    }
}

fn get_win(play: &Play) -> Play{
    return match play {
        &ROCK => PAPER,
        &PAPER => SCISSOR,
        _ => ROCK
    }
}

fn get_same(play: &Play) -> Play{
    return match play {
        &ROCK => ROCK,
        &PAPER => PAPER,
        _ => SCISSOR
    }
}

fn calculate_score(o: Play, m: Play) -> i32 {
    let play_score: i32 = m.0;
    if o == m {
        return play_score + 3;
    }

    if o == ROCK && m == SCISSOR || o == PAPER && m == ROCK || o == SCISSOR && m == PAPER {
        return play_score + 0;
    }

    return play_score + 6;
}

fn parse_a(raw: char) -> Result<Play, String> {
    return match raw {
        'A' => Ok(ROCK),
        'B' => Ok(PAPER),
        'C' => Ok(SCISSOR),
        'X' => Ok(ROCK),
        'Y' => Ok(PAPER),
        'Z' => Ok(SCISSOR),
        _ => Err(format!("Failed to parse string: {}", raw)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsin_opponent() {
        let result = parse_a('A').expect("Failed to parse");
        assert_eq!(result, ROCK);
    }
    #[test]
    fn test_parse_and_calc() {
        let mut result = parse_and_calculate_score_a("A Y");
        assert_eq!(result, 8);
        result = parse_and_calculate_score_a("B X");
        assert_eq!(result, 1);
        result = parse_and_calculate_score_a("C Z");
        assert_eq!(result, 6);
    }
    #[test]
    fn test_evaluate_strategy_guide() {
        let result = evaluate_strategy_guide("test_1.txt", parse_and_calculate_score_a);
        assert_eq!(result, 15);
    }
}

fn main() {
    let part_1_result = evaluate_strategy_guide("day_2/input.txt", parse_and_calculate_score_a);
    println!("Total score 1: {part_1_result} ");
    let part_2_result = evaluate_strategy_guide("day_2/input.txt", parse_and_calculate_score_b);
    println!("Total score 2: {part_2_result} ");

}
