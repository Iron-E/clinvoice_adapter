use core::time::Duration;
use std::collections::BTreeSet;

use sqlx::{Executor, Result};
use winvoice_match::MatchJob;
use winvoice_schema::{
	chrono::{DateTime, Utc},
	Department,
	Invoice,
	Job,
	Organization,
};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Job`]s.
#[async_trait::async_trait]
pub trait JobAdapter:
	Deletable<Entity = Job>
	+ Retrievable<
		Db = <Self as Deletable>::Db,
		Entity = <Self as Deletable>::Entity,
		Match = MatchJob,
	> + Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Job`] via the `connection`.
	#[allow(clippy::too_many_arguments)]
	async fn create<'connection, Conn>(
		connection: Conn,
		client: Organization,
		date_close: Option<DateTime<Utc>>,
		date_open: DateTime<Utc>,
		departments: BTreeSet<Department>,
		increment: Duration,
		invoice: Invoice,
		notes: String,
		objectives: String,
	) -> Result<<Self as Deletable>::Entity>
	where
		Conn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
