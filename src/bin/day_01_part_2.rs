use core::slice::Iter;
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

    let score = similarity_score(a.iter(), b.iter());
    println!("{score}");
    Ok(())
}

fn similarity_score(a: Iter<i32>, b: Iter<i32>) -> i32 {
    let b_freq = b.fold(HashMap::new(), |mut acc, val| {
        *acc.entry(val).or_insert(0) += 1;
        acc
    });

    a.map(|val| val * b_freq.get(val).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [[i32; 6]; 2] = [[3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3]];

    #[test]
    fn test_similarity_score() {
        assert_eq!(
            similarity_score(TEST_DATA[0].iter(), TEST_DATA[1].iter()),
            31
        );
    }
}
