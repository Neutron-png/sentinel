mod app;
mod commands;
mod db;
mod events;
mod history;
mod intercept;
mod knowledge;
mod methodology;
mod models;
mod network;
mod plugins;
mod proxy;
mod reports;
mod router;
mod scope;
mod screens;
mod services;
mod state;
mod ui;
mod validation;

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
