use sqlx::{Executor, Result};
use winvoice_match::MatchEmployee;
use winvoice_schema::Employee;

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
	async fn create<'connection, Conn>(
		connection: Conn,
		name: String,
		status: String,
		title: String,
	) -> Result<<Self as Deletable>::Entity>
	where
		Conn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
