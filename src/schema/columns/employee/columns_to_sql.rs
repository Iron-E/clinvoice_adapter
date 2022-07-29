use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::EmployeeColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<Column> ColumnsToSql for EmployeeColumns<Column>
where
	Column: Copy + Display,
{
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(self.id)
			.push(self.name)
			.push(self.status)
			.push(self.title);
	}

	fn push_set_to<Db, Values>(&self, query: &mut QueryBuilder<Db>, values_alias: Values)
	where
		Db: Database,
		Values: Copy + Display,
	{
		let values_columns = self.scope(values_alias);
		query
			.push_equal(self.name, values_columns.name)
			.push(',')
			.push_equal(self.status, values_columns.status)
			.push(',')
			.push_equal(self.title, values_columns.title);
	}

	fn push_update_where_to<Db, Table, Values>(
		&self,
		query: &mut QueryBuilder<Db>,
		table_alias: Table,
		values_alias: Values,
	) where
		Db: Database,
		Table: Copy + Display,
		Values: Copy + Display,
	{
		query.push_equal(self.scope(table_alias).id, self.scope(values_alias).id);
	}
}
