use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

/// Implementors of this trait are able to generate SQL which references all columns of a given
/// table.
pub trait ColumnsToSql
{
	/// Push a comma-separated list of column names to the `query`, e.g.:
	/// `column_1,column_2,`…`column_n`.
	///
	/// # Examples
	///
	/// * See [`EmployeeColumns::unique`](crate::schema::columns::EmployeeColumns::unique), which
	///   uses [`QueryBuilderExt::push_columns`](crate::fmt::QueryBuilderExt::push_columns).
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database;

	/// Push the `SET` clause (keyword not included) to the `query`.
	///
	/// # Examples
	///
	/// ```rust
	/// use winvoice_adapter::{
	///   fmt::{As, ColumnsToSql, QueryBuilderExt, SnakeCase, sql, TableToSql},
	///   schema::columns::EmployeeColumns,
	/// };
	/// use winvoice_schema::{Id, Employee};
	/// # use pretty_assertions::assert_eq;
	/// use sqlx::{Execute, QueryBuilder, Postgres};
	///
	/// let columns = EmployeeColumns::default();
	/// let employees = [
	///   Employee {
	///     id: Id::new_v4(), // NOTE: you normally want to avoid assigning an arbitrary ID like this
	///     name: "Bob".into(),
	///     status: "Employed".into(),
	///     title: "CEO".into(),
	///   },
	///   Employee {
	///     id: Id::new_v4(), // NOTE: you normally want to avoid assigning an arbitrary ID like this
	///     name: "John".into(),
	///     status: "Employed".into(),
	///     title: "Janitor".into(),
	///   },
	/// ];
	///
	///
	/// // guarantees a uniqueness for `values_alias`, even if `DEFAULT_ALIAS` changes.
	/// let values_alias = SnakeCase::from((EmployeeColumns::DEFAULT_ALIAS, 'V'));
	///
	/// let mut query = QueryBuilder::<Postgres>::new(sql::UPDATE);
	/// query
	///   .push(As(
	///     EmployeeColumns::TABLE_NAME,
	///     EmployeeColumns::DEFAULT_ALIAS,
	///   ))
	///   .push(sql::SET);
	///
	/// columns.push_set_to(&mut query, values_alias);
	///
	/// query
	///   .push(sql::FROM)
	///   .push('(')
	///   .push_values(
	///     employees.iter(),
	///     |mut q, e| {
	///       q.push_bind(e.id)
	///        .push_bind(&e.name)
	///        .push_bind(&e.status)
	///        .push_bind(&e.title);
	///     }
	///   )
	///   .push(')')
	///   .push(sql::AS)
	///   .push(values_alias)
	///   .push(" (")
	///   .push_columns(&columns)
	///   .push(')')
	///   .push(sql::WHERE);
	///
	/// columns.push_update_where_to(&mut query, EmployeeColumns::DEFAULT_ALIAS, values_alias);
	///
	/// assert_eq!(
	///   query.prepare().sql(),
	///   " UPDATE employees AS E \
	///     SET name=E_V.name,status=E_V.status,title=E_V.title \
	///     FROM (VALUES ($1, $2, $3, $4), ($5, $6, $7, $8)) AS E_V (id,name,status,title) \
	///     WHERE E.id=E_V.id;"
	/// );
	/// ```
	fn push_set_to<Db, Values>(&self, query: &mut QueryBuilder<Db>, values_alias: Values)
	where
		Db: Database,
		Values: Copy + Display;

	/// Push the `WHERE` clause of an `UPDATE` statement (`WHERE` keyword not included) to the
	/// `query`, e.g.:
	///
	/// # Examples
	///
	/// * See [`ColumnsToSql::push_set_to`].
	fn push_update_where_to<Db, Table, Values>(
		&self,
		query: &mut QueryBuilder<Db>,
		table_alias: Table,
		values_alias: Values,
	) where
		Db: Database,
		Table: Copy + Display,
		Values: Copy + Display;
}
