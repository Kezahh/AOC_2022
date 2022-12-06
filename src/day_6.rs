use crate::generic;
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;


fn solve_puzzle(input_datastream: &str, unique_count: usize) -> usize {
    let datastream_chars: Vec<char> = input_datastream.chars().collect();
    let mut current_letters: HashSet<char> = HashSet::from_iter(datastream_chars[0..unique_count].iter().cloned());

    let mut i: usize = 1;
    while current_letters.len() < unique_count {
        current_letters = HashSet::from_iter(datastream_chars[i..(i+unique_count)].iter().cloned());
        i += 1;
    }

    println!("Found unique 4 at index={}", i + unique_count - 1);
    return i + unique_count - 1;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4) == 7);
        assert!(solve_puzzle("bvwbjplbgvbhsrlpgdmjqwftvncz", 4) == 5);
        assert!(solve_puzzle("nppdvjthqldpwncqszvftbrmjlhg", 4) == 6);
        assert!(solve_puzzle("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4) == 10);
        assert!(solve_puzzle("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4) == 11);
    }

    #[test]
    fn part_1() {
        let input_lines = generic::read_in_file("src/inputs/day_6/input.txt");
        assert!(solve_puzzle(input_lines[0].as_str(), 4) == 1802);
    }

    #[test]
    fn example_2() {
        assert!(solve_puzzle("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14) == 19);
        assert!(solve_puzzle("bvwbjplbgvbhsrlpgdmjqwftvncz", 14) == 23);
        assert!(solve_puzzle("nppdvjthqldpwncqszvftbrmjlhg", 14) == 23);
        assert!(solve_puzzle("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14) == 29);
        assert!(solve_puzzle("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14) == 26);
    }

    #[test]
    fn part_2() {
        let input_lines = generic::read_in_file("src/inputs/day_6/input.txt");
        assert!(solve_puzzle(input_lines[0].as_str(), 14) == 3551);
    }
}
