use crate::generic;
use std::collections::HashSet;
use std::collections::VecDeque;


#[derive(Debug)]
struct Monkey {
    index: usize,
    items: VecDeque<u64>,
    test_divisor: u64,
    multiplier: u64,
    is_addition: bool,
    is_multiplier: bool,
    is_square: bool,
    target_true: usize,
    target_false: usize,
    items_inspected: usize,
}


impl Monkey {
    fn from_string(input_lines: Vec<String>) -> Self {
        let mut monkey_index_string: String = input_lines[0].split(" ").last().unwrap().to_string();
        monkey_index_string.pop();
        let monkey_index: usize = monkey_index_string.parse::<usize>().unwrap();

        let items_string: String = input_lines[1].split(": ").last().unwrap().to_string();
        let items: VecDeque<u64> = items_string.split(", ").map(|x| x.parse::<u64>().unwrap()).collect();
        
        let multiplier_string: String = input_lines[2].split(" = ").last().unwrap().to_string();
        let mut is_addition = false;
        let mut is_multiplier = false;
        let mut is_square = false;
        let mut multiplier = 0;
        if multiplier_string == "old * old" {
            is_square = true;
        } else {
            multiplier = multiplier_string.split(" ").last().unwrap().parse::<u64>().unwrap();
            if multiplier_string.contains("*") {
                is_multiplier = true;
            } else if multiplier_string.contains("+") {
                is_addition = true;
            }
            assert!(is_multiplier || is_addition);
        }

        let test_divisor: u64 = input_lines[3].split(" ").last().unwrap().parse::<u64>().unwrap();
        let target_true: usize = input_lines[4].split(" ").last().unwrap().parse::<usize>().unwrap();
        let target_false: usize = input_lines[5].split(" ").last().unwrap().parse::<usize>().unwrap();

        return Self {
            index: monkey_index,
            items: items,
            test_divisor: test_divisor,
            multiplier: multiplier,
            is_addition: is_addition,
            is_multiplier: is_multiplier,
            is_square: is_square,
            target_true: target_true,
            target_false: target_false,
            items_inspected: 0,
        };
    }

    fn throw_item(&mut self, part2: bool) -> (usize, u64) {
        self.items_inspected += 1;
        let mut target_item = self.items.pop_front().unwrap();
        if self.is_addition {
            target_item += self.multiplier;
        } else if self.is_multiplier {
            target_item *= self.multiplier;
        } else if self.is_square {
            //println!("{:?}", target_item);
            target_item *= target_item;
        }
        if !part2 {
            target_item /= 3;
        } else {
            //target_item %= 19*17*13*11*7*5*3*2; //LCM of 2,3,5,7,11,13,17,19
            target_item %= 9699690; //LCM of 2,3,5,7,11,13,17,19
            //target_item %= 96577; //LCM of 13,13,19,23
        }

        if target_item % self.test_divisor == 0 {
            return (self.target_true, target_item);
        } else {
            return (self.target_false, target_item);
        }
    }
}

fn parse_input(input_lines: &Vec<String>) -> Vec<Monkey> {
    let mut output_monkeys: Vec<Monkey> = Vec::new();

    for i in 0..(input_lines.len()/7 + 1) {
        let start_index = i * 7;
        let end_index = start_index + 6;
        output_monkeys.push(Monkey::from_string(input_lines[start_index..end_index].to_vec()))
    }

    return output_monkeys;
}


fn solve_puzzle(input_filename: &str, part2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let mut monkeys: Vec<Monkey> = parse_input(&input_lines);

    let num_of_rounds: usize;
    if part2 {
        num_of_rounds = 10000;
    } else {
        num_of_rounds = 20;
    }

    for round_index in 0..num_of_rounds {
        println!("Running round {} with {} monkeys.", round_index, monkeys.len());
        for monkey_index in 0..monkeys.len() {
            while monkeys[monkey_index].items.len() > 0 {
                let (monkey_target, item) = monkeys[monkey_index].throw_item(part2);
                //println!("Monkey {} throws {} to monkey {}", monkey_index, item, monkey_target);
                monkeys[monkey_target].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    println!("{:?}", monkeys.iter().map(|x| x.items_inspected).collect::<Vec<usize>>());
    let monkey_buisness = monkeys[0].items_inspected * monkeys[1].items_inspected;
    println!("Monkey business = {}", monkey_buisness);
    return monkey_buisness;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_11/input_example_1.txt", false) == 10605);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_11/input.txt", false) == 64032);
    }

    #[test]
    fn example_2() {
        assert!(solve_puzzle("src/inputs/day_11/input_example_1.txt", true) == 2713310158);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle("src/inputs/day_11/input.txt", true) == 12729522272);
    }
}
