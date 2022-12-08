use std::{collections::HashMap, hash::Hash};
#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}
#[derive(Debug, Clone)]
struct Directory {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Directory>,
}
struct FileSystem {
    pwd: Vec<String>,
    tree: HashMap<String, Directory>,
}

impl FileSystem {
    fn new() -> Self {
        let pwd = vec![String::from("/")];
        let mut tree = HashMap::new();
        tree.insert(
            pwd.join(""),
            Directory {
                name: String::from("/"),
                files: Vec::new(),
                subdirs: Vec::new(),
            },
        );
        Self { pwd, tree }
    }
    fn mkdir(&mut self, dir_name: &str) {
        let path = format!("{}{}/", self.pwd.join(""), dir_name);
        self.tree.insert(
            path,
            Directory {
                name: format!("{}/", dir_name),
                files: Vec::new(),
                subdirs: Vec::new(),
            },
        );
        self.tree
            .get_mut(&self.pwd.join(""))
            .unwrap()
            .subdirs
            .push(Directory {
                name: String::from(dir_name),
                files: Vec::new(),
                subdirs: Vec::new(),
            });
    }
    fn touch(&mut self, file_name: &str, file_size: u32) {
        self.tree
            .get_mut(&self.pwd.join(""))
            .unwrap()
            .files
            .push(File {
                name: String::from(file_name),
                size: file_size,
            });
    }
    fn change_directory(&mut self, path: &str) {
        match path {
            "/" => self.pwd = vec![String::from("/")],
            ".." => {
                self.pwd.pop();
            }
            _ => self.pwd.push(format!("{}/", path)),
        }
    }
    fn get_directory_size(&self, path: &str) -> u32 {
        // Get the size of this directory
        // Sum the size of every file in this directory and in every subdirectory
        let mut size = self
            .tree
            .get(path)
            .unwrap()
            .files
            .iter()
            .map(|file| file.size)
            .sum::<u32>();
        for subdir in self.tree.get(path).unwrap().subdirs.iter() {
            size += self.get_directory_size(&format!("{}{}/", path, subdir.name));
        }
        size
    }
    fn print_tree(&self) {
        // This function is just used to print the tree for debugging
        fn print_tree_helper<T: Hash + Eq + std::fmt::Display>(
            tree: &HashMap<T, Directory>,
            path: &str,
        ) {
            for (k, dir) in tree.iter() {
                println!("{}{}: {:?}", path, k, dir);
                print_tree_helper(
                    &dir.subdirs
                        .iter()
                        .cloned()
                        .map(|d| (d.name.clone(), d))
                        .collect(),
                    &format!("{}{}/", path, dir.name),
                );
            }
        }
        print_tree_helper(&self.tree, "");
    }
}

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn input_generator(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    for line in input.lines() {
        match &line[0..4] {
            "$ cd" => fs.change_directory(&line[5..]),
            "$ ls" => {}
            "dir " => fs.mkdir(&line[4..]),
            _ => {
                let mut iter = line.split_whitespace();
                let file_size = iter.next().unwrap().parse::<u32>().unwrap();
                let file_name = iter.next().unwrap();
                fs.touch(file_name, file_size);
            }
        }
    }
    println!("Cleaning tree...");
    fs
}

#[aoc(day7, part1)]
fn solve_part1(fs: &FileSystem) -> u32 {
    // Print tree
    fs.print_tree();
    // Get a vector of directories
    fs.tree
        .iter()
        .map(|(k, _)| fs.get_directory_size(k))
        .filter(|size| *size < 100000)
        .sum()
}

#[aoc(day7, part2)]
fn solve_part2(fs: &FileSystem) -> u32 {
    let unused_space = 70000000 - fs.get_directory_size("/");
    fs.tree
        .iter()
        .map(|(k, _)| fs.get_directory_size(k))
        .filter(|size| *size + unused_space >= 30000000)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(solve_part1(&input_generator(input)), 95437);
    }

    #[test]
    fn test_part2() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(solve_part2(&input_generator(input)), 24933642);
    }
}
