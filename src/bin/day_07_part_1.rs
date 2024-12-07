use std::io;

fn main() -> io::Result<()> {
    let mut eqs: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in std::io::stdin().lines().filter_map(|line| line.ok()) {
        let Some((total, nums)) = line.split_once(": ") else {
            continue;
        };

        eqs.push((
            total.parse::<u64>().unwrap(),
            nums.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect(),
        ));
    }

    let total: u64 = eqs
        .iter()
        .filter(|(total, nums)| is_possible_total(*total, nums, 0, 0))
        .map(|(total, _)| total)
        .sum();

    println!("{total}");
    Ok(())
}

fn is_possible_total(target: u64, nums: &Vec<u64>, start: usize, running_total: u64) -> bool {
    if start == nums.len() {
        return running_total == target;
    } else if running_total > target {
        return false;
    }

    let next = nums[start];
    return is_possible_total(target, nums, start + 1, running_total + next)
        || is_possible_total(target, nums, start + 1, running_total * next);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_invalid_middle_pages() {
        let test_data = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];

        let total: u64 = test_data
            .iter()
            .filter(|(total, nums)| is_possible_total(*total, nums, 0, 0))
            .map(|(total, _)| total)
            .sum();

        assert_eq!(total, 3749);
    }
}
