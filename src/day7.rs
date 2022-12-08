use std::collections::HashMap;

struct File {
    name: String,
    size: u32,
}
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
    fn new() -> FileSystem {
        let pwd = vec![String::from("/")];
        let mut tree = HashMap::new();
        tree.insert(
            String::from("/"),
            Directory {
                name: String::from("/"),
                files: Vec::new(),
                subdirs: Vec::new(),
            },
        );
        FileSystem { pwd, tree }
    }
    fn change_directory(&mut self, path: &str) {
        match path {
            ".." => {
                self.pwd.pop();
            }
            "/" => {
                self.pwd = vec![String::from("/")];
            }
            _ => {
                self.pwd.push(String::from(path));
            }
        }
    }
    fn get_size_of_directory(&self, path: &str) -> u32 {
        let mut size = 0;
        let dir = self.tree.get(path).unwrap();
        for file in &dir.files {
            size += file.size;
        }
        for subdir in &dir.subdirs {
            size += self.get_size_of_directory(&subdir.name);
        }
        size
    }
    fn mkdir(&mut self, folder: Directory) {}
    fn touch(&mut self, file: File) {}
}

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn input_generator(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    for line in input.lines() {
        match &line[0..4] {
            "$ cd" => {
                fs.change_directory(&line[5..]);
            }
            "$ ls" => {}
            "dir " => {
                fs.mkdir(Directory {
                    name: String::from(&line[4..]),
                    files: Vec::new(),
                    subdirs: Vec::new(),
                });
            }
            _ => {
                let mut parts = line.split_whitespace();
                let size = parts.next().unwrap().parse::<u32>().unwrap();
                let name = parts.next().unwrap();
                fs.touch(File {
                    name: String::from(name),
                    size,
                });
            }
        }
    }
    fs
}

#[aoc(day7, part1)]
fn solve_part1(fs: &FileSystem) -> u32 {
    let mut directory_sizes = HashMap::new();
    for (path, dir) in &fs.tree {
        directory_sizes.insert(path, fs.get_size_of_directory(path));
    }

    // Get the directories that are less than 100000 total size and sum their sizes
    directory_sizes
        .iter()
        .filter(|(_, size)| **size < 100000)
        .map(|(_, size)| *size)
        .sum()
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
