use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io,
};

fn main() -> io::Result<()> {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in std::io::stdin().lines().filter_map(|line| line.ok()) {
        if line.contains(",") {
            let pages: Vec<i32> = line
                .split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect();

            updates.push(pages);
        } else if line.contains("|") {
            let pages: Vec<i32> = line
                .split("|")
                .map(|page| page.parse::<i32>().unwrap())
                .collect();

            rules.push((*pages.first().unwrap(), *pages.last().unwrap()));
        }
    }

    let sum = sum_invalid_middle_pages(&updates, &rules);
    println!("{sum}");
    Ok(())
}

fn valid_page_order(pages: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let mut page_indices: HashMap<i32, usize> = HashMap::new();
    for (i, num) in pages.iter().enumerate() {
        page_indices.insert(*num, i);
    }

    rules.iter().all(|(page, must_have)| {
        match (page_indices.get(page), page_indices.get(must_have)) {
            (Some(page_index), Some(must_have_index)) => page_index < must_have_index,
            _ => true,
        }
    })
}

fn sort_and_middle_value(pages: &Vec<i32>, rules: &Vec<(i32, i32)>) -> i32 {
    let contained_pages: HashSet<&i32> = HashSet::from_iter(pages.iter());
    let applied_rules = rules
        .iter()
        .filter(|(a, b)| contained_pages.contains(a) && contained_pages.contains(b))
        .fold(HashMap::new(), |mut acc, (left, right)| {
            acc.insert((left, right), Ordering::Less);
            acc.insert((right, left), Ordering::Greater);
            acc
        });

    let mut ordered = pages.clone();
    ordered.sort_by(|a, b| *applied_rules.get(&(a, b)).unwrap_or(&Ordering::Equal));
    *ordered.get(ordered.len().div_ceil(2) - 1).unwrap()
}

fn sum_invalid_middle_pages(updates: &Vec<Vec<i32>>, rules: &Vec<(i32, i32)>) -> i32 {
    updates
        .iter()
        .filter(|pages| !valid_page_order(pages, rules))
        .map(|pages| sort_and_middle_value(pages, rules))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RULES: [(i32, i32); 21] = [
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];

    #[test]
    fn test_sum_invalid_middle_pages() {
        let test_updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        assert_eq!(
            sum_invalid_middle_pages(&test_updates, &Vec::from(TEST_RULES)),
            123
        );
    }
}
