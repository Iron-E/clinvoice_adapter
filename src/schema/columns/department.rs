mod columns_to_sql;
mod table_to_sql;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::fmt::{As, TableToSql, WithIdentifier};

/// The names of the columns of the `employees` table.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DepartmentColumns<T = &'static str>
{
	/// The name of the `id` column of the `employees` table.
	pub id: T,

	/// The name of the `name` column of the `employees` table.
	pub name: T,
}

impl<T> DepartmentColumns<T>
{
	/// Returns a [`DepartmentColumns`] which aliases the names of these [`DepartmentColumns`] with
	/// the `aliased` columns provided.
	///
	/// # See also
	///
	/// * [`As`]
	#[allow(clippy::missing_const_for_fn)]
	pub fn r#as<Alias>(self, aliased: DepartmentColumns<Alias>) -> DepartmentColumns<As<T, Alias>>
	{
		DepartmentColumns { id: As(self.id, aliased.id), name: As(self.name, aliased.name) }
	}

	/// Add a [scope](DepartmentColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn default_scope(self) -> DepartmentColumns<WithIdentifier<char, T>>
	{
		self.scope(DepartmentColumns::DEFAULT_ALIAS)
	}

	/// Returns a [`DepartmentColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	#[allow(clippy::missing_const_for_fn)]
	pub fn scope<Alias>(self, alias: Alias) -> DepartmentColumns<WithIdentifier<Alias, T>>
	where
		Alias: Copy,
	{
		DepartmentColumns {
			id: WithIdentifier(alias, self.id),
			name: WithIdentifier(alias, self.name),
		}
	}
}

impl DepartmentColumns<&'static str>
{
	/// The names of the columns in `employees` without any aliasing.
	///
	/// # Examples
	///
	/// * See [`DepartmentColumns::unique`].
	pub const fn default() -> Self
	{
		Self { id: "id", name: "name" }
	}

	/// Aliases for the columns in `employees` which are guaranteed to be unique among other
	/// [`columns`](super)'s `unique` aliases.
	pub const fn unique() -> Self
	{
		Self { id: "unique_8_department_id", name: "unique_8_department_name" }
	}
}
