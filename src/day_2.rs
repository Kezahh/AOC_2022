#[warn(dead_code)]
use crate::generic;
use std::collections::HashMap;


#[derive(PartialEq, Clone)]
enum Result {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone)]
enum PlayResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}


fn get_outcome(oponent_play: Result, self_play: Result) -> PlayResult {

    if oponent_play == self_play {
        return PlayResult::Draw;
    } else if oponent_play == Result::Rock && self_play == Result::Paper {
        return PlayResult::Win;
    } else if oponent_play == Result::Rock && self_play == Result::Scissors {
        return PlayResult::Lose;
    } else if oponent_play == Result::Paper && self_play == Result::Rock {
        return PlayResult::Lose;
    } else if oponent_play == Result::Paper && self_play == Result::Scissors {
        return PlayResult::Win;
    } else if oponent_play == Result::Scissors && self_play == Result::Rock {
        return PlayResult::Win;
    } else if oponent_play == Result::Scissors && self_play == Result::Paper {
        return PlayResult::Lose;
    }
    return PlayResult::Lose;
}

fn get_selfplay(oponent_play: Result, outcome: PlayResult) -> Result {
    if outcome == PlayResult::Draw {
        return oponent_play;
    } else if oponent_play == Result::Rock && outcome == PlayResult::Win {
        return Result::Paper;
    } else if oponent_play == Result::Rock && outcome == PlayResult::Lose {
        return Result::Scissors;
    } else if oponent_play == Result::Paper && outcome == PlayResult::Win {
        return Result::Scissors;
    } else if oponent_play == Result::Paper && outcome == PlayResult::Lose {
        return Result::Rock;
    } else if oponent_play == Result::Scissors && outcome == PlayResult::Win {
        return Result::Rock;
    } else if oponent_play == Result::Scissors && outcome == PlayResult::Lose {
        return Result::Paper;
    }
    return Result::Rock;
}


fn solve_puzzle_part_1(input_file: &str) -> usize {
    let input_lines = generic::read_in_file(input_file);
    let mut total_score: usize = 0;
    let oponent_plays = HashMap::from([
        ("A", Result::Rock),
        ("B", Result::Paper),
        ("C", Result::Scissors)
    ]);

    let self_plays = HashMap::from([
        ("X", Result::Rock),
        ("Y", Result::Paper),
        ("Z", Result::Scissors)
    ]);

    for round in input_lines.iter() {
        let line_split: Vec<&str> = round.split(" ").collect();
        let oponent_play = (*oponent_plays.get(line_split[0]).unwrap()).clone();
        let self_play = (*self_plays.get(line_split[1]).unwrap()).clone();

        total_score += (get_outcome(oponent_play.clone(), self_play.clone()) as usize) + (self_play.clone() as usize) + 1;
    }

    println!("Total_score is {}", total_score);
    return total_score
}

fn solve_puzzle_part_2(input_file: &str) -> usize {
    let input_lines = generic::read_in_file(input_file);
    let mut total_score: usize = 0;
    let oponent_plays = HashMap::from([
        ("A", Result::Rock),
        ("B", Result::Paper),
        ("C", Result::Scissors)
    ]);

    let self_plays = HashMap::from([
        ("X", PlayResult::Lose),
        ("Y", PlayResult::Draw),
        ("Z", PlayResult::Win)
    ]);

    for round in input_lines.iter() {
        let line_split: Vec<&str> = round.split(" ").collect();
        let oponent_play = (*oponent_plays.get(line_split[0]).unwrap()).clone();
        let play_outcome = (*self_plays.get(line_split[1]).unwrap()).clone();
        let self_play = get_selfplay(oponent_play, play_outcome.clone());
        

        total_score += (play_outcome as usize) + (self_play.clone() as usize) + 1;
    }

    println!("Total_score is {}", total_score);
    return total_score;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle_part_1("src/inputs/day_2/input_example_1.txt") == 15);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle_part_1("src/inputs/day_2/input.txt") == 13052);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle_part_2("src/inputs/day_2/input.txt") == 13693);
    }
}