use std::{collections::HashMap, io};

fn main() -> io::Result<()> {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in std::io::stdin().lines() {
        for (i, part) in line?.split_whitespace().enumerate() {
            let value = part.parse::<i32>().unwrap();
            match i {
                0 => a.push(value),
                1 => b.push(value),
                _ => (),
            }
        }
    }

    let score = similarity_score(a, b);
    println!("{score}");
    Ok(())
}

fn similarity_score(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut b_freq: HashMap<i32, i32> = HashMap::new();
    for val in b.iter() {
        if let Some(count) = b_freq.get(val) {
            b_freq.insert(*val, count + 1);
        } else {
            b_freq.insert(*val, 1);
        }
    }

    a.iter()
        .map(|val| val * b_freq.get(val).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [[i32; 6]; 2] = [[3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3]];

    #[test]
    fn test_similarity_score() {
        assert_eq!(
            similarity_score(Vec::from(TEST_DATA[0]), Vec::from(TEST_DATA[1])),
            31
        );
    }
}
