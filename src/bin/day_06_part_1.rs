use std::io::Read;
use std::{collections::HashSet, io};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let (position, obstacles) = parse_grid(&buf);
    let count = moves_until_exit(position, &obstacles);
    println!("{count}");

    Ok(())
}

fn parse_grid(grid: &str) -> ((i32, i32), Box<HashSet<(i32, i32)>>) {
    let mut obstacles = HashSet::new();
    let mut init = (0, 0);

    for (y, row) in grid.lines().enumerate() {
        for (x, char) in row.chars().enumerate() {
            if char == '#' {
                obstacles.insert((y.try_into().unwrap(), x.try_into().unwrap()));
            } else if char == '^' {
                init = (y.try_into().unwrap(), x.try_into().unwrap());
            }
        }
    }

    (init, Box::from(obstacles))
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn moves_until_exit(init: (i32, i32), obstacles: &HashSet<(i32, i32)>) -> i32 {
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, x) in obstacles.iter() {
        max_y = i32::max(max_y, *y);
        max_x = i32::max(max_x, *x);
    }

    let mut position = init;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut dir = Dir::Up;
    loop {
        visited.insert(position);

        let next_position = match (&dir, position) {
            (Dir::Up, (y, x)) => (y - 1, x),
            (Dir::Down, (y, x)) => (y + 1, x),
            (Dir::Left, (y, x)) => (y, x - 1),
            (Dir::Right, (y, x)) => (y, x + 1),
        };

        let will_hit_obstacle = obstacles.contains(&next_position);
        let will_exit_bounds = next_position.0 < 0
            || next_position.0 > max_y
            || next_position.1 < 0
            || next_position.1 > max_x;

        if will_exit_bounds {
            break;
        } else if will_hit_obstacle {
            dir = match &dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
        } else {
            position = next_position;
        }
    }

    visited.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_invalid_middle_pages() {
        let test_grid = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let (position, obstacles) = parse_grid(&test_grid);
        assert_eq!(moves_until_exit(position, &obstacles), 41);
    }
}
