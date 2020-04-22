#![warn(clippy::pedantic)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::unimplemented)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]

pub mod cli;
pub mod error;
pub(crate) mod style;
pub mod stylizer;
pub mod token;
