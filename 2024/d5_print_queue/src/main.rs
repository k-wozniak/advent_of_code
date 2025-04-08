use std::collections::HashMap;

fn main() {
    let pages_to_produce: Vec<Vec<u32>> = std::fs::read_to_string("./src/pages_to_produce")
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
}
