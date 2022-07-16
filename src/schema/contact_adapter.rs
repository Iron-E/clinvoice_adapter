use clinvoice_match::MatchContact;
use clinvoice_schema::{Contact, ContactKind};
use sqlx::{Executor, Result};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Contact`]s.
#[async_trait::async_trait]
pub trait ContactAdapter:
	Deletable<Entity = Contact>
	+ Retrievable<
		Db = <Self as Deletable>::Db,
		Entity = <Self as Deletable>::Entity,
		Match = MatchContact,
	> + Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize all of the [`Contact`]s in `contact_info` via the `connection`.
	///
	/// If you want to update an existing [`Contact`] instead, try [`Updatable::update`].
	async fn create<'c, TConn>(
		connection: TConn,
		kind: ContactKind,
		name: String,
	) -> Result<<Self as Deletable>::Entity>
	where
		TConn: Executor<'c, Database = <Self as Deletable>::Db>;
}
