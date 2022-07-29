use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::ExpenseColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<Column> ColumnsToSql for ExpenseColumns<Column>
where
	Column: Copy + Display,
{
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(self.category)
			.push(self.cost)
			.push(self.description)
			.push(self.id)
			.push(self.timesheet_id);
	}

	fn push_set_to<Db, Values>(&self, query: &mut QueryBuilder<Db>, values_alias: Values)
	where
		Db: Database,
		Values: Copy + Display,
	{
		let values_columns = self.scope(values_alias);
		query
			.push_equal(self.category, values_columns.category)
			.push(',')
			.push_equal(self.cost, values_columns.cost)
			.push(',')
			.push_equal(self.description, values_columns.description)
			.push(',')
			.push_equal(self.timesheet_id, values_columns.timesheet_id);
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
