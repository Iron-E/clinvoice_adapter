use core::fmt::{Display, Formatter, Result};

use super::Coalesce;

impl<Left, Right> Display for Coalesce<Left, Right>
where
	Left: Display,
	Right: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "COALESCE({}, {})", self.0, self.1)
	}
}
