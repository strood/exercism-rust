// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.d {
            Direction::North => self.change_direction(Direction::East),
            Direction::East => self.change_direction(Direction::South),
            Direction::South => self.change_direction(Direction::West),
            Direction::West => self.change_direction(Direction::North),
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.d {
            Direction::North => self.change_direction(Direction::West),
            Direction::East => self.change_direction(Direction::North),
            Direction::South => self.change_direction(Direction::East),
            Direction::West => self.change_direction(Direction::South),
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Self {
                x: self.x,
                y: self.y + 1,
                d: self.d,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
                d: self.d,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y - 1,
                d: self.d,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
                d: self.d,
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }

    fn change_direction(&self, d: Direction) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d,
        }
    }
}
