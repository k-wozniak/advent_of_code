fn main() {
    // Read the input reports.
    let reports: Vec<Vec<i32>> = std::fs::read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let valid_report: usize = reports.iter().filter(|r| is_valid(r)).count();
    println!("Valid reports: {}", valid_report);
}

fn is_valid(report: &[i32]) -> bool {
    let r = report
        .windows(2) // Sliding window of size 2
        .map(|window| (window[0] - window[1]).abs()) // Find the difference between the two.
        .all(|diff| diff > 0 && diff <= 3); // Check if the difference is not 0 and max 2.

    let is_increasing = report.windows(2).all(|window| window[0] < window[1]);
    let is_decreasing = report.windows(2).all(|window| window[0] > window[1]);

    r && (is_increasing || is_decreasing)
}

fn is_valid_with_dampener(report: &[i32]) -> bool {
    let r = report
        .windows(2) // Sliding window of size 2
        .map(|window| (window[0] - window[1]).abs()) // Find the difference between the two.
        .all(|diff| diff > 0 && diff <= 3); // Check if the difference is not 0 and max 2.

    let is_increasing = report.windows(2).all(|window| window[0] < window[1]);
    let is_decreasing = report.windows(2).all(|window| window[0] > window[1]);

    r && (is_increasing || is_decreasing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        // Safe reports.
        assert!(is_valid(&[7, 6, 4, 2, 1]));
        assert!(is_valid(&[1, 3, 6, 7, 9]));

        // Unsafe reports.
        assert!(!is_valid(&[1, 2, 7, 8, 9]));
        assert!(!is_valid(&[9, 7, 6, 2, 1]));
        assert!(!is_valid(&[1, 3, 2, 4, 5]));
        assert!(!is_valid(&[8, 6, 4, 4, 1]));
    }

    #[test]
    fn test_is_valid_with_dampener() {
        // Safe reports.
        assert!(is_valid_with_dampener(&[7, 6, 4, 2, 1]));
        assert!(is_valid_with_dampener(&[1, 3, 6, 7, 9]));
        assert!(is_valid_with_dampener(&[1, 3, 2, 4, 5]));
        assert!(is_valid_with_dampener(&[8, 6, 4, 4, 1]));

        // Unsafe reports.
        assert!(!is_valid_with_dampener(&[1, 2, 7, 8, 9]));
        assert!(!is_valid_with_dampener(&[9, 7, 6, 2, 1]));
    }
}
