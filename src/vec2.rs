use num_traits::Zero;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
	pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

	pub fn add<U: Into<Vec2>>(&mut self, rhs: U) -> &mut Self {
		let rhs: Vec2 = rhs.into();
		self.x += rhs.x;
		self.y += rhs.y;
		self
	}
	
	pub fn sub<U: Into<Vec2>>(&mut self, rhs: U) -> &mut Self {
		let rhs: Vec2 = rhs.into();
		self.x -= rhs.x;
		self.y -= rhs.y;
		self
	}
	
	pub fn dot<U: Into<Vec2>>(&self, other: U) -> f32 {
		let other: Vec2 = other.into();
		self.x * other.x + self.y * other.y
	}
	
	pub fn cross<U: Into<Vec2>>(&self, other: U) -> f32 {
		let other: Vec2 = other.into();
		self.x * other.y - self.y * other.x
	}
    
    pub fn mag(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn dir(&self) -> f32 {
		if self.x.is_zero() && self.y.is_zero() {
			f32::zero()
		}
		else {
			self.y.atan2(self.x)
		}
    }

	pub fn normalize(&mut self) -> &mut Self {
        let mag = self.mag();
        if !mag.is_zero() {
            self.scale(1.0/mag);
        }
        self
    }

	pub fn scale(&mut self, value: f32) -> &mut Self {
        self.x *= value;
        self.y *= value;
        self
    }

    pub fn set_direction(&mut self, rad: f32) -> &mut Self {
        let mag = self.mag();
        if mag != 0.0 {
            self.x = rad.cos() * mag;
            self.y = rad.sin() * mag;
        }
        self
    }

    pub fn rotate(&mut self, rad: f32) -> &mut Self {
        self.set_direction(rad + self.dir());
        self
    }

    pub fn rotate_over<U: Into<Vec2>>(&mut self, origin: U, rad: f32) -> &mut Self {
		let origin: Vec2 = origin.into();
        self.sub(origin);
        self.rotate(rad);
        self.add(origin);
        self
    }
	
    pub fn point_towards<U: Into<Vec2>>(&mut self, target: U) -> &mut Self {
		let target: Vec2 = target.into();
        self.set_direction((target - *self).dir())
    }
}



use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::cmp::PartialEq;

impl PartialEq<Vec2> for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        (self.x - other.x).abs() < f32::EPSILON && (self.y - other.y).abs() < f32::EPSILON
    }
}

impl PartialEq<(f32, f32)> for Vec2 {
    fn eq(&self, other: &(f32, f32)) -> bool {
        (self.x - other.0).abs() < f32::EPSILON && (self.y - other.1).abs() < f32::EPSILON
    }
}


impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Add<(f32, f32)> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: (f32, f32)) -> Vec2 {
        Vec2 {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Sub<(f32, f32)> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: (f32, f32)) -> Vec2 {
        Vec2 {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl AddAssign<(f32, f32)> for Vec2 {
    fn add_assign(&mut self, rhs: (f32, f32)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl SubAssign<(f32, f32)> for Vec2 {
    fn sub_assign(&mut self, rhs: (f32, f32)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

impl Add<Vec2> for (f32, f32) {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.0 + rhs.x,
            y: self.1 + rhs.y,
        }
    }
}

impl Sub<Vec2> for (f32, f32) {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.0 - rhs.x,
            y: self.1 - rhs.y,
        }
    }
}

impl AddAssign<Vec2> for (f32, f32) {
    fn add_assign(&mut self, rhs: Vec2) {
        self.0 += rhs.x;
        self.1 += rhs.y;
    }
}

impl SubAssign<Vec2> for (f32, f32) {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.0 -= rhs.x;
        self.1 -= rhs.y;
    }
}

impl From<(f32, f32)> for Vec2 {
	fn from(input: (f32, f32)) -> Self {
		Vec2 {
			x: input.0,
			y: input.1
		}
	}
}


impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let vec = Vec2::new(3.0, 4.0);
        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 4.0);
    }

    #[test]
    fn test_from_tuple() {
        let vec: Vec2 = (3.0, 4.0).into();
        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 4.0);
    }

    #[test]
    fn test_addition() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1 + v2;
        assert_eq!(result, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_addition_with_tuple() {
        let mut vec = Vec2::new(1.0, 2.0);
        vec += (3.0, 4.0);
        assert_eq!(vec, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_subtraction() {
        let v1 = Vec2::new(5.0, 7.0);
        let v2 = Vec2::new(2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_subtraction_with_tuple() {
        let mut vec = Vec2::new(5.0, 7.0);
        vec -= (2.0, 3.0);
        assert_eq!(vec, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.dot(v2);
        assert_eq!(result, 11.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.cross(v2);
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_magnitude() {
        let vec = Vec2::new(3.0, 4.0);
        assert!((vec.mag() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_direction() {
        let vec = Vec2::new(0.0, 1.0);
        assert!((vec.dir() - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
    }

    #[test]
    fn test_normalization() {
        let mut vec = Vec2::new(3.0, 4.0);
        vec.normalize();
        assert!((vec.mag() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_scaling() {
        let mut vec = Vec2::new(1.0, 2.0);
        vec.scale(2.0);
        assert_eq!(vec, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_set_direction() {
        let mut vec = Vec2::new(3.0, 4.0);
        vec.set_direction(std::f32::consts::PI);
        assert_eq!(vec, (-5.0, 0.0));
    }

    #[test]
    fn test_rotation() {
        let mut vec = Vec2::new(1.0, 0.0);
        vec.rotate(std::f32::consts::FRAC_PI_2);
        assert!((vec.x - 0.0).abs() < 1e-6);
        assert!((vec.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotate_over_origin() {
        let mut vec = Vec2::new(1.0, 0.0);
        let origin = Vec2::new(0.0, 0.0);
        vec.rotate_over(origin, std::f32::consts::FRAC_PI_2);
        assert!((vec.x - 0.0).abs() < 1e-6);
        assert!((vec.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_point_towards() {
        let mut vec = Vec2::new(1.0, 0.0);
        let target = Vec2::new(0.0, 1.0);
        vec.point_towards(target);
        assert_eq!(vec, (-0.5f32.sqrt(), 0.5f32.sqrt()));
    }

    #[test]
    fn test_default() {
        let vec: Vec2 = Default::default();
        assert_eq!(vec, Vec2::new(0.0, 0.0));
    }
}
