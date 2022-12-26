use crate::generic;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::thread::current;
use std::fmt;


static START_CHAR: char = 'S';
static END_CHAR: char = 'E';

#[derive(Clone, PartialEq)]
struct Point {
    col: usize,
    row: usize,
    value: char,
    next_point: Option<Vec<usize>>,
    neighbours_set: bool,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}] ({}) next = {:?}", self.row, self.col, self.value, self.next_point)
    }
}


fn get_points_map(input_lines: &Vec<String>) -> Vec<Vec<Point>> {
    let mut output_map: Vec<Vec<Point>> = Vec::new();

    for (row_index, line) in input_lines.iter().enumerate() {
        let mut line_vec: Vec<Point> = Vec::new();
        for (col_index, c) in line.chars().enumerate() {
            line_vec.push(Point { col: col_index, row: row_index, value: c, next_point: None, neighbours_set: false });
        }
        output_map.push(line_vec);
    }

    return output_map;
}

fn get_neighbour_points(current_point: &Point, points_map: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut output_vec: Vec<Point> = Vec::new();

    let above_row: i32 = (current_point.row as i32) - 1;
    let below_row: i32 = (current_point.row as i32) + 1;
    let left_col: i32 = (current_point.col as i32) - 1;
    let right_col: i32 = (current_point.col as i32) + 1;

    if above_row >= 0  {
        output_vec.push(points_map[above_row as usize][current_point.col].clone());
    }
    if below_row < points_map.len() as i32 {
        output_vec.push(points_map[below_row as usize][current_point.col].clone());
    }
    if left_col >= 0 {
        output_vec.push(points_map[current_point.row][left_col as usize].clone());
    }
    if right_col < points_map[0].len() as i32 {
        output_vec.push(points_map[current_point.row][right_col as usize].clone());
    }

    // check if char is 1 step above or anything below.
    let mut real_output_vec: Vec<Point> = Vec::new();
    for neighbour in output_vec {
        let step_change: i32 = current_point.value as i32 - neighbour.value as i32;
        if ((current_point.value == END_CHAR && (neighbour.value == 'z' || neighbour.value == 'y')) || 
            (current_point.value != END_CHAR && current_point.value <= neighbour.value) || 
            step_change == 1 ||
            (neighbour.value == START_CHAR && (current_point.value == 'a' || current_point.value == 'b'))) {
            real_output_vec.push(neighbour);
        }
    }

    return real_output_vec
}

fn set_neighbour_next_point(neighbour: &Point, current_point: &Point, output_graph: &mut Vec<Vec<Point>>) {
    println!("\tSetting {:?}", neighbour);
    output_graph[neighbour.row][neighbour.col].next_point = Some(vec![current_point.row, current_point.col]);
    println!("\t\t Point is now {:?}", output_graph[neighbour.row][neighbour.col]);
}

fn set_current_point_neighbours_done(current_point: &Point, output_graph: &mut Vec<Vec<Point>>) {
    output_graph[current_point.row][current_point.col].neighbours_set = true;
}

fn get_start_point(points_map: &Vec<Vec<Point>>) -> Point {
    for row in points_map.iter() {
        for point in row.iter() {
            if point.value == START_CHAR {
                return point.clone();
            }
        }
    }

    return points_map[0][0].clone();
}

fn get_end_point(points_map: &Vec<Vec<Point>>) -> Point {
    for row in points_map.iter() {
        for point in row.iter() {
            if point.value == END_CHAR {
                return point.clone();
            }
        }
    }

    return points_map[0][0].clone();
}

fn get_distance_to_end(target_point: &Point, points_map: &Vec<Vec<Point>>) -> usize {
    let mut distance = 0;
    let mut current_point = target_point.clone();
    while (current_point.value != END_CHAR) {
        distance += 1;
        //println!("\t(distance) current point is {:?}", current_point);
        if current_point.next_point == None {
            distance = 0;
            break;
        }
        let next_point: Vec<usize> = current_point.next_point.unwrap();
        current_point = points_map[next_point[0]][next_point[1]].clone();
    }

    return distance;
}

fn get_point_from_points_graph(target_point: &Point, points_graph: &Vec<Vec<Point>>) -> Point {
    return points_graph[target_point.row][target_point.col].clone();
}


fn build_points_graph(points_map: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
    let end_point = get_end_point(&points_map);
    let mut output_graph: Vec<Vec<Point>> = points_map.clone();

    let mut points_to_process = VecDeque::new();
    points_to_process.push_back(end_point.clone());

    //let test_point = output_graph[1][7].clone();
    //println!("Neighbours are: {:?}", get_neighbour_points(&test_point, &output_graph));

    //return output_graph;


    let mut loop_count = 0;

    while points_to_process.len() > 0 {
        let current_point = points_to_process.pop_front().unwrap();
        set_current_point_neighbours_done(&current_point, &mut output_graph);
        println!("Processing {:?}", current_point);

        let mut neighbour_points: Vec<Point> = get_neighbour_points(&current_point, &output_graph);
        let current_distance: usize = get_distance_to_end(&current_point, &output_graph);

        println!("\tneighbours are {:?}", neighbour_points);

        for neighbour in neighbour_points.iter() {
            if neighbour.next_point == None {
                set_neighbour_next_point(&neighbour, &current_point, &mut output_graph);
            } else {
                let neighbour_distance: usize = get_distance_to_end(&neighbour, &output_graph);
                println!("\tcurrent distance {} for point {:?}", current_distance, current_point);
                println!("\tneighbour distance {} for point {:?}", neighbour_distance, neighbour);
                if current_distance + 1 < neighbour_distance {
                    set_neighbour_next_point(&neighbour, &current_point, &mut output_graph);
                }
            }
            
        }

        for neighbour in neighbour_points.iter() {
            let real_neighbour = get_point_from_points_graph(&neighbour, &output_graph);
            if !real_neighbour.neighbours_set && !points_to_process.contains(&real_neighbour) {
                println!("Added real neighbour {:?}", real_neighbour);
                points_to_process.push_back(real_neighbour);
            }
        }
        
        loop_count += 1;
        if loop_count == 4 {
            //break;
        }
    }

    return output_graph;
}


fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let mut points_map: Vec<Vec<Point>> = get_points_map(&input_lines);
    let mut points_graph: Vec<Vec<Point>> = build_points_graph(&points_map);

    for r in 0..points_graph.len() {
        for c in 0..points_graph[0].len() {
            //print!("{:?},", get_distance_to_start(&points_graph[r][c], &points_graph));
            //println!("{:?}", points_graph[r][c]);
        }
        //print!("\n");
    }

    if !part_2 {
        let start_point = get_start_point(&points_graph);
        let start_distance = get_distance_to_end(&start_point, &points_graph);

        println!("Distance from start = {}", start_distance);
        return start_distance;
    } else {
        let mut min_distance = 0;

        for points_row in points_graph.iter() {
            for point in points_row.iter() {
                if point.value == 'a' {
                    let distance = get_distance_to_end(&point, &points_graph);
                    if distance > 0 && (distance < min_distance || min_distance == 0) {
                        min_distance = distance;
                    }
                }
            }
        }

        println!("Best distance = {}", min_distance);
        return min_distance;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_12/input_example_1.txt", false) == 31);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_12/input.txt", false) == 447);
    }

    #[test]
    fn example_2() {
        assert!(solve_puzzle("src/inputs/day_12/input_example_1.txt", true) == 29);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle("src/inputs/day_12/input.txt", true) == 446);
    }
}
