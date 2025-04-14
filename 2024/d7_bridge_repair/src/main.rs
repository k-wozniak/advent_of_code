use rayon::prelude::*;

fn main() {
    // Load equations.
    let equations: Vec<(i64, Vec<i64>)> = std::fs::read_to_string("./src/calibration_eq.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let target = parts.next().unwrap().trim().parse::<i64>().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            (target, numbers)
        })
        .collect();

    let solvable_sum: i64 = equations
        .iter()
        .filter(|eq| solve(eq.0, 0, &eq.1))
        .map(|eq| eq.0)
        .sum();

    println!("Sum of solvable equation is: {}", solvable_sum);

    let solvable_sum2: i64 = equations
        .par_iter()
        .filter(|eq| solve2(eq.0, 0, &eq.1))
        .map(|eq| eq.0)
        .sum();

    println!("Sum of solvable2 equation is: {}", solvable_sum2);
}

fn solve(target: i64, current: i64, numbers: &[i64]) -> bool {
    match (
        target == current && numbers.is_empty(),
        numbers.split_first(),
    ) {
        (true, _) => true,
        (_, Some((&n, rest))) => {
            solve(target, current * n, rest) || solve(target, current + n, rest)
        }
        _ => false,
    }
}

fn solve2(target: i64, current: i64, numbers: &[i64]) -> bool {
    match numbers.split_first() {
        None => target == current,
        Some((&first, rest)) => {
            if solve2(target, current + first, rest) {
                return true;
            }

            if solve2(target, current * first, rest) {
                return true;
            }

            // Concatenation
            if current == 0 && solve2(target, first, rest) {
                return true;
            } else {
                let n_digits = (first.abs() as f64).log10().floor() as u32 + 1;
                let concat_result = current * 10i64.pow(n_digits) + first;

                if solve2(target, concat_result, rest) {
                    return true;
                }
            }

            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let equations: Vec<(i64, Vec<i64>)> = vec![
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

        let solvable_sum: i64 = equations
            .iter()
            .filter(|eq| solve(eq.0, 0, &eq.1))
            .map(|eq| eq.0)
            .sum();

        assert_eq!(3749, solvable_sum);
    }

    #[test]
    fn test_part2() {
        let equations: Vec<(i64, Vec<i64>)> = vec![
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (192, vec![17, 8, 14]),
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (161011, vec![16, 10, 13]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];

        let solvable_sum: i64 = equations
            .iter()
            .filter(|eq| solve2(eq.0, 0, &eq.1))
            .map(|eq| eq.0)
            .sum();

        assert_eq!(11387, solvable_sum);
    }
}
