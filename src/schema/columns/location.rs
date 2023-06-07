mod columns_to_sql;
mod table_to_sql;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::fmt::{TableToSql, WithIdentifier};

/// The names of the columns of the `locations` table.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocationColumns<T = &'static str>
{
	/// The name of the `currency` column of the `locations` table.
	pub currency: T,

	/// The name of the `id` column of the `locations` table.
	pub id: T,

	/// The name of the `name` column of the `locations` table.
	pub name: T,

	/// The name of the `outer_id` column of the `locations` table.
	pub outer_id: T,
}

impl<T> LocationColumns<T>
{
	/// Add a [scope](LocationColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn default_scope(self) -> LocationColumns<WithIdentifier<char, T>>
	{
		self.scope(LocationColumns::DEFAULT_ALIAS)
	}

	/// Returns a [`LocationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	#[allow(clippy::missing_const_for_fn)]
	pub fn scope<Alias>(self, alias: Alias) -> LocationColumns<WithIdentifier<Alias, T>>
	where
		Alias: Copy,
	{
		LocationColumns {
			currency: WithIdentifier(alias, self.currency),
			id: WithIdentifier(alias, self.id),
			outer_id: WithIdentifier(alias, self.outer_id),
			name: WithIdentifier(alias, self.name),
		}
	}
}

impl LocationColumns<&'static str>
{
	/// The names of the columns in `locations` without any aliasing.
	pub const fn default() -> Self
	{
		Self { currency: "currency", id: "id", outer_id: "outer_id", name: "name" }
	}
}
