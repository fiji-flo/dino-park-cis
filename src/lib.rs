#![warn(clippy::all)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate failure_derive;

pub mod api;
pub mod db;
pub mod error;
pub mod healthz;
pub mod keys;
pub mod profile;
pub mod settings;
