use std::cmp;

#[allow(dead_code)]
pub fn visible_trees(input: &str) -> usize {
    let mut tree = Vec::<Vec<usize>>::new();
    for line in input.lines() {
        tree.push(
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect(),
        );
    }

    let mut visible_tree = Vec::<Vec<bool>>::new();
    for row in &tree {
        let mut visible_row = Vec::<bool>::new();
        for _ in row {
            visible_row.push(false);
        }
        visible_tree.push(visible_row);
    }

    let n_row = tree.len();
    let n_col = tree[0].len();

    // left to right
    for i in 0..n_row {
        let mut max = -1;
        for j in 0..n_col {
            let val = tree[i][j];
            if max < val as i32 {
                max = val as i32;
                visible_tree[i][j] = true;
            }
        }
    }
    // right to left
    for i in 0..n_row {
        let mut max = -1;
        for j in (0..n_col).rev() {
            let val = tree[i][j];
            if max < val as i32 {
                max = val as i32;
                visible_tree[i][j] = true;
            }
        }
    }
    // top to down
    for j in 0..n_col {
        let mut max = -1;
        for i in 0..n_row {
            let val = tree[i][j];
            if max < val as i32 {
                max = val as i32;
                visible_tree[i][j] = true;
            }
        }
    }
    // down to top
    for j in 0..n_col {
        let mut max = -1;
        for i in (0..n_row).rev() {
            let val = tree[i][j];
            if max < val as i32 {
                max = val as i32;
                visible_tree[i][j] = true;
            }
        }
    }

    let mut result: usize = 0;
    for row in visible_tree {
        for sell in row {
            if sell {
                result += 1;
            }
        }
    }
    result
}

#[allow(dead_code)]
pub fn scenic_score(input: &str) -> usize {
    let mut tree = Vec::<Vec<usize>>::new();
    for line in input.lines() {
        tree.push(
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect(),
        );
    }

    let n_row = tree.len();
    let n_col = tree[0].len();

    let mut result = 0;
    for i in 0..n_row {
        for j in 0..n_col {
            let val = tree[i][j];

            // to right
            let mut right = 0;
            if j != n_col - 1 {
                for jx in j + 1..n_col {
                    right += 1;
                    if val <= tree[i][jx] {
                        break;
                    }
                }
            }

            // to left
            let mut left = 0;
            if j != 0 {
                for jx in (0..j).rev() {
                    left += 1;
                    if val <= tree[i][jx] {
                        break;
                    }
                }
            }

            // to down
            let mut down = 0;
            if i != n_row - 1 {
                for ix in i + 1..n_row {
                    down += 1;
                    if val <= tree[ix][j] {
                        break;
                    }
                }
            }

            // to top
            let mut top = 0;
            if i != 0 {
                for ix in (0..i).rev() {
                    top += 1;
                    if val <= tree[ix][j] {
                        break;
                    }
                }
            }

            result = cmp::max(result, right * left * down * top);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::scenic_score;
    use super::visible_trees;
    use std::fs;

    #[test]
    fn example_part1() {
        let input = "30373\n\
            25512\n\
            65332\n\
            33549\n\
            35390";
        assert_eq!(visible_trees(&input), 21);
    }

    #[test]
    fn example_part2() {
        let input = "30373\n\
            25512\n\
            65332\n\
            33549\n\
            35390";
        assert_eq!(scenic_score(&input), 8);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day8/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(visible_trees(&input), 1679);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day8/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(scenic_score(&input), 536625);
    }
}
