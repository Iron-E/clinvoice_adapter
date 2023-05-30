mod columns_to_sql;
mod table_to_sql;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::fmt::{TableToSql, WithIdentifier};

/// The names of the columns of the `timesheets` table.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimesheetColumns<T = &'static str>
{
	/// The name of the `employee_id` column of the `timesheets` table.
	pub employee_id: T,

	/// The name of the `id` column of the `timesheets` table.
	pub id: T,

	/// The name of the `job_id` column of the `timesheets` table.
	pub job_id: T,

	/// The name of the `time_begin` column of the `timesheets` table.
	pub time_begin: T,

	/// The name of the `time_end` column of the `timesheets` table.
	pub time_end: T,

	/// The name of the `work_notes` column of the `timesheets` table.
	pub work_notes: T,
}

impl<T> TimesheetColumns<T>
{
	/// Add a [scope](ExpenseColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`].
	pub fn default_scope(self) -> TimesheetColumns<WithIdentifier<char, T>>
	{
		self.scope(TimesheetColumns::DEFAULT_ALIAS)
	}

	/// Returns a [`TimesheetColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	#[allow(clippy::missing_const_for_fn)]
	pub fn scope<Alias>(self, alias: Alias) -> TimesheetColumns<WithIdentifier<Alias, T>>
	where
		Alias: Copy,
	{
		TimesheetColumns {
			employee_id: WithIdentifier(alias, self.employee_id),
			id: WithIdentifier(alias, self.id),
			job_id: WithIdentifier(alias, self.job_id),
			time_begin: WithIdentifier(alias, self.time_begin),
			time_end: WithIdentifier(alias, self.time_end),
			work_notes: WithIdentifier(alias, self.work_notes),
		}
	}
}

impl TimesheetColumns<&'static str>
{
	/// The names of the columns in `organizations` without any aliasing.
	pub const fn default() -> Self
	{
		Self {
			id: "id",
			employee_id: "employee_id",
			job_id: "job_id",
			time_begin: "time_begin",
			time_end: "time_end",
			work_notes: "work_notes",
		}
	}
}
