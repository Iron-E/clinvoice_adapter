use super::EmployeeColumns;
use crate::fmt::TableToSql;

impl TableToSql for EmployeeColumns
{
	const DEFAULT_ALIAS: char = 'E';
	const TABLE_NAME: &'static str = "employees";
}
