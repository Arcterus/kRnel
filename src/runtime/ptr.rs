#[author = "Arcterus"];
#[license = "MPL v2.0"];

use runtime::zero::{Eq};

impl<T> Eq for *const T {
	#[inline(always)]
	fn eq(&self, other: &*const T) -> bool {
		(*self as uint) == (*other as uint)
	}

	#[inline(always)]
	fn ne(&self, other: &*const T) -> bool {
		!self.eq(other)
	}
}

/*impl<T> *T {
	pub fn is_null<T>(&const self) -> bool {
		to_const_unsafe_ptr(self) == null()
	}
}*/

#[inline(always)]
pub fn null<T>() -> *T {
	0 as *T
}

#[inline(always)]
pub fn mut_null<T>() -> *mut T {
	0 as *mut T
}

#[inline(always)]
pub fn is_null<T>(ptr: *const T) -> bool {
	ptr == null()
}

#[inline(always)]
pub fn is_not_null<T>(ptr: *const T) -> bool {
	!is_null(ptr)
}

#[inline(always)]
pub fn to_const_unsafe_ptr<T>(thing: &const T) -> *const T {
	thing as *const T
}

#[inline(always)]
pub fn to_mut_unsafe_ptr<T>(thing: &mut T) -> *mut T {
	thing as *mut T
}

#[inline(always)]
pub fn to_unsafe_ptr<T>(thing: &T) -> *T {
	thing as *T
}