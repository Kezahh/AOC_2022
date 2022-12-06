use crate::generic;
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;


struct Move {
    quantity: usize,
    from_index: usize,
    to_index: usize,
}

impl Move {
    fn from_string(input_string: &str) -> Self{
        // move xx from yy to zz
        let split_string: Vec<&str> = input_string.split(" ").collect();
        return Self {
            quantity: split_string[1].parse::<usize>().unwrap(),
            from_index: split_string[3].parse::<usize>().unwrap() - 1,
            to_index: split_string[5].parse::<usize>().unwrap() - 1,
        }
    }
}


#[derive(Debug, Clone)]
struct CraneStack(Vec<char>);

impl CraneStack {
    fn new() -> Self {
        return CraneStack(vec![]);
    }

    fn remove_blanks(&mut self) {
        while *self.last().unwrap() == ' ' {
            self.pop();
        }
    }

    fn move_to_stack(&mut self, to_stack: &Self, quantity: usize) -> Self {
        let mut output: Self = to_stack.clone();
        for i in 0..quantity {
            output.push(self.pop().unwrap());
        }
        return output;
    }

    fn move_to_stack_9001(&mut self, to_stack: &Self, quantity: usize) -> Self {
        //println!("moving {} from {:?}", quantity, self);
        let mut output: Self = to_stack.clone();
        let mut to_move: Vec<char> = self[(self.len()-quantity)..].to_vec();
        let current_length = self.len();
        self.truncate(current_length - quantity);
        output.append(&mut to_move);

        //println!("moved {} to {:?}", quantity, output);
        return output;
    }
}

impl Deref for CraneStack {
    type Target = Vec<char>;
    fn deref(&self) -> &Vec<char> {
        return &self.0;
    }
}

impl DerefMut for CraneStack {
    fn deref_mut(&mut self) -> &mut Vec<char> {
        return &mut self.0;
    }
}

fn parse_stacks(input_lines: &Vec<String>) -> Vec<CraneStack> {
    let mut output: Vec<CraneStack> = Vec::new();
    let mut line_index: usize = 0;

    while input_lines[line_index].contains("[") {
        line_index += 1;
    }

    let stacks_string: Vec<&str> = input_lines[line_index].split(" ").filter(|x| *x != "").collect();
    for i in 0..stacks_string.len() {
        output.push(CraneStack::new());
        for line in input_lines[0..line_index].iter().rev() {
            output[i].push(line.chars().collect::<Vec<char>>()[1+(4*i)]);
        }
    }

    for stack in output.iter_mut() {
        stack.remove_blanks();
    }

    return output;
}

fn parse_moves(input_lines: &Vec<String>) -> Vec<Move> {
    let mut output: Vec<Move> = Vec::new();
    let mut line_index: usize = 0;

    while input_lines[line_index].contains("[") {
        line_index += 1;
    }
    line_index += 2;

    while line_index < input_lines.len() {
        output.push(Move::from_string(input_lines[line_index].as_str()));
        line_index += 1;
    }

    return output;
}

fn solve_puzzle(input_file: &str, is_9001: bool) -> String {
    let mut output: String = String::new();
    let input_lines = generic::read_in_file(input_file);
    let mut all_stacks: Vec<CraneStack> = parse_stacks(&input_lines);
    let all_moves: Vec<Move> = parse_moves(&input_lines);

    for m in all_moves.iter() {
        let target_stack = all_stacks[m.to_index].clone();
        let mut modified_target: CraneStack = CraneStack::new();
        if is_9001 {
            modified_target = all_stacks[m.from_index].move_to_stack_9001(&target_stack, m.quantity);
        } else {
            modified_target = all_stacks[m.from_index].move_to_stack(&target_stack, m.quantity);
        }
        all_stacks[m.to_index] = modified_target;
    }

    for s in all_stacks {
        output.push_str(s.last().unwrap().to_string().as_str());
    }

    println!("Combined string is {:?}", output);
    return output;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_5/input_example_1.txt", false) == "CMZ");
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_5/input.txt", false) == "QMBMJDFTD");
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle("src/inputs/day_5/input.txt", true) == "NBTVTJNFJ");
    }

}
