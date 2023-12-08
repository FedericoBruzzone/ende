// UCS-2 (Universal Character Set 2) is a character encoding that represents each
// character in the Unicode character set with a fixed-size 16-bit code unit.
// It's important to note that UCS-2 is limited to the Basic Multilingual Plane
// (BMP) of Unicode, which includes character codes from U+0000 to U+FFFF.
// Characters outside this range require more than 16 bits and are represented
// using UTF-16 instead.

use std::str;

/// Decodes a UCS-2 string into a vector of code points.
pub fn ucs2_decode<T: AsRef<str>>(s: T) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let mut iter: str::Chars = s.as_ref().chars();
    let len = iter.clone().count();
    for _ in 0..len {
        let c = iter.next().unwrap();
        if c as u32 >= 0xD800 && c as u32 <= 0xDBFF {
            let extra = iter.next().unwrap();
            if (extra as u32 & 0xFC00) == 0xDC00 {
                v.push(((c as u32 & 0x3FF) << 10) + (extra as u32 & 0x3FF) + 0x10000);
            } else {
                panic!("Invalid UCS-2 sequence");
            }
        } else {
            v.push(c as u32);
        }
    }
    v
}

/// Encodes a UTF-8 byte vector into a UCS-2 string.
pub fn ucs2_encode<T: AsRef<Vec<u32>>>(v: T) -> String {
    let mut s: String = String::new();
    let v: Vec<u32> = v.as_ref().to_vec();
    for i in 0..v.len() {
        let mut code_point = v[i];
        if code_point > 0xFFFF {
            let extra = code_point - 0x10000;
            s.push((((extra >> 10) & 0x3FF) + 0xD800) as u8 as char);
            code_point = 0xDC00 | code_point & 0x3FF;
        }
        s.push(code_point as u8 as char);
    }
    s
}
