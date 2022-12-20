#[allow(dead_code)]
pub fn fully_contained_pair(input: &str) -> i32 {
    let mut result = 0;
    for data in parse(input) {
        if (data[0][0] <= data[1][0] && data[0][1] >= data[1][1])
            || (data[0][0] >= data[1][0] && data[0][1] <= data[1][1])
        {
            result += 1;
        }
    }
    result
}

#[allow(dead_code)]
pub fn contained_pair(input: &str) -> i32 {
    let mut result = 0;
    for data in parse(input) {
        if (data[0][0] <= data[1][0] && data[0][1] >= data[1][0])
            || (data[1][0] <= data[0][0] && data[1][1] >= data[0][0])
        {
            result += 1;
        }
    }
    result
}

fn parse(input: &str) -> Vec<[[i32; 2]; 2]> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(input: &str) -> [[i32; 2]; 2] {
    to_array(
        input
            .split(',')
            .map(|x| to_array(x.split('-').map(|x| x.parse().unwrap()).collect()))
            .collect(),
    )
}

fn to_array<T: Copy>(array: Vec<T>) -> [T; 2] {
    [array[0], array[1]]
}

#[cfg(test)]
mod tests {
    use super::contained_pair;
    use super::fully_contained_pair;
    use std::fs;

    #[test]
    fn example_part1() {
        let input = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
        assert_eq!(fully_contained_pair(&input), 2);
    }

    #[test]
    fn example_part2() {
        let input = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
        assert_eq!(contained_pair(&input), 4);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day4/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(fully_contained_pair(&input), 518);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day4/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(contained_pair(&input), 909);
    }
}
