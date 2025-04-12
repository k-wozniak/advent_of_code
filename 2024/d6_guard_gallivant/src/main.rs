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

    println!("The number of steps: {}", solve(&mut map, start_position));
}

struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    fn next_lookup(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solve(map: &mut [Vec<char>], mut pos: Position) -> usize {
    // Mark started position as visited.
    map[pos.y as usize][pos.x as usize] = '*';

    'walk: loop {
        let lookup = pos.next_lookup();

        // Check if time to exit.
        if lookup.0 < 0
            || lookup.1 < 0
            || lookup.0 >= map[0].len() as i32
            || lookup.1 >= map.len() as i32
        {
            break 'walk;
        }

        let c = map[lookup.1 as usize][lookup.0 as usize];

        if c == '#' {
            pos.rotate();
        } else {
            pos.x = lookup.0;
            pos.y = lookup.1;
            map[pos.y as usize][pos.x as usize] = '*';
        }
    }

    map.iter().flatten().filter(|&&c| c == '*').count()
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

        assert_eq!(41, solve(&mut map, pos));
    }
}
