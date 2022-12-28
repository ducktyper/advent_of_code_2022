use std::collections::HashSet;

#[allow(dead_code)]
pub fn n_tail_positions(input: &str) -> usize {
    fn move_tail(head: (i32, i32), tail: &mut (i32, i32), tail_history: &mut HashSet<(i32, i32)>) {
        if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
            return;
        }

        if head.0 == tail.0 {
            // vertical
            if head.1 > tail.1 {
                tail.1 += 1;
            } else {
                tail.1 -= 1;
            }
        } else if head.1 == tail.1 {
            // holizontal
            if head.0 > tail.0 {
                tail.0 += 1;
            } else {
                tail.0 -= 1;
            }
        } else {
            // diagonal
            if head.1 > tail.1 {
                tail.1 += 1;
            } else {
                tail.1 -= 1;
            }
            if head.0 > tail.0 {
                tail.0 += 1;
            } else {
                tail.0 -= 1;
            }
        }

        tail_history.insert(tail.clone());
    }

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut tail_history: HashSet<(i32, i32)> = HashSet::new();
    tail_history.insert(tail);

    for line in input.lines() {
        let (direction, count) = line.split_once(" ").unwrap();
        let count: usize = count.parse().unwrap();

        match direction {
            "L" => {
                for _ in 0..count {
                    head.0 -= 1;
                    move_tail(head, &mut tail, &mut tail_history);
                }
            }
            "R" => {
                for _ in 0..count {
                    head.0 += 1;
                    move_tail(head, &mut tail, &mut tail_history);
                }
            }
            "U" => {
                for _ in 0..count {
                    head.1 += 1;
                    move_tail(head, &mut tail, &mut tail_history);
                }
            }
            "D" => {
                for _ in 0..count {
                    head.1 -= 1;
                    move_tail(head, &mut tail, &mut tail_history);
                }
            }
            _ => {}
        }
    }

    tail_history.len()
}

#[allow(dead_code)]
pub fn n_longer_tail_positions(input: &str) -> usize {
    fn move_tail(
        head: (i32, i32),
        tails: &mut [(i32, i32); 9],
        tail_history: &mut HashSet<(i32, i32)>,
    ) {
        for i in 0..tails.len() {
            let pre_node = if i == 0 { head } else { tails[i - 1] };

            if (pre_node.0 - tails[i].0).abs() <= 1 && (pre_node.1 - tails[i].1).abs() <= 1 {
                continue;
            }

            if pre_node.0 == tails[i].0 {
                // vertical
                if pre_node.1 > tails[i].1 {
                    tails[i].1 += 1;
                } else {
                    tails[i].1 -= 1;
                }
            } else if pre_node.1 == tails[i].1 {
                // holizontal
                if pre_node.0 > tails[i].0 {
                    tails[i].0 += 1;
                } else {
                    tails[i].0 -= 1;
                }
            } else {
                // diagonal
                if pre_node.1 > tails[i].1 {
                    tails[i].1 += 1;
                } else {
                    tails[i].1 -= 1;
                }
                if pre_node.0 > tails[i].0 {
                    tails[i].0 += 1;
                } else {
                    tails[i].0 -= 1;
                }
            }

            if i == tails.len() - 1 {
                tail_history.insert(tails[i].clone());
            }
        }
    }

    let mut head: (i32, i32) = (0, 0);
    let mut tails: [(i32, i32); 9] = [(0, 0); 9];
    let mut tail_history: HashSet<(i32, i32)> = HashSet::new();
    tail_history.insert(tails.last().unwrap().clone());

    for line in input.lines() {
        let (direction, count) = line.split_once(" ").unwrap();
        let count: usize = count.parse().unwrap();

        match direction {
            "L" => {
                for _ in 0..count {
                    head.0 -= 1;
                    move_tail(head, &mut tails, &mut tail_history);
                }
            }
            "R" => {
                for _ in 0..count {
                    head.0 += 1;
                    move_tail(head, &mut tails, &mut tail_history);
                }
            }
            "U" => {
                for _ in 0..count {
                    head.1 += 1;
                    move_tail(head, &mut tails, &mut tail_history);
                }
            }
            "D" => {
                for _ in 0..count {
                    head.1 -= 1;
                    move_tail(head, &mut tails, &mut tail_history);
                }
            }
            _ => {}
        }
    }

    tail_history.len()
}

#[cfg(test)]
mod tests {
    use super::n_longer_tail_positions;
    use super::n_tail_positions;
    use std::fs;

    #[test]
    fn example_part1() {
        let input = "R 4\n\
            U 4\n\
            L 3\n\
            D 1\n\
            R 4\n\
            D 1\n\
            L 5\n\
            R 2";
        assert_eq!(n_tail_positions(&input), 13);
    }

    #[test]
    fn example_part2() {
        let input = "R 5\n\
        U 8\n\
        L 8\n\
        D 3\n\
        R 17\n\
        D 10\n\
        L 25\n\
        U 20";
        assert_eq!(n_longer_tail_positions(&input), 36);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day9/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(n_tail_positions(&input), 6494);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day9/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(n_longer_tail_positions(&input), 2691);
    }
}
