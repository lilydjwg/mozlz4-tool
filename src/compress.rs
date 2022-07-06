use std::fs;
use std::io::{self, Write};
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};
use eyre::Result;

use crate::ffi;

pub fn compress<P: AsRef<Path>>(file: P) -> Result<()> {
  let data = fs::read(file)?;
  let compressed = ffi::compress(&data);

  let stdout = io::stdout();
  let mut stdout = stdout.lock();
  stdout.write_all(b"mozLz40\0")?;
  stdout.write_u32::<LittleEndian>(data.len() as u32)?;
  stdout.write_all(&compressed)?;

  Ok(())
}

