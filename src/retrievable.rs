use sqlx::{Database, Pool, Result};

/// Implementors of this trait are capable of being retrieved from a [`Database`].
#[async_trait::async_trait]
pub trait Retrievable
{
	/// The [`Database`] where data of type [`Updatable::Entity`] is being stored.
	type Db: Database;

	/// The type of data that is to be [`update`](Deletable::update)d.
	type Entity;

	/// The type used for [match](winvoice_match)ing.
	type Match;

	/// Retrieve all [`Contact`]s (via `connection`) that match the `match_condition`.
	async fn retrieve(connection: &Pool<Self::Db>, match_condition: Self::Match) -> Result<Vec<Self::Entity>>;
}
