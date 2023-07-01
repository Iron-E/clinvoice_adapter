use sqlx::{Executor, Result};
use winvoice_match::MatchOrganization;
use winvoice_schema::{Location, Organization};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Organization`]s.
#[async_trait::async_trait]
pub trait OrganizationAdapter:
	Deletable<Entity = Organization>
	+ Retrievable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity, Match = MatchOrganization>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Organization`] via the `connection`.
	async fn create<'connection, Conn>(
		connection: Conn,
		location: Location,
		name: String,
	) -> Result<<Self as Deletable>::Entity>
	where
		Conn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
