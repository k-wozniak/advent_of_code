use std::collections::HashSet;

fn main() {
    let mut map: Vec<Vec<char>> = std::fs::read_to_string("./src/map.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_position = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &c)| {
                if c == '^' {
                    Some(Position {
                        x: x as i32,
                        y: y as i32,
                        direction: Direction::Up,
                    })
                } else {
                    None
                }
            })
        })
        .expect("No starting position found");

    println!(
        "Number of possible loops: {}",
        how_many_loops(map.clone(), start_position)
    );

    println!(
        "The number of steps: {}",
        solve(&mut map, start_position).unwrap()
    );
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    fn new(x: i32, y: i32, direction: Direction) -> Position {
        Self { x, y, direction }
    }

    fn next_lookup(&self) -> Position {
        match self.direction {
            Direction::Up => Position::new(self.x, self.y - 1, self.direction),
            Direction::Down => Position::new(self.x, self.y + 1, self.direction),
            Direction::Left => Position::new(self.x - 1, self.y, self.direction),
            Direction::Right => Position::new(self.x + 1, self.y, self.direction),
        }
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn how_many_loops(mut map: Vec<Vec<char>>, pos: Position) -> usize {
    let base_map: Vec<Vec<char>> = map.clone();
    solve(&mut map, pos).unwrap(); // This must be successful but we don't care about the outcome.

    let mut loops = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '*' {
                let mut temp_map = base_map.clone();
                temp_map[y][x] = '#';

                if solve(&mut temp_map, pos).is_none() {
                    loops += 1;
                }
            }
        }
    }

    loops
}

fn solve(map: &mut [Vec<char>], mut pos: Position) -> Option<usize> {
    let mut visited: HashSet<Position> = HashSet::new();

    // Mark started position as visited.
    map[pos.y as usize][pos.x as usize] = '*';

    'walk: loop {
        let lookup = pos.next_lookup();

        // Check if time to exit.
        if lookup.x < 0
            || lookup.y < 0
            || lookup.x >= map[0].len() as i32
            || lookup.y >= map.len() as i32
        {
            break 'walk;
        }

        if map[lookup.y as usize][lookup.x as usize] == '#' {
            pos.rotate();
        } else {
            pos = lookup;
            map[pos.y as usize][pos.x as usize] = '*';

            if visited.contains(&pos) {
                return None;
            }

            visited.insert(pos);
        }
    }

    Some(map.iter().flatten().filter(|&&c| c == '*').count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut map: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];

        let pos = Position {
            x: 4,
            y: 6,
            direction: Direction::Up,
        };

        assert_eq!(41, solve(&mut map, pos).unwrap());
    }

    #[test]
    fn test_part2() {
        let map: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];

        let pos = Position {
            x: 4,
            y: 6,
            direction: Direction::Up,
        };

        assert_eq!(6, how_many_loops(map, pos));
    }
}
