use super::LocationColumns;
use crate::fmt::TableToSql;

impl TableToSql for LocationColumns
{
	const DEFAULT_ALIAS: char = 'L';
	const TABLE_NAME: &'static str = "locations";
}
