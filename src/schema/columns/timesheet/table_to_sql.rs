use super::TimesheetColumns;
use crate::fmt::TableToSql;

impl TableToSql for TimesheetColumns
{
	const DEFAULT_ALIAS: char = 'T';
	const TABLE_NAME: &'static str = "timesheets";
}
