mod display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Formats an SQL `AS` clause.
///
/// # Warnings
///
/// * `TAs`'s and `TIdent`'s [`to_string`](ToString::to_string) output be non-empty to format
///   correctly.
///
/// # Example
///
/// ```rust
/// use winvoice_adapter::fmt::{As, WithIdentifier};
/// # use pretty_assertions::assert_eq;
///
/// assert_eq!(As(WithIdentifier("foo", "a"), "MyAlias").to_string(), "foo.a AS MyAlias");
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct As<TIdent, TAs>(pub TIdent, pub TAs);
