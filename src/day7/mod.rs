use std::collections::HashMap;

#[allow(dead_code)]
pub fn sum_of_smaller_folder_sizes(input: &str) -> usize {
    sum(tree(file_list(input)))
}

#[allow(dead_code)]
pub fn min_sized_folder_to_delete(input: &str) -> usize {
    let root = tree(file_list(input));
    let min_size = root.size - 40000000;
    min(root, min_size).unwrap()
}

struct Node<'a> {
    size: usize,
    children: HashMap<&'a str, Node<'a>>,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Node {
            size: 0,
            children: HashMap::new(),
        }
    }
}

fn file_list(input: &str) -> Vec<(Vec<&str>, usize)> {
    let mut paths = Vec::<(Vec<&str>, usize)>::new();
    let mut path = Vec::new();
    for line in input.lines() {
        if line == "$ cd /" {
            path.clear();
            path.push("/");
        } else if line == "$ cd .." {
            path.pop();
        } else if line.starts_with("$ cd ") {
            path.push(line.split_at(5).1);
        } else if line == "$ ls" {
            // ls
        } else {
            if line.starts_with("dir ") {
                // Do nothing
            } else {
                let size: usize = line.split_at(line.find(' ').unwrap()).0.parse().unwrap();
                paths.push((path.clone(), size));
            }
        }
    }
    paths
}

fn tree(files: Vec<(Vec<&str>, usize)>) -> Node {
    let mut root = Node::new();
    for file in files {
        let (path, size) = file;
        let mut node = &mut root;
        for folder in path {
            if folder == "/" {
                continue;
            }
            if !node.children.contains_key(folder) {
                let new_node = Node::new();
                node.children.insert(folder, new_node);
            }
            node = node.children.get_mut(folder).unwrap();
        }
        node.size += size;
    }
    update_size(&mut root);
    root
}

fn update_size(node: &mut Node) -> usize {
    let mut total = node.size;
    for (_, mut child) in &mut node.children {
        total += update_size(&mut child);
    }
    node.size = total;
    total
}

fn sum(node: Node) -> usize {
    let mut result = 0;
    for (_, child) in node.children {
        result += sum(child);
    }
    if node.size < 100000 {
        result += node.size;
    }
    result
}

fn min(node: Node, require_size: usize) -> Option<usize> {
    let mut best = 0;
    if require_size < node.size {
        best = node.size;
    }
    for (_, child) in node.children {
        match min(child, require_size) {
            Some(child_best) => {
                if best == 0 || best > child_best {
                    best = child_best;
                }
            }
            None => {}
        }
    }
    if best == 0 {
        None
    } else {
        Some(best)
    }
}

#[cfg(test)]
mod tests {
    use super::min_sized_folder_to_delete;
    use super::sum_of_smaller_folder_sizes;
    use std::fs;

    #[test]
    fn example_part1() {
        let input = "$ cd /\n\
            $ ls\n\
            dir a\n\
            14848514 b.txt\n\
            8504156 c.dat\n\
            dir d\n\
            $ cd a\n\
            $ ls\n\
            dir e\n\
            29116 f\n\
            2557 g\n\
            62596 h.lst\n\
            $ cd e\n\
            $ ls\n\
            584 i\n\
            $ cd ..\n\
            $ cd ..\n\
            $ cd d\n\
            $ ls\n\
            4060174 j\n\
            8033020 d.log\n\
            5626152 d.ext\n\
            7214296 k";
        assert_eq!(sum_of_smaller_folder_sizes(&input), 95437);
    }

    #[test]
    fn example_part2() {
        let input = "$ cd /\n\
        $ ls\n\
        dir a\n\
        14848514 b.txt\n\
        8504156 c.dat\n\
        dir d\n\
        $ cd a\n\
        $ ls\n\
        dir e\n\
        29116 f\n\
        2557 g\n\
        62596 h.lst\n\
        $ cd e\n\
        $ ls\n\
        584 i\n\
        $ cd ..\n\
        $ cd ..\n\
        $ cd d\n\
        $ ls\n\
        4060174 j\n\
        8033020 d.log\n\
        5626152 d.ext\n\
        7214296 k";
        assert_eq!(min_sized_folder_to_delete(&input), 24933642);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("src//day7/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(sum_of_smaller_folder_sizes(&input), 1391690);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("src//day7/input.txt")
            .expect("Should have been able to read the file");
        assert_eq!(min_sized_folder_to_delete(&input), 5469168);
    }
}
