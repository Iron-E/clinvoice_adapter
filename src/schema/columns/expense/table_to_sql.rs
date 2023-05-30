use super::ExpenseColumns;
use crate::fmt::TableToSql;

impl TableToSql for ExpenseColumns
{
	const DEFAULT_ALIAS: char = 'X';
	const TABLE_NAME: &'static str = "expenses";
}
