use sqlx::{Result, Transaction};
use winvoice_match::MatchTimesheet;
use winvoice_schema::{
	chrono::{DateTime, Utc},
	Employee,
	Job,
	Money,
	Timesheet,
};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Timesheet`]s.
#[async_trait::async_trait]
pub trait TimesheetAdapter:
	Deletable<Entity = Timesheet>
	+ Retrievable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity, Match = MatchTimesheet>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Timesheet`] via the `connection`. Will not
	/// [`commit`](Transaction::commit) changes.
	///
	/// # Parameters
	///
	/// * `expenses` is a list of `(String, Money, String)`, which represents `(category, cost, description)` for the
	///   created [`Expense`]s.
	async fn create(
		connection: &mut Transaction<<Self as Deletable>::Db>,
		employee: Employee,
		expenses: Vec<(String, Money, String)>,
		job: Job,
		time_begin: DateTime<Utc>,
		time_end: Option<DateTime<Utc>>,
		work_notes: String,
	) -> Result<<Self as Deletable>::Entity>;
}
