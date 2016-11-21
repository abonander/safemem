//! Safe wrappers for memory-accessing functions like `std::ptr::copy()`.
use std::ptr;

macro_rules! idx_check (
    ($slice:expr, $idx:expr) => {
        assert!($idx < $slice.len(),
            concat!("`", stringify!($idx), "` ({}) out of bounds. Length: {}"),
            $idx, $slice.len());
    }
);

macro_rules! len_check (
    ($slice:expr, $start:ident, $len:ident) => {
        assert!(
            $start.checked_add($len)
                .expect(concat!("Overflow evaluating ", stringify!($start + $len)))
                <= $slice.len(),
            "Length {} starting at {} is out of bounds (slice len {}).", $len, $start, $slice.len()
        )
    }
);

/// Copy `len` elements from `src_idx` to `dest_idx`. Ranges may overlap.
///
/// Safe wrapper for `memmove()`/`std::ptr::copy()`.
///
/// ###Panics
/// * If either `src_idx` or `dest_idx` are out of bounds, or if either of these plus `len` is out of
/// bounds.
/// * If `src_idx + len` or `dest_idx + len` overflows.
pub fn copy<T: Copy>(slice: &mut [T], src_idx: usize, dest_idx: usize, len: usize) {
    idx_check!(slice, src_idx);
    idx_check!(slice, dest_idx);
    len_check!(slice, src_idx, len);
    len_check!(slice, dest_idx, len);

    let src_ptr: *const T = &slice[src_idx];
    let dest_ptr: *mut T = &mut slice[dest_idx];

    unsafe {
        ptr::copy(src_ptr, dest_ptr, len);
    }

}

/// Safe wrapper for `std::ptr::write_bytes()`/`memset()`.
pub fn write_bytes(slice: &mut [u8], byte: u8) {
    unsafe {
        ptr::write_bytes(slice.as_mut_ptr(), byte, slice.len());
    }
}

#[test]
#[should_panic]
fn test_bounds_check() {
    let mut arr = [0i32, 1, 2, 3, 4, 5];

    copy(&mut arr, 2, 1, 7);
}