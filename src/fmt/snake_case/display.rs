use core::fmt::{Display, Formatter, Result};

use super::SnakeCase;

impl<Left, Right> Display for SnakeCase<Left, Right>
where
	Left: Display,
	Right: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::Body(left, right) => write!(f, "{}_{}", left, right),
			Self::Head(left) => left.fmt(f),
		}
	}
}
