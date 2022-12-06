use crate::generic;
use std::collections::HashSet;


struct ElfSection {
    start_section: usize,
    end_section: usize,
}

impl ElfSection {
    fn from_string(input_string: &str) -> Self {
        let split_string: Vec<&str> = input_string.split("-").collect();
        return Self { start_section: split_string[0].parse::<usize>().unwrap(), end_section: split_string[1].parse::<usize>().unwrap() };
    }
}

struct ElfPair {
    elf_sections: Vec<ElfSection>,
}

impl ElfPair {
    fn from_string(input_string: String) -> Self {
        let split_string: Vec<&str> = input_string.split(",").collect();
        return Self { elf_sections: vec![ElfSection::from_string(split_string[0]), ElfSection::from_string(split_string[1])] };
    }

    fn complete_overlap(&self) -> bool {
        return 
            (self.elf_sections[0].start_section <= self.elf_sections[1].start_section 
                && self.elf_sections[0].end_section >= self.elf_sections[1].end_section)
            ||
            (self.elf_sections[1].start_section <= self.elf_sections[0].start_section 
                && self.elf_sections[1].end_section >= self.elf_sections[0].end_section)
    }

    fn any_overlap(&self) -> bool {
        return 
            (self.elf_sections[0].start_section <= self.elf_sections[1].end_section
                && self.elf_sections[0].end_section >= self.elf_sections[1].start_section)
            ||
            (self.elf_sections[1].start_section <= self.elf_sections[0].end_section
                && self.elf_sections[1].end_section >= self.elf_sections[0].start_section)
    }

}


fn loop_through_list(input_file: &str, any_overlap: bool) -> usize {
    let input_lines = generic::read_in_file(input_file);
    let mut elf_pairs: Vec<ElfPair> = input_lines.iter().map(|x| ElfPair::from_string(x.clone())).collect();

    let mut count_overlaps = 0;
    for elf_pair in elf_pairs {
        if (elf_pair.any_overlap() && any_overlap) || (elf_pair.complete_overlap() && !any_overlap)  {
            count_overlaps += 1;
        }
    }

    println!("Overlap of {} Elves.", count_overlaps);
    return count_overlaps;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(loop_through_list("src/inputs/day_4/input_example_1.txt", false) == 2);
    }

    #[test]
    fn part_1() {
        //part 1
        assert!(loop_through_list("src/inputs/day_4/input.txt", false) == 424);
        //part 2
        assert!(loop_through_list("src/inputs/day_4/input.txt", true) == 804);
    }
}
