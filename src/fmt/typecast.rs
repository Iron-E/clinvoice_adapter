mod display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Formats an SQL `CAST` expression.
///
/// # Warnings
///
/// * `Cast`'s and `Column`'s [`to_string`](ToString::to_string) output be non-empty to format
///   correctly.
///
/// # Example
///
/// ```rust
/// use winvoice_adapter::fmt::TypeCast;
/// # use pretty_assertions::assert_eq;
///
/// assert_eq!(TypeCast("foo.a", "numeric").to_string(), " CAST (foo.a AS numeric)");
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeCast<Column, Cast>(pub Column, pub Cast);
