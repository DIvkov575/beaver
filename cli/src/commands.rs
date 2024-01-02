use std::io;
use anyhow::Result;
use clap::{self, Parser};

mod deploy;
use deploy::deploy;
mod destroy;
use destroy::destroy;
mod init;
use init::init;

mod tmp;
use tmp::tmp;

#[derive(Parser, Debug)]
pub enum Command {
    #[command(about="Create a config")]
    Init,
    #[command(about="Create Beaver instance on GCP")]
    Deploy,
    #[command(about="Destroy Beaver instance")]
    Destroy,
}
impl Command {
    pub fn run(self) -> Result<()> {
        use Command::*;
        match self {
            Init => init(),
            Deploy => deploy(),
            Destroy => destroy(),
            _ => {print!("asldkfj"); return Ok(());}
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = None,
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}
impl Args {
    pub fn run(self) -> Result<()> {
        match self.command {
            None => print!("No Command was passed"),
            Some(command) => command.run()?,
        }
        Ok(())
    }
}