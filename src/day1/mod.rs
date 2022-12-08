#[allow(dead_code)]
pub fn top_elf_calory(input: &str) -> i32 {
    let mut max_calory = 0;
    for elf in input.split("\n\n") {
        let total_calory: i32 = elf.split("\n").map(|calory| calory.parse::<i32>().unwrap()).sum();
        if total_calory > max_calory {
            max_calory = total_calory;
        }
    }
    max_calory
}

#[allow(dead_code)]
pub fn sum_of_top_three_elf_calory(input: &str) -> i32 {
    let mut top_calories: [i32;3] = [0, 0, 0];
    let mut min_index: usize = 0;

    for elf in input.split("\n\n") {
        let total_calory: i32 = elf.split("\n").map(|calory| calory.parse::<i32>().unwrap()).sum();
        if total_calory > top_calories[min_index] {
            top_calories[min_index] = total_calory;
            let min_value = top_calories.iter().min().unwrap();
            min_index = top_calories.iter().position(|calory| calory == min_value).unwrap();
        }
    }
    top_calories.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::top_elf_calory;
    use super::sum_of_top_three_elf_calory;
    use std::fs;

    #[test]
    fn one_elf_with_multiple_calory() {
        let input = "10\n10\n10";
        assert_eq!(top_elf_calory(&input), 30);
    }

    #[test]
    fn two_elf_with_multiple_calory() {
        let input = "10\n10\n10\n\n5\n5";
        assert_eq!(top_elf_calory(&input), 30);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day1/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(top_elf_calory(&input), 66719);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day1/input.txt")
        .expect("Should have been able to read the file");
        assert_eq!(sum_of_top_three_elf_calory(&input), 198551);
    }
}
