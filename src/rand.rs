extern "C" {
    fn rand(ptr: *mut u8, len: usize) -> usize;
}

pub enum RNGSourceError {
    RangeError,
    QuotaError,
    UnknownError,
}

pub fn fill_random(buf: &mut [u8]) -> Result<(), RNGSourceError> {
    let rv = unsafe { rand(buf.as_mut_ptr(), buf.len()) };
    match rv {
        0 => Ok(()),
        1 => Err(RNGSourceError::RangeError),
        2 => Err(RNGSourceError::QuotaError),
        _ => Err(RNGSourceError::UnknownError),
    }
}

const TWO_POW_64: f64 = 1.8446744073709552e19;

pub fn random_f64_array(buf: &mut [f64]) -> Result<(), RNGSourceError> {
    let mut tmp = vec![0u64; buf.len()];
    unsafe {
        let mem = ::std::slice::from_raw_parts_mut(tmp.as_mut_ptr() as *mut u8, tmp.len() * 8);
        fill_random(mem)?;
    };
    for (src, dst) in tmp.iter().zip(buf.iter_mut()) {
        *dst = (*src as f64) / TWO_POW_64;
    }
    Ok(())
}

pub fn random_f64() -> Result<f64, RNGSourceError> {
    let mut buf = [0u8; 8];
    fill_random(&mut buf)?;
    let num = unsafe { ::std::mem::transmute::<[u8; 8], u64>(buf) };
    Ok((num as f64) / TWO_POW_64)
}
