use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct File {
    name: String,
    size: usize,
}

struct FileSystem {
    pwd: String,
    tree: HashMap<String, Vec<File>>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            pwd: "".to_string(),
            tree: HashMap::new(),
        }
    }

    fn change_directory(&mut self, path: &str) {
        if path.starts_with("/") {
            self.pwd = path.to_string();
        } else {
            self.pwd = format!("{}/{}", self.pwd, path);
        }
        // Check that pwd exists in tree
        if !self.tree.contains_key(&self.pwd) {
            self.tree.insert(self.pwd.clone(), vec![]);
        }
    }

    fn list(&self) -> &Vec<File> {
        self.tree.get(&self.pwd).unwrap()
    }

    fn print(&self) {
        println!("pwd: {}", self.pwd);
        for entry in self.list() {
            println!("{} {}", entry.size, entry.name);
        }
    }

    fn size_of_folder(&self, path: &str) -> usize {
        let mut size = 0;
        for entry in self.tree.get(path).unwrap() {
            size += entry.size;
        }
        size
    }

    fn recursive_size_of_folder(&self, path: &str) -> usize {
        let mut size = 0;
        for entry in self.tree.get(path).unwrap() {
            size += entry.size;
            if self.tree.contains_key(&format!("{}/{}", path, entry.name)) {
                size += self.recursive_size_of_folder(&format!("{}/{}", path, entry.name));
            }
        }
        size
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> FileSystem {
    // Each line in input is either a command or the description of a file
    // Command are of the format: "$ command args"
    // File description are of the format: "size name"

    // For each line, if it starts with a $, it's a command and we need to parse it
    // Otherwise, it's a file description and we need to parse it
    let mut fs = FileSystem::new();

    for line in input.lines() {
        let s = line.trim();
        if s.starts_with("$") {
            // Parse command
            let mut command = s.split_whitespace();
            let command_name = command.next().unwrap();
            let args = command.collect::<Vec<&str>>();
            match command_name {
                "cd" => {
                    fs.change_directory(args[0]);
                }
                _ => {}
            }
        } else {
            // Parse entry description
            let mut file = s.split_whitespace();

            // If the first information is 'dir', it's a directory
            // Otherwise it's the size of the file
            let data = file.next().unwrap();
            match data {
                "dir" => {
                    // Make a new directory in the current directory
                    let name = file.next().unwrap();
                    // Check that the directory doesn't already exist
                    if !fs.tree.contains_key(&format!("{}/{}", fs.pwd, name)) {
                        fs.tree.insert(format!("{}/{}", fs.pwd, name), vec![]);
                    }
                }
                _ => {
                    // Check that current directory exists
                    if !fs.tree.contains_key(&fs.pwd) {
                        fs.tree.insert(fs.pwd.clone(), vec![]);
                    }
                    // Make a new file in the current directory
                    let size = data.parse().unwrap();
                    let name = file.next().unwrap();
                    fs.tree.get_mut(&fs.pwd).unwrap().push(File {
                        name: name.to_string(),
                        size,
                    });
                }
            }
        }
    }
    return fs;
}

#[aoc(day7, part1)]
fn solve_part1(input: &FileSystem) -> usize {
    // Find all directories with a total size of at least 100000
    let mut big_directories = vec![];
    for (path, _) in input.tree.iter() {
        if input.recursive_size_of_folder(path) >= 100000 {
            big_directories.push(path);
        }
    }
    // Return the sum of the total size of these directories
    let mut size = 0;
    for path in big_directories {
        size += input.size_of_folder(path);
    }
    size
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
        let fs = input_generator(input);
        assert_eq!(solve_part1(&fs), 95437);
    }
}
