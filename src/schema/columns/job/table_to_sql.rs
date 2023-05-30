use super::JobColumns;
use crate::fmt::TableToSql;

impl TableToSql for JobColumns
{
	const DEFAULT_ALIAS: char = 'J';
	const TABLE_NAME: &'static str = "jobs";
}
