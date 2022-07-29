use core::fmt::{Display, Formatter, Result};

use super::TypeCast;
use crate::fmt::{sql, As};

impl<Cast, Column> Display for TypeCast<Cast, Column>
where
	Cast: Display,
	Column: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}({})", sql::CAST, As(&self.0, &self.1))
	}
}
