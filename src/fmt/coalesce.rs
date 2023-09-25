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
/// use winvoice_adapter::fmt::{Coalesce, WithIdentifier};
/// # use pretty_assertions::assert_eq;
///
/// assert_eq!(Coalesce(WithIdentifier("foo", "a"), "3").to_string(), "COALESCE(foo.a, 3)");
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coalesce<Left, Right>(pub Left, pub Right);
