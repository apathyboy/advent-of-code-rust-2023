advent_of_code::solution!(23);

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn parse_map_no_slopes(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '<' | '>' | 'v' | '^' => '.',
                    _ => c,
                })
                .collect()
        })
        .collect()
}

fn longest_path(
    maze: &Vec<Vec<char>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut longest_path = Vec::new();
    let mut current_path = Vec::new();
    let mut longest_path_reached_counter = 0;

    dfs(
        maze,
        start,
        goal,
        &mut visited,
        &mut current_path,
        &mut longest_path,
        &mut longest_path_reached_counter,
    );

    longest_path
}

fn dfs(
    maze: &Vec<Vec<char>>,
    pos: (usize, usize),
    goal: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    current_path: &mut Vec<(usize, usize)>,
    longest_path: &mut Vec<(usize, usize)>,
    longest_path_reached_counter: &mut u32,
) {
    if pos.0 >= maze[0].len()
        || pos.1 >= maze.len()
        || maze[pos.1][pos.0] == '#'
        || visited[pos.1][pos.0]
        || *longest_path_reached_counter > 9
    {
        return;
    }

    visited[pos.1][pos.0] = true;
    current_path.push((pos.0, pos.1));

    if pos == goal && current_path.len() > longest_path.len() {
        longest_path.clear();
        longest_path.extend(current_path.iter());

        *longest_path_reached_counter += 1;
    }

    let check_directions = match maze[pos.1][pos.0] {
        '.' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        '<' => vec![(-1, 0)],
        '^' => vec![(0, -1)],
        '>' => vec![(1, 0)],
        'v' => vec![(0, 1)],
        _ => unreachable!(),
    };

    for (dx, dy) in &check_directions {
        let new_x = pos.0 as i32 + dx;
        let new_y = pos.1 as i32 + dy;

        if new_x >= 0 && new_x < maze[0].len() as i32 && new_y >= 0 && new_y < maze.len() as i32 {
            dfs(
                maze,
                (new_x as usize, new_y as usize),
                goal,
                visited,
                current_path,
                longest_path,
                longest_path_reached_counter,
            );
        }
    }

    visited[pos.1][pos.0] = false;
    current_path.pop();
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let start = (1, 0);
    let end = (map[0].len() - 2, map.len() - 1);

    let longest_path = longest_path(&map, start, end);

    // steps taken is 1 less than the number of tiles visited
    Some(longest_path.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map_no_slopes(input);

    let start = (1, 0);
    let end = (map[0].len() - 2, map.len() - 1);

    let longest_path = longest_path(&map, start, end);

    // steps taken is 1 less than the number of tiles visited
    Some(longest_path.len() as u32 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(154));
    }
}
