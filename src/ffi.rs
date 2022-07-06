use libc::c_int;

use eyre::{Result, eyre};

#[link(name="lz4")]
extern "C" {
  fn LZ4_decompress_safe(
    src: *const u8, dst: *mut u8,
    compressedSize: c_int, dstCapacity: c_int,
  ) -> c_int;

  fn LZ4_compress_default(
    src: *const u8, dst: *mut u8,
    srcSize: c_int, dstCapacity: c_int,
  ) -> c_int;

  fn LZ4_compressBound(inputSize: c_int) -> c_int;
}

pub fn decompress(src: &[u8], dst_size: u32) -> Result<Vec<u8>> {
  let mut buffer = Vec::with_capacity(dst_size as usize);
  let n = unsafe {
    buffer.set_len(dst_size as usize);
    LZ4_decompress_safe(
      src.as_ptr(), buffer.as_mut_ptr(),
      src.len() as i32, dst_size as i32,
    )
  };
  if n < 0 {
    Err(eyre!("LZ4 returned error code {}", n))
  } else {
    Ok(buffer)
  }
}

pub fn compress(src: &[u8]) -> Vec<u8> {
  let dst_size = unsafe { LZ4_compressBound(src.len() as i32) };
  let mut buffer = Vec::with_capacity(dst_size as usize);
  unsafe {
    let n = LZ4_compress_default(
      src.as_ptr(), buffer.as_mut_ptr(),
      src.len() as i32, dst_size as i32,
    );
    buffer.set_len(n as usize);
  }
  buffer
}
