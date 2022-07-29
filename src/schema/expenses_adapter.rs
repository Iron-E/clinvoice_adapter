use clinvoice_match::MatchExpense;
use clinvoice_schema::{Expense, Id, Money};
use sqlx::{Executor, Result};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Employee`]s.
#[async_trait::async_trait]
pub trait ExpensesAdapter:
	Deletable<Entity = Expense>
	+ Retrievable<
		Db = <Self as Deletable>::Db,
		Entity = <Self as Deletable>::Entity,
		Match = MatchExpense,
	> + Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return new [`Expense`]s via the `connection`.
	///
	/// # Parameters
	///
	/// `expenses` is a slice of `(String, Money, String)`, which represents `(category, cost,
	/// description)` for the created [`Expense`]s.
	async fn create<'connection, TConn>(
		connection: TConn,
		expenses: Vec<(String, Money, String)>,
		timesheet_id: Id,
	) -> Result<Vec<Expense>>
	where
		TConn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
