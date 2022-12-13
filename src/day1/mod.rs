#[allow(dead_code)]
pub fn top_elf_calory(input: &str) -> i32 {
    calculator(CalculationLogic::OneTopCalculator, calories_of_elfs(input)).calculate()
}

#[allow(dead_code)]
pub fn sum_of_top_three_elf_calory(input: &str) -> i32 {
    calculator(
        CalculationLogic::ThreeTopSumCalculator,
        calories_of_elfs(input),
    )
    .calculate()
}

enum CalculationLogic {
    OneTopCalculator,
    ThreeTopSumCalculator,
}

fn calories_of_elfs(input: &str) -> Vec<Vec<i32>> {
    Reader { input: input }.calories_of_elfs()
}

fn calculator(logic: CalculationLogic, input: Vec<Vec<i32>>) -> Box<dyn Calculator> {
    match logic {
        CalculationLogic::OneTopCalculator => Box::new(OneTopCalculator::new(input)),
        CalculationLogic::ThreeTopSumCalculator => Box::new(ThreeTopSumCalculator::new(input)),
    }
}

struct Reader<'a> {
    input: &'a str,
}

impl<'a> Reader<'a> {
    fn calories_of_elfs(&self) -> Vec<Vec<i32>> {
        self.split_by_calories(self.split_by_elfs())
    }

    fn split_by_elfs(&self) -> Vec<&str> {
        self.input.split("\n\n").collect()
    }

    fn split_by_calories(&self, input: Vec<&str>) -> Vec<Vec<i32>> {
        input
            .iter()
            .map(|x| self.to_int(x.split("\n").collect()))
            .collect()
    }

    fn to_int(&self, input: Vec<&str>) -> Vec<i32> {
        input.iter().map(|x| x.parse::<i32>().unwrap()).collect()
    }
}

trait Calculator {
    fn new(input: Vec<Vec<i32>>) -> Self
    where
        Self: Sized;
    fn calculate(&mut self) -> i32;
}

struct OneTopCalculator {
    input: Vec<Vec<i32>>,
}

impl Calculator for OneTopCalculator {
    fn new(input: Vec<Vec<i32>>) -> Self {
        OneTopCalculator { input: input }
    }

    fn calculate(&mut self) -> i32 {
        self.input.iter().map(|x| x.iter().sum()).max().unwrap()
    }
}

struct ThreeTopSumCalculator {
    input: Vec<Vec<i32>>,
    top_calories: [i32; 3],
}

impl ThreeTopSumCalculator {
    fn calory_sum_of_elves(&self) -> Vec<i32> {
        self.input.iter().map(|x| x.iter().sum()).collect()
    }

    fn add(&mut self, calory: i32) {
        if calory > self.min() {
            self.replace_min(calory);
        }
    }

    fn sum(&self) -> i32 {
        self.top_calories.iter().sum()
    }

    fn min(&self) -> i32 {
        *self.top_calories.iter().min().unwrap()
    }

    fn replace_min(&mut self, calory: i32) {
        self.top_calories[self.min_index()] = calory;
    }

    fn min_index(&self) -> usize {
        self.top_calories
            .iter()
            .position(|calory| *calory == self.min())
            .unwrap()
    }
}

impl Calculator for ThreeTopSumCalculator {
    fn new(input: Vec<Vec<i32>>) -> Self {
        ThreeTopSumCalculator {
            input: input,
            top_calories: [0, 0, 0],
        }
    }

    fn calculate(&mut self) -> i32 {
        for sum in self.calory_sum_of_elves() {
            self.add(sum);
        }
        self.sum()
    }
}

#[cfg(test)]
mod tests {
    use super::sum_of_top_three_elf_calory;
    use super::top_elf_calory;
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
