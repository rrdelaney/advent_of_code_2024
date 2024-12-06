use std::hash::Hash;
use std::io::Read;
use std::{collections::HashSet, io};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let (position, obstacles) = parse_grid(&buf);
    let count = num_potential_loops(position, &obstacles);
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

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn next_dir(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

fn next_position(dir: Dir, (y, x): (i32, i32)) -> (i32, i32) {
    match dir {
        Dir::Up => (y - 1, x),
        Dir::Down => (y + 1, x),
        Dir::Left => (y, x - 1),
        Dir::Right => (y, x + 1),
    }
}

fn path_to_exit(
    init_dir: Dir,
    init_position: (i32, i32),
    max_y: i32,
    max_x: i32,
    obstacles: &HashSet<(i32, i32)>,
) -> Box<Option<HashSet<(Dir, (i32, i32))>>> {
    let mut history = HashSet::new();
    let mut position = init_position;
    let mut dir = init_dir;
    loop {
        let is_loop = history.contains(&(dir, position));
        if is_loop {
            return Box::from(None);
        }

        history.insert((dir, position));

        let next_position = next_position(dir, position);
        let will_hit_obstacle = obstacles.contains(&next_position);
        let will_exit_bounds = next_position.0 < 0
            || next_position.0 > max_y
            || next_position.1 < 0
            || next_position.1 > max_x;

        if will_exit_bounds {
            break;
        } else if will_hit_obstacle {
            dir = next_dir(dir);
        } else {
            position = next_position;
        }
    }

    Box::from(Some(history))
}

fn num_potential_loops(init: (i32, i32), obstacles: &HashSet<(i32, i32)>) -> usize {
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, x) in obstacles.iter() {
        max_y = i32::max(max_y, *y);
        max_x = i32::max(max_x, *x);
    }

    let Some(path) = *path_to_exit(Dir::Up, init, max_y, max_x, obstacles) else {
        return 0;
    };

    let mut obstacles_causing_loop: HashSet<(i32, i32)> = HashSet::new();
    let mut potential_obstacles = obstacles.clone();
    for (_dir, position) in path {
        if obstacles_causing_loop.contains(&position) || obstacles.contains(&position) {
            continue;
        }

        potential_obstacles.insert(position);

        let has_loop = path_to_exit(Dir::Up, init, max_y, max_x, &potential_obstacles).is_none();
        if has_loop {
            obstacles_causing_loop.insert(position);
        }

        potential_obstacles.remove(&position);
    }

    obstacles_causing_loop.len()
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
        assert_eq!(num_potential_loops(position, &obstacles), 6);
    }
}
