use crate::generic;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use std::thread::current;
 use std::fmt;


#[derive(Clone, Eq, PartialEq)]
enum Unit {
    Air = 0,
    Rock = 1,
    Sand = 2,
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Unit::Air => write!(f, "."),
            Unit::Rock => write!(f, "#"),
            Unit::Sand => write!(f, "o"),
        }
    }
}

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn from_string(input_string: &str) -> Self {
        let split_string: Vec<&str> = input_string.split(",").collect();
        return Self { row: split_string[1].parse::<usize>().unwrap(), col: split_string[0].parse::<usize>().unwrap() };
    }
}

#[derive(Debug)]
struct RockLine {
    start: Point,
    end: Point,
}

impl RockLine {
    fn from_points(point1: Point, point2: Point) -> Self {
        if point1.col < point2.col || point1.row < point2.row {
            return Self{start: point1, end: point2};
        } else {
            return Self{start: point2, end: point1};
        }
    }

    fn is_horiztonal(&self) -> bool {
        return self.start.row == self.end.row;
    }
}

fn get_rock_lines(input_filename: &str) -> Vec<RockLine> {
    let mut rock_lines: Vec<RockLine> = Vec::new();
    let input_lines = generic::read_in_file(input_filename);

    for input_line in input_lines.iter() {
        let split_line: Vec<&str> = input_line.split(" -> ").collect();
        let mut point_index = 0;
        while point_index < split_line.len() - 1 {
            let start_point: Point = Point::from_string(split_line[point_index]);
            let end_point: Point = Point::from_string(split_line[point_index + 1]);
            rock_lines.push(RockLine::from_points(start_point, end_point));
            point_index += 1;
        }
    }

    return rock_lines;
}

fn get_scan_map(rock_lines: &Vec<RockLine>, part_2: bool) -> (usize, Vec<Vec<Unit>>) {
    let extra_starting_rows: usize = 0;
    let extra_columns: usize = 300;

    let min_row: usize = rock_lines.iter().map(|x| x.start.row).min().unwrap();
    let max_row: usize = rock_lines.iter().map(|x| x.end.row).max().unwrap();
    let min_col: usize = rock_lines.iter().map(|x| x.start.col).min().unwrap();
    let max_col: usize = rock_lines.iter().map(|x| x.end.col).max().unwrap();

    let column_count: usize = max_col - min_col + 1 + (extra_columns * 2);
    let row_count: usize = max_row + 1;

    let mut scan_map: Vec<Vec<Unit>> = vec![vec![Unit::Air; column_count]; row_count];
    for rock_line in rock_lines.iter() {
        if rock_line.is_horiztonal() {
            for col_index in rock_line.start.col..(rock_line.end.col + 1) {
                scan_map[rock_line.start.row][col_index - min_col + extra_columns] = Unit::Rock;
            }
        } else {
            for row_index in rock_line.start.row..(rock_line.end.row + 1) {
                scan_map[row_index][rock_line.start.col - min_col + extra_columns] = Unit::Rock;
            }
        }

    }
    
    let mut output_scan_map = vec![vec![Unit::Air; column_count]; extra_starting_rows];
    output_scan_map.append(&mut scan_map);

    if part_2 {
        output_scan_map.push(vec![Unit::Air; column_count]);
        output_scan_map.push(vec![Unit::Rock; column_count]);
    }

    return ((500 - min_col + extra_columns), output_scan_map);
}

fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let rock_lines = get_rock_lines(input_filename);
    let (start_col, mut scan_map) = get_scan_map(&rock_lines, part_2);
    //print_scan_map(&scan_map);

    let mut sand_count = 0;
    let mut overflow: bool = false;
    let mut col_index = start_col;
    let mut row_index = 0;

    while !overflow {    
        while scan_map[row_index][col_index] == Unit::Air {
            row_index += 1;
            if row_index == scan_map.len() {
                overflow = true;
                break;
            }
        }
        //println!("row = {}, col = {}, sand_count = {}", row_index, col_index, sand_count);
        //print_scan_map(&scan_map);
        if !overflow {
            if col_index == 0  || row_index == 0 {
                overflow = true;
            } else {
                if scan_map[row_index][col_index - 1] == Unit::Air {
                    col_index -= 1;
                } else {
                    if col_index == scan_map[row_index].len() - 1 {
                        overflow = true;
                    } else {
                        if scan_map[row_index][col_index + 1] == Unit::Air {
                            col_index += 1;
                        } else {
                            scan_map[row_index-1][col_index] = Unit::Sand;
                            sand_count += 1;

                            // reset for next sand
                            col_index = start_col;
                            row_index = 0;
                        }
                    }
                }
            }
        }
    }

    print_scan_map(&scan_map);

    println!("Sand count is {}", sand_count);
    return sand_count;
}

fn print_scan_map(scan_map: &Vec<Vec<Unit>>) {
    for (row_index, row) in scan_map.iter().enumerate() {
        print!("{:<3} ", row_index);
        for col in row {
            print!("{:?}", col);
        }
        print!("\n");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_14/input_example_1.txt", false) == 24);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_14/input.txt", false) == 610);
    }

    #[test]
    fn example_2() {
        assert!(solve_puzzle("src/inputs/day_14/input_example_1.txt", true) == 93);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle("src/inputs/day_14/input.txt", true) == 27194);
    }

    #[test]
    fn ksdjfksdjfkfsd() {
        let mut col_offset = 0;
        let mut count = 0;
        let mut col_index = 500;
        while count < 20 {
            if col_offset % 2 != 0 {
                col_index -= col_offset;
            } else {
                col_index += col_offset
            }
            col_offset += 1;
            count += 1;

            println!("col index = {}", col_index);
        }
    }
}
