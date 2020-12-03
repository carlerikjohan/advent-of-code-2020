mod part_1 {
  pub struct Password {
    required_letter: String,
    required_min: u32,
    required_max: u32,
    password_input: String
  }

  pub fn parse_requirement(input: &str) -> Password {
    let split = input.split(": ").collect::<Vec<&str>>();
    let requirements = split[0].split(" ").collect::<Vec<&str>>();
    let quantities = requirements[0].split("-").collect::<Vec<&str>>();

    let min = quantities[0].parse::<u32>().unwrap();
    let max = quantities[1].parse::<u32>().unwrap();
    let letter = requirements[1];
    let password = split[1];

    Password {
      password_input: password.to_string(),
      required_letter: letter.to_string(),
      required_min: min,
      required_max: max
    }
  }

  pub fn password_requirements_fulfilled(password: &Password) -> bool {
    let mut occurrence = 0;
    for c in password.password_input.chars() {
      if c.to_string() == password.required_letter {
        occurrence = occurrence + 1
      }
    }

    password.required_min <= occurrence && occurrence <= password.required_max
  }
}

mod part_2 {
  pub struct Password {
    required_letter: String,
    first_position: u32,
    last_position: u32,
    password_input: String
  }

  pub fn parse_requirement(input: &str) -> Password {
    let split = input.split(": ").collect::<Vec<&str>>();
    let requirements = split[0].split(" ").collect::<Vec<&str>>();
    let quantities = requirements[0].split("-").collect::<Vec<&str>>();

    let min = quantities[0].parse::<u32>().unwrap();
    let max = quantities[1].parse::<u32>().unwrap();
    let letter = requirements[1];
    let password = split[1];

    Password {
      password_input: password.to_string(),
      required_letter: letter.to_string(),
      first_position: min,
      last_position: max
    }
  }

  pub fn password_requirements_fulfilled(password: &Password) -> bool {
    let mut found_occurrence = 0;
    let mut index = 1;
    for c in password.password_input.chars() {
      if index == password.first_position || index == password.last_position {
        if c.to_string() == password.required_letter {
          found_occurrence = found_occurrence + 1;
        }
      }

      index = index + 1;
    }

    found_occurrence == 1
  }
}

/* Part 1 */
#[aoc_generator(day2, part1)]
fn parse_input_day_2_part_1(input: &str) -> Vec<part_1::Password> {
  input
    .lines()
    .map(|l| part_1::parse_requirement(l))
    .collect()
}

#[aoc(day2, part1)]
fn day_2_part_1(passwords: &Vec<part_1::Password>) -> usize {
  let valid_results: Vec<bool> = passwords
    .iter()
    .map(|p| part_1::password_requirements_fulfilled(p))
    .filter(|r| *r)
    .collect();

  valid_results.len()
}

/* Part 2 */
#[aoc_generator(day2, part2)]
fn parse_input_day_2_part_2(input: &str) -> Vec<part_2::Password> {
  input
    .lines()
    .map(|l| part_2::parse_requirement(l))
    .collect()
}

#[aoc(day2, part2)]
fn day_2_part_2(passwords: &Vec<part_2::Password>) -> usize {
  let valid_results: Vec<bool> = passwords
    .iter()
    .map(|p| part_2::password_requirements_fulfilled(p))
    .filter(|r| *r)
    .collect();

  valid_results.len()
}
