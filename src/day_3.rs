use crate::generic;
use std::collections::HashSet;


fn get_common_item(input_string: String) -> char {
    let first_half = input_string[0..input_string.len()/2].to_string();
    let first_half_set: HashSet<char> = first_half.chars().collect();
    let second_half = input_string[first_half.len()..].to_string();
    let second_half_set: HashSet<char> = second_half.chars().collect();

    let common_char: char = *first_half_set.intersection(&second_half_set).next().unwrap();
    return common_char;
}

fn get_common_item_in_groups(input_vec: Vec<String>) -> char {
    let first: HashSet<char> = input_vec[0].chars().collect();
    let second: HashSet<char> = input_vec[1].chars().collect();
    let third: HashSet<char> = input_vec[2].chars().collect();

    let first_second: HashSet<char> = first.intersection(&second).map(|x| *x).collect();
    let common_char: char = *first_second.intersection(&third).next().unwrap();
    return common_char;
}

fn loop_through_list(input_file: &str) -> usize {
    let input_lines = generic::read_in_file(input_file);
    let mut total_score: usize = 0;
    for line in input_lines {
        let common_char: char = get_common_item(line);
        let mut score = common_char as usize;

        if score >= 65 && score <= 90 {
            score = score - 65 + 27;
        } else if score >= 97 && score <=122 {
            score = score - 97 + 1;
        }
        //println!("Common char is {}, score = {}", common_char, score);
        total_score += score;
    }

    println!("{}", total_score);
    return total_score;
}

fn loop_through_list_groups(input_file: &str) -> usize {
    let input_lines = generic::read_in_file(input_file);
    let mut total_score: usize = 0;

    let mut line_index: usize = 0;
    while line_index < input_lines.len() {
        let common_char: char = get_common_item_in_groups(input_lines[line_index..(line_index+3)].to_vec());
        let mut score = common_char as usize;

        if score >= 65 && score <= 90 {
            score = score - 65 + 27;
        } else if score >= 97 && score <=122 {
            score = score - 97 + 1;
        }
        //println!("Common char is {}, score = {}", common_char, score);
        total_score += score;
        line_index += 3;
    }

    println!("{}", total_score);
    return total_score;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(loop_through_list("src/inputs/day_3/input_example_1.txt") == 157);
    }

    #[test]
    fn part_1() {
        assert!(loop_through_list("src/inputs/day_3/input.txt") == 7875);
    }

    #[test]
    fn example_2() {
        assert!(loop_through_list_groups("src/inputs/day_3/input_example_1.txt") == 70);
    }

    #[test]
    fn part_2() {
        assert!(loop_through_list_groups("src/inputs/day_3/input.txt") == 2479)
    }
}
