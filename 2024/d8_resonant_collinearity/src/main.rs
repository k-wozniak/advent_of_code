use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");

    // Load antennas from a file.
    let antennas_str = std::fs::read_to_string("./src/antenna_map.txt").unwrap();

    let antinodes = process_map(&antennas_str);

    // Print unique antinodes.
    println!("Unique antinodes: {}", antinodes.len());
}

fn process_map(antennas_str: &str) -> HashSet<(usize, usize)> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (row_idx, line) in antennas_str.lines().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push((row_idx, col_idx));
            }
        }
    }

    // Find the maximum row and column indices.
    let max_row = antennas
        .values()
        .flat_map(|v| v.iter().map(|&(r, _)| r))
        .max()
        .unwrap();

    let max_col = antennas
        .values()
        .flat_map(|v| v.iter().map(|&(_, c)| c))
        .max()
        .unwrap();

    find_antinodes(
        &antennas.values().flat_map(|v| v.iter().cloned()).collect(),
        max_row,
        max_col,
    )
}

fn find_antinodes(
    antennas: &Vec<(usize, usize)>,
    max_row: usize,
    max_col: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    for antenna in antennas {
        let idx = antennas.iter().position(|a| a == antenna).unwrap();
        let found_antinodes =
            find_antinodes_for_antenna(antenna, &antennas[idx + 1..], max_row, max_col);
        antinodes.extend(found_antinodes);
    }

    antinodes
}

fn find_antinodes_for_antenna(
    antenna: &(usize, usize),
    antennas: &[(usize, usize)],
    max_row: usize,
    max_col: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    for (row, col) in antennas {
        let x = antenna.0 as isize - *row as isize;
        let y = antenna.1 as isize - *col as isize;

        // Check if the current position is a valid antinode.
        if x >= 0 && y >= 0 {
            antinodes.insert((*row, *col));
        }

        let x = antenna.0 as isize + *row as isize;
        let y = antenna.1 as isize + *col as isize;

        if x <= max_row as isize && y <= max_col as isize {
            antinodes.insert((x as usize, y as usize));
        }
    }

    antinodes
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

        let antinodes = process_map(input);

        assert_eq!(antinodes.len(), 14);
    }
}
