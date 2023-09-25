mod display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Formats an SQL `COALESCE` expression.
///
/// # Warnings
///
/// * `Left` and `Right`'s [`to_string`](ToString::to_string) output be non-empty to format correctly.
///
/// # Example
///
/// ```rust
/// use winvoice_adapter::fmt::{NullIf, WithIdentifier};
/// # use pretty_assertions::assert_eq;
///
/// assert_eq!(NullIf(WithIdentifier("foo", "a"), "3").to_string(), "NULLIF(foo.a, 3)");
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NullIf<Left, Right>(pub Left, pub Right);
