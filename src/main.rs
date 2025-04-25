#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

use crate::app::config::start::salvo_application_start;

mod app;
mod shared;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    salvo_application_start().await;
}
