use aoc_runner_derive::aoc;

struct ElfDevice {
    buffer: Vec<char>,
}

impl ElfDevice {
    fn new() -> Self {
        Self { buffer: vec![] }
    }

    fn find_marker(&mut self, sequence: &str, marker_length: usize) -> usize {
        // Reset buffer to empty
        self.buffer.clear();
        let mut ret = 0;

        for (i, c) in sequence.chars().enumerate() {
            // Is c in the buffer?
            if self.buffer.contains(&c) {
                // Yes, remove all entries from start of buffer to c
                self.buffer
                    .drain(..=self.buffer.iter().position(|&x| x == c).unwrap());
                // Then push c onto the end of the buffer
                self.buffer.push(c);
            } else {
                // No, then is the buffer full?
                if self.buffer.len() == marker_length - 1 {
                    // Yes, we have a packet!
                    ret = i + 1;
                    break;
                } else {
                    // No, then push c onto the end of the buffer
                    self.buffer.push(c);
                }
            }
        }
        ret
    }
}

#[aoc(day6, part1)]
fn solve_day6_part1(input: &str) -> usize {
    // The start of a packet is indicated by a sequence of four characters that are all different
    // How many characters need to be processed before the first start-of-packet marker is detected?
    // Find a sequence of four characters that are all different
    // The first character is at index 0
    let mut elf_device = ElfDevice::new();
    elf_device.find_marker(input, 4)
}

#[aoc(day6, part2)]
fn solve_day6_part2(input: &str) -> usize {
    // The start of a packet is indicated by a sequence of four characters that are all different
    // How many characters need to be processed before the first start-of-packet marker is detected?
    // Find a sequence of four characters that are all different
    // The first character is at index 0
    let mut elf_device = ElfDevice::new();
    elf_device.find_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        assert_eq!(solve_day6_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_day6_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_day6_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_day6_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(solve_day6_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_day6_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_day6_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_day6_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_day6_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
