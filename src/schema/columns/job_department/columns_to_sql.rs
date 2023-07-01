use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::JobDepartmentColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<Column> ColumnsToSql for JobDepartmentColumns<Column>
where
	Column: Copy + Display,
{
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query.separated(',').push(self.department_id).push(self.job_id);
	}

	fn push_set_to<Db, Values>(&self, query: &mut QueryBuilder<Db>, values_alias: Values)
	where
		Db: Database,
		Values: Copy + Display,
	{
		query.push_equal(self.job_id, self.scope(values_alias).job_id);
	}

	fn push_update_where_to<Db, Table, Values>(&self, _: &mut QueryBuilder<Db>, _: Table, _: Values)
	where
		Db: Database,
		Table: Copy + Display,
		Values: Copy + Display,
	{
		unimplemented!(
			"This table has only two keys, and they comprise the primary key. No row should be updated, only \
			 `INSERT`ed or `DELETE`d"
		);
	}
}
