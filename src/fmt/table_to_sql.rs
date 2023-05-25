/// Defines attributes of a table in a database which was
/// [`init`](crate::Initializable::init)ialized for use with Winvoice.
///
/// # Examples
///
/// * See [`QueryBuilderExt::push_default_from`](super::QueryBuilderExt::push_default_from).
pub trait TableToSql
{
	/// Get the standard alias that can be used to refer to this table.
	///
	/// # Warnings
	///
	/// * Must be unique among other implementors of [`TableToSql`].
	const DEFAULT_ALIAS: char;

	/// Get the name of this table.
	///
	/// # Warnings
	///
	/// * Must be unique among other implementors of [`TableToSql`].
	const TABLE_NAME: &'static str;
}
