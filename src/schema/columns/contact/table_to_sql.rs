use super::ContactColumns;
use crate::fmt::TableToSql;

impl TableToSql for ContactColumns
{
	const DEFAULT_ALIAS: char = 'C';
	const TABLE_NAME: &'static str = "contact_information";
}
