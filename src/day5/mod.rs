use std::collections::LinkedList;

#[allow(dead_code)]
pub fn top_chars(input: &str) -> String {
    let (mut stacks, commands) = parse(input);
    for command in commands {
        let (count, from, to) = (
            command[0],
            (command[1] - 1) as usize,
            (command[2] - 1) as usize,
        );
        for _ in 0..count {
            let val = stacks[from].pop_front().unwrap();
            stacks[to].push_front(val);
        }
    }
    fn to_str(val: Option<&char>) -> String {
        match val {
            None => "".to_string(),
            Some(x) => x.to_string(),
        }
    }
    stacks
        .iter()
        .map(|stack| to_str(stack.front()))
        .collect::<Vec<String>>()
        .join("")
}

#[allow(dead_code)]
pub fn top_chars2(input: &str) -> String {
    let (mut stacks, commands) = parse(input);
    for command in commands {
        let (count, from, to) = (
            command[0],
            (command[1] - 1) as usize,
            (command[2] - 1) as usize,
        );
        let mut container: Vec<char> = Vec::new();
        for _ in 0..count {
            let val = stacks[from].pop_front().unwrap();
            container.push(val);
        }
        for char in container.iter().rev() {
            stacks[to].push_front(*char);
        }
    }
    fn to_str(val: Option<&char>) -> String {
        match val {
            None => "".to_string(),
            Some(x) => x.to_string(),
        }
    }
    stacks
        .iter()
        .map(|stack| to_str(stack.front()))
        .collect::<Vec<String>>()
        .join("")
}

fn parse(input: &str) -> (Vec<LinkedList<char>>, Vec<[i32; 3]>) {
    let mut stacks = emtpy_stacks(input);
    let stacks_size = stacks.len();
    let mut commands = Vec::<[i32; 3]>::new();

    let mut read_commands = false;

    for line in input.lines() {
        if line == "" {
            read_commands = true;
            continue;
        }

        if read_commands {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            commands.push([
                words[1].parse().unwrap(),
                words[3].parse().unwrap(),
                words[5].parse().unwrap(),
            ]);
        } else {
            for i in 0..stacks_size {
                let index: usize = 1 + i * 4;
                if line.chars().count() <= index + 1 {
                    break;
                }
                let char = line.chars().nth(index).unwrap();
                if char != ' ' {
                    stacks[i].push_back(char);
                }
            }
        }
    }
    (stacks, commands)
}

fn emtpy_stacks(input: &str) -> Vec<LinkedList<char>> {
    let mut stack_label_line = "";

    for line in input.lines() {
        if line == "" {
            break;
        }
        stack_label_line = line;
    }

    let mut stacks: Vec<LinkedList<char>> = Vec::new();
    for _ in stack_label_line.split_whitespace().collect::<Vec<&str>>() {
        stacks.push(LinkedList::new());
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::top_chars;
    use super::top_chars2;
    use std::fs;

    #[test]
    fn parse_test() {
        let input = "    [D]\n\
            [N] [C]\n\
            [Z] [M] [P]\n\
            1   2   3\n\
            \n\
            move 1 from 2 to 1\n\
            move 3 from 1 to 3\n\
            move 2 from 2 to 1\n\
            move 1 from 1 to 2";
        let (stacks, commands) = parse(input);
        assert_eq!(
            format!("{:?}", stacks),
            "[['N', 'Z'], ['D', 'C', 'M'], ['P']]"
        );
        assert_eq!(
            format!("{:?}", commands),
            "[[1, 2, 1], [3, 1, 3], [2, 2, 1], [1, 1, 2]]"
        );
    }

    #[test]
    fn example_part1() {
        let input = "    [D]\n\
            [N] [C]\n\
            [Z] [M] [P]\n\
             1   2   3\n\
            \n\
            move 1 from 2 to 1\n\
            move 3 from 1 to 3\n\
            move 2 from 2 to 1\n\
            move 1 from 1 to 2";
        assert_eq!(top_chars(&input), "CMZ");
    }
    #[test]
    fn example_part2() {
        let input = "    [D]\n\
            [N] [C]\n\
            [Z] [M] [P]\n\
            1   2   3\n\
            \n\
            move 1 from 2 to 1\n\
            move 3 from 1 to 3\n\
            move 2 from 2 to 1\n\
            move 1 from 1 to 2";
        assert_eq!(top_chars2(&input), "MCD");
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day5/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(top_chars(&input), "ZWHVFWQWW");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day5/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(top_chars2(&input), "HZFZCCWWV");
    }
}
