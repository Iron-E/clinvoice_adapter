use super::JobDepartmentColumns;
use crate::fmt::TableToSql;

impl TableToSql for JobDepartmentColumns
{
	const DEFAULT_ALIAS: char = 'O';
	const TABLE_NAME: &'static str = "job_departments";
}