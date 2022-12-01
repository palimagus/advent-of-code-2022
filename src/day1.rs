use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    // For each line, parse the line into a number
    // If line is "" (empty), return 0
    input
        .lines()
        .map(|line| line.parse().unwrap_or(0))
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[i32]) -> i32 {
    let mut elves = Vec::new();
    let mut calories = 0;
    // For each line, we add the value to the total
    // If the value is 0 we push the total to the elves
    // and reset the total
    for value in input {
        if *value == 0 {
            elves.push(calories);
            calories = 0;
        } else {
            calories += value;
        }
    }
    elves.push(calories);
    *elves.iter().max().unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[i32]) -> i32 {
    let mut elves = Vec::new();
    let mut calories = 0;
    // For each line, we add the value to the total
    // If the value is 0 we push the total to the elves
    // and reset the total
    for value in input {
        if *value == 0 {
            elves.push(calories);
            calories = 0;
        } else {
            calories += value;
        }
    }
    // We sort the elves by calories
    elves.sort();
    // Return the sum of the 3 most caloric elves
    elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "1000\n2000\n3000\n\n1000\n2000\n3000\n\n4000\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(solve_part1(&parse_input(input)), 24000);
    }

    #[test]
    fn test_part2() {
        let input =
            "1000\n2000\n3000\n\n1000\n2000\n3000\n\n4000\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(solve_part2(&parse_input(input)), 45000);
    }
}
