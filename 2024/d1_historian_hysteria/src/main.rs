use std::collections::HashMap;

fn main() {
    // Vectors to store the numbers from both lists.
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Read input.txt line by line and separate into two vectors.
    std::fs::read_to_string("./src/input.txt")
        .unwrap()
        .split("\n")
        .for_each(|x| {
            let temp: Vec<i32> = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            left.push(temp[0]);
            right.push(temp[1]);
        });

    // Sort the vectors.
    left.sort();
    right.sort();

    // Iterate over both list and find the distance.
    let distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    // Find the similarity between the two vectors.
    let l = vec_to_frequency_map(&left);
    let r = vec_to_frequency_map(&right);

    let similarity: i32 = l
        .iter()
        .map(|(k, l_v)| {
            let r_v = r.get(k).unwrap_or(&0);
            k * l_v * r_v
        })
        .sum();

    println!("Distance: {}", distance);
    println!("Similarity: {}", similarity);
}

// Convert a vector to a frequency map
fn vec_to_frequency_map(vec: &[i32]) -> HashMap<i32, i32> {
    let mut frequency_map = HashMap::new();

    for &num in vec {
        // entry API is perfect for counting - it either gets existing entry
        // or inserts a new one with 0, then increments by 1.
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    frequency_map
}
