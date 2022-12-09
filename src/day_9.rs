use crate::generic;
use std::collections::HashSet;

struct StepCommand {
    direction: Direction,
    size: usize,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn up(&mut self) {
        self.row += 1;
    }

    fn down(&mut self) {
        self.row -= 1;
    }

    fn left(&mut self) {
        self.col -= 1;
    }

    fn right(&mut self) {
        self.col += 1;
    }

    fn chase_head(&mut self, head_position: &Self) {
        let row_diff = (self.row - head_position.row).abs();
        let col_diff = (self.col - head_position.col).abs();

        if row_diff > 1 && col_diff >= 1 || row_diff >= 1 && col_diff > 1 {
            if head_position.row > self.row {
                self.up();
            } else {
                self.down();
            }
            if head_position.col > self.col {
                self.right();
            } else {
                self.left();
            }
        } else if head_position.row - self.row > 1 {
            self.up();
        } else if self.row - head_position.row > 1 {
            self.down();
        } else if head_position.col - self.col > 1 {
            self.right();
        } else if self.col - head_position.col > 1 {
            self.left()
        } 

        //println!("Head = {:?}, tail = {:?}", head_position, self);
    }
}


fn solve_puzzle(input_filename: &str, knots_count: usize) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let mut steps: Vec<StepCommand> = Vec::new();

    for line in input_lines.iter() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let mut position: Direction = Direction::Left;
        match split_line[0] {
            "L" => position = Direction::Left,
            "R" => position = Direction::Right,
            "U" => position = Direction::Up,
            "D" => position = Direction::Down,
            _ => println!("Bad input"),
        }
        steps.push(StepCommand { direction: position, size: split_line[1].parse::<usize>().unwrap() });
    }

    let mut row_index: i32 = 0;
    let mut col_index: i32 = 0;

    let mut knots: Vec<Position> = vec![Position{row: 0, col: 0}; knots_count];

    //let mut head_position: Position = Position{row: 0, col: 0};
    //let mut tail_position: Position = Position{row: 0, col: 0};

    let mut positions: HashSet<Position> = HashSet::new();
    let mut positions_order: Vec<Position> = Vec::new();

    for step in steps.iter() {

        for _i in 0..step.size {
            match step.direction {
                Direction::Left => knots[0].left(),
                Direction::Right => knots[0].right(),
                Direction::Up => knots[0].up(),
                Direction::Down => knots[0].down(),
                _ => println!("Bad time"),
            }

            for knot_index in 1..knots.len() {
                let previous_knot = knots[knot_index - 1].clone();
                knots[knot_index].chase_head(&previous_knot);
            }
            //tail_position.chase_head(&head_position);

            positions.insert(knots.last().unwrap().clone());
            positions_order.push(knots.last().unwrap().clone());
        }
    }
    


    //println!("all positions = {:?}", positions_order);
    println!("Total positions = {}", positions.len());
    return positions.len();
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_9/input_example_1.txt", 2) == 13);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_9/input.txt", 2) == 6269);
    }

    #[test]
    fn example_2() {
        assert!(solve_puzzle("src/inputs/day_9/input_example_1.txt", 10) == 1);
        assert!(solve_puzzle("src/inputs/day_9/input_example_2.txt", 10) == 36);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle("src/inputs/day_9/input.txt", 10) == 2557);
    }
}
