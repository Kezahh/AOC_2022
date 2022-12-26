use crate::generic;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::thread::current;
use std::fmt;


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
                println!("adding number");
                if parent_packet_index != None {
                    let new_packet_index = Self::add_new(all_packets, Some(number), false);
                    all_packets[parent_packet_index.unwrap()].data.push(new_packet_index);
                    return parent_packet_index.unwrap();
                } else {
                    return Self::add_new(all_packets, Some(number), false);
                }
            }
            Err(error) => {
                println!("adding list");
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
}


fn solve_puzzle(input_filename: &str) -> usize {
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

    for packets in ordered_packets.iter() {
        for p in packets.iter() {
            if !all_packets[*p].is_integer {
                println!("{:?}", all_packets[*p].get_string(&all_packets));
            }
        }
    }

    return 0
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_13/input_example_1.txt") == 31);
    }

    #[test]
    fn part_1() {
    }

    #[test]
    fn example_2() {
    }

    #[test]
    fn part_2() {
    }
}
