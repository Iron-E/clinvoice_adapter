//! This crate provides traits which can are used to provide an abstraction for CLInvoice frontends
//! to enable the option of different types of storage facilities (e.g. Postgres vs MySQL)
//!
//! Contains various tools, such as those to [generate](clinvoice_adapter::fmt) SQL and [reference
//! table columns](clinvoice_adapter::schema::columns).
//!
//! # Features
//!
//! * `serde` enables the use of [`serde`] with the types in this crate.
//! * `sqlx_runtime_tokio_rustls` enables [`sqlx`]'s `runtime-tokio-rustls` feature, which is only
//!   set so that the project can be built on its own.
//!   * __This crate should be compiled with__ `--no-default-features`.
//!
//! # Usage
//!
//! If you are looking to create a new adapter:
//!
//! 1. Create newtypes for each trait in [`clinvoice_adapter::schema`].
//! 2. Implement each newtype's corresponding `Adapter` trait.
//! 3. Add a new variant in [`Adapters`][adapters].
//! 4. Create a new feature flag for the adapter in the corresponding frontend you want to support
//!    the new adapter.
//! 5. Write add new `match` arms in areas that `match` on [`Adapters`][adapters].
//!
//! [adapters]: clinvoice_config::Adapters

#![allow(clippy::drop_non_drop)]
#![forbid(unsafe_code)]
#![warn(
	missing_docs,
	clippy::alloc_instead_of_core,
	clippy::allow_attributes_without_reason,
	clippy::as_underscore,
	clippy::branches_sharing_code,
	clippy::cast_lossless,
	clippy::checked_conversions,
	clippy::cloned_instead_of_copied,
	clippy::dbg_macro,
	clippy::debug_assert_with_mut_call,
	clippy::doc_link_with_quotes,
	clippy::doc_markdown,
	clippy::empty_line_after_outer_attr,
	clippy::empty_structs_with_brackets,
	clippy::enum_glob_use,
	clippy::equatable_if_let,
	clippy::exit,
	clippy::explicit_into_iter_loop,
	clippy::explicit_iter_loop,
	clippy::fallible_impl_from,
	clippy::filetype_is_file,
	clippy::filter_map_next,
	clippy::flat_map_option,
	clippy::fn_to_numeric_cast_any,
	clippy::format_push_string,
	clippy::from_iter_instead_of_collect,
	clippy::get_unwrap,
	clippy::implicit_clone,
	clippy::inefficient_to_string,
	clippy::items_after_statements,
	clippy::manual_assert,
	clippy::manual_ok_or,
	clippy::map_unwrap_or,
	clippy::match_same_arms,
	clippy::missing_const_for_fn,
	clippy::missing_panics_doc,
	clippy::multiple_inherent_impl,
	clippy::mut_mut,
	clippy::needless_continue,
	clippy::option_if_let_else,
	clippy::option_option,
	clippy::range_minus_one,
	clippy::range_plus_one,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_else,
	clippy::ref_binding_to_reference,
	clippy::ref_option_ref,
	clippy::same_functions_in_if_condition,
	clippy::single_char_lifetime_names,
	clippy::std_instead_of_core,
	clippy::str_to_string,
	clippy::string_add,
	clippy::string_add_assign,
	clippy::string_to_string,
	clippy::try_err,
	clippy::unnecessary_join,
	clippy::unnecessary_wraps,
	clippy::use_self,
	clippy::used_underscore_binding,
	clippy::wildcard_imports
)]

mod deletable;
pub mod fmt;
mod initializable;
mod retrievable;
pub mod schema;
mod updatable;
mod write_where_clause;

pub use deletable::Deletable;
pub use initializable::Initializable;
pub use retrievable::Retrievable;
pub use updatable::Updatable;
pub use write_where_clause::{WriteContext, WriteWhereClause};
