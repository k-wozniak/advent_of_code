use std::collections::HashMap;

fn main() {
    let pages_to_produce: Vec<Vec<u32>> = std::fs::read_to_string("./src/pages_to_produce.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_terminator(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // The order is reversed from the original, where the 'key' is the current value
    // and the 'value' indicates keys which can not be a after this value.
    let page_ordering_rules: HashMap<u32, Vec<u32>> =
        std::fs::read_to_string("./src/page_ordering_rules.txt")
            .unwrap()
            .lines()
            .map(|line| {
                let mut parts = line.split('|');
                let first = parts.next().unwrap().parse::<u32>().unwrap();
                let second = parts.next().unwrap().parse::<u32>().unwrap();
                (first, second)
            })
            .fold(HashMap::new(), |mut acc, (first, second)| {
                acc.entry(second).or_default().push(first);
                acc
            });

    println!(
        "Middle value sum of valid pages: {}",
        filter_pages(&pages_to_produce, &page_ordering_rules)
    );
}

fn filter_pages(pages_to_produce: &[Vec<u32>], ordering_rules: &HashMap<u32, Vec<u32>>) -> u32 {
    pages_to_produce
        .iter()
        .filter(|page| validate(page, ordering_rules))
        .map(|page| page[page.len() / 2])
        .sum()
}

fn validate(page: &[u32], ordering_rules: &HashMap<u32, Vec<u32>>) -> bool {
    for i in 0..page.len() {
        let empty_vec = vec![];
        let v = ordering_rules.get(&page[i]).unwrap_or(&empty_vec);

        if page[i..].iter().any(|&value| v.contains(&value)) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid1() {
        let pages_to_produce: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        let page_ordering_rules: HashMap<u32, Vec<u32>> = vec![
            (13, vec![53, 75, 47, 29, 61, 97]),
            (29, vec![47, 61, 53, 97, 75]),
            (53, vec![97, 61, 75, 47]),
            (61, vec![75, 47, 97]),
            (75, vec![97]),
        ]
        .into_iter()
        .collect();

        assert_eq!(143, filter_pages(&pages_to_produce, &page_ordering_rules));
    }
}
