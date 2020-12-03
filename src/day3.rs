type TileMap = Vec<Vec<char>>;

enum TileType {
  Tree,
  Empty
}

enum TileTypeResult {
  Ok(TileType),
  OutOfBounds
}

fn get_chars_in_string(line: &str) -> Vec<char> {
  line
    .chars()
    .collect()
}

#[aoc_generator(day3)]
fn parse_input_day_3(input: &str) -> TileMap {
  input
    .lines()
    .map(get_chars_in_string)
    .collect()
}

fn get_tile_type(map: &TileMap, x: usize, y: usize) -> TileTypeResult {
  if y + 1 > map.len() {
    return TileTypeResult::OutOfBounds;
  }

  let row = &map[y];

  let squished_column = x % row.len();

  let tile_type: TileType = match row[squished_column] {
    '#' => TileType::Tree,
    _ => TileType::Empty
  };

  TileTypeResult::Ok(tile_type)
}

fn get_trees_in_path(map: &TileMap, step_x: usize, step_y: usize) -> u32 {
  let mut current_x = step_x;
  let mut current_y = step_y;
  let mut trees_found = 0;

  loop {
    match get_tile_type(map, current_x, current_y) {
      TileTypeResult::Ok(TileType::Tree) => trees_found = trees_found + 1,
      TileTypeResult::Ok(_) => (), // Since we only want trees we do nothing

      TileTypeResult::OutOfBounds => break
    }

    current_x = current_x + step_x;
    current_y = current_y + step_y;
  }

  trees_found
}

#[aoc(day3, part1)]
fn day_3_part_1(map: &TileMap) -> u32 {
  get_trees_in_path(map, 3, 1)
}

#[aoc(day3, part2)]
fn day3(map: &TileMap) -> u32 {
  let directions = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
  ];

  let trees: Vec<u32> = directions
    .iter()
    .map(|settings| get_trees_in_path(map, settings.0, settings.1))
    .collect();

  let mut result = 1;
  for x in trees.iter() {
    result = result * x;
  };

  result
}
