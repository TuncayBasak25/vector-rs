use crate::Vec2;

impl<To: From<Base>, Base> From<Vec2<Base>> for Vec2<To> {
	fn from(input: Vec2<Base>) -> Self {
		Vec2 {
			x: To::from(input.x),
			y: To::from(input.y)
		}
	}
}

impl<T> From<(T, T)> for Vec2<T> {
	fn from(input: (T, T)) -> Self {
		Vec2 {
			x: input.0,
			y: input.1
		}
	}
}