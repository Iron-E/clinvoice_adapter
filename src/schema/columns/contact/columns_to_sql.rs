use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::ContactColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<Column> ColumnsToSql for ContactColumns<Column>
where
	Column: Copy + Display,
{
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query.separated(',').push(self.address_id).push(self.email).push(self.label).push(self.other).push(self.phone);
	}

	fn push_set_to<Db, Values>(&self, query: &mut QueryBuilder<Db>, values_alias: Values)
	where
		Db: Database,
		Values: Copy + Display,
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
		query.push_equal(self.scope(table_alias).label, self.scope(values_alias).label);
	}
}
