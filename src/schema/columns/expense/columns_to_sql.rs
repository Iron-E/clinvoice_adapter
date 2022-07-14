use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::ExpenseColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<TColumn> ColumnsToSql for ExpenseColumns<TColumn>
where
	TColumn: Copy + Display,
{
	fn push_to<TDb>(&self, query: &mut QueryBuilder<TDb>)
	where
		TDb: Database,
	{
		query
			.separated(',')
			.push(self.category)
			.push(self.cost)
			.push(self.description)
			.push(self.id)
			.push(self.timesheet_id);
	}

	fn push_set_to<TDb, TValues>(&self, query: &mut QueryBuilder<TDb>, values_alias: TValues)
	where
		TDb: Database,
		TValues: Copy + Display,
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

	fn push_update_where_to<TDb, TTable, TValues>(
		&self,
		query: &mut QueryBuilder<TDb>,
		table_alias: TTable,
		values_alias: TValues,
	) where
		TDb: Database,
		TTable: Copy + Display,
		TValues: Copy + Display,
	{
		query.push_equal(self.scope(table_alias).id, self.scope(values_alias).id);
	}
}
