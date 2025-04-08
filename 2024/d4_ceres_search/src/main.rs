use ndarray::Array2;

fn main() {
    // Load the characters into an array.
    let puzzle: Vec<Vec<char>> = std::fs::read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let puzzle = Array2::from_shape_vec(
        (puzzle.len(), puzzle[0].len()),
        puzzle.into_iter().flatten().collect(),
    )
    .unwrap();

    println!("Input puzzle solution: {}", solve_puzzle(puzzle.clone()));
    println!("Input puzzle2 solution: {}", solve_puzzle2(puzzle));
}

pub fn solve_puzzle(puzzle: Array2<char>) -> usize {
    // Rows
    let h: usize = puzzle
        .rows()
        .into_iter()
        .map(|r| {
            r.windows(4)
                .into_iter()
                .filter(|w| {
                    let w: [char; 4] = w.iter().copied().collect::<Vec<_>>().try_into().unwrap();
                    is_xmas(&w)
                })
                .count()
        })
        .sum();

    // Columns
    let r: usize = puzzle
        .t()
        .rows()
        .into_iter()
        .map(|r| {
            r.windows(4)
                .into_iter()
                .filter(|w| {
                    let w: [char; 4] = w.iter().copied().collect::<Vec<_>>().try_into().unwrap();
                    is_xmas(&w)
                })
                .count()
        })
        .sum();

    // Diagonal
    let mut d = 0;
    for i in 0..puzzle.dim().0 - 3 {
        for j in 0..puzzle.dim().1 - 3 {
            // Check Diagonal left to right and right to left.
            let diagonal: [[char; 4]; 2] = [
                [
                    puzzle[(i, j)],
                    puzzle[(i + 1, j + 1)],
                    puzzle[(i + 2, j + 2)],
                    puzzle[(i + 3, j + 3)],
                ],
                [
                    puzzle[(i, j + 3)],
                    puzzle[(i + 1, j + 2)],
                    puzzle[(i + 2, j + 1)],
                    puzzle[(i + 3, j)],
                ],
            ];

            d += diagonal.iter().filter(|l| is_xmas(l)).count();
        }
    }

    h + r + d
}

// Checks if the slice contains a valid xmas sequence.
fn is_xmas(s: &[char; 4]) -> bool {
    *s == ['X', 'M', 'A', 'S'] || *s == ['S', 'A', 'M', 'X']
}

fn solve_puzzle2(mut puzzle: Array2<char>) -> usize {
    let mut total = 0;

    for _ in 0..4 {
        for i in 0..puzzle.dim().0 - 2 {
            for j in 0..puzzle.dim().1 - 2 {
                let window = puzzle.slice(ndarray::s![i..i + 3, j..j + 3]);
                if window_has_xmas(window) {
                    total += 1;
                }
            }
        }

        // Rotate by 90 degrees.
        puzzle = puzzle.reversed_axes();
        puzzle = puzzle.slice(ndarray::s![.., ..;-1]).to_owned();
    }

    total
}

// 3x3 array
fn window_has_xmas(window: ndarray::ArrayView2<char>) -> bool {
    window[(0, 0)] == 'S'
        && window[(0, 2)] == 'S'
        && window[(1, 1)] == 'A'
        && window[(2, 0)] == 'M'
        && window[(2, 2)] == 'M'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid1() {
        let input = Array2::from_shape_vec(
            (5, 6),
            vec![
                '.', '.', 'X', '.', '.', '.', //
                '.', 'S', 'A', 'M', 'X', '.', //
                '.', 'A', '.', '.', 'A', '.', //
                'X', 'M', 'A', 'S', '.', 'S', //
                '.', 'X', '.', '.', '.', '.',
            ],
        )
        .unwrap();

        // Safe reports.
        assert_eq!(solve_puzzle(input), 4);
    }

    #[test]
    fn test_is_valid() {
        let input = Array2::from_shape_vec(
            (10, 10),
            vec![
                'M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M', //
                'M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A', //
                'A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M', //
                'M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X', //
                'X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M', //
                'X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A', //
                'S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S', //
                'S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A', //
                'M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M', //
                'M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X',
            ],
        )
        .unwrap();

        // Safe reports.
        assert_eq!(solve_puzzle(input), 18);
    }

    #[test]
    fn test_is_valid_x_mas() {
        let input = Array2::from_shape_vec(
            (10, 10),
            vec![
                'M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M', //
                'M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A', //
                'A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M', //
                'M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X', //
                'X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M', //
                'X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A', //
                'S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S', //
                'S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A', //
                'M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M', //
                'M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X',
            ],
        )
        .unwrap();

        // Safe reports.
        assert_eq!(solve_puzzle2(input), 9);
    }
}
