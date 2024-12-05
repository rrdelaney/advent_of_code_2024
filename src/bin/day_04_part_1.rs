use std::io;

fn main() -> io::Result<()> {
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in std::io::stdin().lines() {
        let mut chars = Vec::new();
        for part in line?.chars() {
            chars.push(part);
        }

        input.push(chars);
    }

    let count = count_matches("XMAS", &input);
    println!("{count}");
    Ok(())
}

fn is_match(
    search: &str,
    input: &Vec<Vec<char>>,
    init_x: i32,
    step_x: i32,
    init_y: i32,
    step_y: i32,
) -> bool {
    search.chars().enumerate().all(|(step, char)| {
        let step_len: i32 = step.try_into().unwrap();
        let x: i32 = init_x + (step_x * step_len);
        let y: i32 = init_y + (step_y * step_len);

        Option::zip(x.try_into().ok(), y.try_into().ok())
            .and_then(|(x_size, y_size): (usize, usize)| {
                input.get(y_size).and_then(|row| row.get(x_size))
            })
            .map(|char_comp| *char_comp == char)
            .unwrap_or(false)
    })
}

fn count_matches(search: &str, input: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let num_rows: i32 = input.len().try_into().unwrap();
    let num_cols: i32 = input.first().unwrap().len().try_into().unwrap();

    for y in 0..num_rows {
        for x in 0..num_cols {
            for step_x in -1..=1 {
                for step_y in -1..=1 {
                    if is_match(
                        search,
                        input,
                        x.try_into().unwrap(),
                        step_x,
                        y.try_into().unwrap(),
                        step_y,
                    ) {
                        count += 1
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ops() {
        let test_data = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];

        assert_eq!(count_matches("XMAS", &test_data), 18);
    }
}
