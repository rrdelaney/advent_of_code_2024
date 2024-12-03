use std::io;

fn main() -> io::Result<()> {
    let reports = std::io::stdin().lines().filter_map(|line| {
        let report = line.ok()?;
        let items = report.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        Some(Vec::from_iter(items))
    });

    let count = reports.filter(is_report_safe).count();
    println!("{count}");
    Ok(())
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let comp: fn(i32, i32) -> bool = if report[0] < report[1] {
        |a, b| a < b
    } else {
        |a, b| a > b
    };

    let mut prev = report[0];
    for item in report.iter().skip(1) {
        if !comp(prev, *item) {
            return false;
        }

        let diff = item.abs_diff(prev);
        if diff < 1 || diff > 3 {
            return false;
        }

        prev = *item;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [[i32; 5]; 6] = [
        [7, 6, 4, 2, 1],
        [1, 2, 7, 8, 9],
        [9, 7, 6, 2, 1],
        [1, 3, 2, 4, 5],
        [8, 6, 4, 4, 1],
        [1, 3, 6, 7, 9],
    ];

    #[test]
    fn test_is_safe_report() {
        assert_eq!(
            TEST_DATA
                .iter()
                .map(Vec::from)
                .filter(is_report_safe)
                .count(),
            2
        );
    }
}
