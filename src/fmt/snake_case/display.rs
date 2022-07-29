use core::fmt::{Display, Formatter, Result};

use super::SnakeCase;

impl<TLeft, TRight> Display for SnakeCase<TLeft, TRight>
where
	TLeft: Display,
	TRight: Display,
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
