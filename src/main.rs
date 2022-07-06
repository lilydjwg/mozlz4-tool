mod ffi;
mod cat;
mod compress;

use eyre::Result;
use clap::{App, Arg, ArgGroup};

fn main() -> Result<()> {
  let matches = App::new("mozlz4 tools")
    .arg(Arg::with_name("compress")
         .short('c')
         .help("compress a file"))
    .arg(Arg::with_name("decompress")
         .short('d')
         .help("decompress a file (default)"))
    .arg(Arg::with_name("file")
         .required(true))
    .group(ArgGroup::with_name("action")
           .args(&["compress", "decompress"]))
    .get_matches();

  if matches.is_present("compress") {
    compress::compress(matches.value_of_os("file").unwrap())?;
  } else {
    cat::cat(matches.value_of_os("file").unwrap())?;
  }

  Ok(())
}
