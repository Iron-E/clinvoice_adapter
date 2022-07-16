use clinvoice_match::MatchOrganization;
use clinvoice_schema::{Location, Organization};
use sqlx::{Executor, Result};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Organization`]s.
#[async_trait::async_trait]
pub trait OrganizationAdapter:
	Deletable<Entity = Organization>
	+ Retrievable<
		Db = <Self as Deletable>::Db,
		Entity = <Self as Deletable>::Entity,
		Match = MatchOrganization,
	> + Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Organization`] via the `connection`.
	async fn create<'c, TConn>(
		connection: TConn,
		location: Location,
		name: String,
	) -> Result<<Self as Deletable>::Entity>
	where
		TConn: Executor<'c, Database = <Self as Deletable>::Db>;
}
