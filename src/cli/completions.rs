use std::io;

use clap::CommandFactory;
use clap_complete::{generate, Shell};

use super::opt::Command;

pub fn run(shell: Shell) {
  generate(
    shell,
    &mut Command::command(),
    "linode-ddns",
    &mut io::stdout(),
  )
}
