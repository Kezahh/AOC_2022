use crate::generic;
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::thread::current;


#[derive(Debug)]
struct FileObject {
    index: usize,
    name: String,
    size: usize,
    is_directory: bool,
    contents: HashMap<String, usize>,
    parent_index: usize,
}

impl FileObject {
    fn new(index: usize, name: String, size: usize, is_directory: bool, parent_index: usize) -> FileObject {
        return FileObject {
            index: index,
            name: name.clone(),
            size: size,
            is_directory: is_directory,
            contents: HashMap::new(),
            parent_index: parent_index,
        };
    }

    fn get_size(&self, all_files: &Vec<FileObject>) -> usize {
        if !self.is_directory {
            return self.size;
        } else {
            return self.contents.values().map(|x| all_files[*x].get_size(&all_files)).sum();
        }
    }
}

fn solve_puzzle(input_file: &str, part_1: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_file);
    let mut all_files: Vec<FileObject> = Vec::new();

    let mut line_index = 0;
    all_files.push(FileObject {
        index: 0,
        name: input_lines[line_index]["$ cd ".len()..].to_string(),
        size: 0,
        is_directory: false,
        contents: HashMap::new(),
        parent_index: 0,
    });

    let mut current_directory_index = 0;

    line_index += 1;

    while line_index < input_lines.len() {
        let mut current_line: String = input_lines[line_index].clone();

        if current_line == "$ ls" {
            all_files[current_directory_index].is_directory = true;

            line_index += 1;
            current_line = input_lines[line_index].clone();

            while !current_line.starts_with("$") {
                let line_split: Vec<String> = current_line.split(" ").map(|x| x.to_string()).collect();
                let new_directory_name = line_split[1].clone();
                let new_index = all_files.len();
                
                if line_split[0] == "dir" {
                    all_files.push(FileObject::new(new_index, new_directory_name.clone(), 0, true, current_directory_index));
                } else {
                    all_files.push(FileObject::new(new_index, new_directory_name.clone(), line_split[0].parse::<usize>().unwrap(), false, current_directory_index));
                }
                all_files[current_directory_index].contents.insert(new_directory_name.clone(), new_index);
                line_index += 1;
                if line_index == input_lines.len() {
                    break;
                }

                current_line = input_lines[line_index].clone();
            }
        } else if current_line == "$ cd .." {
            current_directory_index = all_files[current_directory_index].parent_index;
            line_index += 1;
        } else if current_line.starts_with("$ cd ") {
            let target_directory: String = current_line.split(" ").last().unwrap().to_string();
            
            //println!("looking for: {:?} in:", target_directory);
            //println!("{:?}", all_files[current_directory_index].contents.keys().clone());
            assert!(all_files[current_directory_index].contents.contains_key(&target_directory));
            
            current_directory_index = all_files[current_directory_index].contents[&target_directory];
            line_index += 1;
        }
    }

    println!("finished parsing");

    

    if part_1 {
        let mut sizes_under_max: Vec<usize> = Vec::new();
        let max_size = 100000;

        for f in all_files.iter() {
            if f.get_size(&all_files) < max_size && f.is_directory {
            sizes_under_max.push(f.get_size(&all_files));
            }
        }
        
        println!("All sizes are: {:?}", sizes_under_max);
        println!("Total sum is: {}", sizes_under_max.iter().sum::<usize>());
        return sizes_under_max.iter().sum();
    } else {
        let total_size: usize = 70000000;
        let unused_space = total_size - all_files[0].get_size(&all_files);
        let required_space = 30000000;

        let max_size = required_space - unused_space;
        let mut sizes_under_max: Vec<usize> = Vec::new();

        for f in all_files.iter() {
            if f.get_size(&all_files) > max_size && f.is_directory {
                sizes_under_max.push(f.get_size(&all_files));
            }
        }

        println!("All sizes are: {:?}", sizes_under_max);
        println!("Filesize to delete is: {}", *sizes_under_max.iter().min().unwrap());
        return *sizes_under_max.iter().min().unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_7/input_example_1.txt", true) == 95437);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_7/input.txt", true) == 1423358);
    }

    #[test]
    fn example_2() {
        assert!(solve_puzzle("src/inputs/day_7/input_example_1.txt", false) == 24933642);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle("src/inputs/day_7/input.txt", false) == 545729);
    }
}
