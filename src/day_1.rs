#[warn(dead_code)]
use crate::generic;

#[derive(Debug)]
struct Elf {
    index: usize,
    calories: usize,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {

        let input_lines = generic::read_in_file("src/inputs/day_1/input.txt");

        let mut elf_index = 0;
        let mut calories = 0;
        let mut elves: Vec<Elf> = Vec::new();
        for line in input_lines {
            if line == "" {
                elves.push(Elf{index: elf_index, calories: calories});
                calories = 0;
                elf_index += 1;
            } else {
                calories += line.parse::<usize>().unwrap();
            }
        }

        elves.sort_by(|a, b| b.calories.cmp(&a.calories));

        let mut max_calories = 0;
        let mut max_elf_index = 0;
        for elf in elves.iter() {
            if elf.calories > max_calories {
                max_calories = elf.calories;
                max_elf_index = elf.index;
            }
        }

        println!("elves = {:?}", elves);
        println!("Top 3 = {}", elves[0].calories + elves[1].calories + elves[2].calories);
    }
}