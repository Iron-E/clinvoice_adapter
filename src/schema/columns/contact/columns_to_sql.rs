use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::ContactColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<TColumn> ColumnsToSql for ContactColumns<TColumn>
where
	TColumn: Copy + Display,
{
	fn push_to<TDb>(&self, query: &mut QueryBuilder<TDb>)
	where
		TDb: Database,
	{
		query
			.separated(',')
			.push(self.address_id)
			.push(self.email)
			.push(self.label)
			.push(self.other)
			.push(self.phone);
	}

	fn push_set_to<TDb, TValues>(&self, query: &mut QueryBuilder<TDb>, values_alias: TValues)
	where
		TDb: Database,
		TValues: Copy + Display,
	{
		let values_columns = self.scope(values_alias);
		query
			.push_equal(self.address_id, values_columns.address_id)
			.push(',')
			.push_equal(self.email, values_columns.email)
			.push(',')
			.push_equal(self.label, values_columns.label)
			.push(',')
			.push_equal(self.other, values_columns.other)
			.push(',')
			.push_equal(self.phone, values_columns.phone);
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
		query.push_equal(
			self.scope(table_alias).label,
			self.scope(values_alias).label,
		);
	}
}
