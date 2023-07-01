use sqlx::{Executor, Result};
use winvoice_match::MatchDepartment;
use winvoice_schema::Department;

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Department`]s.
#[async_trait::async_trait]
pub trait DepartmentAdapter:
	Deletable<Entity = Department>
	+ Retrievable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity, Match = MatchDepartment>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Department`] via the `connection`.
	async fn create<'connection, Conn>(connection: Conn, name: String) -> Result<<Self as Deletable>::Entity>
	where
		Conn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
