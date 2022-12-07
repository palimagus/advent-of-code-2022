use std::collections::HashMap;

type Node = (String, usize);
struct ElfDeviceFileSystem {
    memory: HashMap<String, Node>,
    cursor: String,
}

impl ElfDeviceFileSystem {
    fn new() -> Self {
        let mut memory = HashMap::new();
        let cursor = "/";
        memory.insert(cursor.to_string(), ("/".to_string(), 0 as usize));

        Self {
            memory,
            cursor: cursor.to_string(),
        }
    }
    fn print_system(&self) {
        // Order the memory by key (from shortest to longest)
        let mut memory = self.memory.clone();
        let mut keys = memory.keys().collect::<Vec<&String>>();
        keys.sort_by(|a, b| a.len().cmp(&b.len()));

        // Print the memory
        for key in keys {
            let (name, size) = memory.get(key).unwrap();
            println!("{}\t{}", key, size);
        }
    }
    fn change_directory(&mut self, directory: &str) {
        // Print the current directory
        println!("cd {}", directory);

        // Change the cursor
        if directory == "/" {
            self.cursor = "/".to_string();
        } else if directory == ".." {
            // Remove the last directory from the cursor
            let mut cursor = self.cursor.split("/").collect::<Vec<&str>>();
            cursor.pop();
            self.cursor = cursor.join("");
        } else {
            // Add the directory to the cursor
            self.cursor = format!("{}{}/", self.cursor, directory);
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> ElfDeviceFileSystem {
    let mut elf_device = ElfDeviceFileSystem::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();

        let command = tokens.next().unwrap();
        match command {
            "$" => match tokens.next().unwrap() {
                "cd" => {
                    elf_device.change_directory(tokens.next().unwrap());
                }
                _ => (),
            },
            "dir" => {
                let name = tokens.next().unwrap();
                elf_device.memory.insert(
                    format!("{}/{}/", elf_device.cursor, name),
                    (name.to_string(), 0 as usize),
                );
            }
            _ => {
                let size = command.parse::<usize>().unwrap();
                let name = tokens.next().unwrap();
                elf_device.memory.insert(
                    format!("{}{}", elf_device.cursor, name),
                    (name.to_string(), size),
                );
            }
        }
    }
    elf_device.print_system();
    elf_device
}

#[aoc(day7, part1)]
fn solve_day7_part1(input: &ElfDeviceFileSystem) -> usize {
    0
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
        assert_eq!(solve_day7_part1(&input_generator(input)), 95437);
    }
}
