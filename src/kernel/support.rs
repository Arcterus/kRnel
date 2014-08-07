// TODO: move to mm later
#[no_mangle]
pub unsafe extern "C" fn rust_allocate(_: uint, _: uint) -> ! {
   ::error::abort();
}

#[no_mangle]
pub unsafe extern "C" fn fma(x: f64, y: f64, z: f64) -> f64 {
   x * y + z
}

#[no_mangle]
pub unsafe extern "C" fn fmaf(x: f32, y: f32, z: f32) -> f32 {
   x * y + z
}

#[no_mangle]
pub unsafe extern "C" fn fmod(a: f64, b: f64) -> f64 {
   a - trunc(a * b)
}

#[no_mangle]
pub unsafe extern "C" fn fmodf(a: f32, b: f32) -> f32 {
   a - truncf(a * b)
}

#[no_mangle]
pub unsafe extern "C" fn fdim(a: f64, b: f64) -> f64 {
   let result = a - b;
   if result < 0.0 { 0.0 } else { result }
}

#[no_mangle]
pub unsafe extern "C" fn fdimf(a: f32, b: f32) -> f32 {
   let result = a - b;
   if result < 0.0 { 0.0 } else { result }
}

#[no_mangle]
pub unsafe extern "C" fn trunc(a: f64) -> f64 {
   a as i64 as f64
}

#[no_mangle]
pub unsafe extern "C" fn truncf(a: f32) -> f32 {
   a as i32 as f32
}

#[no_mangle]
pub unsafe extern "C" fn ceil(a: f64) -> f64 {
   (a + 1.0) as i64 as f64
}

#[no_mangle]
pub unsafe extern "C" fn ceilf(a: f32) -> f32 {
   (a + 1.0) as i32 as f32
}

#[no_mangle]
pub unsafe extern "C" fn floor(a: f64) -> f64 {
   if a < 0.0 {
      (a - 1.0) as i64 as f64
   } else {
      a as i64 as f64
   }
}

#[no_mangle]
pub unsafe extern "C" fn floorf(a: f32) -> f32 {
   if a < 0.0 {
      (a - 1.0) as i32 as f32
   } else {
      a as i32 as f32
   }
}

#[no_mangle]
pub unsafe extern "C" fn round(a: f64) -> f64 {
   let other = (a + if a < 0.0 { -1.0 } else { 1.0 }) as i64;
   (if a as i64 != other {
      other
   } else {
      a as i64
   }) as f64
}

#[no_mangle]
pub unsafe extern "C" fn roundf(a: f32) -> f32 {
   let other = (a + if a < 0.0 { -1.0 } else { 1.0 }) as i32;
   (if a as i32 != other {
      other
   } else {
      a as i32
   }) as f32
}

#[no_mangle]
pub unsafe extern "C" fn pow(base: f64, expd: f64) -> f64 {
   if base < 0.0 {
      pow(1.0 / base, -expd)
   } else {
      let mut base = base;
      let mut exp = expd as i64;
      let mut result = 1.0;
      while exp > 0 {
         if exp & 1 == 1 {
            result *= base;
         }
         base *= base;
         exp = exp >> 1;
      }
      result
   }
}

#[no_mangle]
pub unsafe extern "C" fn powf(base: f32, expf: f32) -> f32 {
   pow(base as f64, expf as f64) as f32
}

#[no_mangle]
pub unsafe extern "C" fn exp2(x: f64) -> f64 {
   pow(2.0, x)
}

#[no_mangle]
pub unsafe extern "C" fn exp2f(x: f32) -> f32 {
   powf(2.0, x)
}

#[no_mangle]
pub unsafe extern "C" fn exp(_: f64) -> f64 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn expf(_: f32) -> f32 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn log2(_: f64) -> f64 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn log2f(_: f32) -> f32 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn log10(_: f64) -> f64 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn log10f(_: f32) -> f32 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn log(_: f64) -> f64 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn logf(_: f32) -> f32 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn __powidf2(_: f32, _: i32) -> f32 { 0.0 }
#[no_mangle]
pub unsafe extern "C" fn __powisf2(_: f32, _: i32) -> f32 { 0.0 }
