use clinvoice_match::MatchEmployee;
use clinvoice_schema::Employee;
use sqlx::{Executor, Result};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Employee`]s.
#[async_trait::async_trait]
pub trait EmployeeAdapter:
	Deletable<Entity = Employee>
	+ Retrievable<
		Db = <Self as Deletable>::Db,
		Entity = <Self as Deletable>::Entity,
		Match = MatchEmployee,
	> + Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Employee`] via the `connection`.
	async fn create<'connection, TConn>(
		connection: TConn,
		name: String,
		status: String,
		title: String,
	) -> Result<<Self as Deletable>::Entity>
	where
		TConn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
