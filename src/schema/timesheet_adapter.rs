use clinvoice_match::MatchTimesheet;
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Employee,
	Job,
	Timesheet,
};
use sqlx::{Acquire, Pool, Result};

use crate::{Deletable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Timesheet`]s.
#[async_trait::async_trait]
pub trait TimesheetAdapter:
	Deletable<Entity = Timesheet>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Timesheet`] via the `connection`.
	async fn create<'c, TConn>(
		connection: TConn,
		employee: Employee,
		job: Job,
		time_begin: DateTime<Utc>,
		time_end: Option<DateTime<Utc>>,
	) -> Result<<Self as Deletable>::Entity>
	where
		TConn: Acquire<'c, Database = <Self as Deletable>::Db>;

	/// Retrieve all [`Timesheet`]s (via `connection`) that match the `match_condition`.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchTimesheet,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
