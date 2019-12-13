use std::any::type_name;
use std::mem;
use zerocopy::{AsBytes, FromBytes, LayoutVerified};

pub fn to_byte_vec<T: AsBytes>(mut v: Vec<T>) -> Vec<u8> {
    if v.is_empty() {
        return Vec::new();
    }
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let size = mem::size_of::<T>();

    unsafe {
        mem::forget(v);
        Vec::from_raw_parts(p as *mut u8, len * size, cap * size)
    }
}

pub fn from_byte_vec<T: FromBytes>(mut v: Vec<u8>) -> Result<Vec<T>, String> {
    let size = mem::size_of::<T>();
    if size == 0 {
        return Err(format!(
            "Cannot construct size 0 vector (type {}) from bytes.",
            type_name::<T>()
        ));
    }
    if v.is_empty() {
        return Ok(Vec::new());
    }
    let len = v.len();
    let cap = v.capacity();

    if cap % size != 0 {
        return Err(format!(
            "Byte vector capacity {} must be a multiple of {} size: {}",
            cap,
            type_name::<T>(),
            size
        ));
    }

    LayoutVerified::<&[u8], [T]>::new_slice(v.as_slice()).ok_or_else(|| {
        format!(
            "Bytes are not sized or aligned for conversion to {}",
            type_name::<T>()
        )
    })?;

    let p = v.as_mut_ptr();
    unsafe {
        mem::forget(v);
        Ok(Vec::from_raw_parts(p as *mut T, len / size, cap / size))
    }
}
