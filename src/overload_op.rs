use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::Vec2;

impl<T> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Add<(T, T)> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: (T, T)) -> Vec2<T> {
        Vec2 {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl<T> Sub<(T, T)> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: (T, T)) -> Vec2<T> {
        Vec2 {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl<T> AddAssign<(T, T)> for Vec2<T> {
    fn add_assign(&mut self, rhs: (T, T)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl<T> SubAssign<(T, T)> for Vec2<T> {
    fn sub_assign(&mut self, rhs: (T, T)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}