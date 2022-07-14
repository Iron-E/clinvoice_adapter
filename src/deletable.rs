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
	async fn delete<'c, 'e, 'i, TConn, TIter>(connection: TConn, entities: TIter) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
		TConn: Executor<'c, Database = Self::Db>,
		TIter: Iterator<Item = &'i Self::Entity> + Send;
}
