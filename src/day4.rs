use aoc_runner_derive::{aoc, aoc_generator};

struct Assignment {
    first_team: [u32; 2],
    second_team: [u32; 2],
}

impl Assignment {
    fn new(first_team: [u32; 2], second_team: [u32; 2]) -> Self {
        Self {
            first_team,
            second_team,
        }
    }
    
    fn create_ranges(&self) -> [Vec<u32>; 2]{
        let mut first_team_range = Vec::new();
        let mut second_team_range = Vec::new();
        
        for i in self.first_team[0]..=self.first_team[1] {
            first_team_range.push(i);
        }
        
        for i in self.second_team[0]..=self.second_team[1] {
            second_team_range.push(i);
        }
        
        [first_team_range, second_team_range]
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Assignment> {
    // One line example: 2-4,6-8
    // Parse each line into an assignment
    // Return a vector of assignments
    let mut assignments = Vec::new();
    for line in input.lines() {
        let data = line.split(',');
        let mut first_team = [0; 2];
        let mut second_team = [0; 2];
        for (i, d) in data.enumerate() {
            let mut team = [0; 2];
            for (j, n) in d.split('-').enumerate() {
                team[j] = n.parse().unwrap();
            }
            if i == 0 {
                first_team = team;
            } else {
                second_team = team;
            }
        }
        assignments.push(Assignment::new(first_team, second_team));
    }
    assignments
}

#[aoc(day4, part1)]
fn solve_part1(input: &Vec<Assignment>) -> u32 {
    // In how many assignment pairs does one range fully contain the other?
    let mut count = 0;
    for assignment in input {
        if assignment.first_team[0] >= assignment.second_team[0] && assignment.first_team[1] <= assignment.second_team[1] {
            count += 1;
        } else if assignment.second_team[0] >= assignment.first_team[0] && assignment.second_team[1] <= assignment.first_team[1] {
            count += 1;
        }
    }
    count
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Assignment]) -> u32 {
    // In how many assignment pairs do the ranges overlap?
    let mut count = 0;
    for assignment in input {
        let ranges = assignment.create_ranges();
        let mut overlap = false;
        for i in ranges[0].iter() {
            if ranges[1].contains(i) {
                overlap = true;
                break;
            }
        }
        if overlap {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(solve_part1(&input_generator(input)), 2);
    }

    #[test]
    fn test_part2() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(solve_part2(&input_generator(input)), 4);
    }
}