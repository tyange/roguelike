#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y : i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn step(self, dir: Dir) -> Point {
        let (dx, dy) = dir.delta();
        Point::new(self.x + dx, self.y + dy)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl Dir {
    pub const ALL: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

    pub fn delta(self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0)
        }
    }
}