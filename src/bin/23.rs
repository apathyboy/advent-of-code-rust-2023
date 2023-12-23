advent_of_code::solution!(23);

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn longest_path(maze: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut longest_path = Vec::new();
    let mut current_path = Vec::new();

    dfs1(
        maze,
        x,
        y,
        &mut visited,
        &mut current_path,
        &mut longest_path,
    );

    longest_path
}

fn dfs1(
    maze: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    visited: &mut Vec<Vec<bool>>,
    current_path: &mut Vec<(usize, usize)>,
    longest_path: &mut Vec<(usize, usize)>,
) {
    if x >= maze[0].len() || y >= maze.len() || maze[y][x] == '#' || visited[y][x] {
        //println!("Invalid ({x}, {y})");
        return;
    }

    //println!("Visiting ({x}, {y})");

    visited[y][x] = true;
    current_path.push((x, y));

    if current_path.len() > longest_path.len() {
        longest_path.clear();
        longest_path.extend(current_path.iter());
    }

    // Explore neighbors (up, down, left, right)
    let dx = [-1, 1, 0, 0];
    let dy = [0, 0, -1, 1];

    if maze[y][x] == '.' {
        for i in 0..4 {
            let new_x = x as i32 + dx[i];
            let new_y = y as i32 + dy[i];

            //println!("Checking ({new_x}, {new_y})");

            if new_x >= 0 && new_x < maze[0].len() as i32 && new_y >= 0 && new_y < maze.len() as i32
            {
                dfs1(
                    maze,
                    new_x as usize,
                    new_y as usize,
                    visited,
                    current_path,
                    longest_path,
                );
            }
        }
    } else {
        let slope_directions = vec![('<', (-1, 0)), ('^', (0, -1)), ('>', (1, 0)), ('v', (0, 1))];

        for (slope_char, (dx, dy)) in &slope_directions {
            if maze[y][x] == *slope_char {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x >= 0
                    && new_x < maze[0].len() as i32
                    && new_y >= 0
                    && new_y < maze.len() as i32
                {
                    dfs1(
                        maze,
                        new_x as usize,
                        new_y as usize,
                        visited,
                        current_path,
                        longest_path,
                    );
                }
            }
        }
    }

    visited[y][x] = false;
    current_path.pop();
}

fn longest_path2(
    maze: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut longest_path = Vec::new();
    let mut current_path = Vec::new();

    dfs2(
        maze,
        start,
        end,
        &mut visited,
        &mut current_path,
        &mut longest_path,
    );

    longest_path
}

fn dfs2(
    maze: &Vec<Vec<char>>,
    pos: (usize, usize),
    end: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    current_path: &mut Vec<(usize, usize)>,
    longest_path: &mut Vec<(usize, usize)>,
) {
    if pos.0 >= maze[0].len()
        || pos.1 >= maze.len()
        || maze[pos.1][pos.0] == '#'
        || visited[pos.1][pos.0]
    {
        //println!("Invalid ({x}, {y})");
        return;
    }

    //println!("Visiting ({x}, {y})");

    visited[pos.1][pos.0] = true;
    current_path.push((pos.0, pos.1));

    if pos.0 == end.0 && pos.1 == end.1 && current_path.len() > longest_path.len() {
        longest_path.clear();
        longest_path.extend(current_path.iter());

        //println!("New longest path: {:?}", longest_path.len() - 1);
    }

    // Explore neighbors (up, down, left, right)
    let dx = [-1, 1, 0, 0];
    let dy = [0, 0, -1, 1];

    for i in 0..4 {
        let new_x = pos.0 as i32 + dx[i];
        let new_y = pos.1 as i32 + dy[i];

        if new_x >= 0 && new_x < maze[0].len() as i32 && new_y >= 0 && new_y < maze.len() as i32 {
            dfs2(
                maze,
                (new_x as usize, new_y as usize),
                end,
                visited,
                current_path,
                longest_path,
            );
        }
    }

    visited[pos.1][pos.0] = false;
    current_path.pop();
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let longest_path = longest_path(&map, 1, 0);

    Some(longest_path.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let start = (1, 0);
    let end = (map[0].len() - 2, map.len() - 1);

    let longest_path = longest_path2(&map, start, end);

    //println!();
    //
    //for y in 0..map.len() {
    //    for x in 0..map[0].len() {
    //        if longest_path.contains(&(x, y)) {
    //            print!("O");
    //        } else {
    //            print!("{}", map[y][x]);
    //        }
    //    }
    //    println!();
    //}
    //
    //println!();
    //println!("{:?}", longest_path.len());
    //println!("{:?}", longest_path.iter().unique().count());

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
