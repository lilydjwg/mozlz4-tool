#[macro_use(format_err)] extern crate failure;
extern crate libc;
extern crate byteorder;

use std::env::args;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt};

mod ffi;

use failure::Error;

fn main2() -> Result<(), Error> {
  let file = args().nth(1).expect("mozlz4 filename");
  let f = File::open(file)?;
  let mut f = BufReader::new(f);
  let mut buffer = [0; 4096];

  let n = f.read(&mut buffer[..8])?;
  if n != 8 || &buffer[..8] != b"mozLz40\0" {
    return Err(format_err!("bad header: {:?}", &buffer[..n]));
  }

  let dst_size = f.read_u32::<LittleEndian>()?;

  let mut input = vec![];
  f.read_to_end(&mut input)?;

  let output = ffi::decompress(&input, dst_size)?;

  let stdout = io::stdout();
  let mut stdout = stdout.lock();
  stdout.write(&output)?;

  Ok(())
}

fn main() {
  if let Err(e) = main2() {
    eprintln!("Error: {:?}", e);
  }
}
