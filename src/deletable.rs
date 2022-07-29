use sqlx::{Database, Executor, Result};

/// Implementors of this trait are capable of deleting values of type [`Deletable::Entity`] being
/// stored on a [`Deletable::Db`].
#[async_trait::async_trait]
pub trait Deletable
{
	/// The [`Database`] that is housing the values of type [`Deletable::Entitiy`].
	type Db: Database;

	/// The type of data that is to be [`delete`](Deletable::delete)d.
	type Entity;

	/// Send instruction over the `connection` to delete some `entities`.
	///
	/// # Errors
	///
	/// * If any [`Self::Entity`] in `entities` does not exist over the `connection`.
	async fn delete<'connection, 'entity, TConn, TIter>(
		connection: TConn,
		entities: TIter,
	) -> Result<()>
	where
		Self::Entity: 'entity,
		TConn: Executor<'connection, Database = Self::Db>,
		TIter: Iterator<Item = &'entity Self::Entity> + Send;
}
