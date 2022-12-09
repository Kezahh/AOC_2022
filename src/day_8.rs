use crate::generic;
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;


fn solve_puzzle(input_filename: &str) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let mut tree_map: Vec<Vec<i32>> = Vec::new();

    tree_map.push(vec![-1; input_lines[0].len()+2]);
    for line in input_lines.iter() {
        let mut tree_line: Vec<i32> = Vec::new();
        tree_line.push(-1);
        tree_line.append(&mut line.chars().map(|x| (x as i32)).collect());
        tree_line.push(-1);
        tree_map.push(tree_line);
    }
    tree_map.push(vec![-1; input_lines[0].len()+2]);

    //generic::print_2d_map(&tree_map);

    let mut tree_count = 0;
    for row_index in 1..tree_map.len() - 1 {
        for col_index in 1..tree_map[0].len() - 1 {
            let current_tree = tree_map[row_index][col_index];
            
            let max_above = tree_map[..row_index].iter().map(|x| x[col_index]).max().unwrap();
            let max_below = tree_map[(row_index + 1)..].iter().map(|x| x[col_index]).max().unwrap();
            let max_left = *tree_map[row_index][..col_index].iter().max().unwrap();
            let max_right = *tree_map[row_index][(col_index+1)..].iter().max().unwrap();

            let surrounding_trees = vec![max_above, max_below, max_left, max_right];
            //println!("current_tree is {}, surrounded by {:?}", current_tree, surrounding_trees);

            if current_tree > *vec![max_above, max_below, max_left, max_right].iter().min().unwrap() {
                //println!("tree is visible");
                tree_count += 1;
            }
        }
    }

    println!("Tree count is {}", tree_count);
    return tree_count;
}

fn count_trees(current_tree: &i32, neighbour_trees: Vec<i32>) -> usize {
    if neighbour_trees.len() == 0 || neighbour_trees[0] == -1 {
        //println!("Counting Trees: no neighbours.");
        return 0;
    }

    let mut last_tree = neighbour_trees[0];
    let mut tree_index = 1;

    //println!("Counting Trees: current_tree is {} with neighbours {:?}", current_tree, neighbour_trees);

    while (tree_index < neighbour_trees.len() && neighbour_trees[tree_index] <= *current_tree
            && neighbour_trees[tree_index] != -1  && last_tree < *current_tree) {
        last_tree = neighbour_trees[tree_index];
        tree_index += 1
    }

    // if last tree was a big one.
    if (tree_index < neighbour_trees.len() && neighbour_trees[tree_index] != -1 
            && last_tree <= *current_tree && neighbour_trees[tree_index] > last_tree  && last_tree != *current_tree) {
        tree_index += 1;
        //println!("did a big one");
    }

    println!("counted {} trees", tree_index);
    return tree_index;
}

fn solve_puzzle_part2(input_filename: &str) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let mut tree_map: Vec<Vec<i32>> = Vec::new();

    tree_map.push(vec![-1; input_lines[0].len()+2]);
    for line in input_lines.iter() {
        let mut tree_line: Vec<i32> = Vec::new();
        tree_line.push(-1);
        tree_line.append(&mut line.chars().map(|x| (x as i32) - 48).collect());
        tree_line.push(-1);
        tree_map.push(tree_line);
    }
    tree_map.push(vec![-1; input_lines[0].len()+2]);

    //generic::print_2d_map(&tree_map);

    let mut scenic_scores: Vec<usize> = Vec::new();
    for row_index in 1..tree_map.len() - 1 {
        for col_index in 1..tree_map[0].len() - 1 {
            let current_tree = tree_map[row_index][col_index];
            
            let count_above = count_trees(&current_tree, tree_map[..row_index].iter().map(|x| x[col_index]).rev().collect());
            let count_below = count_trees(&current_tree, tree_map[(row_index+1)..].iter().map(|x| x[col_index]).collect());
            let count_left = count_trees(&current_tree, tree_map[row_index][..col_index].iter().map(|x| *x).rev().collect());
            let count_right = count_trees(&current_tree, tree_map[row_index][(col_index+1)..].iter().map(|x| *x).collect());

            let surrounding_trees = vec![count_above, count_below, count_left, count_right];
            //println!("current_tree is {}, surrounded by {:?}", current_tree, surrounding_trees);

            let mut product: usize = 1;
            for x in surrounding_trees {
                if x != 0 {
                    product *= x;
                }
            }
            scenic_scores.push(product);
        }
        //break;
    }

    let max_scenic_score = *scenic_scores.iter().max().unwrap();
    println!("all scenic scores are {:?}", scenic_scores);
    println!("Best scenic score is {}", max_scenic_score);
    return max_scenic_score;
    //360 is too low.
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(solve_puzzle("src/inputs/day_8/input_example_1.txt") == 21);
    }

    #[test]
    fn part_1() {
        assert!(solve_puzzle("src/inputs/day_8/input.txt") == 1684);
    }

    #[test]
    fn example_2() {
        assert!(solve_puzzle_part2("src/inputs/day_8/input_example_1.txt") == 8);
    }

    #[test]
    fn part_2() {
        assert!(solve_puzzle_part2("src/inputs/day_8/input.txt") == 486540);
    }

    #[test]
    fn test_tree_count() {
        assert!(count_trees(&4, vec![1, 2, 3, 4, 5, 6, 7]) == 4);
        assert!(count_trees(&4, vec![1, 2, -1, 4, 5, 6, 7]) == 2);
        assert!(count_trees(&4, vec![1, 2, 9, 4, 5, 6, 7]) == 3);
        assert!(count_trees(&4, vec![9, 2, 9, 4, 5, 6, 7]) == 1);
        assert!(count_trees(&4, vec![-1, 2, 9, 4, 5, 6, 7]) == 0);
        assert!(count_trees(&3, vec![2, 6, 3, 3, -1]) == 2);
        assert!(count_trees(&4, vec![1, 2, 3, 3, 3, 4, 7]) == 6);
        assert!(count_trees(&7, vec![1, 2, 3, 2, 2, 7, 7]) == 6);
    }
}
