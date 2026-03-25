#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(static_mut_refs)]

#[derive(PartialEq, Clone, Copy)]
pub enum CPUFeature
{
  kSSE2,
  kSSE3,
  kSlowSSSE3,
  kSSE4_1,
  kAVX,
  kAVX2,
  kNEON,
  kMIPS32,
  kMIPSdspR2,
  kMSA
}

pub fn ConvertWRGBToYUV(
  mut best_y: &[u16],
  mut best_uv: &[i16],
  mut y_ptr: &mut [u8],
  y_stride: i32,
  mut u_ptr: &mut [u8],
  u_stride: i32,
  mut v_ptr: &mut [u8],
  v_stride: i32,
  rgb_bit_depth: i32,
  yuv_bit_depth: i32,
  width: i32,
  height: i32,
  yuv_matrix: &[SharpYuvConversionMatrix]
) ->
    i32
{
  let mut i: i32;
  let mut j: i32;
  let best_uv_base: (&[i16], &[i16]) = best_uv.split_at(0usize);
  let w: i32 = width.wrapping_add(1i32) & ! 1i32;
  let h: i32 = height.wrapping_add(1i32) & ! 1i32;
  let uv_w: i32 = w.wrapping_shr(1u32);
  let uv_h: i32 = h.wrapping_shr(1u32);
  let sfix: i32 = GetPrecisionShift(rgb_bit_depth);
  let yuv_max: i32 = 1i32.wrapping_shl(yuv_bit_depth as u32).wrapping_sub(1i32);
  best_uv = best_uv_base.1;
  j = 0i32;
  loop
  {
    {
      i = 0i32;
      loop
      {
        {
          let off: i32 = i.wrapping_shr(1u32);
          let W: i32 = best_y[i as usize] as i32;
          let r: i32 =
              (best_uv[off.wrapping_add(0i32.wrapping_mul(uv_w)) as usize] as i32).wrapping_add(W);
          let g: i32 =
              (best_uv[off.wrapping_add(1i32.wrapping_mul(uv_w)) as usize] as i32).wrapping_add(W);
          let b: i32 =
              (best_uv[off.wrapping_add(2i32.wrapping_mul(uv_w)) as usize] as i32).wrapping_add(W);
          let y: i32 = RGBToYUVComponent(r, g, b, (yuv_matrix[0usize]).rgb_to_y, sfix);
          if yuv_bit_depth <= 8i32
          { y_ptr[i as usize] = clip_8b(y as i16) }
          else
          { crate::scylla_glue::scylla_u16_of_u8_mut(y_ptr)[i as usize] = clip(y, yuv_max) }
        };
        if
        {
          i = i.wrapping_add(1i32);
          i
        }
        >=
        width
        { break }
      };
      best_y = &best_y[w as usize..];
      best_uv = &best_uv[(j & 1i32).wrapping_mul(3i32).wrapping_mul(uv_w) as usize..];
      y_ptr = &mut y_ptr[y_stride as usize..]
    };
    if
    {
      j = j.wrapping_add(1i32);
      j
    }
    >=
    height
    { break }
  };
  best_uv = best_uv_base.1;
  j = 0i32;
  loop
  {
    {
      i = 0i32;
      loop
      {
        {
          let r: i32 = best_uv[i.wrapping_add(0i32.wrapping_mul(uv_w)) as usize] as i32;
          let g: i32 = best_uv[i.wrapping_add(1i32.wrapping_mul(uv_w)) as usize] as i32;
          let b: i32 = best_uv[i.wrapping_add(2i32.wrapping_mul(uv_w)) as usize] as i32;
          let u: i32 = RGBToYUVComponent(r, g, b, (yuv_matrix[0usize]).rgb_to_u, sfix);
          let v: i32 = RGBToYUVComponent(r, g, b, (yuv_matrix[0usize]).rgb_to_v, sfix);
          if yuv_bit_depth <= 8i32
          {
            u_ptr[i as usize] = clip_8b(u as i16);
            v_ptr[i as usize] = clip_8b(v as i16)
          }
          else
          {
            crate::scylla_glue::scylla_u16_of_u8_mut(u_ptr)[i as usize] = clip(u, yuv_max);
            crate::scylla_glue::scylla_u16_of_u8_mut(v_ptr)[i as usize] = clip(v, yuv_max)
          }
        };
        if
        {
          i = i.wrapping_add(1i32);
          i
        }
        >=
        uv_w
        { break }
      };
      best_uv = &best_uv[3i32.wrapping_mul(uv_w) as usize..];
      u_ptr = &mut u_ptr[u_stride as usize..];
      v_ptr = &mut v_ptr[v_stride as usize..]
    };
    if
    {
      j = j.wrapping_add(1i32);
      j
    }
    >=
    uv_h
    { break }
  };
  return 1i32
}

pub fn DoSharpArgbToYuv(
  mut r_ptr: &[u8],
  mut g_ptr: &[u8],
  mut b_ptr: &[u8],
  rgb_step: i32,
  rgb_stride: i32,
  rgb_bit_depth: i32,
  y_ptr: &mut [u8],
  y_stride: i32,
  u_ptr: &mut [u8],
  u_stride: i32,
  v_ptr: &mut [u8],
  v_stride: i32,
  yuv_bit_depth: i32,
  width: i32,
  height: i32,
  yuv_matrix: &[SharpYuvConversionMatrix],
  transfer_type: SharpYuvTransferFunctionType
) ->
    i32
{
  let w: i32 = width.wrapping_add(1i32) & ! 1i32;
  let h: i32 = height.wrapping_add(1i32) & ! 1i32;
  let uv_w: i32 = w.wrapping_shr(1u32);
  let uv_h: i32 = h.wrapping_shr(1u32);
  let y_bit_depth: i32 = rgb_bit_depth.wrapping_add(GetPrecisionShift(rgb_bit_depth));
  let mut prev_diff_y_sum: u64 = ! 0i32 as u64;
  let mut j: i32;
  let mut iter: i32;
  let tmp_buffer_size: u64 = (w as u64).wrapping_mul(3u64).wrapping_mul(2u64);
  let best_y_base_size: u64 = (w as u64).wrapping_mul(h as u64);
  let target_y_base_size: u64 = (w as u64).wrapping_mul(h as u64);
  let best_rgb_y_size: u64 = (w as u64).wrapping_mul(2u64);
  let best_uv_base_size: u64 = (uv_w as u64).wrapping_mul(3u64).wrapping_mul(uv_h as u64);
  let target_uv_base_size: u64 = (uv_w as u64).wrapping_mul(3u64).wrapping_mul(uv_h as u64);
  let best_rgb_uv_size: u64 = (uv_w as u64).wrapping_mul(3u64);
  let mut tmp_buffer: Box<[u16]> =
      vec![0u16;
          tmp_buffer_size.wrapping_add(best_y_base_size).wrapping_add(target_y_base_size).wrapping_add(
            best_rgb_y_size
          ).wrapping_add(
            best_uv_base_size.wrapping_add(target_uv_base_size).wrapping_add(best_rgb_uv_size)
          )
          as
          usize].into_boxed_slice();
  let tmp_buffer0: (&mut [u16], &mut [u16]) = tmp_buffer.split_at_mut(0usize);
  let best_y_base: (&mut [u16], &mut [u16]) =
      (tmp_buffer0.1).split_at_mut(tmp_buffer_size as usize);
  let target_y_base: (&mut [u16], &mut [u16]) =
      (best_y_base.1).split_at_mut(
        tmp_buffer_size.wrapping_add(best_y_base_size) as usize - tmp_buffer_size as usize
      );
  let best_rgb_y: (&mut [u16], &mut [u16]) =
      (target_y_base.1).split_at_mut(
        tmp_buffer_size.wrapping_add(best_y_base_size).wrapping_add(target_y_base_size) as usize
        -
        tmp_buffer_size.wrapping_add(best_y_base_size) as usize
      );
  let best_uv_base1: (&mut [u16], &mut [u16]) =
      (best_rgb_y.1).split_at_mut(
        tmp_buffer_size.wrapping_add(best_y_base_size).wrapping_add(target_y_base_size).wrapping_add(
          best_rgb_y_size
        )
        as
        usize
        -
        tmp_buffer_size.wrapping_add(best_y_base_size).wrapping_add(target_y_base_size) as usize
      );
  let target_uv_base1: (&mut [u16], &mut [u16]) =
      (best_uv_base1.1).split_at_mut(
        tmp_buffer_size.wrapping_add(best_y_base_size).wrapping_add(target_y_base_size).wrapping_add(
          best_rgb_y_size
        ).wrapping_add(best_uv_base_size)
        as
        usize
        -
        tmp_buffer_size.wrapping_add(best_y_base_size).wrapping_add(target_y_base_size).wrapping_add(
          best_rgb_y_size
        )
        as
        usize
      );
  let best_rgb_uv1: (&mut [u16], &mut [u16]) =
      (target_uv_base1.1).split_at_mut(
        tmp_buffer_size.wrapping_add(best_y_base_size).wrapping_add(target_y_base_size).wrapping_add(
          best_rgb_y_size
        ).wrapping_add(best_uv_base_size).wrapping_add(target_uv_base_size)
        as
        usize
        -
        tmp_buffer_size.wrapping_add(best_y_base_size).wrapping_add(target_y_base_size).wrapping_add(
          best_rgb_y_size
        ).wrapping_add(best_uv_base_size)
        as
        usize
      );
  let best_uv_base: &mut [i16] = crate::scylla_glue::scylla_i16_of_u16_mut(target_uv_base1.0);
  let target_uv_base: &mut [i16] = crate::scylla_glue::scylla_i16_of_u16_mut(best_rgb_uv1.0);
  let best_rgb_uv: &mut [i16] = crate::scylla_glue::scylla_i16_of_u16_mut(best_rgb_uv1.1);
  let mut best_y_ofs: usize = 0usize;
  let mut target_y_ofs: usize = 0usize;
  let mut best_uv_ofs: usize = 0usize;
  let mut target_uv_ofs: usize = 0usize;
  let diff_y_threshold: u64 = (3.0f64 * w as f64 * h as f64) as u64;
  let mut ok: i32;
  {
    j = 0i32;
    while
    j < height
    {
      {
        let is_last_row: i32 = (j == height.wrapping_sub(1i32)) as i32;
        let src1: (&mut [u16], &mut [u16]) =
            (best_y_base.0).split_at_mut(0i32.wrapping_mul(w) as usize);
        let src2: (&mut [u16], &mut [u16]) = (src1.1).split_at_mut(3i32.wrapping_mul(w) as usize);
        ImportOneRow(r_ptr, g_ptr, b_ptr, rgb_step, rgb_bit_depth, width, src2.0);
        if is_last_row == 0i32
        {
          ImportOneRow(
            &r_ptr[rgb_stride as usize..],
            &g_ptr[rgb_stride as usize..],
            &b_ptr[rgb_stride as usize..],
            rgb_step,
            rgb_bit_depth,
            width,
            src2.1
          )
        }
        else
        {
          (src2.1[0usize..3i32.wrapping_mul(w) as usize]).copy_from_slice(
            &src2.0[0usize..3i32.wrapping_mul(w) as usize]
          )
        };
        StoreGray(src2.0, &mut target_y_base.0[best_y_ofs.wrapping_add(0usize)..], w);
        StoreGray(src2.1, &mut target_y_base.0[best_y_ofs.wrapping_add(w as usize)..], w);
        UpdateW(src2.0, &mut best_rgb_y.0[target_y_ofs..], w, y_bit_depth, transfer_type);
        UpdateW(
          src2.1,
          &mut best_rgb_y.0[target_y_ofs.wrapping_add(w as usize)..],
          w,
          y_bit_depth,
          transfer_type
        );
        UpdateChroma(
          src2.0,
          src2.1,
          &mut target_uv_base[target_uv_ofs..],
          uv_w,
          y_bit_depth,
          transfer_type
        );
        ((&mut best_uv_base[best_uv_ofs..])[0usize..3i32.wrapping_mul(uv_w) as usize]).copy_from_slice(
          &(&target_uv_base[target_uv_ofs..])[0usize..3i32.wrapping_mul(uv_w) as usize]
        );
        best_y_ofs = best_y_ofs.wrapping_add(2i32.wrapping_mul(w) as usize);
        best_uv_ofs = best_uv_ofs.wrapping_add(3i32.wrapping_mul(uv_w) as usize);
        target_y_ofs = target_y_ofs.wrapping_add(2i32.wrapping_mul(w) as usize);
        target_uv_ofs = target_uv_ofs.wrapping_add(3i32.wrapping_mul(uv_w) as usize);
        r_ptr = &r_ptr[2i32.wrapping_mul(rgb_stride) as usize..];
        g_ptr = &g_ptr[2i32.wrapping_mul(rgb_stride) as usize..];
        b_ptr = &b_ptr[2i32.wrapping_mul(rgb_stride) as usize..]
      };
      j = j.wrapping_add(2i32)
    }
  };
  {
    iter = 0i32;
    while
    iter < kNumIterations
    {
      {
        let mut cur_uv_ofs: usize = 0usize;
        let mut prev_uv_ofs: usize = 0usize;
        let mut diff_y_sum: u64 = 0u64;
        best_y_ofs = 0usize;
        best_uv_ofs = 0usize;
        target_y_ofs = 0usize;
        target_uv_ofs = 0usize;
        j = 0i32;
        loop
        {
          {
            let src1: (&mut [u16], &mut [u16]) =
                (best_y_base.0).split_at_mut(0i32.wrapping_mul(w) as usize);
            let src2: (&mut [u16], &mut [u16]) =
                (src1.1).split_at_mut(3i32.wrapping_mul(w) as usize);
            {
              let next_uv_ofs: usize =
                  cur_uv_ofs.wrapping_add(
                    if j < h.wrapping_sub(2i32) { 3i32.wrapping_mul(uv_w) } else { 0i32 } as usize
                  );
              InterpolateTwoRows(
                &target_y_base.0[best_y_ofs..],
                &best_uv_base[prev_uv_ofs..],
                &best_uv_base[cur_uv_ofs..],
                &best_uv_base[next_uv_ofs..],
                w,
                src2.0,
                src2.1,
                y_bit_depth
              );
              prev_uv_ofs = cur_uv_ofs;
              cur_uv_ofs = next_uv_ofs
            };
            UpdateW(
              src2.0,
              &mut best_uv_base1.0[0i32.wrapping_mul(w) as usize..],
              w,
              y_bit_depth,
              transfer_type
            );
            UpdateW(
              src2.1,
              &mut best_uv_base1.0[1i32.wrapping_mul(w) as usize..],
              w,
              y_bit_depth,
              transfer_type
            );
            UpdateChroma(src2.0, src2.1, best_rgb_uv, uv_w, y_bit_depth, transfer_type);
            diff_y_sum =
                diff_y_sum.wrapping_add(
                  SharpYuvUpdateY(
                    &best_rgb_y.0[target_y_ofs..],
                    best_uv_base1.0,
                    &mut target_y_base.0[best_y_ofs..],
                    2i32.wrapping_mul(w),
                    y_bit_depth
                  )
                );
            SharpYuvUpdateRGB(
              &target_uv_base[target_uv_ofs..],
              best_rgb_uv,
              &mut best_uv_base[best_uv_ofs..],
              3i32.wrapping_mul(uv_w)
            );
            best_y_ofs = best_y_ofs.wrapping_add(2i32.wrapping_mul(w) as usize);
            best_uv_ofs = best_uv_ofs.wrapping_add(3i32.wrapping_mul(uv_w) as usize);
            target_y_ofs = target_y_ofs.wrapping_add(2i32.wrapping_mul(w) as usize);
            target_uv_ofs = target_uv_ofs.wrapping_add(3i32.wrapping_mul(uv_w) as usize);
            j = j.wrapping_add(2i32)
          };
          if j >= h { break }
        };
        if iter > 0i32
        {
          if diff_y_sum < diff_y_threshold { break };
          if diff_y_sum > prev_diff_y_sum { break }
        };
        prev_diff_y_sum = diff_y_sum
      };
      iter = iter.wrapping_add(1i32)
    }
  };
  ok =
      ConvertWRGBToYUV(
        target_y_base.0,
        best_uv_base,
        y_ptr,
        y_stride,
        u_ptr,
        u_stride,
        v_ptr,
        v_stride,
        rgb_bit_depth,
        yuv_bit_depth,
        width,
        height,
        yuv_matrix
      );
  return ok
}

#[inline] pub fn Filter2(A: i32, B: i32, W0: i32, bit_depth: i32) -> u16
{
  let v0: i32 = A.wrapping_mul(3i32).wrapping_add(B).wrapping_add(2i32).wrapping_shr(2u32);
  return clip_bit_depth(v0.wrapping_add(W0), bit_depth)
}

pub fn GetPrecisionShift(rgb_bit_depth: i32) -> i32
{
  return
  if rgb_bit_depth.wrapping_add(2i32) <= kMaxBitDepth
  { 2i32 }
  else
  { kMaxBitDepth.wrapping_sub(rgb_bit_depth) }
}

pub fn ImportOneRow(
  r_ptr: &[u8],
  g_ptr: &[u8],
  b_ptr: &[u8],
  rgb_step: i32,
  rgb_bit_depth: i32,
  pic_width: i32,
  dst: &mut [u16]
)
{
  let step: i32 = if rgb_bit_depth > 8i32 { rgb_step.wrapping_div(2i32) } else { rgb_step };
  let mut i: i32 = 0i32;
  let w: i32 = pic_width.wrapping_add(1i32) & ! 1i32;
  loop
  {
    {
      let off: i32 = i.wrapping_mul(step);
      let shift: i32 = GetPrecisionShift(rgb_bit_depth);
      if rgb_bit_depth == 8i32
      {
        dst[i.wrapping_add(0i32.wrapping_mul(w)) as usize] =
            Shift(r_ptr[off as usize] as i32, shift) as u16;
        dst[i.wrapping_add(1i32.wrapping_mul(w)) as usize] =
            Shift(g_ptr[off as usize] as i32, shift) as u16;
        dst[i.wrapping_add(2i32.wrapping_mul(w)) as usize] =
            Shift(b_ptr[off as usize] as i32, shift) as u16
      }
      else
      {
        dst[i.wrapping_add(0i32.wrapping_mul(w)) as usize] =
            Shift(crate::scylla_glue::scylla_u16_of_u8(r_ptr)[off as usize] as i32, shift) as u16;
        dst[i.wrapping_add(1i32.wrapping_mul(w)) as usize] =
            Shift(crate::scylla_glue::scylla_u16_of_u8(g_ptr)[off as usize] as i32, shift) as u16;
        dst[i.wrapping_add(2i32.wrapping_mul(w)) as usize] =
            Shift(crate::scylla_glue::scylla_u16_of_u8(b_ptr)[off as usize] as i32, shift) as u16
      }
    };
    if
    {
      i = i.wrapping_add(1i32);
      i
    }
    >=
    pic_width
    { break }
  };
  if pic_width & 1i32 != 0i32
  {
    dst[pic_width.wrapping_add(0i32.wrapping_mul(w)) as usize] =
        dst[pic_width.wrapping_add(0i32.wrapping_mul(w)).wrapping_sub(1i32) as usize];
    dst[pic_width.wrapping_add(1i32.wrapping_mul(w)) as usize] =
        dst[pic_width.wrapping_add(1i32.wrapping_mul(w)).wrapping_sub(1i32) as usize];
    dst[pic_width.wrapping_add(2i32.wrapping_mul(w)) as usize] =
        dst[pic_width.wrapping_add(2i32.wrapping_mul(w)).wrapping_sub(1i32) as usize]
  }
}

pub fn InterpolateTwoRows(
  best_y: &[u16],
  mut prev_uv: &[i16],
  mut cur_uv: &[i16],
  mut next_uv: &[i16],
  w: i32,
  mut out1: &mut [u16],
  mut out2: &mut [u16],
  bit_depth: i32
)
{
  let uv_w: i32 = w.wrapping_shr(1u32);
  let len: i32 = w.wrapping_sub(1i32).wrapping_shr(1u32);
  let mut k: i32 = 3i32;
  while
  {
    let old_value: i32 = k;
    k = k.wrapping_sub(1i32);
    old_value
  }
  >
  0i32
  {
    out1[0usize] =
        Filter2(cur_uv[0usize] as i32, prev_uv[0usize] as i32, best_y[0usize] as i32, bit_depth);
    out2[0usize] =
        Filter2(cur_uv[0usize] as i32, next_uv[0usize] as i32, best_y[w as usize] as i32, bit_depth);
    SharpYuvFilterRow(
      cur_uv,
      prev_uv,
      len,
      &best_y[0usize.wrapping_add(1usize)..],
      &mut out1[1usize..],
      bit_depth
    );
    SharpYuvFilterRow(
      cur_uv,
      next_uv,
      len,
      &best_y[(w as usize).wrapping_add(1usize)..],
      &mut out2[1usize..],
      bit_depth
    );
    if w & 1i32 == 0i32
    {
      out1[w.wrapping_sub(1i32) as usize] =
          Filter2(
            cur_uv[uv_w.wrapping_sub(1i32) as usize] as i32,
            prev_uv[uv_w.wrapping_sub(1i32) as usize] as i32,
            best_y[w.wrapping_sub(1i32).wrapping_add(0i32) as usize] as i32,
            bit_depth
          );
      out2[w.wrapping_sub(1i32) as usize] =
          Filter2(
            cur_uv[uv_w.wrapping_sub(1i32) as usize] as i32,
            next_uv[uv_w.wrapping_sub(1i32) as usize] as i32,
            best_y[w.wrapping_sub(1i32).wrapping_add(w) as usize] as i32,
            bit_depth
          )
    };
    out1 = &mut out1[w as usize..];
    out2 = &mut out2[w as usize..];
    prev_uv = &prev_uv[uv_w as usize..];
    cur_uv = &cur_uv[uv_w as usize..];
    next_uv = &next_uv[uv_w as usize..]
  }
}

pub fn RGBToGray(r: i64, g: i64, b: i64) -> i32
{
  let luma: i64 =
      13933i64.wrapping_mul(r).wrapping_add(46871i64.wrapping_mul(g)).wrapping_add(
        4732i64.wrapping_mul(b)
      ).wrapping_add(kYuvHalf as i64);
  return luma.wrapping_shr(16u32) as i32
}

#[inline] pub fn RGBToYUVComponent(r: i32, g: i32, b: i32, coeffs: [i32; 4], sfix: i32) -> i32
{
  let srounder: i32 = 1i32.wrapping_shl(16i32.wrapping_add(sfix).wrapping_sub(1i32) as u32);
  let luma: i32 =
      (coeffs[0usize]).wrapping_mul(r).wrapping_add((coeffs[1usize]).wrapping_mul(g)).wrapping_add(
        (coeffs[2usize]).wrapping_mul(b)
      ).wrapping_add(coeffs[3usize]).wrapping_add(srounder);
  return luma.wrapping_shr(16i32.wrapping_add(sfix) as u32)
}

pub fn ScaleDown(
  a: u16,
  b: u16,
  c: u16,
  d: u16,
  bit_depth: i32,
  transfer_type: SharpYuvTransferFunctionType
) ->
    u32
{
  let A: u32 = SharpYuvGammaToLinear(a, bit_depth, transfer_type);
  let B: u32 = SharpYuvGammaToLinear(b, bit_depth, transfer_type);
  let C: u32 = SharpYuvGammaToLinear(c, bit_depth, transfer_type);
  let D: u32 = SharpYuvGammaToLinear(d, bit_depth, transfer_type);
  return
  SharpYuvLinearToGamma(
    A.wrapping_add(B).wrapping_add(C).wrapping_add(D).wrapping_add(2u32).wrapping_shr(2u32),
    bit_depth,
    transfer_type
  )
  as
  u32
}

#[derive(PartialEq, Clone, Copy, Default)]
#[repr(C)]
pub
struct SharpYuvConversionMatrix
{ pub rgb_to_y: [i32; 4], pub rgb_to_u: [i32; 4], pub rgb_to_v: [i32; 4] }

pub fn SharpYuvConvert(
  r_ptr: &[u8],
  g_ptr: &[u8],
  b_ptr: &[u8],
  rgb_step: i32,
  rgb_stride: i32,
  rgb_bit_depth: i32,
  y_ptr: &mut [u8],
  y_stride: i32,
  u_ptr: &mut [u8],
  u_stride: i32,
  v_ptr: &mut [u8],
  v_stride: i32,
  yuv_bit_depth: i32,
  width: i32,
  height: i32,
  yuv_matrix: &[SharpYuvConversionMatrix]
) ->
    i32
{
  let options: SharpYuvOptions =
      SharpYuvOptions
      { yuv_matrix, transfer_type: SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb };
  return
  SharpYuvConvertWithOptions(
    r_ptr,
    g_ptr,
    b_ptr,
    rgb_step,
    rgb_stride,
    rgb_bit_depth,
    y_ptr,
    y_stride,
    u_ptr,
    u_stride,
    v_ptr,
    v_stride,
    yuv_bit_depth,
    width,
    height,
    std::slice::from_ref::<SharpYuvOptions>(&options)
  )
}

pub fn SharpYuvConvertWithOptions(
  r_ptr: &[u8],
  g_ptr: &[u8],
  b_ptr: &[u8],
  rgb_step: i32,
  rgb_stride: i32,
  rgb_bit_depth: i32,
  y_ptr: &mut [u8],
  y_stride: i32,
  u_ptr: &mut [u8],
  u_stride: i32,
  v_ptr: &mut [u8],
  v_stride: i32,
  yuv_bit_depth: i32,
  width: i32,
  height: i32,
  options: &[SharpYuvOptions]
) ->
    i32
{
  let yuv_matrix: &[SharpYuvConversionMatrix] = (options[0usize]).yuv_matrix;
  let transfer_type: SharpYuvTransferFunctionType = (options[0usize]).transfer_type;
  let mut scaled_matrix: SharpYuvConversionMatrix = Default::default();
  let rgb_max: i32 = 1i32.wrapping_shl(rgb_bit_depth as u32).wrapping_sub(1i32);
  let rgb_round: i32 = 1i32.wrapping_shl(rgb_bit_depth.wrapping_sub(1i32) as u32);
  let yuv_max: i32 = 1i32.wrapping_shl(yuv_bit_depth as u32).wrapping_sub(1i32);
  let sfix: i32 = GetPrecisionShift(rgb_bit_depth);
  if
  width < 1i32 || height < 1i32 || width == 2147483647i32 || height == 2147483647i32 || false
  ||
  false
  ||
  false
  ||
  false
  ||
  false
  ||
  false
  { return 0i32 };
  if
  rgb_bit_depth != 8i32 && rgb_bit_depth != 10i32 && rgb_bit_depth != 12i32
  &&
  rgb_bit_depth != 16i32
  { return 0i32 };
  if yuv_bit_depth != 8i32 && yuv_bit_depth != 10i32 && yuv_bit_depth != 12i32 { return 0i32 };
  if
  rgb_bit_depth > 8i32
  &&
  (rgb_step.wrapping_rem(2i32) != 0i32 || rgb_stride.wrapping_rem(2i32) != 0i32)
  { return 0i32 };
  if
  yuv_bit_depth > 8i32
  &&
  (y_stride.wrapping_rem(2i32) != 0i32 || u_stride.wrapping_rem(2i32) != 0i32
  ||
  v_stride.wrapping_rem(2i32) != 0i32)
  { return 0i32 };
  SharpYuvInit();
  if rgb_bit_depth == yuv_bit_depth
  {
    (std::slice::from_mut::<SharpYuvConversionMatrix>(&mut scaled_matrix)[0usize..1usize]).copy_from_slice(
      &yuv_matrix[0usize..1usize]
    )
  }
  else
  {
    let mut i: i32 = 0i32;
    while
    i < 3i32
    {
      {
        scaled_matrix.rgb_to_y[i as usize] =
            ((yuv_matrix[0usize]).rgb_to_y[i as usize]).wrapping_mul(yuv_max).wrapping_add(
              rgb_round
            ).wrapping_div(rgb_max);
        scaled_matrix.rgb_to_u[i as usize] =
            ((yuv_matrix[0usize]).rgb_to_u[i as usize]).wrapping_mul(yuv_max).wrapping_add(
              rgb_round
            ).wrapping_div(rgb_max);
        scaled_matrix.rgb_to_v[i as usize] =
            ((yuv_matrix[0usize]).rgb_to_v[i as usize]).wrapping_mul(yuv_max).wrapping_add(
              rgb_round
            ).wrapping_div(rgb_max)
      };
      i = i.wrapping_add(1i32)
    }
  };
  scaled_matrix.rgb_to_y[3usize] = Shift((yuv_matrix[0usize]).rgb_to_y[3usize], sfix);
  scaled_matrix.rgb_to_u[3usize] = Shift((yuv_matrix[0usize]).rgb_to_u[3usize], sfix);
  scaled_matrix.rgb_to_v[3usize] = Shift((yuv_matrix[0usize]).rgb_to_v[3usize], sfix);
  return
  DoSharpArgbToYuv(
    r_ptr,
    g_ptr,
    b_ptr,
    rgb_step,
    rgb_stride,
    rgb_bit_depth,
    y_ptr,
    y_stride,
    u_ptr,
    u_stride,
    v_ptr,
    v_stride,
    yuv_bit_depth,
    width,
    height,
    std::slice::from_ref::<SharpYuvConversionMatrix>(&scaled_matrix),
    transfer_type
  )
}

pub fn SharpYuvGetVersion() -> i32
{ return 0i32.wrapping_shl(24u32) | 4i32.wrapping_shl(16u32) | 2i32 }

pub fn SharpYuvInit() { SharpYuvInitGammaTables() }

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub
struct SharpYuvOptions <'a>
{
  pub yuv_matrix: &'a [SharpYuvConversionMatrix],
  pub transfer_type: SharpYuvTransferFunctionType
}

#[inline] pub fn SharpYuvOptionsInit<'a>(
  yuv_matrix: &'a[SharpYuvConversionMatrix],
  options: &mut [SharpYuvOptions<'a>]
) ->
    i32
{
  return
  SharpYuvOptionsInitInternal(
    yuv_matrix,
    options,
    0i32.wrapping_shl(24u32) | 4i32.wrapping_shl(16u32) | 2i32
  )
}

pub fn SharpYuvOptionsInitInternal<'a>(
  yuv_matrix: &'a[SharpYuvConversionMatrix],
  options: &mut [SharpYuvOptions<'a>],
  version: i32
) ->
    i32
{
  let major: i32 = version.wrapping_shr(24u32);
  let minor: i32 = version.wrapping_shr(16u32) & 255i32;
  if false || false || major == 0i32 && major == 0i32 && minor != 4i32 || major != 0i32
  { return 0i32 };
  options[0usize] =
      SharpYuvOptions
      { yuv_matrix, transfer_type: SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb };
  return 1i32
}

#[derive(PartialEq, Clone, Copy)]
pub enum SharpYuvTransferFunctionType
{
  kSharpYuvTransferFunctionBt709 =1,
  kSharpYuvTransferFunctionBt470M =4,
  kSharpYuvTransferFunctionBt470Bg =5,
  kSharpYuvTransferFunctionBt601 =6,
  kSharpYuvTransferFunctionSmpte240 =7,
  kSharpYuvTransferFunctionLinear =8,
  kSharpYuvTransferFunctionLog100 =9,
  kSharpYuvTransferFunctionLog100_Sqrt10 =10,
  kSharpYuvTransferFunctionIec61966 =11,
  kSharpYuvTransferFunctionBt1361 =12,
  kSharpYuvTransferFunctionSrgb =13,
  kSharpYuvTransferFunctionBt2020_10Bit =14,
  kSharpYuvTransferFunctionBt2020_12Bit =15,
  kSharpYuvTransferFunctionSmpte2084 =16,
  kSharpYuvTransferFunctionSmpte428 =17,
  kSharpYuvTransferFunctionHlg =18,
  kSharpYuvTransferFunctionNum
}

pub fn StoreGray(rgb: &[u16], y: &mut [u16], w: i32)
{
  let mut i: i32 = 0i32;
  loop
  {
    y[i as usize] =
        RGBToGray(
          rgb[0i32.wrapping_mul(w).wrapping_add(i) as usize] as i64,
          rgb[1i32.wrapping_mul(w).wrapping_add(i) as usize] as i64,
          rgb[2i32.wrapping_mul(w).wrapping_add(i) as usize] as i64
        )
        as
        u16;
    if
    {
      i = i.wrapping_add(1i32);
      i
    }
    >=
    w
    { break }
  }
}

pub fn UpdateChroma(
  mut src1: &[u16],
  mut src2: &[u16],
  mut dst: &mut [i16],
  uv_w: i32,
  bit_depth: i32,
  transfer_type: SharpYuvTransferFunctionType
)
{
  let mut i: i32 = 0i32;
  loop
  {
    {
      let r: i32 =
          ScaleDown(
            src1[0i32.wrapping_mul(uv_w).wrapping_add(0i32) as usize],
            src1[0i32.wrapping_mul(uv_w).wrapping_add(1i32) as usize],
            src2[0i32.wrapping_mul(uv_w).wrapping_add(0i32) as usize],
            src2[0i32.wrapping_mul(uv_w).wrapping_add(1i32) as usize],
            bit_depth,
            transfer_type
          )
          as
          i32;
      let g: i32 =
          ScaleDown(
            src1[2i32.wrapping_mul(uv_w).wrapping_add(0i32) as usize],
            src1[2i32.wrapping_mul(uv_w).wrapping_add(1i32) as usize],
            src2[2i32.wrapping_mul(uv_w).wrapping_add(0i32) as usize],
            src2[2i32.wrapping_mul(uv_w).wrapping_add(1i32) as usize],
            bit_depth,
            transfer_type
          )
          as
          i32;
      let b: i32 =
          ScaleDown(
            src1[4i32.wrapping_mul(uv_w).wrapping_add(0i32) as usize],
            src1[4i32.wrapping_mul(uv_w).wrapping_add(1i32) as usize],
            src2[4i32.wrapping_mul(uv_w).wrapping_add(0i32) as usize],
            src2[4i32.wrapping_mul(uv_w).wrapping_add(1i32) as usize],
            bit_depth,
            transfer_type
          )
          as
          i32;
      let W: i32 = RGBToGray(r as i64, g as i64, b as i64);
      dst[0i32.wrapping_mul(uv_w) as usize] = r.wrapping_sub(W) as i16;
      dst[1i32.wrapping_mul(uv_w) as usize] = g.wrapping_sub(W) as i16;
      dst[2i32.wrapping_mul(uv_w) as usize] = b.wrapping_sub(W) as i16;
      dst = &mut dst[1usize..];
      src1 = &src1[2usize..];
      src2 = &src2[2usize..]
    };
    if
    {
      i = i.wrapping_add(1i32);
      i
    }
    >=
    uv_w
    { break }
  }
}

#[inline] pub fn UpdateW(
  src: &[u16],
  dst: &mut [u16],
  w: i32,
  bit_depth: i32,
  transfer_type: SharpYuvTransferFunctionType
)
{
  let mut i: i32 = 0i32;
  loop
  {
    {
      let R: u32 =
          SharpYuvGammaToLinear(
            src[0i32.wrapping_mul(w).wrapping_add(i) as usize],
            bit_depth,
            transfer_type
          );
      let G: u32 =
          SharpYuvGammaToLinear(
            src[1i32.wrapping_mul(w).wrapping_add(i) as usize],
            bit_depth,
            transfer_type
          );
      let B: u32 =
          SharpYuvGammaToLinear(
            src[2i32.wrapping_mul(w).wrapping_add(i) as usize],
            bit_depth,
            transfer_type
          );
      let Y: u32 = RGBToGray(R as i64, G as i64, B as i64) as u32;
      dst[i as usize] = SharpYuvLinearToGamma(Y, bit_depth, transfer_type) as u16
    };
    if
    {
      i = i.wrapping_add(1i32);
      i
    }
    >=
    w
    { break }
  }
}

pub fn clip_8b(v: i16) -> u8
{
  return
  if v as i32 & ! 255i32 == 0i32
  { v as u8 as u32 }
  else if (v as i32) < 0i32 { 0u32 } else { 255u32 }
  as
  u8
}

pub fn clip_bit_depth(y: i32, bit_depth: i32) -> u16
{
  let max: i32 = 1i32.wrapping_shl(bit_depth as u32).wrapping_sub(1i32);
  return if y & ! max == 0i32 { y as u16 as i32 } else if y < 0i32 { 0i32 } else { max } as u16
}

pub type fixed_t = i16;

pub type fixed_y_t = u16;

pub const kMaxBitDepth: i32 = 14i32;

pub const kNumIterations: i32 = 4i32;

pub const kYuvHalf: i32 = 1i32.wrapping_shl(16i32.wrapping_sub(1i32) as u32);

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub
struct SharpYuvColorSpace
{ pub kr: f32, pub kb: f32, pub bit_depth: i32, pub range: SharpYuvRange }

pub fn SharpYuvComputeConversionMatrix(
  yuv_color_space: &[SharpYuvColorSpace],
  matrix: &mut [SharpYuvConversionMatrix]
)
{
  let kr: f32 = (yuv_color_space[0usize]).kr;
  let kb: f32 = (yuv_color_space[0usize]).kb;
  let kg: f32 = 1.0f32 - kr - kb;
  let cb: f32 = 0.5f32 / (1.0f32 - kb);
  let cr: f32 = 0.5f32 / (1.0f32 - kr);
  let shift: i32 = ((yuv_color_space[0usize]).bit_depth).wrapping_sub(8i32);
  let denom: f32 =
      1i32.wrapping_shl((yuv_color_space[0usize]).bit_depth as u32).wrapping_sub(1i32) as f32;
  let mut scale_y: f32 = 1.0f32;
  let mut add_y: f32 = 0.0f32;
  let mut scale_u: f32 = cb;
  let mut scale_v: f32 = cr;
  let add_uv: f32 = 128i32.wrapping_shl(shift as u32) as f32;
  if (yuv_color_space[0usize]).range == SharpYuvRange::kSharpYuvRangeLimited
  {
    scale_y *= 219i32.wrapping_shl(shift as u32) as f32 / denom;
    scale_u *= 224i32.wrapping_shl(shift as u32) as f32 / denom;
    scale_v *= 224i32.wrapping_shl(shift as u32) as f32 / denom;
    add_y = 16i32.wrapping_shl(shift as u32) as f32
  };
  (matrix[0usize]).rgb_to_y[0usize] = ToFixed16(kr * scale_y);
  (matrix[0usize]).rgb_to_y[1usize] = ToFixed16(kg * scale_y);
  (matrix[0usize]).rgb_to_y[2usize] = ToFixed16(kb * scale_y);
  (matrix[0usize]).rgb_to_y[3usize] = ToFixed16(add_y);
  (matrix[0usize]).rgb_to_u[0usize] = ToFixed16((0f32 - kr) * scale_u);
  (matrix[0usize]).rgb_to_u[1usize] = ToFixed16((0f32 - kg) * scale_u);
  (matrix[0usize]).rgb_to_u[2usize] = ToFixed16((1f32 - kb) * scale_u);
  (matrix[0usize]).rgb_to_u[3usize] = ToFixed16(add_uv);
  (matrix[0usize]).rgb_to_v[0usize] = ToFixed16((1f32 - kr) * scale_v);
  (matrix[0usize]).rgb_to_v[1usize] = ToFixed16((0f32 - kg) * scale_v);
  (matrix[0usize]).rgb_to_v[2usize] = ToFixed16((0f32 - kb) * scale_v);
  (matrix[0usize]).rgb_to_v[3usize] = ToFixed16(add_uv)
}

pub fn SharpYuvGetConversionMatrix <'a>(matrix_type: SharpYuvMatrixType) ->
    &'a [SharpYuvConversionMatrix]
{
  match matrix_type
  {
    SharpYuvMatrixType::kSharpYuvMatrixWebp =>
      return std::slice::from_ref::<SharpYuvConversionMatrix>(&kWebpMatrix),
    SharpYuvMatrixType::kSharpYuvMatrixRec601Limited =>
      return std::slice::from_ref::<SharpYuvConversionMatrix>(&kRec601LimitedMatrix),
    SharpYuvMatrixType::kSharpYuvMatrixRec601Full =>
      return std::slice::from_ref::<SharpYuvConversionMatrix>(&kRec601FullMatrix),
    SharpYuvMatrixType::kSharpYuvMatrixRec709Limited =>
      return std::slice::from_ref::<SharpYuvConversionMatrix>(&kRec709LimitedMatrix),
    SharpYuvMatrixType::kSharpYuvMatrixRec709Full =>
      return std::slice::from_ref::<SharpYuvConversionMatrix>(&kRec709FullMatrix),
    SharpYuvMatrixType::kSharpYuvMatrixNum => return &[],
    _ => panic!("Incomplete pattern matching")
  };
  return &[]
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub enum SharpYuvMatrixType
{
  kSharpYuvMatrixWebp =0,
  kSharpYuvMatrixRec601Limited,
  kSharpYuvMatrixRec601Full,
  kSharpYuvMatrixRec709Limited,
  kSharpYuvMatrixRec709Full,
  kSharpYuvMatrixNum
}

#[derive(PartialEq, Clone, Copy)]
pub enum SharpYuvRange
{
  kSharpYuvRangeFull,
  kSharpYuvRangeLimited
}

pub fn ToFixed16(f: f32) -> i32
{ return crate::math::floor((f * 1i32.wrapping_shl(16u32) as f32 + 0.5f32) as f64) as i32 }

pub const kRec601FullMatrix: SharpYuvConversionMatrix =
    SharpYuvConversionMatrix
    {
      rgb_to_y: [19595i32, 38470i32, 7471i32, 0i32],
      rgb_to_u:
      [0i32.wrapping_sub(11058i32), 0i32.wrapping_sub(21710i32), 32768i32,
          128i32.wrapping_shl(16u32)],
      rgb_to_v:
      [32768i32, 0i32.wrapping_sub(27439i32), 0i32.wrapping_sub(5329i32), 128i32.wrapping_shl(16u32)]
    };

pub const kRec601LimitedMatrix: SharpYuvConversionMatrix =
    SharpYuvConversionMatrix
    {
      rgb_to_y: [16829i32, 33039i32, 6416i32, 16i32.wrapping_shl(16u32)],
      rgb_to_u:
      [0i32.wrapping_sub(9714i32), 0i32.wrapping_sub(19071i32), 28784i32, 128i32.wrapping_shl(16u32)],
      rgb_to_v:
      [28784i32, 0i32.wrapping_sub(24103i32), 0i32.wrapping_sub(4681i32), 128i32.wrapping_shl(16u32)]
    };

pub const kRec709FullMatrix: SharpYuvConversionMatrix =
    SharpYuvConversionMatrix
    {
      rgb_to_y: [13933i32, 46871i32, 4732i32, 0i32],
      rgb_to_u:
      [0i32.wrapping_sub(7509i32), 0i32.wrapping_sub(25259i32), 32768i32, 128i32.wrapping_shl(16u32)],
      rgb_to_v:
      [32768i32, 0i32.wrapping_sub(29763i32), 0i32.wrapping_sub(3005i32), 128i32.wrapping_shl(16u32)]
    };

pub const kRec709LimitedMatrix: SharpYuvConversionMatrix =
    SharpYuvConversionMatrix
    {
      rgb_to_y: [11966i32, 40254i32, 4064i32, 16i32.wrapping_shl(16u32)],
      rgb_to_u:
      [0i32.wrapping_sub(6596i32), 0i32.wrapping_sub(22189i32), 28784i32, 128i32.wrapping_shl(16u32)],
      rgb_to_v:
      [28784i32, 0i32.wrapping_sub(26145i32), 0i32.wrapping_sub(2639i32), 128i32.wrapping_shl(16u32)]
    };

pub const kWebpMatrix: SharpYuvConversionMatrix =
    SharpYuvConversionMatrix
    {
      rgb_to_y: [16839i32, 33059i32, 6420i32, 16i32.wrapping_shl(16u32)],
      rgb_to_u:
      [0i32.wrapping_sub(9719i32), 0i32.wrapping_sub(19081i32), 28800i32, 128i32.wrapping_shl(16u32)],
      rgb_to_v:
      [28800i32, 0i32.wrapping_sub(24116i32), 0i32.wrapping_sub(4684i32), 128i32.wrapping_shl(16u32)]
    };

pub fn SharpYuvFilterRow(
  A: &[i16],
  B: &[i16],
  len: i32,
  best_y: &[u16],
  out: &mut [u16],
  bit_depth: i32
)
{ SharpYuvFilterRow_C(A, B, len, best_y, out, bit_depth) }

pub fn SharpYuvFilterRow_C(
  mut A: &[i16],
  mut B: &[i16],
  len: i32,
  best_y: &[u16],
  out: &mut [u16],
  bit_depth: i32
)
{
  let mut i: i32;
  let max_y: i32 = 1i32.wrapping_shl(bit_depth as u32).wrapping_sub(1i32);
  i = 0i32;
  while
  i < len
  {
    {
      let v0: i32 =
          (A[0usize] as i32).wrapping_mul(9i32).wrapping_add((A[1usize] as i32).wrapping_mul(3i32)).wrapping_add(
            (B[0usize] as i32).wrapping_mul(3i32)
          ).wrapping_add(B[1usize] as i32).wrapping_add(8i32).wrapping_shr(4u32);
      let v1: i32 =
          (A[1usize] as i32).wrapping_mul(9i32).wrapping_add((A[0usize] as i32).wrapping_mul(3i32)).wrapping_add(
            (B[1usize] as i32).wrapping_mul(3i32)
          ).wrapping_add(B[0usize] as i32).wrapping_add(8i32).wrapping_shr(4u32);
      out[2i32.wrapping_mul(i).wrapping_add(0i32) as usize] =
          clip(
            (best_y[2i32.wrapping_mul(i).wrapping_add(0i32) as usize] as i32).wrapping_add(v0),
            max_y
          );
      out[2i32.wrapping_mul(i).wrapping_add(1i32) as usize] =
          clip(
            (best_y[2i32.wrapping_mul(i).wrapping_add(1i32) as usize] as i32).wrapping_add(v1),
            max_y
          )
    };
    {
      {
        i = i.wrapping_add(1i32);
        let _: i32 = i;
        ()
      };
      A = &A[1usize..];
      let _: &[i16] = A;
      ()
    };
    B = &B[1usize..];
    let _: &[i16] = B;
    ()
  }
}

pub fn SharpYuvInitDsp() { () }

pub fn SharpYuvUpdateRGB(src: &[i16], r#ref: &[i16], dst: &mut [i16], len: i32)
{ SharpYuvUpdateRGB_C(src, r#ref, dst, len) }

pub fn SharpYuvUpdateRGB_C(r#ref: &[i16], src: &[i16], dst: &mut [i16], len: i32)
{
  for i in 0i32..len
  {
    let diff_uv: i32 = (r#ref[i as usize] as i32).wrapping_sub(src[i as usize] as i32);
    dst[i as usize] = (dst[i as usize] as i32).wrapping_add(diff_uv) as i16
  }
}

pub fn SharpYuvUpdateY(src: &[u16], r#ref: &[u16], dst: &mut [u16], len: i32, bit_depth: i32) ->
    u64
{ return SharpYuvUpdateY_C(src, r#ref, dst, len, bit_depth) }

pub fn SharpYuvUpdateY_C(r#ref: &[u16], src: &[u16], dst: &mut [u16], len: i32, bit_depth: i32) ->
    u64
{
  let mut diff: u64 = 0u64;
  let max_y: i32 = 1i32.wrapping_shl(bit_depth as u32).wrapping_sub(1i32);
  for i in 0i32..len
  {
    let diff_y: i32 = (r#ref[i as usize] as i32).wrapping_sub(src[i as usize] as i32);
    let new_y: i32 = (dst[i as usize] as i32).wrapping_add(diff_y);
    dst[i as usize] = clip(new_y, max_y);
    diff = diff.wrapping_add(crate::_stdlib::abs(diff_y) as u64)
  };
  return diff
}

pub fn clip(v: i32, max: i32) -> u16
{ return if v < 0i32 { 0u16 } else if v > max { max as u16 } else { v as u16 } }

#[inline] pub fn FixedPointInterpolation(
  v: i32,
  tab: &[u32],
  tab_pos_shift_right: i32,
  tab_value_shift: i32
) ->
    u32
{
  let tab_pos: u32 = Shift(v, 0i32.wrapping_sub(tab_pos_shift_right)) as u32;
  let x: u32 = (v as u32).wrapping_sub(tab_pos.wrapping_shl(tab_pos_shift_right as u32));
  let v0: u32 = Shift(tab[tab_pos.wrapping_add(0u32) as usize] as i32, tab_value_shift) as u32;
  let v1: u32 = Shift(tab[tab_pos.wrapping_add(1u32) as usize] as i32, tab_value_shift) as u32;
  let v2: u32 = v1.wrapping_sub(v0).wrapping_mul(x);
  let half: i32 =
      if tab_pos_shift_right > 0i32
      { 1i32.wrapping_shl(tab_pos_shift_right.wrapping_sub(1i32) as u32) }
      else
      { 0i32 };
  let result: u32 =
      v0.wrapping_add(v2.wrapping_add(half as u32).wrapping_shr(tab_pos_shift_right as u32));
  return result
}

pub fn FromLinear470Bg(linear: f32) -> f32
{
  return
  Powf(
    if linear < 0.0f32 { 0.0f32 } else if 1.0f32 < linear { 1.0f32 } else { linear },
    1.0f32 / 2.79999995232f32
  )
}

pub fn FromLinear470M(linear: f32) -> f32
{
  return
  Powf(
    if linear < 0.0f32 { 0.0f32 } else if 1.0f32 < linear { 1.0f32 } else { linear },
    1.0f32 / 2.20000004768f32
  )
}

pub fn FromLinear709(linear: f32) -> f32
{
  if linear < 0.0f32
  { return 0.0f32 }
  else if linear < 0.0180539693683f32
  { return linear * 4.5f32 }
  else if linear < 1.0f32
  { return 1.09929680824f32 * Powf(linear, 0.449999988079f32) - 0.099296823144f32 };
  return 1.0f32
}

pub fn FromLinearBt1361(linear: f32) -> f32
{
  if linear < 0f32 - 0.25f32
  { return 0f32 - 0.25f32 }
  else if linear < 0.0f32
  {
    return
    (0f32 - 0.274824202061f32) * Powf((0f32 - 4.0f32) * linear, 0.449999988079f32)
    +
    0.024824205786f32
  }
  else if linear < 0.0180539693683f32
  { return linear * 4.5f32 }
  else if linear < 1.0f32
  { return 1.09929680824f32 * Powf(linear, 0.449999988079f32) - 0.099296823144f32 };
  return 1.0f32
}

pub fn FromLinearHlg(mut linear: f32) -> f32
{
  linear = Powf(linear, 1.0f32 / 1.20000004768f32);
  if linear < 0.0f32
  { return 0.0f32 }
  else if linear <= 1.0f32 / 12.0f32 { return crate::math::sqrtf(3.0f32 * linear) };
  return
  0.178832769394f32 * crate::math::logf(12.0f32 * linear - 0.284668922424f32) + 0.559910714626f32
}

pub fn FromLinearIec61966(linear: f32) -> f32
{
  if linear <= 0f32 - 0.0180539693683f32
  {
    return (0f32 - 1.09929680824f32) * Powf(0f32 - linear, 0.449999988079f32) + 0.099296823144f32
  }
  else if linear < 0.0180539693683f32 { return linear * 4.5f32 };
  return 1.09929680824f32 * Powf(linear, 0.449999988079f32) - 0.099296823144f32
}

pub fn FromLinearLog100(linear: f32) -> f32
{
  return
  if linear < 0.00999999977648f32
  { 0.0f32 }
  else
  { 1.0f32 + Log10f(if linear < 1.0f32 { linear } else { 1.0f32 }) / 2.0f32 }
}

pub fn FromLinearLog100Sqrt10(linear: f32) -> f32
{
  return
  if linear < 0.0031622776296f32
  { 0.0f32 }
  else
  { 1.0f32 + Log10f(if linear < 1.0f32 { linear } else { 1.0f32 }) / 2.5f32 }
}

pub fn FromLinearPq(linear: f32) -> f32
{
  if linear > 0.0f32
  {
    let pow_linear: f32 = Powf(linear, 653.0f32 / 4096.0f32);
    let num: f32 = 107.0f32 / 128.0f32 + 2413.0f32 / 128.0f32 * pow_linear;
    let den: f32 = 1.0f32 + 2392.0f32 / 128.0f32 * pow_linear;
    return Powf(num / den, 2523.0f32 / 32.0f32)
  };
  return 0.0f32
}

pub fn FromLinearSmpte240(linear: f32) -> f32
{
  if linear < 0.0f32
  { return 0.0f32 }
  else if linear < 0.0228215847164f32
  { return linear * 4.0f32 }
  else if linear < 1.0f32
  { return 1.11157214642f32 * Powf(linear, 0.449999988079f32) - 0.11157219857f32 };
  return 1.0f32
}

pub fn FromLinearSmpte428(linear: f32) -> f32
{
  return
  Powf(
    0.916555285454f32 * if linear > 0.0f32 { linear } else { 0.0f32 },
    1.0f32 / 2.59999990463f32
  )
}

pub fn FromLinearSrgb(value: u32, bit_depth: i32) -> u16
{
  return
  FixedPointInterpolation(
    value as i32,
    unsafe { &kLinearToGammaTabS },
    16i32.wrapping_sub(9i32),
    bit_depth.wrapping_sub(16i32)
  )
  as
  u16
}

#[inline] pub fn Log10f(x: f32) -> f32 { return crate::math::log10(x as f64) as f32 }

#[inline] pub fn Powf(base: f32, exp: f32) -> f32
{ return crate::math::pow(base as f64, exp as f64) as f32 }

#[inline] pub fn Roundf(x: f32) -> f32
{
  if x < 0f32
  { return crate::math::ceil((x - 0.5f32) as f64) as f32 }
  else
  { return crate::math::floor((x + 0.5f32) as f64) as f32 }
}

pub fn SharpYuvGammaToLinear(
  v: u16,
  bit_depth: i32,
  transfer_type: SharpYuvTransferFunctionType
) ->
    u32
{
  let mut v_float: f32;
  let mut linear: f32;
  if transfer_type == SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb
  { return ToLinearSrgb(v, bit_depth) };
  v_float = v as f32 / 1i32.wrapping_shl(bit_depth as u32).wrapping_sub(1i32) as f32;
  match transfer_type
  {
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt709 => linear = ToLinear709(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt601 => linear = ToLinear709(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt2020_10Bit =>
      linear = ToLinear709(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt2020_12Bit =>
      linear = ToLinear709(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt470M => linear = ToLinear470M(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt470Bg =>
      linear = ToLinear470Bg(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte240 =>
      linear = ToLinearSmpte240(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLinear => return v as u32,
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLog100 =>
      linear = ToLinearLog100(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLog100_Sqrt10 =>
      linear = ToLinearLog100Sqrt10(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionIec61966 =>
      linear = ToLinearIec61966(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt1361 =>
      linear = ToLinearBt1361(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte2084 =>
      linear = ToLinearPq(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte428 =>
      linear = ToLinearSmpte428(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionHlg => linear = ToLinearHlg(v_float),
    _ => linear = 0f32
  };
  return Roundf(linear * 1i32.wrapping_shl(16u32).wrapping_sub(1i32) as f32) as u32
}

pub fn SharpYuvInitGammaTables()
{
  if unsafe { kGammaTablesSOk } == 0i32
  {
    let a: f64 = 0.0992968268094f64;
    let thresh: f64 = 0.0180539685108f64;
    let final_scale: f64 = 1i32.wrapping_shl(16u32) as f64;
    {
      let norm: f64 = 1.0f64 / 1i32.wrapping_shl(10u32) as f64;
      let a_rec: f64 = 1.0f64 / (1.0f64 + a);
      for v in 0i32..=1i32.wrapping_shl(10u32)
      {
        let g: f64 = norm * v as f64;
        let mut value: f64;
        if g <= thresh * 4.5f64
        { value = g / 4.5f64 }
        else
        { value = crate::math::pow(a_rec * (g + a), kGammaF) };
        unsafe { kGammaToLinearTabS[v as usize] = (value * final_scale + 0.5f64) as u32 }
      };
      unsafe { kGammaToLinearTabS[1i32.wrapping_shl(10u32).wrapping_add(1i32) as usize] =
          kGammaToLinearTabS[1i32.wrapping_shl(10u32) as usize] }
    };
    {
      let scale: f64 = 1.0f64 / 1i32.wrapping_shl(9u32) as f64;
      for v in 0i32..=1i32.wrapping_shl(9u32)
      {
        let g: f64 = scale * v as f64;
        let mut value: f64;
        if g <= thresh
        { value = 4.5f64 * g }
        else
        { value = (1.0f64 + a) * crate::math::pow(g, 1.0f64 / kGammaF) - a };
        unsafe { kLinearToGammaTabS[v as usize] = (final_scale * value + 0.5f64) as u32 }
      };
      unsafe { kLinearToGammaTabS[1i32.wrapping_shl(9u32).wrapping_add(1i32) as usize] =
          kLinearToGammaTabS[1i32.wrapping_shl(9u32) as usize] }
    };
    unsafe { kGammaTablesSOk = 1i32 }
  }
}

pub fn SharpYuvLinearToGamma(
  v: u32,
  bit_depth: i32,
  transfer_type: SharpYuvTransferFunctionType
) ->
    u16
{
  let mut v_float: f32;
  let mut linear: f32;
  if transfer_type == SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb
  { return FromLinearSrgb(v, bit_depth) };
  v_float = v as f32 / 1i32.wrapping_shl(16u32).wrapping_sub(1i32) as f32;
  match transfer_type
  {
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt709 => linear = FromLinear709(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt601 => linear = FromLinear709(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt2020_10Bit =>
      linear = FromLinear709(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt2020_12Bit =>
      linear = FromLinear709(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt470M =>
      linear = FromLinear470M(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt470Bg =>
      linear = FromLinear470Bg(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte240 =>
      linear = FromLinearSmpte240(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLinear => return v as u16,
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLog100 =>
      linear = FromLinearLog100(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLog100_Sqrt10 =>
      linear = FromLinearLog100Sqrt10(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionIec61966 =>
      linear = FromLinearIec61966(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt1361 =>
      linear = FromLinearBt1361(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte2084 =>
      linear = FromLinearPq(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte428 =>
      linear = FromLinearSmpte428(v_float),
    SharpYuvTransferFunctionType::kSharpYuvTransferFunctionHlg => linear = FromLinearHlg(v_float),
    _ => linear = 0f32
  };
  return Roundf(linear * 1i32.wrapping_shl(bit_depth as u32).wrapping_sub(1i32) as f32) as u16
}

#[inline] pub fn Shift(v: i32, shift: i32) -> i32
{
  return
  if shift >= 0i32
  { v.wrapping_shl(shift as u32) }
  else
  { v.wrapping_shr(0i32.wrapping_sub(shift) as u32) }
}

pub fn ToLinear470Bg(gamma: f32) -> f32
{
  return
  Powf(
    if gamma < 0.0f32 { 0.0f32 } else if 1.0f32 < gamma { 1.0f32 } else { gamma },
    2.79999995232f32
  )
}

pub fn ToLinear470M(gamma: f32) -> f32
{
  return
  Powf(
    if gamma < 0.0f32 { 0.0f32 } else if 1.0f32 < gamma { 1.0f32 } else { gamma },
    2.20000004768f32
  )
}

pub fn ToLinear709(gamma: f32) -> f32
{
  if gamma < 0.0f32
  { return 0.0f32 }
  else if gamma < 4.5f32 * 0.0180539693683f32
  { return gamma / 4.5f32 }
  else if gamma < 1.0f32
  { return Powf((gamma + 0.099296823144f32) / 1.09929680824f32, 1.0f32 / 0.449999988079f32) };
  return 1.0f32
}

pub fn ToLinearBt1361(gamma: f32) -> f32
{
  if gamma < 0f32 - 0.25f32
  { return 0f32 - 0.25f32 }
  else if gamma < 0.0f32
  {
    return
    Powf((gamma - 0.024824205786f32) / (0f32 - 0.274824202061f32), 1.0f32 / 0.449999988079f32)
    /
    (0f32 - 4.0f32)
  }
  else if gamma < 4.5f32 * 0.0180539693683f32
  { return gamma / 4.5f32 }
  else if gamma < 1.0f32
  { return Powf((gamma + 0.099296823144f32) / 1.09929680824f32, 1.0f32 / 0.449999988079f32) };
  return 1.0f32
}

pub fn ToLinearHlg(gamma: f32) -> f32
{
  if gamma < 0.0f32
  { return 0.0f32 }
  else if gamma <= 0.5f32 { return Powf(gamma * gamma * (1.0f32 / 3.0f32), 1.20000004768f32) };
  return
  Powf(
    (crate::math::expf((gamma - 0.559910714626f32) / 0.178832769394f32) + 0.284668922424f32)
    /
    12.0f32,
    1.20000004768f32
  )
}

pub fn ToLinearIec61966(gamma: f32) -> f32
{
  if gamma <= (0f32 - 4.5f32) * 0.0180539693683f32
  {
    return
    Powf(
      (0f32 - gamma + 0.099296823144f32) / (0f32 - 1.09929680824f32),
      1.0f32 / 0.449999988079f32
    )
  }
  else if gamma < 4.5f32 * 0.0180539693683f32 { return gamma / 4.5f32 };
  return Powf((gamma + 0.099296823144f32) / 1.09929680824f32, 1.0f32 / 0.449999988079f32)
}

pub fn ToLinearLog100(gamma: f32) -> f32
{
  let mid_interval: f32 = 0.00999999977648f32 / 2.0f32;
  return
  if gamma <= 0.0f32
  { mid_interval }
  else
  { Powf(10.0f32, 2.0f32 * (if gamma < 1.0f32 { gamma } else { 1.0f32 } - 1.0f32)) }
}

pub fn ToLinearLog100Sqrt10(gamma: f32) -> f32
{
  let mid_interval: f32 = 0.0031622776296f32 / 2.0f32;
  return
  if gamma <= 0.0f32
  { mid_interval }
  else
  { Powf(10.0f32, 2.5f32 * (if gamma < 1.0f32 { gamma } else { 1.0f32 } - 1.0f32)) }
}

pub fn ToLinearPq(gamma: f32) -> f32
{
  if gamma > 0.0f32
  {
    let pow_gamma: f32 = Powf(gamma, 32.0f32 / 2523.0f32);
    let num: f32 =
        if pow_gamma - 107.0f32 / 128.0f32 > 0.0f32
        { pow_gamma - 107.0f32 / 128.0f32 }
        else
        { 0.0f32 };
    let den: f32 =
        if 2413.0f32 / 128.0f32 - 2392.0f32 / 128.0f32 * pow_gamma > 1.17549435082e-38f32
        { 2413.0f32 / 128.0f32 - 2392.0f32 / 128.0f32 * pow_gamma }
        else
        { 1.17549435082e-38f32 };
    return Powf(num / den, 4096.0f32 / 653.0f32)
  };
  return 0.0f32
}

pub fn ToLinearSmpte240(gamma: f32) -> f32
{
  if gamma < 0.0f32
  { return 0.0f32 }
  else if gamma < 4.0f32 * 0.0228215847164f32
  { return gamma / 4.0f32 }
  else if gamma < 1.0f32
  { return Powf((gamma + 0.11157219857f32) / 1.11157214642f32, 1.0f32 / 0.449999988079f32) };
  return 1.0f32
}

pub fn ToLinearSmpte428(gamma: f32) -> f32
{
  return Powf(if gamma > 0.0f32 { gamma } else { 0.0f32 }, 2.59999990463f32) / 0.916555285454f32
}

pub fn ToLinearSrgb(v: u16, bit_depth: i32) -> u32
{
  let shift: i32 = 10i32.wrapping_sub(bit_depth);
  if shift > 0i32 { return unsafe { kGammaToLinearTabS[(v as i32).wrapping_shl(shift as u32) as usize] } };
  return FixedPointInterpolation(v as i32, unsafe { &kGammaToLinearTabS }, 0i32.wrapping_sub(shift), 0i32)
}

pub const kGammaF: f64 = 1.0f64 / 0.45f64;

pub static mut  kGammaTablesSOk: i32 = 0i32;

pub static mut kGammaToLinearTabS: [u32; 1026] = [0u32; 1026usize];

pub static mut kLinearToGammaTabS: [u32; 514] = [0u32; 514usize];
