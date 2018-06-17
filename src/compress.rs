use std::fs;
use std::io::{self, Write};
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};
use failure::Error;

use ffi;

pub fn compress<P: AsRef<Path>>(file: P) -> Result<(), Error> {
  let data = fs::read(file)?;
  let compressed = ffi::compress(&data);

  let stdout = io::stdout();
  let mut stdout = stdout.lock();
  stdout.write(b"mozLz40\0")?;
  stdout.write_u32::<LittleEndian>(data.len() as u32)?;
  stdout.write(&compressed)?;

  Ok(())
}

