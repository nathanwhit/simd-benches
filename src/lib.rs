use rand::RngCore;

pub fn rand_bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0; len];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}

pub type FnGroup<T> = [(&'static str, T)];

pub fn map_collect<T, U, C>(iter: impl IntoIterator<Item = T>, f: impl FnMut(T) -> U) -> C
where
    C: FromIterator<U>,
{
    iter.into_iter().map(f).collect()
}

pub mod faster_hex {
    #[inline]
    pub fn hex_check(src: &[u8]) -> bool {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if cfg!(target_feature = "sse4.1") {
                return unsafe { ::faster_hex::hex_check_sse(src) };
            }

            #[cfg(feature = "detect")]
            if is_x86_feature_detected!("sse4.1") {
                return unsafe { ::faster_hex::hex_check_sse(src) };
            }
        }
        ::faster_hex::hex_check_fallback(src)
    }
}

pub fn std_utf8_to_utf16(src: &str, dst: &mut [u16]) -> usize {
    assert!(src.len() <= dst.len() / 2);

    let mut count = 0;
    for (x, y) in src.encode_utf16().zip(dst.iter_mut()) {
        *y = x;
        count += 1;
    }
    count
}

pub fn simdutf_utf8_to_utf16(src: &str, dst: &mut [u16]) -> usize {
    assert!(src.len() <= dst.len() / 2);

    let len = src.len();
    let src = src.as_ptr();
    let dst = dst.as_mut_ptr();
    unsafe { simdutf::convert_valid_utf8_to_utf16(src, len, dst) }
}
