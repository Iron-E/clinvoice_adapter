mod write_context;

use core::fmt::Display;

use sqlx::{Database, QueryBuilder};
pub use write_context::WriteContext;

/// Implementors of this trait can generate `WHERE` clauses out of some [match
/// condition](clinvoice_match) `M` for the given `Db`'s SQL syntax.
pub trait WriteWhereClause<TDb, TMatch>
where
	TDb: Database,
{
	/// Append an SQL `WHERE` clause, with a `match_condition` that references `ident`, to `query`.
	///
	/// The [`WriteContext`] for the state of the `query` after all writes is returned.
	fn write_where_clause<TIdent>(
		context: WriteContext,
		ident: TIdent,
		match_condition: TMatch,
		query: &mut QueryBuilder<TDb>,
	) -> WriteContext
	where
		TIdent: Copy + Display;
}
