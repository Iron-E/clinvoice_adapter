use sqlx::{Executor, Result};
use winvoice_match::MatchContact;
use winvoice_schema::{Contact, ContactKind};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Contact`]s.
#[async_trait::async_trait]
pub trait ContactAdapter:
	Deletable<Entity = Contact>
	+ Retrievable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity, Match = MatchContact>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize all of the [`Contact`]s in `contact_info` via the `connection`.
	///
	/// If you want to update an existing [`Contact`] instead, try [`Updatable::update`].
	async fn create<'connection, Conn>(
		connection: Conn,
		kind: ContactKind,
		name: String,
	) -> Result<<Self as Deletable>::Entity>
	where
		Conn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
