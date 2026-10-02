use clap::Parser;

#[derive(Parser)]
#[command(
    name = "sentinel",
    about = "A professional web application security assessment workflow tool"
)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    sentinel::app::App::run()?;
    Ok(())
}
