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
    let re = Regex::new(r"(mul|do|don't)\((\d*)(,?)(\d*)\)").unwrap();

    let (_, sum) = re
        .captures_iter(input)
        .fold((true, 0), |(enabled, sum), matches| {
            let (_, op) = matches.extract();

            match op {
                ["do", "", "", ""] => (true, sum),
                ["don't", "", "", ""] => (false, sum),
                ["mul", a_str, ",", b_str] => {
                    let a = a_str.parse::<i32>().unwrap();
                    let b = b_str.parse::<i32>().unwrap();
                    if enabled {
                        (enabled, sum + (a * b))
                    } else {
                        (enabled, sum)
                    }
                }
                _ => (enabled, sum),
            }
        });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_ops() {
        assert_eq!(parse_ops(TEST_DATA), 48);
    }
}
