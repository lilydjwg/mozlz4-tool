mod ffi;
mod cat;
mod compress;

use std::ffi::OsString;
use eyre::Result;
use clap::{Command, Arg, ArgGroup, ArgAction};

fn main() -> Result<()> {
  let matches = Command::new("mozlz4 tools")
    .arg(Arg::new("compress")
         .short('c')
         .action(ArgAction::SetTrue)
         .help("compress a file"))
    .arg(Arg::new("decompress")
         .short('d')
         .action(ArgAction::SetTrue)
         .help("decompress a file (default)"))
    .arg(Arg::new("file")
         .required(true))
    .group(ArgGroup::new("action")
           .args(&["compress", "decompress"]))
    .get_matches();

  if matches.get_flag("compress") {
    compress::compress(matches.get_one::<OsString>("file").unwrap())?;
  } else {
    cat::cat(matches.get_one::<OsString>("file").unwrap())?;
  }

  Ok(())
}
