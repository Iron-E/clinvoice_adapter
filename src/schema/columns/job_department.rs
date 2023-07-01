mod columns_to_sql;
mod table_to_sql;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::fmt::{As, TableToSql, WithIdentifier};

/// The names of the columns of the `employees` table.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JobDepartmentColumns<T = &'static str>
{
	/// The name of the `department_id` column of the `job_departments` table.
	pub department_id: T,

	/// The name of the `job_id` column of the `job_departments` table.
	pub job_id: T,
}

impl<T> JobDepartmentColumns<T>
{
	/// Returns a [`DepartmentColumns`] which aliases the names of these [`DepartmentColumns`] with
	/// the `aliased` columns provided.
	///
	/// # See also
	///
	/// * [`As`]
	#[allow(clippy::missing_const_for_fn)]
	pub fn r#as<Alias>(self, aliased: JobDepartmentColumns<Alias>) -> JobDepartmentColumns<As<T, Alias>>
	{
		JobDepartmentColumns {
			department_id: As(self.department_id, aliased.department_id),
			job_id: As(self.job_id, aliased.job_id),
		}
	}

	/// Add a [scope](DepartmentColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn default_scope(self) -> JobDepartmentColumns<WithIdentifier<char, T>>
	{
		self.scope(JobDepartmentColumns::DEFAULT_ALIAS)
	}

	/// Returns a [`DepartmentColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	#[allow(clippy::missing_const_for_fn)]
	pub fn scope<Alias>(self, alias: Alias) -> JobDepartmentColumns<WithIdentifier<Alias, T>>
	where
		Alias: Copy,
	{
		JobDepartmentColumns {
			department_id: WithIdentifier(alias, self.department_id),
			job_id: WithIdentifier(alias, self.job_id),
		}
	}
}

impl JobDepartmentColumns<&'static str>
{
	/// The names of the columns in `employees` without any aliasing.
	///
	/// # Examples
	///
	/// * See [`DepartmentColumns::unique`].
	pub const fn default() -> Self
	{
		Self { department_id: "department_id", job_id: "job_id" }
	}

	/// Aliases for the columns in `employees` which are guaranteed to be unique among other
	/// [`columns`](super)'s `unique` aliases.
	pub const fn unique() -> Self
	{
		Self { department_id: "unique_8_job_department_department_id", job_id: "unique_8_job_department_job_id" }
	}
}
