use super::DepartmentColumns;
use crate::fmt::TableToSql;

impl TableToSql for DepartmentColumns
{
	const DEFAULT_ALIAS: char = 'D';
	const TABLE_NAME: &'static str = "departments";
}
