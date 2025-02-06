extern crate core;

pub mod api;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod server;
pub mod service;

pub use api::dto;
pub use domain::{errors, models};
pub use server::run_server;
pub use service::ShortenerService;
