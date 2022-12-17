#[allow(dead_code)]
pub fn total_priority(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let (mut pmap_a, mut pmap_b) = ([false; 52], [false; 52]);
        for i in 0..line.len() / 2 {
            let char = line.chars().nth(i).unwrap();
            pmap_a[priority_index(char)] = true;
        }
        for i in line.len() / 2..line.len() {
            let char = line.chars().nth(i).unwrap();
            if pmap_a[priority_index(char)] == true && pmap_b[priority_index(char)] == false {
                result += priority_index(char) as i32 + 1
            }
            pmap_b[priority_index(char)] = true;
        }
    }
    result
}

#[allow(dead_code)]
pub fn total_priority_2(input: &str) -> i32 {
    let mut result = 0;
    let mut line_group = Vec::new();
    for line in input.lines() {
        line_group.push(line);
        if line_group.len() == 3 {
            let (mut pmap_a, mut pmap_b, mut pmap_c) = ([false; 52], [false; 52], [false; 52]);
            for char in line_group[0].chars() {
                pmap_a[priority_index(char)] = true;
            }
            for char in line_group[1].chars() {
                pmap_b[priority_index(char)] = true;
            }
            for char in line_group[2].chars() {
                if pmap_a[priority_index(char)] == true
                    && pmap_b[priority_index(char)] == true
                    && pmap_c[priority_index(char)] == false
                {
                    result += priority_index(char) as i32 + 1
                }
                pmap_c[priority_index(char)] = true;
            }
            line_group.clear();
        }
    }
    result
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
    use super::total_priority_2;
    use std::fs;

    #[test]
    fn example_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
            PmmdzqPrVvPwwTWBwg\n\
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
            ttgJtRGJQctTZtZT\n\
            CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(total_priority(&input), 157);
    }

    #[test]
    fn example_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(total_priority_2(&input), 70);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day3/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(total_priority(&input), 7795);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day3/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(total_priority_2(&input), 2703);
    }
}
