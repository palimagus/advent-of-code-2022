use std::{collections::HashMap, fmt::format};

struct File {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

struct FileSystem {
    pwd: String,
    tree: HashMap<String, Directory>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            pwd: "".to_string(),
            tree: HashMap::new(),
        }
    }
    fn print(&self) {
        println!("pwd: {}", self.pwd);
        for (key, value) in &self.tree {
            println!("{}: {}", key, value.name);
        }
    }
    fn ls(&self) {
        println!("ls {}", self.pwd);
        self.print();
        // Get directory from pwd
        let dir = self.tree.get(&self.pwd).unwrap();
        // Print all files
        for file in &dir.files {
            println!("file {}", file.name);
        }
        // Print all directories
        for dir in &dir.directories {
            println!("dir {}", dir.name);
        }
    }
    fn cd(&mut self, path: &str) {
        // If path doesn't exists make an empty directory
        println!("cd {}", path);
        match path {
            "/" => {
                self.pwd = "/".to_string();
            }
            ".." => {
                let mut path = self.pwd.clone();
                path.pop();
                self.pwd = path;
            }
            _ => {
                self.pwd.push_str(path);
            }
        }
        // Make directory if it doesn't exist
        if !self.tree.contains_key(&self.pwd) {
            self.mkdir(path);
        }
    }
    fn mkdir(&mut self, name: &str) {
        self.tree.insert(
            name.to_string(),
            Directory {
                name: name.to_string(),
                files: vec![],
                directories: vec![],
            },
        );
    }
}

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day7, part1)]
fn solve_part1(input: &[String]) -> usize {
    let mut fs = FileSystem::new();
    for line in input {
        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap();
        match cmd {
            "$" => match parts.next().unwrap() {
                "cd" => {
                    let path = parts.next().unwrap();
                    fs.cd(path);
                }
                "ls" => fs.ls(),
                _ => {}
            },
            "dir" => {
                let name = parts.next().unwrap();
                fs.mkdir(name);
            }
            _ => {
                let size = cmd.parse::<usize>().unwrap();
                let name = parts.next().unwrap();
                fs.tree.get_mut(&fs.pwd).unwrap().files.push(File {
                    name: name.to_string(),
                    size,
                });
            }
        }
    }
    fs.tree.len()
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
}
