use std::io;

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

    let diff = sum_distances(a, b);
    println!("{diff}");
    Ok(())
}

fn sum_distances(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut a_s = a.clone();
    let mut b_s = b.clone();

    a_s.sort();
    b_s.sort();

    a_s.iter().zip(b_s).map(|(a, b)| i32::abs(a - b)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [[i32; 6]; 2] = [[3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3]];

    #[test]
    fn test_sum_distances() {
        assert_eq!(
            sum_distances(Vec::from(TEST_DATA[0]), Vec::from(TEST_DATA[1])),
            11
        );
    }
}
