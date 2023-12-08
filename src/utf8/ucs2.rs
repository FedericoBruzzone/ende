// UCS-2 is a character encoding standard in which characters are represented
// by a fixed-length 16 bits (2 bytes).
// UCS-2 represents a possible maximum of 65,536 characters,
// or in hexadecimals from 0000h - FFFFh (2 bytes).

use std::str;

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

pub fn ucs2_encode<T: AsRef<Vec<u32>>>(_v: T) -> String {
    "".to_string()
}
