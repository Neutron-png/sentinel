mod app;
mod auth;
mod browser;
mod commands;
mod crawler;
mod db;
mod events;
mod evidence;
mod findings;
mod graphql;
mod grpc;
mod har;
mod history;
mod intelligence;
mod intercept;
mod jwt;
mod knowledge;
mod methodology;
mod models;
mod network;
mod oauth;
mod openapi;
mod orchestrator;
mod plugins;
mod policy;
mod proxy;
mod repeater;
mod reporting;
mod reports;
mod resources;
mod router;
mod scanner;
mod scope;
mod screens;
mod sdk;
mod services;
mod session;
mod sitemap;
mod state;
mod ui;
mod validation;
mod websocket;
mod workflow;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "sentinel",
    about = "A professional web application security assessment workflow tool"
)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    app::App::run()?;
    Ok(())
}
