advent_of_code::solution!(13);

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Self {
        let width = map[0].len();
        let height = map.len();

        Self { map, width, height }
    }
}

fn parse_map(input: &str) -> Map {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    Map::new(map)
}

fn parse(input: &str) -> Vec<Map> {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(parse_map)
        .collect()
}

fn compare_columns(map: &Map, col1: usize, col2: usize) -> bool {
    map.map
        .iter()
        .map(|row| (row[col1], row[col2]))
        .all(|(a, b)| a == b)
}

fn count_reflected_columns(map: &Map) -> usize {
    for cols in 0..map.width - 1 {
        if compare_columns(map, cols, cols + 1) {
            let mut is_reflected = true;

            for check_col in 0..=cols {
                if cols + check_col + 1 > map.width - 1 {
                    break;
                }
                if !compare_columns(map, cols - check_col, cols + check_col + 1) {
                    is_reflected = false;
                    break;
                }
            }
            if is_reflected {
                return cols + 1;
            }
        }
    }

    0
}

fn compare_rows(map: &Map, row1: usize, row2: usize) -> bool {
    map.map[row1]
        .iter()
        .zip(map.map[row2].iter())
        .all(|(a, b)| a == b)
}

fn count_reflected_rows(map: &Map) -> usize {
    for rows in 0..map.height - 1 {
        if compare_rows(map, rows, rows + 1) {
            let mut is_reflected = true;

            for check_row in 0..=rows {
                if rows + check_row + 1 > map.height - 1 {
                    break;
                }

                if !compare_rows(map, rows - check_row, rows + check_row + 1) {
                    is_reflected = false;
                    break;
                }
            }
            if is_reflected {
                return rows + 1;
            }
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);

    let mirror_summary = map
        .iter()
        .map(|map| (count_reflected_rows(map) * 100) + count_reflected_columns(map))
        .sum::<usize>();

    Some(mirror_summary)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_reflected_columns() {
        let map = Map::new(vec![
            vec!['#', '.', '#', '#', '.', '.', '#', '#', '.'],
            vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '.'],
            vec!['#', '.', '#', '.', '#', '#', '.', '#', '.'],
        ]);

        assert_eq!(count_reflected_rows(&map), 0);

        assert_eq!(count_reflected_columns(&map), 5);
    }

    #[test]
    fn test_count_reflected_rows() {
        let map = Map::new(vec![
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
            vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
        ]);

        assert_eq!(count_reflected_rows(&map), 4);

        assert_eq!(count_reflected_columns(&map), 0);
    }

    #[test]
    fn test_count_reflections() {
        let map = Map::new(vec![
            vec![
                '#', '.', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '.', '#', '#',
            ],
            vec![
                '.', '#', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#', '#', '#', '#',
            ],
            vec![
                '.', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '.', '#', '#',
            ],
            vec![
                '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '#',
            ],
            vec![
                '.', '.', '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '#', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '.',
            ],
            vec![
                '#', '.', '#', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '.', '.',
            ],
            vec![
                '.', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '.', '.',
            ],
            vec![
                '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#',
            ],
        ]);

        assert_eq!(count_reflected_rows(&map), 12);

        assert_eq!(count_reflected_columns(&map), 0);
    }

    #[test]
    fn test_count_reflected_columns2() {
        let map = Map::new(vec![
            vec![
                '#', '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '#', '#', '.', '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#',
            ],
            vec![
                '.', '.', '#', '.', '.', '.', '#', '#', '.', '#', '.', '.', '#', '.', '#',
            ],
            vec![
                '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '#', '#', '.', '.', '#', '.', '.', '#', '#', '#', '.', '.', '#', '#', '#',
            ],
            vec![
                '#', '#', '.', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#',
            ],
            vec![
                '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '#', '#', '.', '.', '#', '#', '#', '#', '.', '.', '#', '#', '#',
            ],
            vec![
                '#', '.', '#', '.', '.', '.', '#', '#', '.', '#', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '.', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#', '.', '.',
            ],
        ]);

        assert_eq!(count_reflected_columns(&map), 11);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
