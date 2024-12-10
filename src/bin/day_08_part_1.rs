use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let (nodes, bounds) = parse_map(&buf);
    let count = count_antinodes(&nodes, bounds).len();
    println!("{count}");

    Ok(())
}

fn parse_map(map: &str) -> (Box<Vec<(char, i32, i32)>>, (i32, i32)) {
    let mut nodes: Vec<(char, i32, i32)> = Vec::new();
    let mut max_y = 0;
    let mut max_x = 0;

    for (y, line) in map.lines().enumerate() {
        max_y = max_y.max(y);
        for (x, char) in line.chars().enumerate() {
            if y == 0 {
                max_x = max_x.max(x);
            }

            if char == '.' {
                continue;
            }

            nodes.push((char, y.try_into().unwrap(), x.try_into().unwrap()));
        }
    }

    (
        Box::from(nodes),
        (max_y.try_into().unwrap(), max_x.try_into().unwrap()),
    )
}

fn count_antinodes(
    nodes: &Vec<(char, i32, i32)>,
    (max_y, max_x): (i32, i32),
) -> Box<HashSet<(i32, i32)>> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (char, y, x) in nodes {
        for (anti_char, anti_y, anti_x) in nodes.iter() {
            if char != anti_char {
                continue;
            } else if x == anti_x && y == anti_y {
                continue;
            }

            let antinode_y = y + (y - anti_y);
            let antinode_x = x + (x - anti_x);
            if 0 <= antinode_y && antinode_y <= max_y && 0 <= antinode_x && antinode_x <= max_x {
                antinodes.insert((antinode_y, antinode_x));
            }
        }
    }

    Box::from(antinodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_invalid_middle_pages() {
        let test_data = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let (nodes, bounds) = parse_map(test_data);
        let antinodes = count_antinodes(&nodes, bounds);
        assert_eq!(antinodes.len(), 14);
    }
}
