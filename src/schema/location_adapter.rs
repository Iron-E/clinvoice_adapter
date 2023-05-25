use sqlx::{Executor, Result};
use winvoice_match::MatchLocation;
use winvoice_schema::Location;

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Location`]s.
#[async_trait::async_trait]
pub trait LocationAdapter:
	Deletable<Entity = Location>
	+ Retrievable<
		Db = <Self as Deletable>::Db,
		Entity = <Self as Deletable>::Entity,
		Match = MatchLocation,
	> + Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Location`] via the `connection`.
	async fn create<'connection, Conn>(
		connection: Conn,
		name: String,
		outer: Option<<Self as Deletable>::Entity>,
	) -> Result<<Self as Deletable>::Entity>
	where
		Conn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
