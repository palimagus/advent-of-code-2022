use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut ret = vec![vec![]];
    for l in input.lines() {
        let s = l.trim();
        match s {
            "" => ret.push(vec![]),
            _ => ret.last_mut().unwrap().push(s.parse().unwrap()),
        }
    }
    return ret;
}

#[aoc(day1, part1)]
fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|e| e.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[Vec<u32>]) -> u32 {
    let mut elves = input.iter().map(|e| e.iter().sum()).collect::<Vec<u32>>();
    elves.sort();
    elves.reverse();
    elves[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1000
            2000
            3000
            
            4000
            
            5000
            6000
            
            7000
            8000
            9000
            
            10000";
        assert_eq!(solve_part1(&parse_input(input)), 24000);
    }

    #[test]
    fn test_part2() {
        let input = "1000
            2000
            3000
            
            4000
            
            5000
            6000
            
            7000
            8000
            9000
            
            10000";
        assert_eq!(solve_part2(&parse_input(input)), 45000);
    }
}
