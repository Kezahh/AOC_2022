use crate::generic;
use std::collections::HashSet;


fn get_x_register(input_filename: &str) -> Vec<i32> {
    let input_lines = generic::read_in_file(input_filename);
    let mut x_register: Vec<i32> = Vec::new();
    
    // X starts with 1.
    x_register.push(1);
    x_register.push(1);

    for line in input_lines {
        let last_value: i32 = *x_register.last().unwrap();
        if line == "noop" {
            x_register.push(last_value);
        } else if line.starts_with("addx") {
            let value_to_add: i32 = line.split(" ").last().unwrap().parse().unwrap();
            x_register.push(last_value);
            x_register.push(last_value + value_to_add);
        }
    }

    return x_register;
}

fn solve_puzzle(input_filename: &str) -> i32 {
    let x_register = get_x_register(input_filename);

    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut cycle_index = 20;

    while cycle_index < x_register.len() {
        signal_strengths.push((cycle_index as i32) * x_register[cycle_index]);
        cycle_index += 40;
    }

    let signal_sum = signal_strengths.iter().sum::<i32>();

    println!("x_register = {:?}", x_register);
    println!("signal strengths = {:?}", signal_strengths);
    println!("Signal Strength sum = {}", signal_sum);
    return signal_sum;
}

fn solve_puzzle_part2(input_filename: &str) {
    let x_register = get_x_register(input_filename);

    let mut cycle_index: usize = 0;
    let mut col_index: i32 = 0;

    for cycle_index in 1..240 {
        if x_register[cycle_index] - 1 <= col_index && x_register[cycle_index] + 1 >= col_index {
            print!("#");
        } else {
            print!(".");
        }
        
        col_index += 1;
        col_index = col_index % 40;
        
        if cycle_index % 40 == 0 {
            print!("\n");
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let x_register = get_x_register("src/inputs/day_10/input_example_2.txt");
        assert!(x_register[20] == 21);
        assert!(x_register[60] == 19);
        assert!(x_register[100] == 18);
        assert!(x_register[140] == 21);
        assert!(x_register[180] == 16);
        assert!(x_register[220] == 18);
        assert!(solve_puzzle("src/inputs/day_10/input_example_2.txt") == 13140);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_10/input.txt") == 17380);
    }

    #[test]
    fn example_2() {
        solve_puzzle_part2("src/inputs/day_10/input_example_2.txt");
    }

    #[test]
    fn part_2() {
        solve_puzzle_part2("src/inputs/day_10/input.txt");
    }
}
