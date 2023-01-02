use crate::generic;
use std::collections::HashSet;
// use std::collections::VecDeque;
// use std::thread::current;
use std::fmt;
use std::hash::Hash;


#[derive(Debug, PartialEq,Eq, Clone)]
enum BeaconAvailability {
    NotAvailable,
    PartialAvailable,
    Available,
}

#[derive(Debug, Clone)]
struct PointsGrid {
    size: i32,
    top_left: Point,
    availibility: BeaconAvailability,
}

impl PointsGrid {
    fn get_availability(&self, sensor: &Sensor) -> BeaconAvailability {
        let top_left = self.top_left.clone();
        let bottom_right = Point{x: top_left.x + self.size, y: top_left.y + self.size};
        let top_right = Point{x: bottom_right.x, y: top_left.y};
        let bottom_left = Point{x: top_left.x, y: bottom_right.y};

        let sensor_distance = sensor.get_exclusion_distance();

        let corner_points = vec![top_left, top_right, bottom_right, bottom_left];
        let mut corner_inside: Vec<bool> = Vec::new();
        for corner in corner_points {
            corner_inside.push(sensor.location.get_distance_from(&corner) <= sensor_distance);
        }

        if !corner_inside.contains(&false) {
            return BeaconAvailability::NotAvailable;
        } else if !corner_inside.contains(&true) {
            return BeaconAvailability::Available;
        } else {
            return BeaconAvailability::PartialAvailable;
        }
    }

    fn get_smaller_grids(&self, new_size: i32) -> Vec<Self> {
        let mut new_grids: Vec<Self> = Vec::new();

        for y in 0..(self.size/new_size) {
            for x in 0..(self.size/new_size) {
                new_grids.push(Self { size: new_size, top_left: self.top_left.add_offset(x*new_size, y*new_size), availibility: BeaconAvailability::Available })
            }
        }

        return new_grids;
    }

    fn get_points(&self) -> Vec<Point> {
        let mut output_points: Vec<Point> = Vec::new();
        for y in 0..self.size {
            for x in 0..self.size {
                output_points.push(self.top_left.add_offset(x, y));
            }
        }

        return output_points;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Sensor {
    location: Point,
    beacon: Point,
}

impl Sensor {
    fn get_exclusion_distance(&self) -> i32 {
        return self.beacon.get_distance_from(&self.location);
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "[{},{}]", self.x, self.y);
    }
}

impl Point {
    fn from_string(input_string: &str) -> Self {
        let split_string: Vec<&str> = input_string.split(", ").collect();
        return Self { x: split_string[0][2..].parse::<i32>().unwrap(), y: split_string[1][2..].parse::<i32>().unwrap() };
    }

    fn get_distance_from(&self, other_point: &Point) -> i32 {
        return (self.x - other_point.x).abs() + (self.y - other_point.y).abs();
    }

    fn add_offset(&self, x_off: i32, y_off: i32) -> Self {
        return Point { x: self.x + x_off, y: self.y + y_off };
    }
}

fn get_all_sensors(input_filename: &str) -> Vec<Sensor> {
    let input_lines = generic::read_in_file(input_filename);
    let mut all_sensors: Vec<Sensor> = Vec::new();

    for line in input_lines.iter() {
        let split_line: Vec<&str> = line.split(": ").collect();
        let sensor_location = Point::from_string(&split_line[0]["Sensor at ".len()..]);
        let beacon_location = Point::from_string(&split_line[1]["closest beacon is at ".len()..]);
        all_sensors.push(Sensor { location: sensor_location, beacon: beacon_location });
    }

    return all_sensors;
}


fn solve_puzzle(input_filename: &str, target_y: i32) -> usize {
    let all_sensors = get_all_sensors(input_filename);

    let mut x_positions: HashSet<i32> = HashSet::new();
    let mut used_x_positions: HashSet<i32> = HashSet::new();

    for s in all_sensors {
        let sensor_exclusion_distance = s.get_exclusion_distance();
        let largest_x_distance = sensor_exclusion_distance - (s.location.y - target_y).abs();
        let start_x = s.location.x - largest_x_distance;
        let end_x = s.location.x + largest_x_distance;

        let mut new_x_positions: Vec<i32> = Vec::new();
        for x in start_x..(end_x + 1) {
            new_x_positions.push(x);
        }

        //println!("{:?}", s);
        //println!("\tsensor_distance = {}, largest_x_distance = {}, start_x = {}, end_x = {}", sensor_exclusion_distance, largest_x_distance, start_x, end_x);
        //println!("\t{:?}", new_x_positions);
        x_positions.extend::<HashSet<i32>>(HashSet::from_iter(new_x_positions));

        if s.location.y == target_y {
            used_x_positions.insert(s.location.x);
        }
        if s.beacon.y == target_y {
            used_x_positions.insert(s.beacon.x);
        }
    }
    
    for x_pos in used_x_positions.iter() {
        if x_positions.contains(x_pos) {
            x_positions.remove(x_pos);
        }
    }

    println!("There are {} positions that cannot have a beacon.", x_positions.len());
    let mut sorted_x_positions: Vec<i32> = x_positions.clone().into_iter().collect();
    sorted_x_positions.sort();
    //println!("{:?}", sorted_x_positions);
    return x_positions.len();
}

fn get_grids(max_coord: i32) -> Vec<PointsGrid> {
    let mut all_grids: Vec<PointsGrid> = Vec::new();
    for y in 0..(max_coord/10000) {
        //let mut grid_row: Vec<PointsGrid> = Vec::new();
        for x in 0..(max_coord/10000) {
            all_grids.push(PointsGrid{size: 10000, top_left: Point { x: x*10000, y: y*10000 }, availibility: BeaconAvailability::Available});
        }
        //all_grids.push(grid_row);
    }

    return all_grids
}

fn check_points_grids_against_sensors(all_grids: &Vec<PointsGrid>, all_sensors: &Vec<Sensor>) -> Vec<PointsGrid> {
    let mut grid_is_good: Vec<bool> = vec![true; all_grids.len()];

    for s in all_sensors {
        println!("{:?}", s);
        println!("\tdistance = {}", s.get_exclusion_distance());

        let mut grid_index: usize = 0;
        while grid_index < all_grids.len() {
            if grid_is_good[grid_index] {
                if all_grids[grid_index].get_availability(&s) == BeaconAvailability::NotAvailable {
                    grid_is_good[grid_index] = false;
                }
            }
            grid_index += 1;
        }
    }

    let mut return_grids: Vec<PointsGrid> = Vec::new();
    let mut grid_index: usize = 0;
    while grid_index < all_grids.len() {
        if grid_is_good[grid_index] {
            return_grids.push(all_grids[grid_index].clone());
        }
        grid_index += 1;
    }

    println!("There are {} points_grids left.", return_grids.len());

    return return_grids;
}

fn break_down_grids(all_grids: &Vec<PointsGrid>, new_size: i32) -> Vec<PointsGrid> {
    let mut new_grids: Vec<PointsGrid> = Vec::new();
    for grid in all_grids {
        new_grids.append(&mut grid.get_smaller_grids(new_size));
    }
    println!("Split up, there are {} smaller grids left.", new_grids.len());

    return new_grids;
}

fn solve_puzzle_part_2(input_filename: &str, max_coord: i32) -> usize {
    let all_sensors = get_all_sensors(input_filename);
    let mut all_grids = get_grids(max_coord);
    all_grids = check_points_grids_against_sensors(&all_grids, &all_sensors);

    all_grids = break_down_grids(&all_grids, 1000);
    all_grids = check_points_grids_against_sensors(&all_grids, &all_sensors);

    all_grids = break_down_grids(&all_grids, 100);
    all_grids = check_points_grids_against_sensors(&all_grids, &all_sensors);

    all_grids = break_down_grids(&all_grids, 50);
    all_grids = check_points_grids_against_sensors(&all_grids, &all_sensors);

    all_grids = break_down_grids(&all_grids, 10);
    all_grids = check_points_grids_against_sensors(&all_grids, &all_sensors);

    let mut all_points: Vec<Point> = Vec::new();
    for g in all_grids {
        all_points.append(&mut g.get_points());
    }

    println!("There are {} points to check.", all_points.len());
    let mut point_is_good: Vec<bool> = vec![true; all_points.len()];

    for sensor in all_sensors {
        let mut point_index: usize = 0;
        let sensor_distance = sensor.get_exclusion_distance();
        println!("{:?}", sensor);
        println!("\tdistance = {}", sensor_distance);
        
        while point_index < all_points.len() {
            if point_is_good[point_index] {
                if sensor.location.get_distance_from(&all_points[point_index]) <= sensor_distance {
                    point_is_good[point_index] = false;
                }
            }
            point_index += 1;
        }
    }
    
    let mut good_points: Vec<Point> = Vec::new();
    let mut point_index: usize = 0;
    while point_index < all_points.len() {
        if point_is_good[point_index] {
            good_points.push(all_points[point_index].clone());
        }
        point_index += 1;
    }
    println!("There are {} point left.", good_points.len());
    println!("Points are {:?}", good_points);

    let best_point = good_points[0].clone();
    let tuning_frequency: u64 = (best_point.x as u64 * 4000000) + best_point.y as u64;
    println!("Tuning frequency is {}", tuning_frequency);
    return tuning_frequency as usize;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_15/input_example_1.txt", 10) == 26);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_15/input.txt", 2000000) == 5878678);
    }

    #[test]
    fn example_2() {
        //assert!(solve_puzzle_part_2("src/inputs/day_15/input_example_1.txt", 20) == 56000011);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle_part_2("src/inputs/day_15/input.txt", 4000000) == 11796491041245);
    }

    #[test]
    fn skdjfksdjf() {
        let x: i32 = 2949122;
        let y: i32 = 3041245;

        let tuning_frequency: u64 = (x as u64 * 4000000) + y as u64;
        println!("Tuning frequency is {}", tuning_frequency);
    }
}
