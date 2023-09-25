use core::fmt::{Display, Formatter, Result};

use super::NullIf;

impl<Left, Right> Display for NullIf<Left, Right>
where
	Left: Display,
	Right: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "NULLIF({}, {})", self.0, self.1)
	}
}
