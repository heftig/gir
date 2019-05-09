#![allow(clippy::let_and_return)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::write_literal)]

#[macro_use]
extern crate bitflags;
extern crate git2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
extern crate stripper_lib;
extern crate toml;
extern crate xml;

/// Log warning only if type in generated library
macro_rules! warn_main {
    ($tid: expr, $target:expr, $($arg:tt)*) => (
        if $tid.ns_id == crate::library::MAIN_NAMESPACE {
            warn!($target, $($arg)*);
        }
    );
}

//generated by build.rs
mod gir_version;

pub mod analysis;
mod case;
mod chunk;
mod codegen;
mod config;
mod consts;
mod custom_type_glib_priority;
mod env;
mod file_saver;
mod git;
pub mod library;
mod library_postprocessing;
mod library_preprocessing;
mod nameutil;
mod parser;
mod traits;
pub mod update_version;
mod version;
mod visitors;
mod writer;
mod xmlparser;

pub use crate::analysis::class_hierarchy::run as class_hierarchy_run;
pub use crate::analysis::namespaces::run as namespaces_run;
pub use crate::analysis::run as analysis_run;
pub use crate::analysis::symbols::run as symbols_run;
pub use crate::codegen::generate as codegen_generate;
pub use crate::config::{Config, WorkMode};
pub use crate::env::Env;
pub use crate::library::Library;
