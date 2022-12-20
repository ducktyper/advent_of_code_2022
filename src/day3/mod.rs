use std::str::Lines;

#[allow(dead_code)]
pub fn total_priority<'a>(rucksacks: impl Iterator<Item = Vec<&'a str>>) -> i32 {
    let mut result = 0;
    for compartments in rucksacks {
        result += rucksack_score(compartments);
    }
    result
}

fn rucksack_score(compartments: Vec<&str>) -> i32 {
    let mut result = 0;
    let mut priority_map = [0; 52];
    let mut binary_mark = 1; // 0b1 -> 0b10 -> 0b100
    let last_compartment = compartments[compartments.len() - 1];
    for compartment in compartments {
        for char in compartment.chars() {
            if compartment == last_compartment
                && priority_map[priority_index(char)] == binary_mark - 1
            {
                result += priority_index(char) as i32 + 1;
            }
            priority_map[priority_index(char)] |= binary_mark;
        }
        binary_mark *= 2;
    }
    result
}

struct EachTwo<'a> {
    lines: Lines<'a>,
}

impl<'a> EachTwo<'a> {
    fn new(input: &'a str) -> Self {
        EachTwo {
            lines: input.lines(),
        }
    }
}

impl<'a> Iterator for EachTwo<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        if line == None {
            return None;
        }
        let line = line.unwrap();
        let (compartment_1, compartment_2) = line.split_at(line.len() / 2);
        Option::Some(vec![compartment_1, compartment_2])
    }
}

struct EachThree<'a> {
    lines: Lines<'a>,
}

impl<'a> EachThree<'a> {
    fn new(input: &'a str) -> Self {
        EachThree {
            lines: input.lines(),
        }
    }
}

impl<'a> Iterator for EachThree<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let compartment_1 = self.lines.next();
        if compartment_1 == None {
            return None;
        }
        let compartment_1 = compartment_1.unwrap();
        let compartment_2 = self.lines.next().unwrap();
        let compartment_3 = self.lines.next().unwrap();
        Option::Some(vec![compartment_1, compartment_2, compartment_3])
    }
}

fn priority_index(c: char) -> usize {
    let ascii_num = c as u32; // 'A' 65, 'Z' 90, 'a' 97, 'z' 122
    if ascii_num <= 90 {
        (ascii_num - 39) as usize // 'A' 64 => priority index 26
    } else {
        (ascii_num - 97) as usize // 'a' 97 => priority index 0
    }
}

#[cfg(test)]
mod tests {
    use super::total_priority;
    use super::EachThree;
    use super::EachTwo;
    use std::fs;

    #[test]
    fn example_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
            PmmdzqPrVvPwwTWBwg\n\
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
            ttgJtRGJQctTZtZT\n\
            CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(total_priority(EachTwo::new(&input)), 157);
    }

    #[test]
    fn example_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(total_priority(EachThree::new(&input)), 70);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day3/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(total_priority(EachTwo::new(&input)), 7795);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day3/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(total_priority(EachThree::new(&input)), 2703);
    }
}
