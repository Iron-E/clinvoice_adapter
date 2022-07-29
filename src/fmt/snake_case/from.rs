use core::fmt::Display;

use super::SnakeCase;

impl<T> From<T> for SnakeCase<T, &'static str>
where
	T: Display,
{
	fn from(head: T) -> Self
	{
		Self::Head(head)
	}
}

impl<Left, Right> From<(Left, Right)> for SnakeCase<Left, Right>
where
	Left: Display,
	Right: Display,
{
	fn from(body: (Left, Right)) -> Self
	{
		Self::Body(body.0, body.1)
	}
}
