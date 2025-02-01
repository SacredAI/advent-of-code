use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub const ORIGIN: Pos = Pos::new(0, 0);
pub const UP: Pos = Pos::new(0, -1);
pub const DOWN: Pos = Pos::new(0, 1);
pub const LEFT: Pos = Pos::new(-1, 0);
pub const RIGHT: Pos = Pos::new(1, 0);
pub const ORTHOGONAL: [Pos; 4] = [UP, DOWN, LEFT, RIGHT];
// Left to right and top to bottom.
pub const DIAGONAL: [Pos; 8] = [
    Pos::new(-1, -1),
    UP,
    Pos::new(1, -1),
    LEFT,
    RIGHT,
    Pos::new(-1, 1),
    DOWN,
    Pos::new(1, 1),
];

impl Pos {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Pos::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Pos::new(self.y, -self.x)
    }

    #[inline]
    #[must_use]
    pub fn manhattan(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    #[must_use]
    pub fn signum(self, other: Self) -> Self {
        Pos::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }
}

impl Hash for Pos {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

impl Add for Pos {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Pos {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Pos {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Pos::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Pos {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Pos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Pos {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
