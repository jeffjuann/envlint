use std::{io, process};

mod cli;

mod commands;

mod macros;

mod utils;

mod common;

mod parse;

mod file;

fn main() -> io::Result<()> 
{    
  let result = cli::run();

  match result
  {
    Ok(_) =>
    {
      process::exit(0);
    },
    Err(e) => 
    {
      emit_error!("{}", e);
    }
  }
}