use int_enum::IntEnum;
// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Clone, Copy, Debug, IntEnum)]
#[repr(i8)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn from_i8(i8_number: i8) -> Direction {
        match i8_number {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("value not in range!!!!"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: Direction::from_i8((self.d as i8 + 1 + 4) % 4),
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: Direction::from_i8((self.d as i8 - 1 + 4) % 4),
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Self {
                y: self.y + 1,
                ..self
            },
            Direction::East => Self {
                x: self.x + 1,
                ..self
            },
            Direction::South => Self {
                y: self.y - 1,
                ..self
            },
            Direction::West => Self {
                x: self.x - 1,
                ..self
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot: Robot = self;

        instructions.chars().for_each(|action: char| {
            match action {
                'R' => {
                    robot = Robot::turn_right(robot);
                    println!("RIGHT {:#?}", robot);
                }
                'L' => {
                    robot = Robot::turn_left(robot);
                    println!("LEFT {:#?}", robot);
                }
                'A' => {
                    robot = Robot::advance(robot);
                    println!("ADVANCE {:#?}", robot);
                }
                _ => panic!("Value not in range"),
            };
        });
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
