use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::LocationColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<TColumn> ColumnsToSql for LocationColumns<TColumn>
where
	TColumn: Copy + Display,
{
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(self.id)
			.push(self.name)
			.push(self.outer_id);
	}

	fn push_set_to<TDb, TValues>(&self, query: &mut QueryBuilder<TDb>, values_alias: TValues)
	where
		TDb: Database,
		TValues: Copy + Display,
	{
		let values_columns = self.scope(values_alias);
		query
			.push_equal(self.name, values_columns.name)
			.push(',')
			.push_equal(self.outer_id, values_columns.outer_id);
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
