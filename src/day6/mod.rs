#[allow(dead_code)]
pub fn marker(input: &str, distinct_chars: usize) -> i32 {
    if input.len() < distinct_chars {
        return -1;
    }

    fn is_distinct(input: &str) -> bool {
        for i in 0..input.len() - 1 {
            for j in i + 1..input.len() {
                if input.chars().nth(i) == input.chars().nth(j) {
                    return false;
                }
            }
        }
        true
    }

    for i in 0..input.len() - distinct_chars + 1 {
        if is_distinct(&input[i..i + distinct_chars]) {
            return (i + distinct_chars) as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::marker;
    use std::fs;

    #[test]
    fn example_part1() {
        assert_eq!(marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn example_part2() {
        assert_eq!(marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day6/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(marker(&input, 4), 1042);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day6/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(marker(&input, 14), 2980);
    }
}
