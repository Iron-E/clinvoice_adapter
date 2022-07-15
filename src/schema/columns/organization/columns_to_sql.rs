use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::OrganizationColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<TColumn> ColumnsToSql for OrganizationColumns<TColumn>
where
	TColumn: Copy + Display,
{
	fn push_to<TDb>(&self, query: &mut QueryBuilder<TDb>)
	where
		TDb: Database,
	{
		query
			.separated(',')
			.push(self.id)
			.push(self.location_id)
			.push(self.name);
	}

	fn push_set_to<TDb, TValues>(&self, query: &mut QueryBuilder<TDb>, values_alias: TValues)
	where
		TDb: Database,
		TValues: Copy + Display,
	{
		let values_columns = self.scope(values_alias);
		query
			.push_equal(self.location_id, values_columns.location_id)
			.push(',')
			.push_equal(self.name, values_columns.name);
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
