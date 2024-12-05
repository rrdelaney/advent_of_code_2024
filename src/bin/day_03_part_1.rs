use regex::Regex;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let result = parse_ops(&buf);
    println!("{result}");

    Ok(())
}

fn parse_ops(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|op| {
            let (_, [a_str, b_str]) = op.extract();
            let a = a_str.parse::<i32>().unwrap();
            let b = b_str.parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_parse_ops() {
        assert_eq!(parse_ops(TEST_DATA), 161);
    }
}
