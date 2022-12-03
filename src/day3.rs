use std::vec;

use aoc_runner_derive::{aoc, aoc_generator};

struct Sack {
    items: Vec<u32>
}

impl Sack {
    fn split (&self) -> (Sack, Sack) {
        let (split1, split2) = self.items.split_at(self.items.len() / 2);
        (Sack { items: split1.to_vec() }, Sack { items: split2.to_vec() })
    }
    fn find_is_in_both (&self, other: &Sack) -> u32 {
        for i in &self.items {
            if other.items.contains(i) {
                return *i;
            }
        }
        return 0;
    }
    fn merge (&self, other: &Sack) -> Sack {
        let mut ret = self.items.clone();
        ret.append(&mut other.items.clone());
        Sack { items: ret }
    }
    fn count_items_for_value (&self, value: usize) -> u32 {
        // For each item in the sack, count how many times it appears
        let mut ret = 0;
        let mut count = 0;
        for i in &self.items {
            if *i == value as u32 {
                count += 1;
            }
        }
        ret
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Sack> {
    let mut ret = vec![];
    for line in input.lines() {
        let mut sack = Sack { items: vec![] };
        line.chars().for_each(|c| {
            if c.is_lowercase() {
                sack.items.push(c as u32 - 96);
            } else if c.is_uppercase() {
                sack.items.push(c as u32 - 64 + 26);
            }
        });
        ret.push(sack);
    }
    ret
}

#[aoc(day3, part1, naive)]
fn solve_part1(input: &Vec<Sack>) -> u32 {
    input.iter().map(|sack| {
        let (sack1, sack2) = sack.split();
        sack1.find_is_in_both(&sack2)
    }).sum()
}

#[aoc(day3, part2, naive)]
fn solve_part2(input: &Vec<Sack>) -> u32 {
    // Every set of three lines in input corresponds to a single group
    let mut ret = 0;
    for i in (0..input.len()).step_by(6) {
        let big_sack_one = &input[i].merge(&input[i + 1]).merge(&input[i + 2]);
        let big_sack_two = &input[i + 3].merge(&input[i + 4]).merge(&input[i + 5]);

        let a = big_sack_one.count_items_for_value(3);
        let b = big_sack_two.count_items_for_value(3);
        println!("{} {}", a, b);
        ret = a + b;
    }
    ret
}

fn one(input: &str) -> u32 {
    input.lines()
    .map(|bag| bag.split_at(bag.len() / 2))
    .map(|(l, r)| [l, r].map(input_generator))
    .map(|[l, r]| u64::trailing_zeros(l & r))
    .sum()
}

fn two(input: &str) -> u32 {
    input.lines()
    .array_chunks::<3>() // unstable
    .map(|bags| bags.map(input_generator))
    .map(|[a, b, c]| a & b & c)
    .map(u64::trailing_zeros)
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_part1(&input_generator(input)), 157);
    }

    #[test]
    fn test_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_part2(&input_generator(input)), 70);
    }
}