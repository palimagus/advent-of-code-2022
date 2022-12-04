use aoc_runner_derive::{aoc};

fn unique_items(s: &str) -> u64 {
    s.bytes()
    .map(|b| match b {
        b'a'..=b'z' => 1 + b - b'a',
        b'A'..=b'Z' => 27 + b - b'A',
        _ => unreachable!("Unexpected byte: {}", b),
    })
    .fold(0, |acc, b| acc | (1u64 << b))
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> u32 {
    input.lines()
    .map(|bag| bag.split_at(bag.len() / 2))
    .map(|(l, r)| [l, r].map(unique_items))
    .map(|[l, r]| u64::trailing_zeros(l & r))
    .sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> u32 {
    input.lines().array_chunks::<3>()
    .map(|bags| bags.map(unique_items))
    .map(|[a, b, c]| a & b & c)
    .map(u64::trailing_zeros)
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_part1(input), 157);
    }

    #[test]
    fn test_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_part2(input), 70);
    }
}