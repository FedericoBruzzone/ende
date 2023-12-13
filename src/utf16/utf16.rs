/*!
UTF-16 encoding and decoding.

# Encoding
A unicode code point is represented using two or four bytes in UTF-16, depending its value.
* If the unicode code point is less than 0xFFFF, it is represented using [two bytes](#two-bytes).
* If the unicode code point is greater than or equal to 0xFFFF, it is represented using [four bytes](#four-bytes).

# Decoding
A UTF-16 code point is decoded into a unicode code point using the following rules.
* If the UTF-16 code point is less than 0xD800 or greater than 0xDBFF, it is a valid unicode code point.
* If the UTF-16 code point is between 0xD800 and 0xDBFF, it is a [surrogate pair](#surrogate-pair).
*/

fn encode_code_point(unicode_cp: u32) -> Vec<u16> {
    if unicode_cp < 0xFFFF {
        // unicode_cp: 0x0251 -> 0b0000_0010_0101_0001
        return vec![unicode_cp as u16];
    }

    let mut byte_vec: Vec<u16> = Vec::new();
    let extra = unicode_cp - 0x10000;
    byte_vec.push((((extra >> 10) & 0x3FF) + 0xD800) as u16);
    let unicode_cp = 0xDC00 | unicode_cp & 0x3FF;
    byte_vec.push(unicode_cp as u16);

    byte_vec
}

fn decode_symbol(utf16_cp: &Vec<u16>, i: usize) -> Option<(u32, usize)> {
    if i > utf16_cp.len() {
        panic!("Index out of bounds");
    }

    if i == utf16_cp.len() {
        return None;
    }

    let mut code_point: u32;
    let mut offset: usize = 0;

    code_point = utf16_cp[i] as u32;
    offset += 1;
    if code_point < 0xD800 || code_point > 0xDBFF {
        return Some((code_point as u32, offset));
    }

    let extra = utf16_cp[i + 1] as u32;
    offset += 1;
    if (extra & 0xFC00) == 0xDC00 {
        code_point = (((code_point & 0x3FF) << 10) + (extra & 0x3FF)) + 0x10000;
        return Some((code_point, offset));
    } else {
        panic!("Invalid UCS-2 sequence");
    }
}

// ============================================================================
// ================================ Public API ================================
// ============================================================================

pub fn encode_in_utf16<T: AsRef<Vec<u32>>>(unicode_cp: T) -> Vec<u16> {
    let unicode_cp: Vec<u32> = unicode_cp.as_ref().to_vec();
    let len: usize = unicode_cp.len();
    let mut utf16_cp: Vec<u16> = Vec::new();
    for i in 0..len {
        let cp = unicode_cp[i];
        utf16_cp.append(&mut encode_code_point(cp));
    }
    utf16_cp
}

pub fn decode_from_utf16<T: AsRef<Vec<u16>>>(utf16_cp: T) -> Vec<u32> {
    let utf16_cp: Vec<u16> = utf16_cp.as_ref().to_vec();
    let len: usize = utf16_cp.len();
    let mut i = 0;
    let mut unicode_cp: Vec<u32> = Vec::new();
    while i < len {
        let (cp, offset) = decode_symbol(&utf16_cp, i).unwrap();
        i += offset;
        unicode_cp.push(cp);

        // let code_point = utf16_cp[i];
        // if code_point >= 0xD800 && code_point <= 0xDBFF {
        //     let extra = utf16_cp[i + 1];
        //     if (extra & 0xFC00) == 0xDC00 {
        //         unicode_cp.push(((((code_point & 0x3FF) << 10) + (extra & 0x3FF)) as u32) + 0x10000);
        //         i += 1;
        //     } else {
        //         panic!("Invalid UCS-2 sequence");
        //     }
        // } else {
        //     unicode_cp.push(code_point as u32);
        // }
        // i += 1;
    }
    unicode_cp
}
