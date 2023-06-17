mod columns_to_sql;
mod table_to_sql;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::fmt::{As, TableToSql, WithIdentifier};

/// The names of the columns of the `organizations` table.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrganizationColumns<T = &'static str>
{
	/// The name of the `id` column of the `organizations` table.
	pub id: T,

	/// The name of the `location_id` column of the `organizations` table.
	pub location_id: T,

	/// The name of the `name` column of the `organizations` table.
	pub name: T,
}

impl<T> OrganizationColumns<T>
{
	/// Returns a [`OrganizationColumns`] which aliases the names of these [`OrganizationColumns`]
	/// with the `aliased` columns provided.
	///
	/// # See also
	///
	/// * [`As`]
	#[allow(clippy::missing_const_for_fn)]
	pub fn r#as<Alias>(
		self,
		aliased: OrganizationColumns<Alias>,
	) -> OrganizationColumns<As<T, Alias>>
	{
		OrganizationColumns {
			id: As(self.id, aliased.id),
			location_id: As(self.location_id, aliased.location_id),
			name: As(self.name, aliased.name),
		}
	}

	/// Add a [scope](OrganizationColumns::scope) using the [default
	/// alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn default_scope(self) -> OrganizationColumns<WithIdentifier<char, T>>
	{
		self.scope(OrganizationColumns::DEFAULT_ALIAS)
	}

	/// Returns a [`OrganizationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	#[allow(clippy::missing_const_for_fn)]
	pub fn scope<Alias>(self, alias: Alias) -> OrganizationColumns<WithIdentifier<Alias, T>>
	where
		Alias: Copy,
	{
		OrganizationColumns {
			id: WithIdentifier(alias, self.id),
			location_id: WithIdentifier(alias, self.location_id),
			name: WithIdentifier(alias, self.name),
		}
	}
}

impl OrganizationColumns<&'static str>
{
	/// The names of the columns in `organizations` without any aliasing.
	///
	/// # Examples
	///
	/// * See [`OrganizationColumns::unique`].
	pub const fn default() -> Self
	{
		Self { id: "id", location_id: "location_id", name: "name" }
	}

	/// Aliases for the columns in `employees` which are guaranteed to be unique among other
	/// [`columns`](super)'s `unique` aliases.
	///
	/// # Examples
	///
	/// ```rust
	/// use winvoice_adapter::{
	///   fmt::{QueryBuilderExt, sql},
	///   schema::columns::{EmployeeColumns, OrganizationColumns},
	/// };
	/// # use pretty_assertions::assert_str_eq as assert_eq;
	/// use sqlx::{Execute, Postgres, QueryBuilder};
	///
	/// {
	///   let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	///   // `sqlx::Row::get` ignores scopes (e.g. "E." in "E.id") so "E.id" and "O.id", as well as
	///   // "E.name" and "O.name", clobber each other.
	///   assert_eq!(
	///     query
	///       .push_columns(&EmployeeColumns::default().default_scope())
	///       .push_more_columns(&OrganizationColumns::default().default_scope())
	///       .prepare()
	///       .sql(),
	///     " SELECT E.active,E.department,E.id,E.name,E.title,O.id,O.location_id,O.name;"
	///   );
	/// }
	///
	/// {
	///   let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	///   // no clobbering
	///   assert_eq!(
	///     query
	///       .push_columns(&EmployeeColumns::default().default_scope())
	///       .push_more_columns(
	///         &OrganizationColumns::default()
	///           .default_scope()
	///           .r#as(OrganizationColumns::unique())
	///       )
	///       .prepare()
	///       .sql(),
	///     " SELECT E.active,E.department,E.id,E.name,E.title,\
	///         O.id AS unique_6_organization_id,\
	///         O.location_id AS unique_6_organization_location_id,\
	///         O.name AS unique_6_organization_name;"
	///   );
	/// }
	/// ```
	pub const fn unique() -> Self
	{
		Self {
			id: "unique_6_organization_id",
			location_id: "unique_6_organization_location_id",
			name: "unique_6_organization_name",
		}
	}
}
