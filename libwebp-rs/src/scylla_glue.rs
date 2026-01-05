// HAND-WRITTEN FILE

pub(crate)
fn scylla_u8_of_u32(x: & [u32]) -> & [u8] {
    unsafe {
        std::slice::from_raw_parts((&raw const *x).cast::<u8>(), x.len()*4)
    }
}

pub(crate)
fn scylla_u8_of_u32_mut(x: & mut [u32]) -> & mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u8>(), x.len()*4)
    }
}

pub(crate)
fn scylla_u16_of_u32(x: & [u32]) -> & [u16] {
    unsafe {
        std::slice::from_raw_parts((&raw const *x).cast::<u16>(), x.len()*2)
    }
}

pub(crate)
fn scylla_u16_of_u32_mut(x: & mut [u32]) -> & mut [u16] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u16>(), x.len()*2)
    }
}

pub(crate)
fn scylla_u16_of_u8(x: & [u8]) -> & [u16] {
    unsafe {
        std::slice::from_raw_parts((&raw const *x).cast::<u16>(), x.len()/2)
    }
}

pub(crate)
fn scylla_u16_of_u8_mut(x: & mut [u8]) -> & mut [u16] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u16>(), x.len()/2)
    }
}

pub(crate)
fn scylla_i16_of_u16_mut(x: & mut [u16]) -> & mut [i16] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<i16>(), x.len())
    }
}
