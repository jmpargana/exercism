// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    dir: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            pos: (x, y),
            dir: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot { dir, ..self }
    }

    pub fn turn_left(self) -> Self {
        let dir = match self.dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Robot { dir, ..self }
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self.dir {
            Direction::North => (self.pos.0, self.pos.1 + 1),
            Direction::South => (self.pos.0, self.pos.1 - 1),
            Direction::East => (self.pos.0 + 1, self.pos.1),
            Direction::West => (self.pos.0 - 1, self.pos.1),
        };
        Robot { pos: (x, y), ..self }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.to_string().chars().fold(self, |acc, c| {
            match c {
                'A' => acc.advance(),
                'R' => acc.turn_right(),
                'L' => acc.turn_left(),
                _ => unreachable!(),
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.pos.0, self.pos.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
