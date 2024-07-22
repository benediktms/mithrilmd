#![deny(
    unreachable_pub,
    missing_debug_implementations,
    rustdoc::broken_intra_doc_links,
    clippy::all,
    clippy::perf,
    clippy::pedantic,
    clippy::fn_to_numeric_cast_any
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::missing_panics_doc,
    clippy::used_underscore_binding
)]

pub mod repository;
pub mod startup;
pub mod state;
pub mod system_tray;
pub mod util;
