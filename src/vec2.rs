use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub struct Vec2 {
    x: isize,
    y: isize,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<isize> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
