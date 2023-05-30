use super::OrganizationColumns;
use crate::fmt::TableToSql;

impl TableToSql for OrganizationColumns
{
	const DEFAULT_ALIAS: char = 'O';
	const TABLE_NAME: &'static str = "organizations";
}
