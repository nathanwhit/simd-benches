use rand::RngCore;
use widestring::Utf16Str;

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

pub fn wikipedia_mars() -> Vec<(String, String)> {
    let dir = std::fs::read_dir("dataset/wikipedia_mars").unwrap();
    let mut ans = Vec::new();
    for entry in dir {
        let entry = entry.unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        let content = std::fs::read_to_string(entry.path()).unwrap();
        ans.push((name, content));
    }
    ans.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    ans
}

// ----------------------------

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

// ----------------------------

pub fn simdutf_utf16_to_utf8(src: &Utf16Str, dst: &mut [u8]) -> usize {
    assert!(src.len() <= dst.len() / 4);

    let len = src.len();
    let src = src.as_ptr();
    let dst = dst.as_mut_ptr();
    unsafe { simdutf::convert_valid_utf16_to_utf8(src, len, dst) }
}

#[cfg(target_endian = "little")]
pub fn encoding_rs_utf16_to_utf8(src: &Utf16Str, dst: &mut [u8]) -> usize {
    assert!(src.len() <= dst.len() / 4);

    // Is this optimal?

    let bytes: &[u8] = bytemuck::cast_slice(src.as_slice());
    let mut utf16le = encoding_rs::UTF_16LE.new_decoder();
    let (_, _, written) = utf16le.decode_to_utf8_without_replacement(bytes, dst, true);
    written
}

pub fn widestring_utf16_to_utf8(src: &Utf16Str, dst: &mut [u8]) -> usize {
    assert!(src.len() <= dst.len() / 4);

    let mut count = 0;
    for (x, y) in src.encode_utf8().zip(dst.iter_mut()) {
        *y = x;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    use widestring::Utf16String;

    #[test]
    fn utf16_to_utf8() {
        for (name, content) in wikipedia_mars() {
            let utf16 = Utf16String::from(content.as_str());
            assert!(simdutf::validate_utf16(utf16.as_slice()));

            let mut dst = vec![0; utf16.len() * 4];

            let written = simdutf_utf16_to_utf8(&utf16, &mut dst);
            assert!(written > 0);

            let ans = &dst[..written];
            assert!(ans == content.as_bytes(), "{name} failed");
        }
    }
}
