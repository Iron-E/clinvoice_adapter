use core::fmt::{Display, Formatter, Result};

use super::WithIdentifier;

impl<Column, Ident> Display for WithIdentifier<Column, Ident>
where
	Column: Display,
	Ident: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}.{}", self.0, self.1)
	}
}
