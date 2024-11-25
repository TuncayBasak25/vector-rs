use num_traits::Float;

use crate::Vec2;

impl<T: Float> Vec2<T> {
    pub fn mag(&self) -> T {
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    }

    pub fn dir(&self) -> T {
		if self.x == T::zero() && self.y == T::zero() {
			T::zero()
		}
		else {
			y.atan2(x)
		}
    }

	pub fn normalize(&mut self) -> &mut Self {
        let mag = self.mag();
        if mag != 0.0 {
            self.scale(1.0/mag);
        }
        self
    }

	pub fn scale(&mut self, value: T) -> &mut Self {
        self.x *= value;
        self.y *= value;
        self
    }

    pub fn set_direction(&mut self, rad: T) -> &mut Self {
        self.x = rad.cos() * self.mag();
        self.y = rad.sin() * self.mag();
        self
    }

    pub fn rotate(&mut self, rad: T) -> &mut Self {
        self.set_direction(rad + self.dir());
        self
    }

    pub fn rotate_over<U: Into<Vec2<T>>>(&mut self, origin: U, rad: T) -> &mut Self {
		let origin: Vec2<T> = origin.into();
        self.sub(origin);
        self.rotate(rad);
        self.add(origin);
        self
    }
	
    pub fn point_towards<U: Into<Vec2<T>>>(&mut self, target: U) -> &mut Self {
		let mut target: Vec2<T> = target.into();
        self.set_direction(target.sub(*self).dir())
    }
}