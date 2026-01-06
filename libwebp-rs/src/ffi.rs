#[unsafe(no_mangle)]
pub unsafe extern "C" 
fn SharpYuvConvert(
  r_ptr: *const u8,
  g_ptr: *const u8,
  b_ptr: *const u8,
  rgb_step: i32,
  rgb_stride: i32,
  rgb_bit_depth: i32,
  y_ptr: *mut u8,
  y_stride: i32,
  u_ptr: *mut u8,
  u_stride: i32,
  v_ptr: *mut u8,
  v_stride: i32,
  yuv_bit_depth: i32,
  width: i32,
  height: i32,
  yuv_matrix: *const crate::sharpyuv::SharpYuvConversionMatrix
) ->
    i32
{
    let r = unsafe { std::slice::from_raw_parts(r_ptr, (height*rgb_stride + width) as usize) };
    let g = unsafe { std::slice::from_raw_parts(g_ptr, (height*rgb_stride + width) as usize) };
    let b = unsafe { std::slice::from_raw_parts(b_ptr, (height*rgb_stride + width) as usize) };
    let sub_len = |stride| {
        let h = height as f64;
        let s = stride as f64;
        let w = width as f64;
        ((h/2f64)*s+w/2f64).ceil() as usize
    };
    let y = unsafe { std::slice::from_raw_parts_mut(y_ptr, (height*y_stride + width) as usize) };
    let u = unsafe { std::slice::from_raw_parts_mut(u_ptr, sub_len(u_stride)) };
    let v = unsafe { std::slice::from_raw_parts_mut(v_ptr, sub_len(v_stride)) };
    let yuv_matrix = unsafe { std::slice::from_raw_parts(yuv_matrix, 1) };
    crate::sharpyuv::SharpYuvConvert(
      r,
      g,
      b,
      rgb_step,
      rgb_stride,
      rgb_bit_depth,
      y,
      y_stride,
      u,
      u_stride,
      v,
      v_stride,
      yuv_bit_depth,
      width,
      height,
      yuv_matrix,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" 
fn SharpYuvInit() { crate::sharpyuv::SharpYuvInit() }

#[unsafe(no_mangle)]
pub unsafe extern "C" 
fn SharpYuvGetVersion() -> i32 { crate::sharpyuv::SharpYuvGetVersion() }

#[unsafe(no_mangle)]
pub unsafe extern "C" 
fn SharpYuvGetConversionMatrix (matrix_type: crate::sharpyuv::SharpYuvMatrixType) -> *const crate::sharpyuv::SharpYuvConversionMatrix {
    crate::sharpyuv::SharpYuvGetConversionMatrix(matrix_type).as_ptr()
}
