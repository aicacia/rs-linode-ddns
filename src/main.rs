use std::io;

use clap::Parser;
use linode_ddns::cli::{opt::Opt, run::run};

#[tokio::main]
async fn main() -> io::Result<()> {
  run(Opt::parse()).await?;
  Ok(())
}
