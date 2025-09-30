use std::{io, time::Instant};
use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[command(
  name = env!("CARGO_PKG_NAME"),
  bin_name = env!("CARGO_PKG_NAME"),
  version = env!("CARGO_PKG_VERSION"),
  about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Cli 
{
  #[command(subcommand)]
  command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands 
{
  Lint(LintFlags),
}

#[derive(Parser, Debug)]
pub struct LintFlags
{
  #[arg(
    short, 
    long,
    default_value = ".env",
    help = "The .env file to lint"
  )]
  pub file: Option<String>,

  #[arg(
    short, 
    long,
    default_value = ".env.template",
    help = "The template file to use"
  )]
  pub template: Option<String>,
}

pub fn run() -> io::Result<()> 
{
  let cli = Cli::parse();
  
  match cli.command
  {
    Some(Subcommands::Lint(flags)) =>
    {
      let start_time = Instant::now();
      commands::lint::lint(&flags)?;
      println!("elapsed in: {:?}", start_time.elapsed());
    },
    None =>
    {
      println!("Welcome to {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
      println!("No command provided. Use --help to see available commands.");
    }
  }
  return Ok(());
}