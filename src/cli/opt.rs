use clap::Parser;
#[cfg(feature = "completions")]
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct Opt {
  #[arg(long, short = 'c', default_value = "./config.json")]
  pub config: String,
  #[clap(subcommand)]
  pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
  Run {
    #[clap(long, short = 'd')]
    domains: Option<Vec<String>>,
  },
  Watch {
    #[clap(long, short = 'd')]
    domains: Option<Vec<String>>,
    #[clap(long = "interval-in-seconds", short = 'i', default_value = "3600")]
    interval_in_seconds: u64,
  },

  #[cfg(feature = "completions")]
  Completions { shell: Shell },
}
