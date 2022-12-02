use aoc_runner_derive::{aoc, aoc_generator};

enum Choices {
    Rock     = 1,
    Paper    = 2,
    Scissors = 3,
    X        = 4,
    Y        = 5,
    Z        = 6,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut ret = Vec::new();
    for l in input.lines() {
        let s = l.trim().split(" ");
        let mut round = Vec::<u32>::new();
        for c in s {
            match c {
                "A" => round.push(Choices::Rock as u32),
                "B" => round.push(Choices::Paper as u32),
                "C" => round.push(Choices::Scissors as u32),
                "X" => round.push(Choices::X as u32),
                "Y" => round.push(Choices::Y as u32),
                "Z" => round.push(Choices::Z as u32),
                _ => panic!("Invalid input"),
            }
        }
        ret.push(round);
    }
    return ret;
}

enum WinState {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn shifumi(a: u32, b: u32) -> u32 {
    match (a, b) {
        (1, 4) => WinState::Draw as u32 + 1,
        (1, 5) => WinState::Win  as u32 + 2,
        (1, 6) => WinState::Lose as u32 + 3,
        (2, 4) => WinState::Lose as u32 + 1,
        (2, 5) => WinState::Draw as u32 + 2,
        (2, 6) => WinState::Win  as u32 + 3,
        (3, 4) => WinState::Win  as u32 + 1,
        (3, 5) => WinState::Lose as u32 + 2,
        (3, 6) => WinState::Draw as u32 + 3,
        _ => panic!("Invalid input"),
    }
}

fn shifumi_2(a: u32, b: u32) -> u32 {
    match (a, b) {
        (1, 4) => WinState::Lose as u32 + Choices::Scissors as u32,
        (1, 5) => WinState::Draw as u32 + Choices::Rock     as u32,
        (1, 6) => WinState::Win  as u32 + Choices::Paper    as u32,
        (2, 4) => WinState::Lose as u32 + Choices::Rock     as u32,
        (2, 5) => WinState::Draw as u32 + Choices::Paper    as u32,
        (2, 6) => WinState::Win  as u32 + Choices::Scissors as u32,
        (3, 4) => WinState::Lose as u32 + Choices::Paper    as u32,
        (3, 5) => WinState::Draw as u32 + Choices::Scissors as u32,
        (3, 6) => WinState::Win  as u32 + Choices::Rock     as u32,
        _ => panic!("Invalid input"),
    }
}

#[aoc(day2, part1)]
fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|e| shifumi(e[0], e[1])).sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|e| shifumi_2(e[0], e[1])).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "A Y
        B X
        C Z";
        assert_eq!(solve_part1(&parse_input(input)), 15);
    }

    #[test]
    fn test_part2() {
        let input = "A Y
        B X
        C Z";
        assert_eq!(solve_part2(&parse_input(input)), 12);
    }
}
