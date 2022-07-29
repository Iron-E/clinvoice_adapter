use core::fmt::{Display, Formatter, Result};

use super::WriteContext;
use crate::fmt::sql;

impl Display for WriteContext
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		match self
		{
			Self::AcceptingAnotherWhereCondition => sql::AND.fmt(f),
			Self::BeforeWhereClause => sql::WHERE.fmt(f),
			Self::InWhereCondition => Ok(()),
		}
	}
}
