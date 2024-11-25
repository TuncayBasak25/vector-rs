mod conversion;
mod float;
mod overload_op;

use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2<T>
{
	pub x: T,
    pub y: T
}

impl<T> Vec2<T>
where T: Copy + 
        Add<Output = T> + Sub<Output = T> + Mul<Output = T> +
        AddAssign + SubAssign
{
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }

	pub fn add<U: Into<Vec2<T>>>(&mut self, rhs: U) -> &mut Self {
		let rhs = rhs.into();
		self.x += rhs.x;
		self.y += rhs.y;
		self
	}
	
	pub fn sub<U: Into<Vec2<T>>>(&mut self, rhs: U) -> &mut Self {
		let rhs = rhs.into();
		self.x -= rhs.x;
		self.y -= rhs.y;
		self
	}
	
	pub fn dot<U: Into<Vec2<T>>>(&self, other: U) -> T {
		let other = other.into();
		self.x * other.x + self.y * other.y
	}
	
	pub fn cross<U: Into<Vec2<T>>>(&self, other: U) -> T {
		let other = other.into();
		self.x * other.y - self.y * other.x
	}
}