use libc::c_int;

use failure::Error;

#[link(name="lz4")]
extern "C" {
  fn LZ4_decompress_safe(
    src: *const u8, dst: *mut u8,
    compressedSize: c_int, dstCapacity: c_int,
  ) -> c_int;
}

pub fn decompress(src: &[u8], dst_size: u32) -> Result<Vec<u8>, Error> {
  let mut buffer = Vec::with_capacity(dst_size as usize);
  let n = unsafe {
    buffer.set_len(dst_size as usize);
    LZ4_decompress_safe(
      src.as_ptr(), buffer.as_mut_ptr(),
      src.len() as i32, dst_size as i32,
    )
  };
  if n < 0 {
    Err(format_err!("LZ4 returned error code {}", n))
  } else {
    Ok(buffer)
  }
}
