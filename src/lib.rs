use std::mem;
use zerocopy::{AsBytes, FromBytes};

pub fn to_byte_vec<T: AsBytes>(mut v: Vec<T>) -> Vec<u8> {
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
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let size = mem::size_of::<T>();

    if len % size != 0 {
        return Err(format!(
            "Byte vector {} {} must be a multiple of {}",
            "length", len, size
        ));
    }
    if cap % size != 0 {
        return Err(format!(
            "Byte vector {} {} must be a multiple of {}",
            "capacity", cap, size
        ));
    }

    #[allow(clippy::cast_ptr_alignment)]
    unsafe {
        mem::forget(v);
        Ok(Vec::from_raw_parts(p as *mut T, len / size, cap / size))
    }
}
