mod display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Formats a dot-access of `Column`.
///
/// # Warnings
///
/// * `Column`'s and `Ident`'s [`to_string`](ToString::to_string) output be non-empty to format
///   correctly.
///
/// # Example
///
/// ```rust
/// use clinvoice_adapter::fmt::WithIdentifier;
/// # use pretty_assertions::assert_eq;
///
/// assert_eq!(WithIdentifier("foo", "a").to_string(), "foo.a");
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithIdentifier<Ident, Column>(pub Ident, pub Column);
