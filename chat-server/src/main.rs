mod service;
mod entity;
mod dto;
mod message;
mod manager;
mod shared_state;
mod repository;
mod config;
mod app_runner;

mod auth;

use dotenv::dotenv;
use crate::app_runner::AppRunner;
use crate::config::Config;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let config = Config::init();

    AppRunner::run(config).await;
}