use std::{collections::HashMap, vec};

use aoc_runner_derive::{aoc, aoc_generator};

struct ShipCommand {
    movement: (usize, usize),
    n_crates: usize,
}

impl ShipCommand {
    fn new(movement: (usize, usize), n_crates: usize) -> Self {
        Self { movement, n_crates }
    }

    fn get_command(&self) -> (usize, usize, usize) {
        (self.movement.0, self.movement.1, self.n_crates)
    }

    fn clone_command(&self) -> Self {
        Self {
            movement: self.movement,
            n_crates: self.n_crates,
        }
    }
}
struct Ship {
    piles: HashMap<usize, Vec<char>>,
    commands: Vec<ShipCommand>,
    cursor: usize,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            piles: HashMap::new(),
            commands: vec![],
            cursor: 1,
        }
    }

    fn print_piles(&self, msg: &str) {
        println!("{}", msg);
        // Print piles in order
        // Reverse them first
        let piles = self.piles.clone();
        let mut keys = piles.keys().collect::<Vec<&usize>>();
        // Sort keys in reverse order
        keys.sort_by(|a, b| b.cmp(a));
        keys.reverse();
        for key in keys {
            let pile = piles.get(key).unwrap();
            // Print optional message
            println!("Pile {}: {:?}", key, pile);
        }
    }

    fn modelize_line(&mut self, line: &str) {
        for (i, c) in line.chars().enumerate() {
            if i == 0 {
                self.cursor = 0
            }
            if i % 4 == 1 {
                self.cursor += 1
            }
            match c {
                'A'..='Z' => {
                    // Make a crate
                    self.piles.entry(self.cursor).or_insert(vec![]).push(c);
                }
                _ => continue,
            }
        }
    }

    fn modelize_command(&mut self, line: &str) {
        // move 1 from 2 to 1
        // The first number is the number of crates to move
        // The second number is the pile to move from
        // The third number is the pile to move to
        // Parse command line to a ShipCommand

        let words: Vec<&str> = line.split_whitespace().collect();
        self.commands.push(ShipCommand::new(
            (words[3].parse().unwrap(), words[5].parse().unwrap()),
            words[1].parse().unwrap(),
        ));
    }

    fn reorder_piles(&mut self) {
        // Reverse the piles
        for (_, pile) in self.piles.iter_mut() {
            pile.reverse();
        }
    }

    fn execute_commands(&mut self) {
        for command in self.commands.iter() {
            let (from, to, n) = command.get_command();
            // Clone the pile to move from
            let mut pile = self.piles.get(&from).unwrap().clone();
            // Clone the pile to move to
            let mut pile_to = self.piles.get(&to).unwrap().clone();
            // Move the crates
            for _ in 0..n {
                if pile.len() > 0 {
                    pile_to.push(pile.pop().unwrap());
                }
            }
            // Update the piles
            self.piles.insert(from, pile);
            self.piles.insert(to, pile_to);
        }
    }

    fn execute_commands_sticky(&mut self) {
        // Execute commands but if multiple crates are moved to the same pile, they stick together and keep their order
        for command in self.commands.iter() {
            let (from, to, n) = command.get_command();
            // Clone the pile to move from
            let mut pile = self.piles.get(&from).unwrap().clone();
            // Clone the pile to move to
            let mut pile_to = self.piles.get(&to).unwrap().clone();
            // Move the crates
            if n == 1 {
                pile_to.push(pile.pop().unwrap());
            } else {
                // If multiple crates are moved, we move them in the order they are in the pile but keep the order of the pile
                // So we need to insert them in the pile in reverse order

                // Get the crates to move
                let mut crates_to_move = vec![];
                for _ in 0..n {
                    crates_to_move.push(pile.pop().unwrap());
                }
                // Insert them in the pile in reverse order
                for c in crates_to_move.iter().rev() {
                    pile_to.push(*c);
                }
            }
            // Update the piles
            self.piles.insert(from, pile);
            self.piles.insert(to, pile_to);
        }
    }

    fn get_crates_on_top(&self) -> String {
        // Order piles by key
        let mut piles = self.piles.iter().collect::<Vec<_>>();
        piles.sort_by(|a, b| a.0.cmp(b.0));
        // Get the crates on top
        let mut crates = String::new();
        for (_, pile) in piles.iter() {
            crates.push(pile.last().unwrap_or(&'.').clone());
        }
        crates
    }

    fn clone_ship(&self) -> Ship {
        let mut ret = Ship::new();
        for (i, pile) in self.piles.iter() {
            ret.piles.insert(*i, pile.clone());
        }
        for command in self.commands.iter() {
            ret.commands.push(command.clone_command());
        }
        ret
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Ship {
    let mut ship = Ship::new();
    let mut commands_mode = false;
    for l in input.lines() {
        if l == "" {
            commands_mode = true;
            continue;
        }
        if commands_mode {
            ship.modelize_command(l);
        } else {
            ship.modelize_line(l);
        }
    }
    ship
}

#[aoc(day5, part1)]
fn solve_part1(input: &Ship) -> String {
    let mut ship = input.clone_ship();
    ship.reorder_piles();
    ship.execute_commands();
    ship.get_crates_on_top()
}

#[aoc(day5, part2)]
fn solve_part2(input: &Ship) -> String {
    let mut ship = input.clone_ship();
    ship.reorder_piles();
    ship.execute_commands_sticky();
    ship.get_crates_on_top()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve_part1(&input_generator(input)), "CMZ".to_string());
    }

    #[test]
    fn test_part2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve_part2(&input_generator(input)), "MCD".to_string());
    }
}
