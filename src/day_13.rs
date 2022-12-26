use crate::generic;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::thread::current;
use std::fmt;


#[derive(Eq, PartialEq)]
enum PacketResult {
    Lose = 0,
    Tie = 1,
    Win = 2,
}

#[derive(Eq, PartialEq, Debug)]
struct PacketData {
    index: usize,
    value: usize,
    is_blank: bool,
    is_integer: bool,
    data: Vec<usize>,
}

impl PacketData {
    fn add_new(all_packets: &mut Vec<PacketData>, value: Option<usize>, is_blank: bool) -> usize {
        let current_length: usize = all_packets.len();
    
        if value == None {
            if is_blank {
                all_packets.push(Self { index: current_length, value: 0, is_blank: true, is_integer: false, data: Vec::new() });
            } else {
                all_packets.push(Self { index: current_length, value: 0, is_blank: false, is_integer: false, data: Vec::new() });
            }
        } else {
            all_packets.push(Self { index: current_length, value: value.unwrap(), is_blank: false, is_integer: true, data: Vec::new() });    
        }

        return current_length;
    }

    fn add_from_string(input_string: String, parent_packet_index: Option<usize>, all_packets: &mut Vec<PacketData>) -> usize {
        println!("Adding packet {}", input_string);

        if input_string.len() == 0 {
            if parent_packet_index != None {
                let new_packet_index = Self::add_new(all_packets, None, true);
                all_packets[parent_packet_index.unwrap()].data.push(new_packet_index);
                return parent_packet_index.unwrap();
            } else {
                return Self::add_new(all_packets, None, false);
            }
        }

        match input_string.parse::<usize>() {
            Ok(number) => {
                if parent_packet_index != None {
                    let new_packet_index = Self::add_new(all_packets, Some(number), false);
                    all_packets[parent_packet_index.unwrap()].data.push(new_packet_index);
                    return parent_packet_index.unwrap();
                } else {
                    return Self::add_new(all_packets, Some(number), false);
                }
            }
            Err(error) => {
                if parent_packet_index != None {
                    let new_packet_index = Self::add_list(input_string, all_packets);
                    all_packets[parent_packet_index.unwrap()].data.push(new_packet_index);
                    return parent_packet_index.unwrap();
                } else {
                    return Self::add_list(input_string, all_packets);
                }
            },
        }
    }

    fn add_list(input_string: String, all_packets: &mut Vec<PacketData>) -> usize {
        let mut new_packet_index: usize = PacketData::add_new(all_packets, None, false);
        // strip outside brackets.
        let packet_chars: Vec<char> = input_string[1..(input_string.len() - 1)].chars().collect();
    
        // split string
        let mut current_string: String = String::new();
        let mut char_index: usize = 0;
        println!("\tgoing to unpack {:?}", packet_chars);
        while char_index < packet_chars.len() {
            let mut current_char = packet_chars[char_index];
            if current_char.is_numeric() {
                current_string.push(current_char);
                char_index += 1;
            } else if current_char == '[' {
                let mut depth: usize = 0;
                while !(current_char == ']' && depth == 1) {
                    //println!("\tcurrent_char = {}, depth = {}", current_char, depth);
                    if current_char == '[' {
                        depth += 1;
                    } else if current_char == ']' {
                        depth -= 1;
                    }
                    current_string.push(current_char);
                    char_index += 1;
                    current_char = packet_chars[char_index];   
                }
                current_string.push(current_char);
                char_index += 1;
            } else if current_char == ',' {
                Self::add_from_string(current_string, Some(new_packet_index), all_packets);
                current_string = String::new();
                char_index += 1;
            } else {
                char_index += 1;
            }
        }
        Self::add_from_string(current_string, Some(new_packet_index), all_packets);

        return new_packet_index;
    }

    fn get_string(&self, all_packets: &Vec<PacketData>) -> String {
        if self.is_integer {
            return self.value.to_string();
        } else if self.is_blank {
            return String::new();
        } else {
            let mut full_string: String = String::new();
            full_string.push('[');
            for p in self.data.iter() {
                full_string.push_str(all_packets[*p].get_string(all_packets).as_str());
                full_string.push(',');
            }
            // remove last comma
            full_string = full_string[..(full_string.len() - 1)].to_string();

            full_string.push(']');
            return full_string
        }
    }

    fn make_integer_into_list(all_packets: &mut Vec<PacketData>, packet_index: usize) -> usize {
        let new_index = Self::add_new(all_packets, None, false);
        all_packets[new_index].data.push(packet_index);

        return new_index;
    }
}

fn left_packet_good_order(all_packets: &mut Vec<PacketData>, left_packet_index: usize, right_packet_index: usize) -> PacketResult {

    //println!("\t(compare) left = {:?}, right = {:?}", all_packets[left_packet_index].get_string(all_packets), all_packets[right_packet_index].get_string(all_packets));
    
    if all_packets[left_packet_index].is_integer && all_packets[right_packet_index].is_integer {
        if all_packets[left_packet_index].value < all_packets[right_packet_index].value {
            return PacketResult::Win;
        } else if all_packets[left_packet_index].value == all_packets[right_packet_index].value {
            return PacketResult::Tie;
        } else {
            return PacketResult::Lose;
        }
    } else if all_packets[left_packet_index].is_blank && all_packets[right_packet_index].is_blank {
        return PacketResult::Tie;
    } else if all_packets[left_packet_index].is_blank && !all_packets[right_packet_index].is_blank {
        return PacketResult::Win;
    } else if !all_packets[left_packet_index].is_blank && all_packets[right_packet_index].is_blank {
        return PacketResult::Lose;
    } else if all_packets[left_packet_index].is_integer {
        let new_left_packet_index = PacketData::make_integer_into_list(all_packets, left_packet_index);
        return left_packet_good_order(all_packets, new_left_packet_index, right_packet_index);
    } else if all_packets[right_packet_index].is_integer {
        let new_right_packet_index = PacketData::make_integer_into_list(all_packets, right_packet_index);
        return left_packet_good_order(all_packets, left_packet_index, new_right_packet_index);
    } else {
        let mut data_index = 0;
        while data_index < all_packets[left_packet_index].data.len() {
            if data_index >= all_packets[right_packet_index].data.len() {
                return PacketResult::Lose;
            }
            let check_packets = left_packet_good_order(all_packets, all_packets[left_packet_index].data[data_index], all_packets[right_packet_index].data[data_index]);
            if check_packets != PacketResult::Tie {
                return check_packets;
            }
            data_index += 1;
        }
        
        if all_packets[right_packet_index].data.len() > data_index {
            return PacketResult::Win;
        } else {
            return PacketResult::Tie;
        }
    }
}


fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let mut all_packets: Vec<PacketData> = Vec::new();
    let mut ordered_packets: Vec<Vec<usize>> = Vec::new();

    let mut packet_index: usize = 0;
    while packet_index < input_lines.len() {
        let packet_1_index = PacketData::add_from_string(input_lines[packet_index].clone(), None, &mut all_packets);
        let packet_2_index = PacketData::add_from_string(input_lines[packet_index+1].clone(), None, &mut all_packets);
        ordered_packets.push(vec![packet_1_index, packet_2_index]);
        packet_index += 3;
    }

    if !part_2 {
        let mut good_indices: Vec<usize> = Vec::new();
        let mut packet_index = 1;
        for packets in ordered_packets {
            println!("(compare) left = {:?}", all_packets[packets[0]].get_string(&all_packets));
            println!("(compare) right = {:?}", all_packets[packets[1]].get_string(&all_packets));
            if left_packet_good_order(&mut all_packets, packets[0], packets[1]) == PacketResult::Win {
                println!("Left wins.");
                good_indices.push(packet_index);
            }

            packet_index += 1;

            // if packet_index == 10 {
            //     break;
            // }
        }

        let index_sum: usize = good_indices.iter().sum();
        println!("Good indices are {:?}", good_indices);
        println!("Sum is {}", index_sum);

        return index_sum;
    } else {
        println!("Begin sorting");
        let mut to_sort_packets: Vec<usize> = Vec::new();
        for packets in ordered_packets {
            for p in packets {
                to_sort_packets.push(p);
            }
        }

        // add divider packets.
        let divider_index_1: usize = PacketData::add_from_string("[[2]]".to_string(), None, &mut all_packets);
        let divider_index_2: usize = PacketData::add_from_string("[[6]]".to_string(), None, &mut all_packets);
        to_sort_packets.push(divider_index_1);
        to_sort_packets.push(divider_index_2);

        let mut target_packet_index: usize = 1;
        let mut packet_index: usize = 0;
        while target_packet_index < to_sort_packets.len() {
            //print_all_packets(&all_packets, &to_sort_packets, Some(target_packet_index));
            packet_index = target_packet_index;

            let mut packet_inserted: bool = false;
            while left_packet_good_order(&mut all_packets, to_sort_packets[target_packet_index], to_sort_packets[packet_index - 1]) == PacketResult::Win {
                //println!("\t(sort) {} < {}", all_packets[to_sort_packets[target_packet_index]].get_string(&all_packets), all_packets[to_sort_packets[packet_index - 1]].get_string(&all_packets));
                packet_index -= 1;
                if packet_index == 0 {
                    let packet_index_to_move = to_sort_packets[target_packet_index];
                    to_sort_packets.remove(target_packet_index);
                    to_sort_packets.insert(0, packet_index_to_move);
                    packet_inserted = true;
                    break;
                }
            }
            //println!("\tsort done: target_packet = {}, target_packet_index = {}, packet_index = {}", all_packets[to_sort_packets[target_packet_index]].get_string(&all_packets), target_packet_index, packet_index);
            if !packet_inserted && (packet_index != target_packet_index) {
                let packet_index_to_move = to_sort_packets[target_packet_index];
                to_sort_packets.remove(target_packet_index);
                to_sort_packets.insert(packet_index, packet_index_to_move);
            }

            target_packet_index += 1;

        }

        print_all_packets(&all_packets, &to_sort_packets, None);
        // 1 2 4 3 5 6

        let mut sorted_divider_index_1 = 0;
        let mut sorted_divider_index_2 = 0;

        for (packet_index, packet) in to_sort_packets.iter().enumerate() {
            if *packet == divider_index_1 {
                sorted_divider_index_1 = packet_index + 1;
            } else if *packet == divider_index_2 {
                sorted_divider_index_2 = packet_index + 1;
            }
        }

        println!("Divider 1 is at index = {}", sorted_divider_index_1);
        println!("Divider 2 is at index = {}", sorted_divider_index_2);
        println!("Product = {}", sorted_divider_index_1 * sorted_divider_index_2);
        return sorted_divider_index_1 * sorted_divider_index_2;
    }
}

fn print_all_packets(all_packets: &Vec<PacketData>, sort_list: &Vec<usize>, target: Option<usize>) {
    println!("Full List:");
    for (packet_index, packet) in sort_list.iter().enumerate() {
        if target != None {
            if packet_index == target.unwrap() {
                println!("\t-> {:?}", all_packets[*packet].get_string(&all_packets));
            } else {
                println!("\t{:?}", all_packets[*packet].get_string(&all_packets));
            }
        } else {
            println!("\t{:?}", all_packets[*packet].get_string(&all_packets));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_13/input_example_1.txt", false) == 13);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_13/input.txt", false) == 5682);
        // 230 is too low
        // 5338 is too low
        // 5964 is too high
    }

    #[test]
    fn example_2() {
        assert!(solve_puzzle("src/inputs/day_13/input_example_1.txt", true) == 140);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle("src/inputs/day_13/input.txt", true) == 20304);
    }
}
