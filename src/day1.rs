#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<u32> {
  input
    .lines()
    .map(|l| l.parse::<u32>().unwrap())
    .collect()
}

#[aoc(day1, part1)]
fn day_1_part_1(numbers: &Vec<u32>) -> u32 {
  for i in numbers {
    for j in numbers {
      if i + j == 2020 {
        return i * j
      }
    }
  };
  return 0;
}

#[aoc(day1, part2)]
fn day_1_part_2(numbers: &Vec<u32>) -> u32 {
  for i in numbers {
    for j in numbers {
      for k in numbers {
        if i + j + k == 2020 {
          return i * j * k
        }
      }
    }
  };
  return 0;
}
