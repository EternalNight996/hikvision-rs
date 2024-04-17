//! HIKVISION-RS
#![allow(
  clippy::cognitive_complexity,
  clippy::large_enum_variant,
  clippy::module_inception,
  clippy::needless_doctest_main
)]
#![warn(
  missing_debug_implementations,
  missing_docs,
  rust_2018_idioms,
  unreachable_pub
)]
#![deny(unused_must_use)]
#![doc(test(
  no_crate_inject,
  attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![cfg_attr(loom, allow(dead_code, unreachable_pub))]

pub mod core;
pub use core::*;
pub mod utils;
