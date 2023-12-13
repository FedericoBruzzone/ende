/*!
UCS-2 encoding and decoding.

UCS-2 (Universal Character Set 2) is a character encoding that represents each
character in the Unicode character set with a fixed-size 16-bit code unit.
It's important to note that UCS-2 is limited to the Basic Multilingual Plane
(BMP) of Unicode, which includes character codes from U+0000 to U+FFFF.
Characters outside this range require more than 16 bits and are represented
using UTF-16 instead.

*/

pub fn ucs2_encode<T: AsRef<Vec<u32>>>(v: T) -> Vec<u16> {
    let mut new_v: Vec<u16> = Vec::new();
    let v: Vec<u32> = v.as_ref().to_vec();
    for i in 0..v.len() {
        let mut code_point = v[i];
        if code_point > 0xFFFF {
            let extra = code_point - 0x10000;
            new_v.push((((extra >> 10) & 0x3FF) + 0xD800) as u16);
            code_point = 0xDC00 | code_point & 0x3FF;
        }
        new_v.push(code_point as u16);
    }
    new_v
}

pub fn ucs2_decode<T: AsRef<Vec<u16>>>(v: T) -> Vec<u32> {
    let mut new_v: Vec<u32> = Vec::new();
    let v: Vec<u16> = v.as_ref().to_vec();
    let mut i = 0;
    while i < v.len() {
        let code_point = v[i];
        if code_point >= 0xD800 && code_point <= 0xDBFF {
            let extra = v[i + 1];
            if (extra & 0xFC00) == 0xDC00 {
                new_v.push(((((code_point & 0x3FF) << 10) + (extra & 0x3FF)) as u32) + 0x10000);
                i += 1;
            } else {
                panic!("Invalid UCS-2 sequence");
            }
        } else {
            new_v.push(code_point as u32);
        }
        i += 1;
    }
    new_v
}
